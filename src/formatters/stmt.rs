#[cfg(feature = "lua52")]
use crate::formatters::lua52::{format_goto, format_goto_no_trivia, format_label};
#[cfg(feature = "luau")]
use crate::formatters::luau::{
    format_compound_assignment, format_exported_type_declaration, format_type_declaration_stmt,
    format_type_specifier,
};
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context, FormatNode},
    fmt_symbol,
    formatters::{
        assignment::{
            format_assignment, format_assignment_no_trivia, format_local_assignment,
            format_local_assignment_no_trivia,
        },
        block::{format_block, format_last_stmt_no_trivia},
        expression::{format_expression, hang_expression_trailing_newline},
        functions::{format_function_call, format_function_declaration, format_local_function},
        general::{
            format_end_token, format_punctuated, format_punctuated_multiline,
            format_token_reference, EndTokenType,
        },
        trivia::{
            strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{
            self, CommentSearch, GetLeadingTrivia, GetTrailingTrivia, HasInlineComments,
        },
    },
    shape::Shape,
};
use full_moon::{
    ast::{
        punctuated::Punctuated, Block, Call, Do, ElseIf, Expression, FunctionArgs, FunctionCall,
        GenericFor, If, NumericFor, Repeat, Stmt, Suffix, While,
    },
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};

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
pub fn remove_condition_parentheses(expression: Expression) -> Expression {
    match expression.to_owned() {
        Expression::Parentheses {
            expression: inner_expression,
            ..
        } => {
            let (_, comments) = trivia_util::take_trailing_comments(&expression);
            inner_expression.update_trailing_trivia(FormatTriviaType::Append(comments))
        }
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
    let end_token = format_end_token(
        ctx,
        do_block.end_token(),
        EndTokenType::IndentComments,
        shape,
    )
    .update_trivia(leading_trivia, trailing_trivia);

    do_block
        .to_owned()
        .with_do_token(do_token)
        .with_block(block)
        .with_end_token(end_token)
}

/// Determine if we should hug the generic for.
/// This should only happen when there is a single expression, which is a function call containing
/// a single table, and there are no comments which will affect it
fn hug_generic_for(expressions: &Punctuated<Expression>) -> bool {
    if expressions.len() != 1 {
        return false;
    }

    let expression = expressions.iter().next().unwrap();

    // Ensure no comments
    if expression.has_leading_comments(CommentSearch::All)
        || expression.has_trailing_comments(CommentSearch::All)
    {
        return false;
    }

    match expression {
        // Ensure is function call
        Expression::FunctionCall(function_call) => {
            let mut suffixes = function_call.suffixes();
            // Test next 2 available suffixes
            match (suffixes.next(), suffixes.next()) {
                // Ensure at least one suffix, and only one suffix
                (Some(suffix), None) => match suffix {
                    // Ensure suffix is a call with a single table constructor as argument
                    Suffix::Call(Call::AnonymousCall(FunctionArgs::TableConstructor(_))) => true,
                    Suffix::Call(Call::AnonymousCall(FunctionArgs::Parentheses {
                        arguments,
                        ..
                    })) => {
                        let mut arguments = arguments.iter();
                        // Test next 2 available arguments
                        match (arguments.next(), arguments.next()) {
                            // Ensure at least one argument, and only one argument
                            // And that the argument is a table constructor
                            (Some(Expression::TableConstructor(_)), None) => true,
                            _ => false,
                        }
                    }
                    _ => false,
                },
                _ => false,
            }
        }
        _ => false,
    }
}

/// Format a GenericFor node
pub fn format_generic_for(ctx: &Context, generic_for: &GenericFor, shape: Shape) -> GenericFor {
    // Create trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let for_token = fmt_symbol!(ctx, generic_for.for_token(), "for ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned()));
    let shape = shape + 4; // 4 = "for "

    // Format the names on a single line
    // If it goes over the column width, or contains comments, then format it multiline
    let singleline_names =
        format_punctuated(ctx, generic_for.names(), shape, format_token_reference);
    let singleline_shape = shape.take_first_line(&singleline_names);

    // Format the type specifiers, if present, and add them to the shape
    #[cfg(feature = "luau")]
    let type_specifiers: Vec<_> = generic_for
        .type_specifiers()
        .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
        .collect();

    #[cfg(feature = "luau")]
    let singleline_shape = singleline_shape
        + type_specifiers.iter().fold(0, |acc, x| {
            acc + x
                .as_ref()
                .map_or(0, |type_specifier| type_specifier.to_string().len())
        });

    let require_names_multiline = trivia_util::contains_comments(generic_for.names())
        || trivia_util::spans_multiple_lines(&singleline_names)
        || singleline_shape.over_budget();

    let for_token = match require_names_multiline {
        true => fmt_symbol!(ctx, generic_for.for_token(), "for", shape)
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned())),
        false => for_token,
    };

    let names = match require_names_multiline {
        true => {
            let shape = shape.reset().increment_additional_indent();
            format_punctuated_multiline(
                ctx,
                generic_for.names(),
                shape,
                format_token_reference,
                None,
            )
            .update_leading_trivia(FormatTriviaType::Append(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
            ]))
        }
        false => singleline_names,
    };

    let shape = match require_names_multiline {
        true => shape.reset() + 3,     // 3 = "in "
        false => singleline_shape + 4, // 4 = " in "
    };

    // Format the expression list on a single line, and see if it needs expanding
    let singleline_expr =
        format_punctuated(ctx, generic_for.expressions(), shape, format_expression);
    let singleline_expr_shape = shape.take_first_line(&singleline_expr);
    let requires_expr_multiline = (generic_for
        .in_token()
        .has_trailing_comments(CommentSearch::All)
        || trivia_util::contains_comments(generic_for.expressions())
        || trivia_util::spans_multiple_lines(&singleline_expr)
        || singleline_expr_shape.over_budget())
        && !hug_generic_for(generic_for.expressions());

    let in_token = match (require_names_multiline, requires_expr_multiline) {
        (true, true) => fmt_symbol!(ctx, generic_for.in_token(), "in", shape)
            .update_leading_trivia(FormatTriviaType::Append(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
            ])),
        (true, false) => fmt_symbol!(ctx, generic_for.in_token(), "in ", shape)
            .update_leading_trivia(FormatTriviaType::Append(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
            ])),
        (false, true) => fmt_symbol!(ctx, generic_for.in_token(), " in", shape),
        (false, false) => fmt_symbol!(ctx, generic_for.in_token(), " in ", shape),
    };

    // Expand the expression list if necessary
    let expr_list = match requires_expr_multiline {
        true => {
            let shape = shape.reset().increment_additional_indent();
            let expr_list = format_punctuated_multiline(
                ctx,
                generic_for.expressions(),
                shape,
                format_expression,
                None,
            );
            trivia_util::prepend_newline_indent(ctx, &expr_list, shape)
        }
        false => singleline_expr,
    };

    let do_token = match requires_expr_multiline {
        true => fmt_symbol!(ctx, generic_for.do_token(), "do", shape).update_leading_trivia(
            FormatTriviaType::Append(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
            ]),
        ),
        false => fmt_symbol!(ctx, generic_for.do_token(), " do", shape),
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, generic_for.block(), block_shape);

    let end_token = format_end_token(
        ctx,
        generic_for.end_token(),
        EndTokenType::IndentComments,
        shape,
    )
    .update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(vec![create_newline_trivia(ctx)]), // trailing_trivia was emptied when it was appended to names_comment_buf
    );

    let generic_for = generic_for.to_owned();
    #[cfg(feature = "luau")]
    let generic_for = generic_for.with_type_specifiers(type_specifiers);
    generic_for
        .with_for_token(for_token)
        .with_names(names)
        .with_in_token(in_token)
        .with_expressions(expr_list)
        .with_do_token(do_token)
        .with_block(block)
        .with_end_token(end_token)
}

