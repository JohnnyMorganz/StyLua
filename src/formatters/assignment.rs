#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::tokenizer::{Token, TokenReference};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        Assignment, Expression, LocalAssignment,
    },
    tokenizer::TokenType,
};

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_specifier;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, format_var, hang_expression},
        general::{
            format_punctuated, format_punctuated_multiline, format_token_reference,
            try_format_punctuated,
        },
        trivia::{
            strip_leading_trivia, strip_trailing_trivia, strip_trivia, FormatTriviaType,
            UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
        trivia_util,
    },
    shape::Shape,
};

/// Hangs each [`Expression`] in a [`Punctuated`] list.
/// The Punctuated list is hung multiline at the comma aswell, and each subsequent item after the first is
/// indented by one.
pub fn hang_punctuated_list(
    ctx: &Context,
    punctuated: &Punctuated<Expression>,
    shape: Shape,
) -> Punctuated<Expression> {
    let mut output = Punctuated::new();

    // Format each expression and hang them
    // We need to format again because we will now take into account the indent increase
    for (idx, pair) in punctuated.pairs().enumerate() {
        let shape = if idx == 0 {
            shape
        } else {
            shape.reset().increment_additional_indent()
        };

        let mut value = hang_expression(ctx, pair.value(), shape, Some(1));
        if idx != 0 {
            value =
                value.update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                    ctx, shape,
                )]));
        }

        output.push(Pair::new(
            value,
            pair.punctuation().map(|x| {
                fmt_symbol!(ctx, x, ",", shape).update_trailing_trivia(FormatTriviaType::Append(
                    vec![create_newline_trivia(ctx)],
                ))
            }),
        ));
    }

    output
}

/// Hangs at the equal token, and indents the first item.
/// Returns the new equal token [`TokenReference`]
fn hang_equal_token(
    ctx: &Context,
    equal_token: TokenReference,
    shape: Shape,
    indent_first_item: bool,
) -> TokenReference {
    let mut equal_token_trailing_trivia = vec![create_newline_trivia(ctx)];
    if indent_first_item {
        equal_token_trailing_trivia.push(create_indent_trivia(
            ctx,
            shape.increment_additional_indent(),
        ))
    }

    let equal_token_trailing_trivia = equal_token
        .trailing_trivia()
        .filter(|x| trivia_util::trivia_is_comment(x))
        .flat_map(|x| vec![Token::new(TokenType::spaces(1)), x.to_owned()])
        .chain(equal_token_trailing_trivia.iter().map(|x| x.to_owned()))
        .collect();

    equal_token.update_trailing_trivia(FormatTriviaType::Replace(equal_token_trailing_trivia))
}

