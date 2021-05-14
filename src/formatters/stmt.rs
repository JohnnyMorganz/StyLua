#[cfg(feature = "lua52")]
use crate::formatters::lua52::{format_goto, format_label};
#[cfg(feature = "luau")]
use crate::formatters::luau::{
    format_compound_assignment, format_exported_type_declaration, format_type_declaration_stmt,
    format_type_specifier,
};
use crate::{
    check_should_format,
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        assignment::{format_assignment, format_local_assignment},
        expression::{format_expression, hang_expression_trailing_newline},
        functions::{format_function_call, format_function_declaration, format_local_function},
        general::{
            format_end_token, format_punctuated_buffer, format_token_reference,
            format_token_reference_mut, EndTokenType,
        },
        trivia::{
            strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util,
        util::{prefix_range, token_range},
    },
    shape::Shape,
};
use full_moon::ast::{
    Do, ElseIf, Expression, FunctionCall, GenericFor, If, NumericFor, Repeat, Stmt, Value, While,
};
use full_moon::node::Node;
use full_moon::tokenizer::{Token, TokenReference, TokenType};

macro_rules! fmt_stmt {
    ($ctx:expr, $value:ident, $shape:ident, { $($(#[$inner:meta])* $operator:ident = $output:ident,)+ }) => {
        match $value {
            $(
                $(#[$inner])*
                Stmt::$operator(stmt) => Stmt::$operator($output($ctx, stmt, $shape)),
            )+
            other => panic!("unknown node {:?}", other),
        }
    };
}

/// Removes parentheses around a condition, if present.
/// Called only for condition expression (if ... then, while ... do, etc.)
fn remove_condition_parentheses(expression: Expression) -> Expression {
    match expression.to_owned() {
        Expression::Parentheses { expression, .. } => *expression,
        Expression::Value { value, .. } => match *value {
            Value::ParenthesesExpression(expression) => remove_condition_parentheses(expression),
            _ => expression,
        },
        _ => expression,
    }
}

/// Format a Do node
pub fn format_do_block<'ast>(ctx: &Context, do_block: &Do<'ast>, _shape: Shape) -> Do<'ast> {
    // Create trivia
    let additional_indent_level = ctx.get_range_indent_increase(token_range(do_block.do_token()));
    let leading_trivia =
        FormatTriviaType::Append(vec![create_indent_trivia(ctx, additional_indent_level)]);
    let trailing_trivia = FormatTriviaType::Append(vec![create_newline_trivia(ctx)]);

    let do_token = fmt_symbol!(ctx, do_block.do_token(), "do")
        .update_trivia(leading_trivia.to_owned(), trailing_trivia.to_owned());
    let end_token = format_end_token(ctx, do_block.end_token(), EndTokenType::BlockEnd)
        .update_trivia(leading_trivia, trailing_trivia);

    do_block
        .to_owned()
        .with_do_token(do_token)
        .with_end_token(end_token)
}

/// Format a GenericFor node
pub fn format_generic_for<'ast>(
    ctx: &mut Context,
    generic_for: &GenericFor<'ast>,
    shape: Shape,
) -> GenericFor<'ast> {
    // Create trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(generic_for.for_token()));
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let mut trailing_trivia = vec![create_newline_trivia(ctx)];

    // TODO: Should we actually update the shape here?
    let for_token = fmt_symbol!(ctx, generic_for.for_token(), "for ")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));
    let (formatted_names, mut names_comments_buf) =
        format_punctuated_buffer(ctx, generic_for.names(), shape, format_token_reference_mut);

    #[cfg(feature = "luau")]
    let type_specifiers = generic_for
        .type_specifiers()
        .map(|x| match x {
            Some(type_specifier) => Some(format_type_specifier(ctx, type_specifier)),
            None => None,
        })
        .collect();

    let in_token = fmt_symbol!(ctx, generic_for.in_token(), " in ");
    let (formatted_expr_list, mut expr_comments_buf) =
        format_punctuated_buffer(ctx, generic_for.expressions(), shape, format_expression);

    // Create comments buffer and append to end of do token
    names_comments_buf.append(&mut expr_comments_buf);
    // Append trailing trivia to the end
    names_comments_buf.append(&mut trailing_trivia);

    let do_token = fmt_symbol!(ctx, generic_for.do_token(), " do")
        .update_trailing_trivia(FormatTriviaType::Append(names_comments_buf));

    let end_token = format_end_token(ctx, generic_for.end_token(), EndTokenType::BlockEnd)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(vec![create_newline_trivia(ctx)]), // trailing_trivia was emptied when it was appended to names_comment_buf
        );

    let generic_for = generic_for
        .to_owned()
        .with_for_token(for_token)
        .with_names(formatted_names)
        .with_in_token(in_token)
        .with_expressions(formatted_expr_list)
        .with_do_token(do_token)
        .with_end_token(end_token);
    #[cfg(feature = "luau")]
    let generic_for = generic_for.with_type_specifiers(type_specifiers);
    generic_for
}

