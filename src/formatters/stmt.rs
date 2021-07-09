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
        block::format_block,
        expression::{format_expression, hang_expression_trailing_newline},
        functions::{format_function_call, format_function_declaration, format_local_function},
        general::{
            format_end_token, format_punctuated_buffer, format_token_reference, EndTokenType,
        },
        trivia::{
            strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util,
    },
    shape::Shape,
};
use full_moon::ast::{
    Do, ElseIf, Expression, FunctionCall, GenericFor, If, NumericFor, Repeat, Stmt, Value, While,
};
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
pub fn format_do_block(ctx: &Context, do_block: &Do, shape: Shape) -> Do {
    // Create trivia
    let leading_trivia = FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]);
    let trailing_trivia = FormatTriviaType::Append(vec![create_newline_trivia(ctx)]);

    let do_token = fmt_symbol!(ctx, do_block.do_token(), "do", shape)
        .update_trivia(leading_trivia.to_owned(), trailing_trivia.to_owned());
    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, do_block.block(), block_shape);
    let end_token = format_end_token(ctx, do_block.end_token(), EndTokenType::BlockEnd, shape)
        .update_trivia(leading_trivia, trailing_trivia);

    do_block
        .to_owned()
        .with_do_token(do_token)
        .with_block(block)
        .with_end_token(end_token)
}

/// Format a GenericFor node
pub fn format_generic_for(ctx: &Context, generic_for: &GenericFor, shape: Shape) -> GenericFor {
    // Create trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let mut trailing_trivia = vec![create_newline_trivia(ctx)];

    // TODO: Should we actually update the shape here?
    let for_token = fmt_symbol!(ctx, generic_for.for_token(), "for ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));
    let (formatted_names, mut names_comments_buf) =
        format_punctuated_buffer(ctx, generic_for.names(), shape, format_token_reference);

    #[cfg(feature = "luau")]
    let type_specifiers = generic_for
        .type_specifiers()
        .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
        .collect();

    let in_token = fmt_symbol!(ctx, generic_for.in_token(), " in ", shape);
    let (formatted_expr_list, mut expr_comments_buf) =
        format_punctuated_buffer(ctx, generic_for.expressions(), shape, format_expression);

    // Create comments buffer and append to end of do token
    names_comments_buf.append(&mut expr_comments_buf);
    // Append trailing trivia to the end
    names_comments_buf.append(&mut trailing_trivia);

    let do_token = fmt_symbol!(ctx, generic_for.do_token(), " do", shape)
        .update_trailing_trivia(FormatTriviaType::Append(names_comments_buf));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, generic_for.block(), block_shape);

    let end_token = format_end_token(ctx, generic_for.end_token(), EndTokenType::BlockEnd, shape)
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
        .with_block(block)
        .with_end_token(end_token);
    #[cfg(feature = "luau")]
    let generic_for = generic_for.with_type_specifiers(type_specifiers);
    generic_for
}

/// Formats an ElseIf node - This must always reside within format_if
fn format_else_if(ctx: &Context, else_if_node: &ElseIf, shape: Shape) -> ElseIf {
    // Calculate trivia
    let shape = shape.reset();
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(else_if_node.condition().to_owned());

    let elseif_token = format_end_token(
        ctx,
        else_if_node.else_if_token(),
        EndTokenType::BlockEnd,
        shape,
    );
    let singleline_condition = format_expression(ctx, &condition, shape + 7);
    let singleline_then_token = fmt_symbol!(ctx, else_if_node.then_token(), " then", shape);

    // Determine if we need to hang the condition
    let singleline_shape = shape + (7 + 5 + strip_trivia(&singleline_condition).to_string().len()); // 7 = "elseif ", 3 = " then"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::token_contains_trailing_comments(else_if_node.else_if_token())
        || trivia_util::token_contains_leading_comments(else_if_node.then_token())
        || trivia_util::contains_comments(&condition);

    let elseif_token = match require_multiline_expression {
        true => elseif_token
            .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)])),
        false => elseif_token.update_trailing_trivia(FormatTriviaType::Append(vec![Token::new(
            TokenType::spaces(1),
        )])),
    }
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    let condition = match require_multiline_expression {
        true => {
            let shape = shape.reset().increment_additional_indent();
            hang_expression_trailing_newline(ctx, &condition, shape, None).update_leading_trivia(
                FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
            )
        }
        false => singleline_condition,
    };

    let then_token = match require_multiline_expression {
        true => format_end_token(
            ctx,
            else_if_node.then_token(),
            EndTokenType::BlockEnd,
            shape,
        )
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia)),
        false => singleline_then_token,
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, else_if_node.block(), block_shape);

    else_if_node
        .to_owned()
        .with_else_if_token(elseif_token)
        .with_condition(condition)
        .with_then_token(then_token)
        .with_block(block)
}