/// Given some trivia (particularly comments), this function computes whether we should
/// indent leading comments on a elseif/else token, one level further. i.e.:
/// ```lua
/// -- comment
/// elseif
///     -- comment
/// else
/// ```
/// If the comment was originally indented at a level higher than the current level,
/// then we will indent it one level further.
fn should_indent_further<'a>(trivia: impl Iterator<Item = &'a Token>, shape: Shape) -> bool {
    let current_indent_level = shape.indent().block_indent() + shape.indent().additional_indent();
    let mut iter = trivia.peekable();

    while let Some(trivia) = iter.next() {
        let next_trivia = iter.peek();

        if let Some(next_trivia) = next_trivia {
            if let TokenType::Whitespace { characters } = trivia.token_type() {
                if matches!(
                    next_trivia.token_kind(),
                    TokenKind::SingleLineComment | TokenKind::MultiLineComment
                ) {
                    // Only use the "last line" of the whitespace, i.e. after all newlines
                    let last_line = characters
                        .chars()
                        .rev()
                        .take_while(|c| !matches!(c, '\n' | '\r'));

                    let indent_level = if last_line.clone().any(|c| matches!(c, '\t')) {
                        last_line.filter(|c| matches!(c, '\t')).count()
                    } else {
                        last_line.filter(|c| matches!(c, ' ')).count()
                            / shape.indent().configured_indent_width()
                    };

                    if indent_level > current_indent_level {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Formats an ElseIf node - This must always reside within format_if
fn format_else_if(ctx: &Context, else_if_node: &ElseIf, shape: Shape) -> ElseIf {
    // Calculate trivia
    let shape = shape.reset();
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(else_if_node.condition().to_owned());

    // Compute the indent
    let end_token_type =
        if should_indent_further(else_if_node.else_if_token().leading_trivia(), shape) {
            EndTokenType::IndentComments
        } else {
            EndTokenType::InlineComments
        };

    let elseif_token = format_end_token(ctx, else_if_node.else_if_token(), end_token_type, shape);
    let singleline_condition = format_expression(ctx, &condition, shape + 7);
    let singleline_then_token = fmt_symbol!(ctx, else_if_node.then_token(), " then", shape);

    // Determine if we need to hang the condition
    let singleline_shape = shape + (7 + 5 + strip_trivia(&singleline_condition).to_string().len()); // 7 = "elseif ", 3 = " then"
    let require_multiline_expression = singleline_shape.over_budget()
        || else_if_node
            .else_if_token()
            .has_trailing_comments(CommentSearch::All)
        || else_if_node
            .then_token()
            .has_leading_comments(CommentSearch::All)
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
            EndTokenType::IndentComments,
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

/// Checks to see whether an [`If`] statement matches the structure of an "if guard".
/// ```lua
/// if condition then return expr end
/// ```
/// If we have an if guard, we can keep the statement single line - provided that a few key observations hold:
/// - There is no elseif/else block
/// - The body of the if block only contains a single [`LastStmt`] (note, this LastStmt may have multiple expressions e.g. `return foo, bar`)
/// - There are no internal comments within the block
/// We will also check if the statement surpasses the column width, or if the condition required multilining (handled outside of this function)
fn is_if_guard(if_node: &If) -> bool {
    if_node.else_if().is_none()
        && if_node.else_block().is_none()
        && trivia_util::is_block_simple(if_node.block())
        && !trivia_util::contains_comments(if_node.block())
        && !trivia_util::contains_comments(if_node.then_token())
}

/// Format an If node
pub fn format_if(ctx: &Context, if_node: &If, shape: Shape) -> If {
    const IF_LEN: usize = "if ".len();
    const THEN_LEN: usize = " then".len();

    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(if_node.condition().to_owned());

    let singleline_if_token = fmt_symbol!(ctx, if_node.if_token(), "if ", shape);
    let singleline_condition = format_expression(ctx, &condition, shape + IF_LEN + THEN_LEN);
    let singleline_then_token = fmt_symbol!(ctx, if_node.then_token(), " then", shape);

    // Determine if we need to hang the condition
    let singleline_shape =
        shape + (IF_LEN + THEN_LEN + strip_trivia(&singleline_condition).to_string().len());
    let require_multiline_expression = singleline_shape.over_budget()
        || if_node.if_token().has_trailing_comments(CommentSearch::All)
        || if_node
            .then_token()
            .has_leading_comments(CommentSearch::All)
        || trivia_util::contains_comments(&condition);

    if !require_multiline_expression
        && ctx.should_collapse_simple_conditionals()
        && is_if_guard(if_node)
    {
        // Rather than deferring to `format_block()`, since we know that there is only a single Stmt or LastStmt in the block, we can format it immediately
        // We need to modify the formatted LastStmt, since it will have automatically added leading/trailing trivia we don't want
        // We assume that there is only a laststmt present in the block - the callee of this function should have already checked for this
        // INVARIANT: this stmt has no leading/trailing comments, as this is checked in `is_if_guard`
        // This means we can replace trivia completely
        debug_assert!(!trivia_util::contains_comments(if_node.block()));
        let stmt_leading_trivia = FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]);
        let stmt_trailing_trivia =
            FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]);

        let block = if let Some(stmt) = if_node.block().stmts().next() {
            let stmt = format_stmt_no_trivia(ctx, stmt, singleline_shape)
                .update_trivia(stmt_leading_trivia, stmt_trailing_trivia);

            Block::new().with_stmts(vec![(stmt, None)])
        } else if let Some(last_stmt) = if_node.block().last_stmt() {
            let last_stmt = format_last_stmt_no_trivia(ctx, last_stmt, singleline_shape)
                .update_trivia(stmt_leading_trivia, stmt_trailing_trivia);

            Block::new().with_last_stmt(Some((last_stmt, None)))
        } else {
            panic!("'if guard' conditional but has no body");
        };

        let end_token = format_end_token(
            ctx,
            if_node.end_token(),
            EndTokenType::IndentComments,
            shape,
        )
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.clone()));

        let singleline_if = if_node
            .to_owned()
            .with_if_token(
                singleline_if_token
                    .update_leading_trivia(FormatTriviaType::Append(leading_trivia.clone())),
            )
            .with_condition(singleline_condition.clone())
            .with_then_token(singleline_then_token.clone())
            .with_block(block)
            .with_end_token(end_token);

        // See if it fits under the column width. If it does, bail early and return this singleline if
        if !shape
            .add_width(strip_trivia(&singleline_if).to_string().len())
            .over_budget()
        {
            return singleline_if;
        }
    }

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
        true => format_end_token(
            ctx,
            if_node.then_token(),
            EndTokenType::IndentComments,
            shape,
        )
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned())),
        false => singleline_then_token,
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, if_node.block(), block_shape);

    let end_token = format_end_token(
        ctx,
        if_node.end_token(),
        EndTokenType::IndentComments,
        shape,
    )
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
            // Compute the indent level on else token
            let end_token_type = if should_indent_further(else_token.leading_trivia(), shape) {
                EndTokenType::IndentComments
            } else {
                EndTokenType::InlineComments
            };

            let else_token = format_end_token(ctx, else_token, end_token_type, shape)
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
    let end_token = format_end_token(
        ctx,
        numeric_for.end_token(),
        EndTokenType::IndentComments,
        shape,
    )
    .update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(trailing_trivia),
    );

    let numeric_for = numeric_for.to_owned();
    #[cfg(feature = "luau")]
    let numeric_for = numeric_for.with_type_specifier(type_specifier);

    numeric_for
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
        .with_end_token(end_token)
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
    let require_multiline_expression =
        singleline_shape.over_budget() || condition.has_inline_comments();

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
        || while_block
            .while_token()
            .has_trailing_comments(CommentSearch::All)
        || while_block
            .do_token()
            .has_leading_comments(CommentSearch::All)
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
        true => format_end_token(
            ctx,
            while_block.do_token(),
            EndTokenType::IndentComments,
            shape,
        )
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia.to_owned())),
        false => singleline_do_token,
    }
    .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()));

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, while_block.block(), block_shape);

    let end_token = format_end_token(
        ctx,
        while_block.end_token(),
        EndTokenType::IndentComments,
        shape,
    )
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

