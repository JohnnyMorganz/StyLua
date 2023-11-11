#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::tokenizer::{Token, TokenReference};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        Assignment, Call, Expression, FunctionArgs, FunctionCall, LocalAssignment, Suffix,
    },
    tokenizer::TokenType,
};

#[cfg(feature = "lua54")]
use crate::formatters::lua54::format_attribute;
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
            UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{
            self, prepend_newline_indent, CommentSearch, GetLeadingTrivia, GetTrailingTrivia,
            HasInlineComments,
        },
    },
    shape::Shape,
};

/// Calculates the hanging level to use when hanging an expression.
/// By default, we indent one further, but we DO NOT want to do this if the expression is just parentheses (or a unary operation on them)
/// https://github.com/JohnnyMorganz/StyLua/issues/274
pub fn calculate_hang_level(expression: &Expression) -> Option<usize> {
    match expression {
        Expression::Parentheses { .. } => None,
        Expression::UnaryOperator { expression, .. } => calculate_hang_level(expression),
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { expression, .. } => calculate_hang_level(expression),
        _ => Some(1),
    }
}

/// Hangs each [`Expression`] in a [`Punctuated`] list.
/// The Punctuated list is hung multiline at the comma as well, and each subsequent item after the first is
/// indented by one.
pub fn hang_punctuated_list(
    ctx: &Context,
    punctuated: &Punctuated<Expression>,
    shape: Shape,
) -> Punctuated<Expression> {
    // WE ACTUALLY ONLY CALL THIS WHEN THE PUNCTUATED LIST HAS ONE ELEMENT
    // SO LETS ENFORCE THIS INVARIANT FOR NOW
    assert!(punctuated.len() == 1);

    let mut output = Punctuated::new();

    // Format each expression and hang them
    // We need to format again because we will now take into account the indent increase
    for (idx, pair) in punctuated.pairs().enumerate() {
        // TODO: UNCOMMENT THIS IF THE INVARIANT ABOVE IS REMOVED
        // let shape = if idx == 0 {
        //     shape
        // } else if calculate_hang_level(pair.value()).is_some() {
        //     shape.reset().increment_additional_indent()
        // } else {
        //     shape.reset()
        // };

        let mut value =
            hang_expression(ctx, pair.value(), shape, calculate_hang_level(pair.value()));
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
pub fn hang_equal_token(
    ctx: &Context,
    equal_token: &TokenReference,
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

fn is_complex_function_call(function_call: &FunctionCall) -> bool {
    let test_function_args = |function_args: &FunctionArgs| match function_args {
        FunctionArgs::Parentheses { arguments, .. } => {
            let mut complexity_count = 0;

            for argument in arguments {
                match argument {
                    Expression::Function(_) => return true,
                    Expression::TableConstructor(_) => complexity_count += 1,
                    // TODO: should we handle embedded expr in Expression::TypeAssertion { expression } here?
                    _ => (),
                }
            }

            complexity_count > 1
        }
        _ => false,
    };

    function_call.suffixes().any(|suffix| match suffix {
        Suffix::Call(Call::AnonymousCall(function_args)) => test_function_args(function_args),
        Suffix::Call(Call::MethodCall(method_call)) => test_function_args(method_call.args()),
        _ => false,
    })
}

/// Determines whether we should prevent hanging at the equals token depending on the RHS expression
fn prevent_equals_hanging(expression: &Expression) -> bool {
    match expression {
        Expression::Function(_) => true,
        Expression::FunctionCall(function_call) => is_complex_function_call(function_call),
        #[cfg(feature = "luau")]
        Expression::IfExpression(_) => true,
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { expression, .. } => prevent_equals_hanging(expression),
        _ => false,
    }
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
        let hanging_equal_token = hang_equal_token(ctx, &equal_token, shape, true);
        let hanging_shape = shape.reset().increment_additional_indent();
        let expr_list = format_punctuated(
            ctx,
            expressions,
            hanging_shape.with_infinite_width(),
            format_expression,
        );

        if trivia_util::punctuated_inline_comments(expressions, true)
            || hanging_shape
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
                let shape = hanging_shape.reset();

                if formatted.value().has_inline_comments()
                    || shape
                        .take_first_line(&strip_leading_trivia(formatted.value()))
                        .over_budget()
                {
                    // Hang the pair, using the original expression for formatting
                    output_expr.push(formatted.map(|_| {
                        let expression =
                            hang_expression(ctx, original, shape, calculate_hang_level(original));
                        if idx == 0 {
                            expression
                        } else {
                            trivia_util::prepend_newline_indent(ctx, &expression, shape)
                        }
                    }))
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
            || expression.has_leading_comments(CommentSearch::Single)
        {
            // We will hang at the equals token, and then format the expression as necessary
            let equal_token = hang_equal_token(ctx, &equal_token, shape, false);

            let shape = shape.reset().increment_additional_indent();

            // As we know that there is only a single element in the list, we can extract it to work with it
            // Format the expression given - if it contains comments, make sure to hang the expression
            // Ignore the leading comments though (as they are solved by hanging at the equals), and the
            // trailing comments, as they don't affect anything
            let expression = if strip_trivia(expression).has_inline_comments() {
                hang_expression(ctx, expression, shape, None)
            } else {
                format_expression(ctx, expression, shape)
            };

            // We need to take all the leading trivia from the expr_list
            let (expression, leading_comments) = trivia_util::take_leading_comments(&expression);

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

        let expr_list = format_punctuated(ctx, expressions, shape, format_expression);
        let formatting_shape = shape.take_first_line(&strip_trailing_trivia(&expr_list));

        // See if we can hang the expression, and if we can, check whether hanging or formatting normally is nicer
        if trivia_util::can_hang_expression(expression) {
            // Create an example hanging the expression - we need to create a new context so that we don't overwrite it
            let hanging_expr_list = hang_punctuated_list(ctx, expressions, shape);
            let hanging_shape = shape.take_first_line(&strip_trivia(&hanging_expr_list));

            if expression.has_inline_comments()
                || hanging_shape.used_width() < formatting_shape.used_width()
            {
                (hanging_expr_list, equal_token)
            } else {
                // TODO: should we hang at equals token?
                (expr_list, equal_token)
            }
        } else if prevent_equals_hanging(expression) {
            (expr_list, equal_token)
        } else {
            // Try both formatting normally, and hanging at the equals token
            let hanging_equal_token = hang_equal_token(ctx, &equal_token, shape, true);
            let equal_token_shape = shape.reset().increment_additional_indent();
            let hanging_equal_token_expr_list =
                format_punctuated(ctx, expressions, equal_token_shape, format_expression);
            let equal_token_shape = equal_token_shape
                .take_first_line(&strip_trailing_trivia(&hanging_equal_token_expr_list));

            // If hanging at the equal token doesn't go over budget, and it produces less lines than hanging normally
            // then go for that instead
            if !equal_token_shape.over_budget()
                && format!("{hanging_equal_token_expr_list}").lines().count() + 1 // Add an extra line since we are hanging
                    < format!("{expr_list}").lines().count()
                || formatting_shape.over_budget()
            {
                (hanging_equal_token_expr_list, hanging_equal_token)
            } else {
                (expr_list, equal_token)
            }
        }
    }
}

pub fn format_assignment_no_trivia(
    ctx: &Context,
    assignment: &Assignment,
    mut shape: Shape,
) -> Assignment {
    // Check if the assignment expressions or equal token contain comments. If they do, we bail out of determining any tactics
    // and format multiline
    let contains_comments = trivia_util::token_contains_comments(assignment.equal_token())
        || trivia_util::punctuated_inline_comments(assignment.expressions(), true);

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

    // If the var list ended with a comment, we need to hang the equals token
    if var_list.has_trailing_comments(trivia_util::CommentSearch::Single) {
        const EQUAL_TOKEN_LEN: usize = "= ".len();
        shape = shape
            .reset()
            .increment_additional_indent()
            .add_width(EQUAL_TOKEN_LEN);
        equal_token = prepend_newline_indent(ctx, &equal_token, shape);
    }

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

    Assignment::new(var_list, expr_list).with_equal_token(equal_token)
}

pub fn format_assignment(ctx: &Context, assignment: &Assignment, shape: Shape) -> Assignment {
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    format_assignment_no_trivia(ctx, assignment, shape).update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(trailing_trivia),
    )
}

fn format_local_no_assignment(
    ctx: &Context,
    assignment: &LocalAssignment,
    shape: Shape,
) -> LocalAssignment {
    let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ", shape);
    let shape = shape + 6; // 6 = "local "
    let name_list = try_format_punctuated(
        ctx,
        assignment.names(),
        shape,
        format_token_reference,
        Some(1),
    );

    #[cfg(feature = "lua54")]
    let attributes = assignment
        .attributes()
        .map(|x| x.map(|attribute| format_attribute(ctx, attribute, shape)))
        .collect();

    #[cfg(feature = "luau")]
    let type_specifiers: Vec<Option<TypeSpecifier>> = assignment
        .type_specifiers()
        .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
        .collect();

    let local_assignment = LocalAssignment::new(name_list);
    #[cfg(feature = "lua54")]
    let local_assignment = local_assignment.with_attributes(attributes);
    #[cfg(feature = "luau")]
    let local_assignment = local_assignment.with_type_specifiers(type_specifiers);

    local_assignment
        .with_local_token(local_token)
        .with_equal_token(None)
        .with_expressions(Punctuated::new())
}

pub fn format_local_assignment_no_trivia(
    ctx: &Context,
    assignment: &LocalAssignment,
    mut shape: Shape,
) -> LocalAssignment {
    if assignment.expressions().is_empty() {
        format_local_no_assignment(ctx, assignment, shape)
    } else {
        // Check if the assignment expression or equals token contain comments. If they do, we bail out of determining any tactics
        // and format multiline
        let contains_comments = assignment
            .equal_token()
            .map_or(false, trivia_util::token_contains_comments)
            || trivia_util::punctuated_inline_comments(assignment.expressions(), true);

        // Firstly attempt to format the assignment onto a single line, using an infinite column width shape
        let local_token = fmt_symbol!(ctx, assignment.local_token(), "local ", shape);

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

        #[cfg(feature = "lua54")]
        let attributes: Vec<Option<_>> = assignment
            .attributes()
            .map(|x| x.map(|attribute| format_attribute(ctx, attribute, shape)))
            .collect();

        #[cfg(feature = "luau")]
        let type_specifiers: Vec<Option<TypeSpecifier>> = assignment
            .type_specifiers()
            .map(|x| x.map(|type_specifier| format_type_specifier(ctx, type_specifier, shape)))
            .collect();

        #[allow(unused_mut)]
        let mut type_specifier_len = 0;
        #[cfg(feature = "lua54")]
        {
            type_specifier_len += attributes.iter().fold(0, |acc, x| {
                acc + x.as_ref().map_or(0, |y| y.to_string().len())
            });
        }
        #[cfg(feature = "luau")]
        {
            type_specifier_len += type_specifiers.iter().fold(0, |acc, x| {
                acc + x.as_ref().map_or(0, |y| y.to_string().len())
            });
        }

        // If the var list ended with a comment, we need to hang the equals token
        if name_list.has_trailing_comments(trivia_util::CommentSearch::Single) {
            const EQUAL_TOKEN_LEN: usize = "= ".len();
            shape = shape
                .reset()
                .increment_additional_indent()
                .add_width(EQUAL_TOKEN_LEN);
            equal_token = prepend_newline_indent(ctx, &equal_token, shape);
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

        let local_assignment = LocalAssignment::new(name_list);
        #[cfg(feature = "lua54")]
        let local_assignment = local_assignment.with_attributes(attributes);
        #[cfg(feature = "luau")]
        let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
        local_assignment
            .with_local_token(local_token)
            .with_equal_token(Some(equal_token))
            .with_expressions(expr_list)
    }
}

pub fn format_local_assignment(
    ctx: &Context,
    assignment: &LocalAssignment,
    shape: Shape,
) -> LocalAssignment {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    format_local_assignment_no_trivia(ctx, assignment, shape).update_trivia(
        FormatTriviaType::Append(leading_trivia),
        FormatTriviaType::Append(trailing_trivia),
    )
}