/// Formats an ElseIf node - This must always reside within format_if
fn format_else_if<'ast>(
    ctx: &mut Context,
    else_if_node: &ElseIf<'ast>,
    shape: Shape,
) -> ElseIf<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(else_if_node.else_if_token()));
    let shape = shape
        .reset()
        .with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(else_if_node.condition().to_owned());

    // Determine if we need to hang the condition
    let singleline_shape = shape + (7 + 5 + strip_trivia(&condition).to_string().len()); // 7 = "elseif ", 5 = " then"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::expression_contains_inline_comments(&condition);

    let (else_if_trailing_trivia, then_text) = if require_multiline_expression {
        (vec![create_newline_trivia(ctx)], "then")
    } else {
        (vec![Token::new(TokenType::spaces(1))], " then")
    };

    let formatted_else_if_token =
        format_end_token(ctx, else_if_node.else_if_token(), EndTokenType::BlockEnd)
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()))
            .update_trailing_trivia(FormatTriviaType::Append(else_if_trailing_trivia));

    let formatted_condition = if require_multiline_expression {
        // Add the expression list into the indent range, as it will be indented by one
        let expr_range = else_if_node
            .condition()
            .range()
            .expect("no range for else if condition");
        ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

        let indent_level = Some(additional_indent_level.unwrap_or(0) + 1);
        let shape = shape.reset().with_additional_indent(indent_level);
        hang_expression_trailing_newline(ctx, &condition, shape, additional_indent_level, None)
            .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                ctx,
                indent_level,
            )]))
    } else {
        format_expression(ctx, &condition, shape + 7) // 7 = "elseif "
    };

    let formatted_then_token = fmt_symbol!(ctx, else_if_node.then_token(), then_text)
        .update_trivia(
            if require_multiline_expression {
                FormatTriviaType::Append(leading_trivia)
            } else {
                FormatTriviaType::NoChange
            },
            FormatTriviaType::Append(trailing_trivia),
        );

    else_if_node
        .to_owned()
        .with_else_if_token(formatted_else_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
}