/// Functions which are used to only format a block within a statement
/// These are used for range formatting
pub(crate) mod stmt_block {
    use crate::{context::Context, formatters::block::format_block, shape::Shape};
    use full_moon::ast::{
        Call, Expression, Field, FunctionArgs, FunctionCall, Index, Prefix, Stmt, Suffix,
        TableConstructor,
    };

    fn format_table_constructor_block(
        ctx: &Context,
        table_constructor: &TableConstructor,
        shape: Shape,
    ) -> TableConstructor {
        let fields = table_constructor
            .fields()
            .pairs()
            .map(|pair| {
                pair.to_owned().map(|field| match field {
                    Field::ExpressionKey {
                        brackets,
                        key,
                        equal,
                        value,
                    } => Field::ExpressionKey {
                        brackets,
                        key: format_expression_block(ctx, &key, shape),
                        equal,
                        value: format_expression_block(ctx, &value, shape),
                    },
                    Field::NameKey { key, equal, value } => Field::NameKey {
                        key,
                        equal,
                        value: format_expression_block(ctx, &value, shape),
                    },
                    Field::NoKey(expression) => {
                        Field::NoKey(format_expression_block(ctx, &expression, shape))
                    }
                    other => panic!("unknown node {:?}", other),
                })
            })
            .collect();

        table_constructor.to_owned().with_fields(fields)
    }