/// Format an If node
pub fn format_if(ctx: &Context, if_node: &If, shape: Shape) -> If {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(if_node.condition().to_owned());

    let singleline_if_token = fmt_symbol!(ctx, if_node.if_token(), "if ", shape);
    let singleline_condition = format_expression(ctx, &condition, shape + 6);
    let singleline_then_token = fmt_symbol!(ctx, if_node.then_token(), " then", shape);

    // Determine if we need to hang the condition
    let singleline_shape = shape + (3 + 5 + strip_trivia(&singleline_condition).to_string().len()); // 3 = "if ", 5 = " then"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::token_contains_trailing_comments(if_node.if_token())
        || trivia_util::token_contains_leading_comments(if_node.then_token())
        || trivia_util::contains_comments(&condition);

    let if_token = match require_multiline_expression {
        true => fmt_symbol!(ctx, if_node.if_token(), "if", shape)
            .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)])),
        false => singleline_if_token,
    }
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    let condition = match require_multiline_expression {
        true => {
            let shape = shape.reset().increment_additional_indent();
            hang_expression_trailing_newline(ctx, &condition, shape, None).update_leading_trivia(
                FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
            )
        }
        false => singleline_condition,
    };

    let then_token = match require_multiline_expression {
        true => format_end_token(ctx, if_node.then_token(), EndTokenType::BlockEnd, shape)
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned())),
        false => singleline_then_token,
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, if_node.block(), block_shape);

    let end_token = format_end_token(ctx, if_node.end_token(), EndTokenType::BlockEnd, shape)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );

    let else_if = if_node.else_if().map(|else_if| {
        else_if
            .iter()
            .map(|else_if| format_else_if(ctx, else_if, shape))
            .collect()
    });

    let (else_token, else_block) = match (if_node.else_token(), if_node.else_block()) {
        (Some(else_token), Some(else_block)) => {
            let else_token = format_end_token(ctx, else_token, EndTokenType::BlockEnd, shape)
                .update_trivia(
                    FormatTriviaType::Append(leading_trivia),
                    FormatTriviaType::Append(trailing_trivia),
                );
            let else_block_shape = shape.reset().increment_block_indent();
            let else_block = format_block(ctx, else_block, else_block_shape);

            (Some(else_token), Some(else_block))
        }
        (None, None) => (None, None),
        _ => unreachable!("Got an else token with no else block or vice versa"),
    };

    if_node
        .to_owned()
        .with_if_token(if_token)
        .with_condition(condition)
        .with_then_token(then_token)
        .with_block(block)
        .with_else_if(else_if)
        .with_else_token(else_token)
        .with_else(else_block)
        .with_end_token(end_token)
}

/// Format a NumericFor node
pub fn format_numeric_for(ctx: &Context, numeric_for: &NumericFor, shape: Shape) -> NumericFor {
    // Create trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let for_token = fmt_symbol!(ctx, numeric_for.for_token(), "for ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));
    let index_variable = format_token_reference(ctx, numeric_for.index_variable(), shape);

    #[cfg(feature = "luau")]
    let type_specifier = numeric_for
        .type_specifier()
        .map(|type_specifier| format_type_specifier(ctx, type_specifier, shape));

    // TODO: Should we actually update the shape here?
    let equal_token = fmt_symbol!(ctx, numeric_for.equal_token(), " = ", shape);
    let start = format_expression(ctx, numeric_for.start(), shape);
    let start_end_comma = fmt_symbol!(ctx, numeric_for.start_end_comma(), ", ", shape);
    let end = format_expression(ctx, numeric_for.end(), shape);

    let (end_step_comma, step) = match (numeric_for.end_step_comma(), numeric_for.step()) {
        (Some(end_step_comma), Some(step)) => (
            Some(fmt_symbol!(ctx, end_step_comma, ", ", shape)),
            Some(format_expression(ctx, step, shape)),
        ),
        (None, None) => (None, None),
        _ => unreachable!("Got numeric for end step comma with no step or vice versa"),
    };

    let do_token = fmt_symbol!(ctx, numeric_for.do_token(), " do", shape)
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));
    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, numeric_for.block(), block_shape);
    let end_token = format_end_token(ctx, numeric_for.end_token(), EndTokenType::BlockEnd, shape)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(trailing_trivia),
        );

    let numeric_for = numeric_for
        .to_owned()
        .with_for_token(for_token)
        .with_index_variable(index_variable)
        .with_equal_token(equal_token)
        .with_start(start)
        .with_start_end_comma(start_end_comma)
        .with_end(end)
        .with_end_step_comma(end_step_comma)
        .with_step(step)
        .with_do_token(do_token)
        .with_block(block)
        .with_end_token(end_token);
    #[cfg(feature = "luau")]
    let numeric_for = numeric_for.with_type_specifier(type_specifier);

    numeric_for
}