/// Attempts different formatting tactics on an expression list being assigned (`= foo, bar`), to find the best
/// formatting output.
fn attempt_assignment_tactics(
    ctx: &Context,
    expressions: &Punctuated<Expression>,
    shape: Shape,
    equal_token: TokenReference,
) -> (Punctuated<Expression>, TokenReference) {
    // The next tactic is to see whether there is more than one item in the punctuated list
    // If there is, we should put it on multiple lines
    if expressions.len() > 1 {
        // First try hanging at the equal token, using an infinite width, to see if its enough
        let hanging_equal_token = hang_equal_token(ctx, equal_token, shape, true);
        let hanging_shape = shape.reset().increment_additional_indent();
        let expr_list = format_punctuated(
            ctx,
            expressions,
            hanging_shape.with_infinite_width(),
            format_expression,
        );

        if expressions.pairs().any(|pair| {
            pair.punctuation()
                .map_or(false, |x| trivia_util::token_contains_comments(x))
                || trivia_util::expression_contains_inline_comments(pair.value())
        }) || hanging_shape
            .take_first_line(&strip_trivia(&expr_list))
            .over_budget()
        {
            // See whether there is more than one item in the punctuated list
            // Hang the expressions on multiple lines
            let multiline_expr = format_punctuated_multiline(
                ctx,
                expressions,
                hanging_shape,
                format_expression,
                None,
            );

            // Look through each punctuated expression to see if we need to hang the item further
            let mut output_expr = Punctuated::new();

            for (idx, (formatted, original)) in
                multiline_expr.into_pairs().zip(expressions).enumerate()
            {
                // Recreate the shape
                let shape = if idx == 0 { shape } else { shape.reset() };

                if trivia_util::contains_comments(&formatted)
                    || shape.take_first_line(&formatted).over_budget()
                {
                    // Hang the pair, using the original expression for formatting
                    output_expr
                        .push(formatted.map(|_| hang_expression(ctx, original, shape, Some(1))))
                } else {
                    // Add the pair as it is
                    output_expr.push(formatted);
                }
            }

            (output_expr, hanging_equal_token)
        } else {
            (expr_list, hanging_equal_token)
        }
    } else {
        // There is only a single element in the list
        let expression = expressions.iter().next().unwrap();

        // Special case: there is a comment in between the equals and the expression
        if trivia_util::token_contains_comments(&equal_token)
            || !trivia_util::expression_leading_comments(expression).is_empty()
        {
            // We will hang at the equals token, and then format the expression as necessary
            let equal_token = hang_equal_token(ctx, equal_token, shape, false);

            let shape = shape.reset().increment_additional_indent();

            // As we know that there is only a single element in the list, we can extract it to work with it
            let expression = format_expression(ctx, expression, shape);

            // We need to take all the leading trivia from the expr_list
            let (expression, leading_comments) =
                trivia_util::take_expression_leading_comments(&expression);

            // Indent each comment and trail them with a newline
            let leading_comments = leading_comments
                .iter()
                .flat_map(|x| {
                    vec![
                        create_indent_trivia(ctx, shape),
                        x.to_owned(),
                        create_newline_trivia(ctx),
                    ]
                })
                .chain(std::iter::once(create_indent_trivia(ctx, shape)))
                .collect();

            let expression =
                expression.update_leading_trivia(FormatTriviaType::Replace(leading_comments));

            // Rebuild expression back into a list
            let expr_list = std::iter::once(Pair::new(expression, None)).collect();

            return (expr_list, equal_token);
        }

        // Create an example hanging the expression - we need to create a new context so that we don't overwrite it
        let hanging_expr_list = hang_punctuated_list(ctx, expressions, shape);
        let hanging_shape = shape.take_first_line(&strip_trivia(&hanging_expr_list));

        // Create an example formatting the expression normally
        let expr_list = format_punctuated(ctx, expressions, shape, format_expression);
        let formatting_shape = shape.take_first_line(&strip_trailing_trivia(&expr_list));

        // Find the better format out of the hanging shape or the normal formatting
        if hanging_shape.used_width() < formatting_shape.used_width() {
            // Hanging version is better
            (hanging_expr_list, equal_token)
        } else {
            // Normal format is better: but check to see if the formatting is still over budget
            if formatting_shape.over_budget() {
                // Hang at the equal token
                let equal_token = hang_equal_token(ctx, equal_token, shape, true);
                // Add the expression list into the indent range, as it will be indented by one
                let shape = shape.increment_additional_indent();
                let expr_list = format_punctuated(ctx, expressions, shape, format_expression);
                (expr_list, equal_token)
            } else {
                (expr_list, equal_token)
            }
        }
    }
}

pub fn format_assignment(ctx: &Context, assignment: &Assignment, shape: Shape) -> Assignment {
    // Calculate trivia
    // Leading trivia added to before the var_list, trailing trivia added to the end of the expr_list
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Check if the assignment expressions or equal token contain comments. If they do, we bail out of determining any tactics
    // and format multiline
    let contains_comments = trivia_util::token_contains_comments(assignment.equal_token())
        || assignment.expressions().pairs().any(|pair| {
            pair.punctuation()
                .map_or(false, |x| trivia_util::token_contains_comments(x))
                || trivia_util::expression_contains_inline_comments(pair.value())
        });

    // Firstly attempt to format the assignment onto a single line, using an infinite column width shape
    let mut var_list = try_format_punctuated(
        ctx,
        assignment.variables(),
        shape.with_infinite_width(),
        format_var,
        Some(1),
    );
    let mut equal_token = fmt_symbol!(ctx, assignment.equal_token(), " = ", shape);
    let mut expr_list = format_punctuated(
        ctx,
        assignment.expressions(),
        shape.with_infinite_width(),
        format_expression,
    );

    // Test the assignment to see if its over width
    let singleline_shape = shape
        + (strip_leading_trivia(&var_list).to_string().len()
            + 3
            + strip_trailing_trivia(&expr_list).to_string().len());
    if contains_comments || singleline_shape.over_budget() {
        // We won't attempt anything else with the var_list. Format it normally
        var_list = try_format_punctuated(ctx, assignment.variables(), shape, format_var, Some(1));
        let shape = shape + (strip_leading_trivia(&var_list).to_string().len() + 3);

        let (new_expr_list, new_equal_token) =
            attempt_assignment_tactics(ctx, assignment.expressions(), shape, equal_token);
        expr_list = new_expr_list;
        equal_token = new_equal_token;
    }

    // Add necessary trivia
    let var_list = var_list.update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Assignment::new(var_list, expr_list).with_equal_token(equal_token)
}