    fn format_function_args_block(
        ctx: &Context,
        function_args: &FunctionArgs,
        shape: Shape,
    ) -> FunctionArgs {
        match function_args {
            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            } => FunctionArgs::Parentheses {
                parentheses: parentheses.to_owned(),
                arguments: arguments
                    .pairs()
                    .map(|pair| {
                        pair.to_owned()
                            .map(|expression| format_expression_block(ctx, &expression, shape))
                    })
                    .collect(),
            },
            FunctionArgs::TableConstructor(table_constructor) => FunctionArgs::TableConstructor(
                format_table_constructor_block(ctx, table_constructor, shape),
            ),
            _ => function_args.to_owned(),
        }
    }

    fn format_function_call_block(
        ctx: &Context,
        function_call: &FunctionCall,
        shape: Shape,
    ) -> FunctionCall {
        let prefix = match function_call.prefix() {
            Prefix::Expression(expression) => {
                Prefix::Expression(Box::new(format_expression_block(ctx, expression, shape)))
            }
            Prefix::Name(name) => Prefix::Name(name.to_owned()),
            other => panic!("unknown node {:?}", other),
        };

        let suffixes = function_call
            .suffixes()
            .map(|suffix| match suffix {
                Suffix::Call(call) => Suffix::Call(match call {
                    Call::AnonymousCall(function_args) => {
                        Call::AnonymousCall(format_function_args_block(ctx, function_args, shape))
                    }
                    Call::MethodCall(method_call) => {
                        let args = format_function_args_block(ctx, method_call.args(), shape);
                        Call::MethodCall(method_call.to_owned().with_args(args))
                    }
                    other => panic!("unknown node {:?}", other),
                }),
                Suffix::Index(index) => Suffix::Index(match index {
                    Index::Brackets {
                        brackets,
                        expression,
                    } => Index::Brackets {
                        brackets: brackets.to_owned(),
                        expression: format_expression_block(ctx, expression, shape),
                    },
                    _ => index.to_owned(),
                }),
                other => panic!("unknown node {:?}", other),
            })
            .collect();

        function_call
            .to_owned()
            .with_prefix(prefix)
            .with_suffixes(suffixes)
    }

    /// Only formats a block within an expression
    pub fn format_expression_block(
        ctx: &Context,
        expression: &Expression,
        shape: Shape,
    ) -> Expression {
        match expression {
            Expression::BinaryOperator { lhs, binop, rhs } => Expression::BinaryOperator {
                lhs: Box::new(format_expression_block(ctx, lhs, shape)),
                binop: binop.to_owned(),
                rhs: Box::new(format_expression_block(ctx, rhs, shape)),
            },
            Expression::Parentheses {
                contained,
                expression,
            } => Expression::Parentheses {
                contained: contained.to_owned(),
                expression: Box::new(format_expression_block(ctx, expression, shape)),
            },
            Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
                unop: unop.to_owned(),
                expression: Box::new(format_expression_block(ctx, expression, shape)),
            },
            Expression::Function((function_token, body)) => {
                let block = format_block(ctx, body.block(), shape);
                Expression::Function((function_token.to_owned(), body.to_owned().with_block(block)))
            }
            Expression::FunctionCall(function_call) => {
                Expression::FunctionCall(format_function_call_block(ctx, function_call, shape))
            }
            Expression::TableConstructor(table_constructor) => Expression::TableConstructor(
                format_table_constructor_block(ctx, table_constructor, shape),
            ),
            #[cfg(feature = "luau")]
            Expression::TypeAssertion {
                expression,
                type_assertion,
            } => Expression::TypeAssertion {
                expression: Box::new(format_expression_block(ctx, expression, shape)),
                type_assertion: type_assertion.to_owned(),
            },
            // TODO: var?
            value => value.to_owned(),
        }
    }

    /// Only formats a block within the statement
    pub(crate) fn format_stmt_block(ctx: &Context, stmt: &Stmt, shape: Shape) -> Stmt {
        let block_shape = shape.reset().increment_block_indent();

        // TODO: Assignment, FunctionCall, LocalAssignment is funky
        match stmt {
            Stmt::Assignment(assignment) => {
                // TODO: var?
                let expressions = assignment
                    .expressions()
                    .pairs()
                    .map(|pair| {
                        pair.to_owned().map(|expression| {
                            format_expression_block(ctx, &expression, block_shape)
                        })
                    })
                    .collect();

                Stmt::Assignment(assignment.to_owned().with_expressions(expressions))
            }
            Stmt::Do(do_block) => {
                let block = format_block(ctx, do_block.block(), block_shape);
                Stmt::Do(do_block.to_owned().with_block(block))
            }
            Stmt::FunctionCall(function_call) => {
                Stmt::FunctionCall(format_function_call_block(ctx, function_call, block_shape))
            }
            Stmt::FunctionDeclaration(function_declaration) => {
                let block = format_block(ctx, function_declaration.body().block(), block_shape);
                let body = function_declaration.body().to_owned().with_block(block);
                Stmt::FunctionDeclaration(function_declaration.to_owned().with_body(body))
            }
            Stmt::GenericFor(generic_for) => {
                let block = format_block(ctx, generic_for.block(), block_shape);
                Stmt::GenericFor(generic_for.to_owned().with_block(block))
            }
            Stmt::If(if_block) => {
                let block = format_block(ctx, if_block.block(), block_shape);
                let else_if = if_block.else_if().map(|else_ifs| {
                    else_ifs
                        .iter()
                        .map(|else_if| {
                            else_if.to_owned().with_block(format_block(
                                ctx,
                                else_if.block(),
                                block_shape,
                            ))
                        })
                        .collect()
                });
                let else_block = if_block
                    .else_block()
                    .map(|block| format_block(ctx, block, block_shape));

                Stmt::If(
                    if_block
                        .to_owned()
                        .with_block(block)
                        .with_else_if(else_if)
                        .with_else(else_block),
                )
            }
            Stmt::LocalAssignment(assignment) => {
                let expressions = assignment
                    .expressions()
                    .pairs()
                    .map(|pair| {
                        pair.to_owned().map(|expression| {
                            format_expression_block(ctx, &expression, block_shape)
                        })
                    })
                    .collect();

                Stmt::LocalAssignment(assignment.to_owned().with_expressions(expressions))
            }
            Stmt::LocalFunction(local_function) => {
                let block = format_block(ctx, local_function.body().block(), block_shape);
                let body = local_function.body().to_owned().with_block(block);
                Stmt::LocalFunction(local_function.to_owned().with_body(body))
            }
            Stmt::NumericFor(numeric_for) => {
                let block = format_block(ctx, numeric_for.block(), block_shape);
                Stmt::NumericFor(numeric_for.to_owned().with_block(block))
            }
            Stmt::Repeat(repeat) => {
                let block = format_block(ctx, repeat.block(), block_shape);
                Stmt::Repeat(repeat.to_owned().with_block(block))
            }
            Stmt::While(while_block) => {
                let block = format_block(ctx, while_block.block(), block_shape);
                Stmt::While(while_block.to_owned().with_block(block))
            }
            #[cfg(feature = "luau")]
            Stmt::CompoundAssignment(compound_assignment) => {
                let rhs = format_expression_block(ctx, compound_assignment.rhs(), block_shape);
                Stmt::CompoundAssignment(compound_assignment.to_owned().with_rhs(rhs))
            }
            #[cfg(feature = "luau")]
            Stmt::ExportedTypeDeclaration(node) => Stmt::ExportedTypeDeclaration(node.to_owned()),
            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(node) => Stmt::TypeDeclaration(node.to_owned()),
            #[cfg(feature = "lua52")]
            Stmt::Goto(node) => Stmt::Goto(node.to_owned()),
            #[cfg(feature = "lua52")]
            Stmt::Label(node) => Stmt::Label(node.to_owned()),
            other => panic!("unknown node {:?}", other),
        }
    }
}

