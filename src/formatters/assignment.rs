#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Expression, LocalAssignment,
};
use full_moon::node::Node;
use full_moon::tokenizer::{Token, TokenKind, TokenReference};

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_specifier;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, format_var, hang_expression},
        general::{
            format_punctuated, format_punctuated_multiline, format_token_reference_mut,
            try_format_punctuated,
        },
        trivia::{
            strip_leading_trivia, strip_trailing_trivia, strip_trivia, FormatTriviaType,
            UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
        trivia_util,
        util::token_range,
    },
    shape::Shape,
};

pub fn hang_punctuated_list<'ast>(
    ctx: &mut Context,
    punctuated: &Punctuated<'ast, Expression<'ast>>,
    shape: Shape,
    additional_indent_level: Option<usize>,
) -> Punctuated<'ast, Expression<'ast>> {
    // Add the expression list into the indent range, as it will be indented by one
    let expr_range = punctuated
        .range()
        .expect("no range for assignment punctuated list");
    ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

    let mut output = Punctuated::new();
    let mut shape = shape;

    // Format each expression and hang them
    // We need to format again because we will now take into account the indent increase
    for pair in punctuated.pairs() {
        let value = hang_expression(ctx, pair.value(), shape, additional_indent_level, None);
        shape = shape.take_last_line(&strip_trivia(&value));

        output.push(Pair::new(
            value,
            pair.punctuation().map(|x| fmt_symbol!(ctx, x, ", ")),
        ));
        shape = shape + 2; // 2 = ", "
    }

    output
}

/// Hangs at the equal token, and indents the first item.
/// Returns the new equal token [`TokenReference`]
fn hang_equal_token<'ast>(
    ctx: &mut Context,
    equal_token: TokenReference<'ast>,
    additional_indent_level: Option<usize>,
) -> TokenReference<'ast> {
    let equal_token_trailing_trivia = vec![
        create_newline_trivia(ctx),
        create_indent_trivia(ctx, additional_indent_level.or(Some(0)).map(|x| x + 1)),
    ]
    .iter()
    .chain(
        // Remove the space that was present after the equal token
        equal_token
            .trailing_trivia()
            .skip_while(|x| x.token_kind() == TokenKind::Whitespace),
    )
    .map(|x| x.to_owned())
    .collect();

    equal_token.update_trailing_trivia(FormatTriviaType::Replace(equal_token_trailing_trivia))
}

pub fn format_assignment<'ast>(
    ctx: &mut Context,
    assignment: &Assignment<'ast>,
    shape: Shape,
) -> Assignment<'ast> {
    // Calculate trivia - pick an arbitrary range within the whole assignment expression to see if
    // indentation is required
    // Leading trivia added to before the var_list, trailing trivia added to the end of the expr_list
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(assignment.equal_token().token()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // Check if the assignment expressions contain comments. If they do, we bail out of determining any tactics
    // and format multiline
    let contains_comments = assignment.expressions().pairs().any(|pair| {
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
    );
    let mut equal_token = fmt_symbol!(ctx, assignment.equal_token(), " = ");
    let mut expr_list = format_punctuated(
        ctx,
        assignment.expressions(),
        shape.with_infinite_width(),
        format_expression,
    );

    // Test the assignment to see if its over width
    let singleline_shape = shape
        .add_width(
            strip_leading_trivia(&var_list).to_string().len() + 3, // 3 = " = "
        )
        .take_first_line(&strip_trailing_trivia(&expr_list));

    if contains_comments || singleline_shape.over_budget() {
        // We won't attempt anything else with the var_list. Format it normally
        var_list = try_format_punctuated(ctx, assignment.variables(), shape, format_var);
        let shape = shape + (strip_leading_trivia(&var_list).to_string().len() + 3);
        // The next tactic will be to see if we can hang the expression
        // We can either hang the expression list, or hang at the equals token
        if assignment
            .expressions()
            .iter()
            .any(|x| trivia_util::can_hang_expression(x))
        {
            expr_list = hang_punctuated_list(
                ctx,
                assignment.expressions(),
                shape,
                additional_indent_level,
            );
        } else {
            // The next tactic is to see whether there is more than one item in the punctuated list
            // If there is, we should put it on multiple lines
            if expr_list.len() > 1 {
                // First try hanging at the equal token, using an infinite width, to see if its enough
                let hanging_equal_token =
                    hang_equal_token(ctx, equal_token.to_owned(), additional_indent_level);
                let hanging_shape = shape
                    .reset()
                    .with_additional_indent(Some(additional_indent_level.unwrap_or(0) + 1));
                expr_list = format_punctuated(
                    ctx,
                    assignment.expressions(),
                    hanging_shape.with_infinite_width(),
                    format_expression,
                );

                if hanging_shape
                    .take_first_line(&strip_trivia(&expr_list))
                    .over_budget()
                {
                    // Hang the expressions on multiple lines
                    expr_list = format_punctuated_multiline(
                        ctx,
                        assignment.expressions(),
                        shape,
                        format_expression,
                        Some(1),
                    );
                } else {
                    equal_token = hanging_equal_token;
                }
            } else {
                // Format the expressions normally. If still over budget, hang at the equals token
                expr_list =
                    format_punctuated(ctx, assignment.expressions(), shape, format_expression);
                let formatting_shape = shape.take_first_line(&strip_trailing_trivia(&expr_list));

                if formatting_shape.over_budget() {
                    equal_token = hang_equal_token(ctx, equal_token, additional_indent_level);
                    // Add the expression list into the indent range, as it will be indented by one
                    let expr_range = assignment
                        .expressions()
                        .range()
                        .expect("no range for assignment punctuated list");
                    ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));
                    expr_list =
                        format_punctuated(ctx, assignment.expressions(), shape, format_expression);
                }
            }
        }
    }

    // Add necessary trivia
    let var_list = var_list.update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Assignment::new(var_list, expr_list).with_equal_token(equal_token)
}