fn format_local_no_assignment(
    ctx: &Context,
    assignment: &LocalAssignment,
    shape: Shape,
    leading_trivia: Vec<Token>,
    trailing_trivia: Vec<Token>,
) -> LocalAssignment {
    let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let shape = shape + 6; // 6 = "local "
    let mut name_list = try_format_punctuated(
        ctx,
        assignment.names(),
        shape,
        format_token_reference,
        Some(1),
    );

    #[cfg(feature = "luau")]
    let mut type_specifiers: Vec<Option<TypeSpecifier>> = assignment
        .type_specifiers()
        .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
        .collect();

    // See if the last variable assigned has a type specifier, and add a new line to that
    #[allow(unused_mut)]
    let mut new_line_added = false;

    #[cfg(feature = "luau")]
    if let Some(Some(specifier)) = type_specifiers.pop() {
        type_specifiers.push(Some(specifier.update_trailing_trivia(
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        )));
        new_line_added = true;
    }

    // Add any trailing trivia to the end of the expression list, if we haven't already added a newline
    if !new_line_added {
        name_list = name_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia))
    }

    let local_assignment = LocalAssignment::new(name_list)
        .with_local_token(local_token)
        .with_equal_token(None)
        .with_expressions(Punctuated::new());

    #[cfg(feature = "luau")]
    let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
    local_assignment
}

pub fn format_local_assignment(
    ctx: &Context,
    assignment: &LocalAssignment,
    shape: Shape,
) -> LocalAssignment {
    // Calculate trivia
    // Leading trivia added to before the local token, and trailing trivia added to the end of the expr_list, or name_list if no expr_list provided
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    if assignment.expressions().is_empty() {
        format_local_no_assignment(ctx, assignment, shape, leading_trivia, trailing_trivia)
    } else {
        // Check if the assignment expression or equals token contain comments. If they do, we bail out of determining any tactics
        // and format multiline
        let contains_comments = assignment
            .equal_token()
            .map_or(false, |x| trivia_util::token_contains_comments(x))
            || assignment.expressions().pairs().any(|pair| {
                pair.punctuation()
                    .map_or(false, |x| trivia_util::token_contains_comments(x))
                    || trivia_util::expression_contains_inline_comments(pair.value())
            });

        // Firstly attempt to format the assignment onto a single line, using an infinite column width shape
        let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ", shape)
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        let mut name_list = try_format_punctuated(
            ctx,
            assignment.names(),
            shape.with_infinite_width(),
            format_token_reference,
            Some(1),
        );
        let mut equal_token = fmt_symbol!(ctx, assignment.equal_token().unwrap(), " = ", shape);
        let mut expr_list = format_punctuated(
            ctx,
            assignment.expressions(),
            shape.with_infinite_width(),
            format_expression,
        );

        #[cfg(feature = "luau")]
        let type_specifiers: Vec<Option<TypeSpecifier>> = assignment
            .type_specifiers()
            .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
            .collect();
        let type_specifier_len;
        #[cfg(feature = "luau")]
        {
            type_specifier_len = type_specifiers.iter().fold(0, |acc, x| {
                acc + x.as_ref().map_or(0, |y| y.to_string().len())
            });
        }
        #[cfg(not(feature = "luau"))]
        {
            type_specifier_len = 0;
        }

        // Test the assignment to see if its over width
        let singleline_shape = shape
            + (strip_leading_trivia(&name_list).to_string().len()
                + 6 // 6 = "local "
                + 3 // 3 = " = "
                + type_specifier_len
                + strip_trailing_trivia(&expr_list).to_string().len());

        if contains_comments || singleline_shape.over_budget() {
            // We won't attempt anything else with the name_list. Format it normally
            name_list = try_format_punctuated(
                ctx,
                assignment.names(),
                shape,
                format_token_reference,
                Some(1),
            );
            let shape = shape
                + (strip_leading_trivia(&name_list).to_string().len() + 6 + 3 + type_specifier_len);

            let (new_expr_list, new_equal_token) =
                attempt_assignment_tactics(ctx, assignment.expressions(), shape, equal_token);
            expr_list = new_expr_list;
            equal_token = new_equal_token;
        }

        // Add necessary trivia
        let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        let local_assignment = LocalAssignment::new(name_list)
            .with_local_token(local_token)
            .with_equal_token(Some(equal_token))
            .with_expressions(expr_list);
        #[cfg(feature = "luau")]
        let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
        local_assignment
    }
}