/// Format a Repeat node
pub fn format_repeat_block(ctx: &Context, repeat_block: &Repeat, shape: Shape) -> Repeat {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let repeat_token = fmt_symbol!(ctx, repeat_block.repeat_token(), "repeat", shape)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );
    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, repeat_block.block(), block_shape);
    let until_token = fmt_symbol!(ctx, repeat_block.until_token(), "until ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(repeat_block.until().to_owned());

    // Determine if we need to hang the condition
    let singleline_shape = shape + (6 + strip_trivia(&condition).to_string().len()); // 6 = "until "
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::expression_contains_inline_comments(&condition);

    let shape = shape + 6; // 6 = "until "
    let until = match require_multiline_expression {
        true => {
            let shape = shape.increment_additional_indent();
            hang_expression_trailing_newline(ctx, &condition, shape, None)
        }
        false => format_expression(ctx, &condition, shape)
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia)),
    };

    repeat_block
        .to_owned()
        .with_repeat_token(repeat_token)
        .with_block(block)
        .with_until_token(until_token)
        .with_until(until)
}

/// Format a While node
pub fn format_while_block(ctx: &Context, while_block: &While, shape: Shape) -> While {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(while_block.condition().to_owned());

    let singleline_while_token = fmt_symbol!(ctx, while_block.while_token(), "while ", shape);
    let singleline_condition = format_expression(ctx, &condition, shape + 6);
    let singleline_do_token = fmt_symbol!(ctx, while_block.do_token(), " do", shape);

    // Determine if we need to hang the condition
    let singleline_shape = shape + (6 + 3 + strip_trivia(&singleline_condition).to_string().len()); // 6 = "while ", 3 = " do"
    let require_multiline_expression = singleline_shape.over_budget()
        || trivia_util::token_contains_trailing_comments(while_block.while_token())
        || trivia_util::token_contains_leading_comments(while_block.do_token())
        || trivia_util::contains_comments(&condition);

    let while_token = match require_multiline_expression {
        true => fmt_symbol!(ctx, while_block.while_token(), "while", shape)
            .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)])),
        false => singleline_while_token,
    }
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));

    let condition = match require_multiline_expression {
        true => {
            let shape = shape.reset().increment_additional_indent();
            hang_expression_trailing_newline(ctx, &condition, shape, None).update_leading_trivia(
                FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
            )
        }
        false => singleline_condition,
    };

    let do_token = match require_multiline_expression {
        true => format_end_token(ctx, while_block.do_token(), EndTokenType::BlockEnd, shape)
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned())),
        false => singleline_do_token,
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, while_block.block(), block_shape);

    let end_token = format_end_token(ctx, while_block.end_token(), EndTokenType::BlockEnd, shape)
        .update_trivia(
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(trailing_trivia),
        );

    while_block
        .to_owned()
        .with_while_token(while_token)
        .with_condition(condition)
        .with_do_token(do_token)
        .with_block(block)
        .with_end_token(end_token)
}

/// Wrapper around `format_function_call`, but also handles adding the trivia around the function call.
/// This can't be done in the original function, as function calls are not always statements, but can also be
/// in expressions.
pub fn format_function_call_stmt(
    ctx: &Context,
    function_call: &FunctionCall,
    shape: Shape,
) -> FunctionCall {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    format_function_call(ctx, function_call, shape).update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(trailing_trivia),
    )
}

pub fn format_stmt(ctx: &Context, stmt: &Stmt, shape: Shape) -> Stmt {
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