fn format_local_no_assignment<'ast>(
    ctx: &mut Context,
    assignment: &LocalAssignment<'ast>,
    shape: Shape,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> LocalAssignment<'ast> {
    let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let shape = shape + 6; // 6 = "local "
    let mut name_list =
        try_format_punctuated(ctx, assignment.names(), shape, format_token_reference_mut);

    #[cfg(feature = "luau")]
    let mut type_specifiers: Vec<Option<TypeSpecifier<'ast>>> = assignment
        .type_specifiers()
        .map(|x| match x {
            Some(type_specifier) => Some(format_type_specifier(ctx, type_specifier)),
            None => None,
        })
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

pub fn format_local_assignment<'ast>(
    ctx: &mut Context,
    assignment: &LocalAssignment<'ast>,
    shape: Shape,
) -> LocalAssignment<'ast> {
    // Calculate trivia - pick an arbitrary range within the whole local assignment expression to see if
    // indentation is required
    // Leading trivia added to before the local token, and trailing trivia added to the end of the expr_list, or name_list if no expr_list provided
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(assignment.local_token().token()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    if assignment.expressions().is_empty() {
        format_local_no_assignment(ctx, assignment, shape, leading_trivia, trailing_trivia)
    } else {
        // Check if the assignment expressions contain comments. If they do, we bail out of determining any tactics
        // and format multiline
        let contains_comments = assignment.expressions().pairs().any(|pair| {
            pair.punctuation()
                .map_or(false, |x| trivia_util::token_contains_comments(x))
                || trivia_util::expression_contains_inline_comments(pair.value())
        });

        // Firstly attempt to format the assignment onto a single line, using an infinite column width shape
        let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ")
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        let mut name_list = try_format_punctuated(
            ctx,
            assignment.names(),
            shape.with_infinite_width(),
            format_token_reference_mut,
        );
        let mut equal_token = fmt_symbol!(ctx, assignment.equal_token().unwrap(), " = ");
        let mut expr_list = format_punctuated(
            ctx,
            assignment.expressions(),
            shape.with_infinite_width(),
            format_expression,
        );

        #[cfg(feature = "luau")]
        let type_specifiers: Vec<Option<TypeSpecifier<'ast>>> = assignment
            .type_specifiers()
            .map(|x| match x {
                Some(type_specifier) => Some(format_type_specifier(ctx, type_specifier)),
                None => None,
            })
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
            .add_width(
                strip_leading_trivia(&name_list).to_string().len()
                + 6 // 6 = "local "
                + 3 // 3 = " = "
                + type_specifier_len,
            )
            .take_first_line(&strip_trailing_trivia(&expr_list));

        if contains_comments || singleline_shape.over_budget() {
            // We won't attempt anything else with the name_list. Format it normally
            name_list =
                try_format_punctuated(ctx, assignment.names(), shape, format_token_reference_mut);
            let shape = shape
                + (strip_leading_trivia(&name_list).to_string().len() + 6 + 3 + type_specifier_len);
            // The next tactic will be to see if we can hang the expression
            // We can either hang the expression list, or hang at the equals token
            if assignment
                .expressions()
                .iter()
                .any(|x| trivia_util::can_hang_expression(x))
            {
                expr_list = hang_punctuated_list(
                    ctx,
                    assignment.expressions(),
                    shape,
                    additional_indent_level,
                );
            } else {
                // The next tactic is to see whether there is more than one item in the punctuated list
                // If there is, we should put it on multiple lines
                if expr_list.len() > 1 {
                    // First try hanging at the equal token, using an infinite width, to see if its enough
                    let hanging_equal_token =
                        hang_equal_token(ctx, equal_token.to_owned(), additional_indent_level);
                    let hanging_shape = shape
                        .reset()
                        .with_additional_indent(Some(additional_indent_level.unwrap_or(0) + 1));
                    expr_list = format_punctuated(
                        ctx,
                        assignment.expressions(),
                        hanging_shape.with_infinite_width(),
                        format_expression,
                    );

                    if hanging_shape
                        .take_first_line(&strip_trivia(&expr_list))
                        .over_budget()
                    {
                        // Hang the expressions on multiple lines
                        expr_list = format_punctuated_multiline(
                            ctx,
                            assignment.expressions(),
                            shape,
                            format_expression,
                            Some(1),
                        );
                    } else {
                        equal_token = hanging_equal_token;
                    }
                } else {
                    // Format the expressions normally. If still over budget, hang at the equals token
                    expr_list =
                        format_punctuated(ctx, assignment.expressions(), shape, format_expression);
                    let formatting_shape =
                        shape.take_first_line(&strip_trailing_trivia(&expr_list));

                    if formatting_shape.over_budget() {
                        equal_token = hang_equal_token(ctx, equal_token, additional_indent_level);
                        // Add the expression list into the indent range, as it will be indented by one
                        let expr_range = assignment
                            .expressions()
                            .range()
                            .expect("no range for assignment punctuated list");
                        ctx.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));
                        expr_list = format_punctuated(
                            ctx,
                            assignment.expressions(),
                            shape,
                            format_expression,
                        );
                    }
                }
            }
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
