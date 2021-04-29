#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Expression, LocalAssignment,
};
use full_moon::node::Node;
use full_moon::tokenizer::{TokenKind, TokenReference};

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_specifier;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, format_var, hang_expression},
        general::{format_punctuated, format_token_reference_mut, try_format_punctuated},
        trivia::{strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        trivia_util,
        util::token_range,
    },
    shape::Shape,
};

/// Returns an Assignment with leading and trailing trivia removed
fn strip_assignment_trivia<'ast>(assignment: &Assignment<'ast>) -> Assignment<'ast> {
    let var_list = assignment
        .variables()
        .update_leading_trivia(FormatTriviaType::Replace(vec![]));
    let expr_list = assignment
        .expressions()
        .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

    Assignment::new(var_list, expr_list).with_equal_token(assignment.equal_token().to_owned())
}

/// Returns a LocalAssignment with leading and trailing trivia removed
fn strip_local_assignment_trivia<'ast>(
    local_assignment: &LocalAssignment<'ast>,
) -> LocalAssignment<'ast> {
    let local_token = local_assignment
        .local_token()
        .update_leading_trivia(FormatTriviaType::Replace(vec![]));

    if local_assignment.expressions().is_empty() {
        let name_list = local_assignment
            .names()
            .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

        LocalAssignment::new(name_list).with_local_token(local_token)
    } else {
        let expr_list = local_assignment
            .expressions()
            .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

        LocalAssignment::new(local_assignment.names().to_owned())
            .with_local_token(local_token)
            .with_equal_token(local_assignment.equal_token().map(|x| x.to_owned()))
            .with_expressions(expr_list)
    }
}

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
        let expr = format_expression(ctx, pair.value(), shape);
        let value = hang_expression(ctx, expr, additional_indent_level, shape, None);
        shape = shape.take_last_line(&strip_trivia(&value));

        output.push(Pair::new(
            value,
            pair.punctuation().map(|x| fmt_symbol!(ctx, x, ", ")),
        ));
        shape = shape + 2; // 2 = ", "
    }

    output
}

/// Checks the list of assigned expressions to see if any were hangable.
/// If not, then we still have a long list of assigned expressions - we split it onto a newline at the equal token.
/// Returns the new equal token [`TokenReference`]
fn check_long_expression<'ast>(
    ctx: &mut Context,
    expressions: &Punctuated<'ast, Expression<'ast>>,
    equal_token: TokenReference<'ast>,
    additional_indent_level: Option<usize>,
) -> TokenReference<'ast> {
    // See if any of our expressions were hangable.
    // If not, then its still a big long line - we should newline at the end of the equals token,
    // then indent the first item
    if !expressions
        .iter()
        .any(|x| trivia_util::can_hang_expression(x))
    {
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
    } else {
        equal_token
    }
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

    let var_list = try_format_punctuated(ctx, assignment.variables(), shape, format_var);
    let mut equal_token = fmt_symbol!(ctx, assignment.equal_token(), " = ");
    let shape = shape + (strip_trivia(&var_list).to_string().len() + 3); // 3 = " = "

    let mut expr_list = format_punctuated(ctx, assignment.expressions(), shape, format_expression); // Don't need to worry about comments in expr_list, as it will automatically force multiline

    // Create preliminary assignment
    let formatted_assignment = Assignment::new(var_list.to_owned(), expr_list.to_owned())
        .with_equal_token(equal_token.to_owned());

    // Test whether we need to hang the expression, using the updated assignment
    // We have to format normally before this, since we may be expanding the expression onto multiple lines
    // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
    let require_multiline_expression = shape
        .reset()
        .take_first_line(&strip_assignment_trivia(&formatted_assignment))
        .over_budget()
        || assignment.expressions().pairs().any(|pair| {
            pair.punctuation()
                .map_or(false, |punc| trivia_util::token_contains_comments(punc))
                || trivia_util::expression_contains_inline_comments(pair.value())
        });

    if require_multiline_expression {
        expr_list = hang_punctuated_list(
            ctx,
            assignment.expressions(),
            shape,
            additional_indent_level,
        );

        equal_token = check_long_expression(
            ctx,
            assignment.expressions(),
            equal_token,
            additional_indent_level,
        );
    }

    // Add any trailing trivia to the end of the expression list
    let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    // Add on leading trivia
    let formatted_var_list =
        var_list.update_leading_trivia(FormatTriviaType::Append(leading_trivia));

    formatted_assignment
        .with_variables(formatted_var_list)
        .with_equal_token(equal_token)
        .with_expressions(expr_list)
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

    if assignment.expressions().is_empty() {
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
    } else {
        let mut equal_token = fmt_symbol!(ctx, assignment.equal_token().unwrap(), " = ");
        let shape = shape + (strip_trivia(&name_list).to_string().len() + 3); // 3 = " = "
        let mut expr_list =
            // Format the expression normally - if there are any comments, it will automatically force multiline
            format_punctuated(ctx, assignment.expressions(), shape, format_expression);

        // Create our preliminary new assignment
        let local_assignment = LocalAssignment::new(name_list)
            .with_local_token(local_token)
            .with_equal_token(Some(equal_token.to_owned()))
            .with_expressions(expr_list.to_owned());
        #[cfg(feature = "luau")]
        let local_assignment = local_assignment.with_type_specifiers(type_specifiers);

        // Test whether we need to hang the expression, using the updated assignment
        // We have to format normally before this, since we may be expanding the expression onto multiple lines
        // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
        let require_multiline_expression = shape
            .reset()
            .take_first_line(&strip_local_assignment_trivia(&local_assignment))
            .over_budget()
            || assignment.expressions().pairs().any(|pair| {
                pair.punctuation()
                    .map_or(false, |punc| trivia_util::token_contains_comments(punc))
                    || trivia_util::expression_contains_inline_comments(pair.value())
            });

        // Format the expression depending on whether we are multline or not
        if require_multiline_expression {
            expr_list = hang_punctuated_list(
                ctx,
                assignment.expressions(),
                shape,
                additional_indent_level,
            );

            equal_token = check_long_expression(
                ctx,
                assignment.expressions(),
                equal_token,
                additional_indent_level,
            );
        }

        // Add any trailing trivia to the end of the expression list
        let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        // Update our local assignment
        local_assignment
            .with_equal_token(Some(equal_token))
            .with_expressions(expr_list)
    }
}