pub fn format_stmt(ctx: &Context, stmt: &Stmt, shape: Shape) -> Stmt {
    let should_format = ctx.should_format_node(stmt);

    if let FormatNode::Skip = should_format {
        return stmt.to_owned();
    } else if let FormatNode::NotInRange = should_format {
        return stmt_block::format_stmt_block(ctx, stmt, shape);
    }

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

pub fn format_stmt_no_trivia(ctx: &Context, stmt: &Stmt, shape: Shape) -> Stmt {
    assert!(
        matches!(ctx.should_format_node(stmt), FormatNode::Normal),
        "!FormatNode::None for format_stmt_no_trivia"
    );

    match stmt {
        Stmt::LocalAssignment(stmt) => {
            Stmt::LocalAssignment(format_local_assignment_no_trivia(ctx, stmt, shape))
        }
        Stmt::Assignment(stmt) => Stmt::Assignment(format_assignment_no_trivia(ctx, stmt, shape)),
        Stmt::FunctionCall(stmt) => Stmt::FunctionCall(format_function_call(ctx, stmt, shape)),
        #[cfg(feature = "lua52")]
        Stmt::Goto(goto) => Stmt::Goto(format_goto_no_trivia(ctx, goto, shape)),
        _ => unreachable!("format_stmt_no_trivia: node != assignment/function call/goto"),
    }
}