/// Format an If node
pub fn format_if<'ast>(ctx: &mut Context, if_node: &If<'ast>, shape: Shape) -> If<'ast> {
    // Calculate trivia
    let additional_indent_level = ctx.get_range_indent_increase(token_range(if_node.if_token()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(if_node.condition().to_owned());

    // Determine if we need to hang the condition
    let singleline_shape = shape + (3 + 5 + strip_trivia(&condition).to_string().len()); // 3 = "if ", 5 = " then"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::expression_contains_inline_comments(&condition);

    let (if_text, then_text) = if require_multiline_expression {
        ("if\n", "then")
    } else {
        ("if ", " then")
    };

    let formatted_if_token = fmt_symbol!(ctx, if_node.if_token(), if_text)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    let formatted_condition = if require_multiline_expression {
        // Add the expression list into the indent range, as it will be indented by one
        let expr_range = if_node
            .condition()
            .range()
            .expect("no range for if condition");
        ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

        let indent_level = Some(additional_indent_level.unwrap_or(0) + 1);
        let shape = shape.reset().with_additional_indent(indent_level);
        hang_expression_trailing_newline(ctx, &condition, shape, additional_indent_level, None)
            .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                ctx,
                indent_level,
            )]))
    } else {
        format_expression(ctx, &condition, shape + 3) // 3 = "if "
    };

    let formatted_then_token = fmt_symbol!(ctx, if_node.then_token(), then_text).update_trivia(
        if require_multiline_expression {
            FormatTriviaType::Append(leading_trivia.to_owned())
        } else {
            FormatTriviaType::NoChange
        },
        FormatTriviaType::Append(trailing_trivia.to_owned()),
    );
    let formatted_end_token = format_end_token(ctx, if_node.end_token(), EndTokenType::BlockEnd)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );

    let formatted_else_if = if_node.else_if().map(|else_if| {
        else_if
            .iter()
            .map(|else_if| format_else_if(ctx, else_if, shape))
            .collect()
    });

    let formatted_else_token = match if_node.else_token() {
        Some(token) => {
            let formatted = format_end_token(ctx, token, EndTokenType::BlockEnd).update_trivia(
                FormatTriviaType::Append(leading_trivia),
                FormatTriviaType::Append(trailing_trivia),
            );
            Some(formatted)
        }
        None => None,
    };

    if_node
        .to_owned()
        .with_if_token(formatted_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
        .with_else_if(formatted_else_if)
        .with_else_token(formatted_else_token)
        .with_end_token(formatted_end_token)
}

/// Format a NumericFor node
pub fn format_numeric_for<'ast>(
    ctx: &mut Context,
    numeric_for: &NumericFor<'ast>,
    shape: Shape,
) -> NumericFor<'ast> {
    // Create trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(numeric_for.for_token()));
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let for_token = fmt_symbol!(ctx, numeric_for.for_token(), "for ")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));
    let formatted_index_variable = format_token_reference(ctx, numeric_for.index_variable());

    #[cfg(feature = "luau")]
    let type_specifier = match numeric_for.type_specifier() {
        Some(type_specifier) => Some(format_type_specifier(ctx, type_specifier)),
        None => None,
    };

    // TODO: Should we actually update the shape here?
    let equal_token = fmt_symbol!(ctx, numeric_for.equal_token(), " = ");
    let formatted_start_expression = format_expression(ctx, numeric_for.start(), shape);
    let start_end_comma = fmt_symbol!(ctx, numeric_for.start_end_comma(), ", ");
    let formatted_end_expression = format_expression(ctx, numeric_for.end(), shape);

    let (end_step_comma, formatted_step_expression) = match numeric_for.step() {
        Some(step) => (
            Some(fmt_symbol!(
                ctx,
                numeric_for.end_step_comma().unwrap(),
                ", "
            )),
            Some(format_expression(ctx, step, shape)),
        ),
        None => (None, None),
    };

    let do_token = fmt_symbol!(ctx, numeric_for.do_token(), " do")
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));
    let end_token = format_end_token(ctx, numeric_for.end_token(), EndTokenType::BlockEnd)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(trailing_trivia),
        );

    let numeric_for = numeric_for
        .to_owned()
        .with_for_token(for_token)
        .with_index_variable(formatted_index_variable)
        .with_equal_token(equal_token)
        .with_start(formatted_start_expression)
        .with_start_end_comma(start_end_comma)
        .with_end(formatted_end_expression)
        .with_end_step_comma(end_step_comma)
        .with_step(formatted_step_expression)
        .with_do_token(do_token)
        .with_end_token(end_token);
    #[cfg(feature = "luau")]
    let numeric_for = numeric_for.with_type_specifier(type_specifier);

    numeric_for
}

/// Format a Repeat node
pub fn format_repeat_block<'ast>(
    ctx: &mut Context,
    repeat_block: &Repeat<'ast>,
    shape: Shape,
) -> Repeat<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(repeat_block.repeat_token()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let repeat_token = fmt_symbol!(ctx, repeat_block.repeat_token(), "repeat").update_trivia(
        FormatTriviaType::Append(leading_trivia.to_owned()),
        FormatTriviaType::Append(trailing_trivia.to_owned()),
    );
    let until_token = fmt_symbol!(ctx, repeat_block.until_token(), "until ")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(repeat_block.until().to_owned());

    // Determine if we need to hang the condition
    let singleline_shape = shape + (6 + strip_trivia(&condition).to_string().len()); // 6 = "until "
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::expression_contains_inline_comments(&condition);

    let shape = shape + 6; // 6 = "until "
    let formatted_until = match require_multiline_expression {
        true => {
            // Add the expression list into the indent range, as it will be indented by one
            let expr_range = repeat_block
                .until()
                .range()
                .expect("no range for repeat until");
            ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));
            hang_expression_trailing_newline(ctx, &condition, shape, additional_indent_level, None)
        }
        false => format_expression(ctx, &condition, shape)
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia)),
    };

    repeat_block
        .to_owned()
        .with_repeat_token(repeat_token)
        .with_until_token(until_token)
        .with_until(formatted_until)
}

/// Format a While node
pub fn format_while_block<'ast>(
    ctx: &mut Context,
    while_block: &While<'ast>,
    shape: Shape,
) -> While<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(while_block.while_token()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(while_block.condition().to_owned());

    // Determine if we need to hang the condition
    let singleline_shape = shape + (6 + 3 + strip_trivia(&condition).to_string().len()); // 6 = "while ", 3 = " do"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::expression_contains_inline_comments(&condition);

    let (while_text, do_text) = if require_multiline_expression {
        ("while\n", "do")
    } else {
        ("while ", " do")
    };

    let while_token = fmt_symbol!(ctx, while_block.while_token(), while_text)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    let formatted_condition = if require_multiline_expression {
        // Add the expression list into the indent range, as it will be indented by one
        let expr_range = while_block
            .condition()
            .range()
            .expect("no range for while condition");
        ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

        let indent_level = Some(additional_indent_level.unwrap_or(0) + 1);
        let shape = shape.reset().with_additional_indent(indent_level);
        hang_expression_trailing_newline(ctx, &condition, shape, additional_indent_level, None)
            .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                ctx,
                indent_level,
            )]))
    } else {
        format_expression(ctx, &condition, shape + 6) // 6 = "while "
    };

    let do_token = fmt_symbol!(ctx, while_block.do_token(), do_text).update_trivia(
        if require_multiline_expression {
            FormatTriviaType::Append(leading_trivia.to_owned())
        } else {
            FormatTriviaType::NoChange
        },
        FormatTriviaType::Append(trailing_trivia.to_owned()),
    );

    let end_token = format_end_token(ctx, while_block.end_token(), EndTokenType::BlockEnd)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(trailing_trivia),
        );

    while_block
        .to_owned()
        .with_while_token(while_token)
        .with_condition(formatted_condition)
        .with_do_token(do_token)
        .with_end_token(end_token)
}

/// Wrapper around `format_function_call`, but also handles adding the trivia around the function call.
/// This can't be done in the original function, as function calls are not always statements, but can also be
/// in expressions.
pub fn format_function_call_stmt<'ast>(
    ctx: &mut Context,
    function_call: &FunctionCall<'ast>,
    shape: Shape,
) -> FunctionCall<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(prefix_range(function_call.prefix()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    format_function_call(ctx, function_call, shape).update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(trailing_trivia),
    )
}

pub fn format_stmt<'ast>(ctx: &mut Context, stmt: &Stmt<'ast>, shape: Shape) -> Stmt<'ast> {
    check_should_format!(ctx, stmt);

    fmt_stmt!(ctx, stmt, shape, {
        Assignment = format_assignment,
        Do = format_do_block,
        FunctionCall = format_function_call_stmt,
        FunctionDeclaration = format_function_declaration,
        GenericFor = format_generic_for,
        If = format_if,
        LocalAssignment = format_local_assignment,
        LocalFunction = format_local_function,
        NumericFor = format_numeric_for,
        Repeat = format_repeat_block,
        While = format_while_block,
        #[cfg(feature = "luau")] CompoundAssignment = format_compound_assignment,
        #[cfg(feature = "luau")] ExportedTypeDeclaration = format_exported_type_declaration,
        #[cfg(feature = "luau")] TypeDeclaration = format_type_declaration_stmt,
        #[cfg(feature = "lua52")] Goto = format_goto,
        #[cfg(feature = "lua52")] Label = format_label,
    })
}
