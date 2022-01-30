use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
    LocalFunction, MethodCall, Parameter, Suffix, Value,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::luau::{format_generic_declaration, format_type_specifier};
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        block::format_block,
        expression::{format_expression, format_prefix, format_suffix, hang_expression},
        general::{
            format_contained_span, format_end_token, format_punctuated, format_symbol,
            format_token_reference, EndTokenType,
        },
        table::format_table_constructor,
        trivia::{
            strip_leading_trivia, strip_trivia, FormatTriviaType, UpdateLeadingTrivia,
            UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util,
    },
    shape::Shape,
};

/// Formats an Anonymous Function
/// This doesn't have its own struct, but it is part of Value::Function
pub fn format_anonymous_function(
    ctx: &Context,
    function_token: &TokenReference,
    function_body: &FunctionBody,
    shape: Shape,
) -> (TokenReference, FunctionBody) {
    let function_token = fmt_symbol!(ctx, function_token, "function", shape);
    let function_body = format_function_body(ctx, function_body, false, shape.reset()); // TODO: do we want to reset this shape?

    (function_token, function_body)
}

/// An enum providing information regarding the next AST node after a function call.
/// Currently, this information is only useful for the `no_call_parentheses` configuration, to determine whether
/// to remove parentheses.
pub enum FunctionCallNextNode {
    /// The syntax is obscure if we remove parentheses around a function call due to the next AST node.
    /// For example, the next AST node could be an index or a method call:
    /// ```lua
    /// getsomething "foobar".setup -> getsomething("foobar").setup
    /// setup { yes = true }:run() -> setup({ yes = true }):run()
    /// ```
    /// It looks like we are indexing the string, or calling a method on the table, but these are actually applied
    /// to the returned value from the call. Removing the parentheses around the arguments to the call makes this obscure.
    ObscureWithoutParens,

    /// There is no important information regarding the next node
    None,
}

/// Formats a Call node
pub fn format_call(
    ctx: &Context,
    call: &Call,
    shape: Shape,
    call_next_node: FunctionCallNextNode,
) -> Call {
    match call {
        Call::AnonymousCall(function_args) => Call::AnonymousCall(format_function_args(
            ctx,
            function_args,
            shape,
            call_next_node,
        )),
        Call::MethodCall(method_call) => {
            Call::MethodCall(format_method_call(ctx, method_call, shape, call_next_node))
        }
        other => panic!("unknown node {:?}", other),
    }
}

fn is_table_constructor(expression: &Expression) -> bool {
    match expression {
        Expression::Value { value, .. } => matches!(**value, Value::TableConstructor(_)),
        _ => false,
    }
}

fn is_complex_arg(value: &Value) -> bool {
    value.to_string().trim().contains('\n')
}

/// Formats a FunctionArgs node.
/// [`call_next_node`] provides information about the node after the FunctionArgs. This only matters if the configuration specifies no call parentheses.
pub fn format_function_args(
    ctx: &Context,
    function_args: &FunctionArgs,
    shape: Shape,
    call_next_node: FunctionCallNextNode,
) -> FunctionArgs {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => {
            // Handle config where parentheses are omitted, and there is only one argument
            if (ctx.should_omit_string_parens() || ctx.should_omit_table_parens())
                && arguments.len() == 1
                && !matches!(call_next_node, FunctionCallNextNode::ObscureWithoutParens)
            {
                let argument = arguments.iter().next().unwrap();
                if let Expression::Value { value, .. } = argument {
                    match &**value {
                        Value::String(token_reference) => {
                            if ctx.should_omit_string_parens() {
                                return format_function_args(
                                    ctx,
                                    &FunctionArgs::String(token_reference.to_owned()),
                                    shape,
                                    call_next_node,
                                );
                            }
                        }
                        Value::TableConstructor(table_constructor) => {
                            if ctx.should_omit_table_parens() {
                                return format_function_args(
                                    ctx,
                                    &FunctionArgs::TableConstructor(table_constructor.to_owned()),
                                    shape,
                                    call_next_node,
                                );
                            }
                        }
                        _ => (),
                    }
                }
            }

            let (start_parens, end_parens) = parentheses.tokens();

            // Format all the arguments on an infinite width, so that we can prepare them and check to see whether they
            // need expanding. We will ignore punctuation for now
            let mut first_iter_formatted_arguments = Vec::new();
            let infinite_shape = shape.with_infinite_width();
            for argument in arguments.iter() {
                let argument = format_expression(ctx, argument, infinite_shape);
                first_iter_formatted_arguments.push(argument);
            }

            // Apply some heuristics to determine whether we should expand the function call
            // TODO: These are subject to change

            // If there is a comment present anywhere in between the start parentheses and end parentheses, we should keep it multiline
            let force_mutliline: bool =
                if trivia_util::token_trivia_contains_comments(start_parens.trailing_trivia())
                    || trivia_util::token_trivia_contains_comments(end_parens.leading_trivia())
                {
                    true
                } else {
                    let mut contains_comments = false;
                    for argument in arguments.pairs() {
                        // Only check the leading and trailing trivia of the expression
                        // If the expression has inline comments, it should be handled elsewhere
                        // Allow this, as this is what rustfmt creates
                        #[allow(clippy::blocks_in_if_conditions)]
                        if trivia_util::get_expression_leading_trivia(argument.value())
                            .iter()
                            .chain(
                                trivia_util::get_expression_trailing_trivia(argument.value())
                                    .iter(),
                            )
                            .any(trivia_util::trivia_is_comment)
                        {
                            contains_comments = true;
                        } else if let Some(punctuation) = argument.punctuation() {
                            if trivia_util::token_contains_comments(punctuation) {
                                contains_comments = true;
                            }
                        };

                        if contains_comments {
                            break;
                        }
                    }

                    contains_comments
                };

            let mut is_multiline = force_mutliline;
            let mut singleline_shape = shape + 1; // 1 = opening parentheses

            if !force_mutliline {
                // If we have no arguments, then we can skip hanging multiline
                if first_iter_formatted_arguments.is_empty() {
                    is_multiline = false;
                } else {
                    // Find how far we are currently indented, we can use this to determine when to expand
                    // We will expand on two occasions:
                    // 1) If a group of arguments fall on a single line, and they surpass the column width setting
                    // 2) If we have a mixture of multiline (tables/anonymous functions) and other values. For
                    //    example, call({ ... }, foo, { ... }), should be expanded, but
                    //    call(foo, { ... }) or call(foo, { ... }, foo) can stay on one line, provided the
                    //    single line arguments dont surpass the column width setting

                    // Use state values to determine the type of arguments we have seen so far
                    let mut seen_multiline_arg = false; // Whether we have seen a multiline table/function already
                    let mut seen_other_arg_after_multiline = false; // Whether we have seen a non multiline table/function after a multiline one. In this case, we should expand

                    for argument in first_iter_formatted_arguments.iter() {
                        match argument {
                            Expression::Value { value, .. } => {
                                match &**value {
                                    // Check to see if we have a table constructor, or anonymous function
                                    Value::Function((_, function_body)) => {
                                        // Check to see whether it has been expanded
                                        let is_expanded =
                                            !trivia_util::is_function_empty(function_body);
                                        if is_expanded {
                                            // If we have a mixture of multiline args, and other arguments
                                            // Then the function args should be expanded
                                            if seen_multiline_arg && seen_other_arg_after_multiline
                                            {
                                                is_multiline = true;
                                                break;
                                            }

                                            seen_multiline_arg = true;

                                            // First check the top line of the anonymous function (i.e. the function token and any parameters)
                                            // If this is over budget, then we should expand
                                            singleline_shape =
                                                singleline_shape.take_first_line(value);
                                            if singleline_shape.over_budget() {
                                                is_multiline = true;
                                                break;
                                            }

                                            // Reset the shape onto a new line // 3 = "end" for the function line
                                            singleline_shape = singleline_shape.reset() + 3;
                                        } else {
                                            // We have a collapsed function (normally indicitive of a noop function)
                                            // add the width, and if it fails, we need to expand
                                            singleline_shape =
                                                singleline_shape + argument.to_string().len();
                                            if singleline_shape.over_budget() {
                                                is_multiline = true;
                                                break;
                                            }
                                        }
                                    }
                                    Value::TableConstructor(table) => {
                                        // Check to see whether it has been expanded
                                        let start_brace = table.braces().tokens().0;
                                        let is_expanded = trivia_util::trivia_contains_newline(
                                            start_brace.trailing_trivia(),
                                        );
                                        if is_expanded {
                                            // If we have a mixture of multiline args, and other arguments
                                            // Then the function args should be expanded
                                            if seen_multiline_arg && seen_other_arg_after_multiline
                                            {
                                                is_multiline = true;
                                                break;
                                            }

                                            seen_multiline_arg = true;

                                            // Reset the shape onto a new line
                                            singleline_shape = singleline_shape.reset() + 1;
                                        // 1 = "}"
                                        } else {
                                            // We have a collapsed table constructor - add the width, and if it fails,
                                            // we need to expand
                                            singleline_shape =
                                                singleline_shape + argument.to_string().len();
                                            if singleline_shape.over_budget() {
                                                is_multiline = true;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        // If we previously had a table/anonymous function, and we have something else
                                        // in the mix, update the state to respond to this
                                        if seen_multiline_arg {
                                            seen_other_arg_after_multiline = true;
                                        }

                                        // If the argument is complex (spans multiple lines), then we will immediately
                                        // exit and span multiline - it is most likely too complex to keep going forward.
                                        if is_complex_arg(value) && arguments.len() > 1 {
                                            is_multiline = true;
                                            break;
                                        }

                                        // Take the first line to see if we are over budget
                                        if singleline_shape.take_first_line(argument).over_budget()
                                        {
                                            is_multiline = true;
                                            break;
                                        }

                                        // Set the shape to the last line, then examine if over budget
                                        singleline_shape =
                                            singleline_shape.take_last_line(argument);
                                        if singleline_shape.over_budget() {
                                            is_multiline = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            // TODO: Parentheses/UnOp, do we need to do more checking?
                            // We will continue counting on the width_passed
                            _ => {
                                // If we previously had a table/anonymous function, and we have something else
                                // in the mix, update the state to respond to this
                                if seen_multiline_arg {
                                    seen_other_arg_after_multiline = true;
                                }

                                // Take the first line to see if we are over budget
                                if singleline_shape.take_first_line(argument).over_budget() {
                                    is_multiline = true;
                                    break;
                                }

                                // Set the shape to the last line, then examine if over budget
                                singleline_shape = singleline_shape.take_last_line(argument);
                                if singleline_shape.over_budget() {
                                    is_multiline = true;
                                    break;
                                }
                            }
                        }

                        // Add width which would be taken up by comment and space
                        singleline_shape = singleline_shape + 2;
                    }

                    // Check the final shape to see if its over budget
                    // -1 because we added +2 for ", " in the last iteration, but we don't want a trailing space and the comma is replaced with a parentheses
                    if singleline_shape.sub_width(1).over_budget() {
                        is_multiline = true;
                    }
                }
            }

            // Handle special case: we want to go multiline, but we have a single argument which is a table constructor
            // In this case, we want to hug the table braces with the parentheses.
            // To do this, we format single line, but include the closing parentheses in the shape
            let hug_table_constructor = is_multiline
                && !force_mutliline
                && arguments.len() == 1
                && is_table_constructor(arguments.iter().next().unwrap());

            if is_multiline && !hug_table_constructor {
                // Format start and end brace properly with correct trivia
                // Calculate to see if the end parentheses requires any additional indentation
                let end_parens_leading_trivia = vec![create_indent_trivia(ctx, shape)];

                // Add new_line trivia to start_parens
                let start_parens_token = fmt_symbol!(ctx, start_parens, "(", shape)
                    .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(
                        ctx,
                    )]));

                let end_parens_token =
                    format_end_token(ctx, end_parens, EndTokenType::ClosingParens, shape)
                        .update_leading_trivia(FormatTriviaType::Append(end_parens_leading_trivia));

                let parentheses = ContainedSpan::new(start_parens_token, end_parens_token);

                let mut formatted_arguments = Punctuated::new();
                let shape = shape.increment_additional_indent();

                for argument in arguments.pairs() {
                    let shape = shape.reset(); // Argument is on a new line, so reset the shape

                    // First format the argument assuming infinite width
                    let infinite_width_argument =
                        format_expression(ctx, argument.value(), shape.with_infinite_width());

                    // If the argument fits, great! Otherwise, see if we can hang the expression
                    // If we can, use that instead (as it provides a nicer output). If not, format normally without infinite width
                    let formatted_argument = if shape
                        .add_width(strip_trivia(&infinite_width_argument).to_string().len())
                        .over_budget()
                    {
                        if trivia_util::can_hang_expression(argument.value()) {
                            hang_expression(ctx, argument.value(), shape, Some(1))
                        } else {
                            format_expression(ctx, argument.value(), shape)
                        }
                    } else {
                        infinite_width_argument
                    }
                    .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                        ctx, shape,
                    )]));

                    // Take any trailing trivia (i.e. comments) from the argument, and append it to the end of the punctuation
                    let (formatted_argument, mut trailing_comments) =
                        trivia_util::take_expression_trailing_comments(&formatted_argument);

                    let punctuation = match argument.punctuation() {
                        Some(punctuation) => {
                            // Continue adding a comma and a new line for multiline function args
                            // Also add any trailing comments we have taken from the expression
                            trailing_comments.push(create_newline_trivia(ctx));
                            let symbol = fmt_symbol!(ctx, punctuation, ",", shape)
                                .update_trailing_trivia(FormatTriviaType::Append(
                                    trailing_comments,
                                ));

                            Some(symbol)
                        }
                        None => Some(TokenReference::new(
                            trailing_comments,
                            create_newline_trivia(ctx),
                            vec![],
                        )),
                    };

                    formatted_arguments.push(Pair::new(formatted_argument, punctuation))
                }

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments: formatted_arguments,
                }
            } else {
                // We don't need to worry about comments here, as if there were comments present, we would have
                // multiline function args

                // If we are hugging a table constructor with the parentheses, we use a shape increment of 2 to include the closing
                // parentheses aswell. Otherwise, we just use 1 = opening parentheses.
                let shape_increment = if hug_table_constructor { 2 } else { 1 };

                let parentheses = format_contained_span(ctx, parentheses, shape);
                let mut arguments =
                    format_punctuated(ctx, arguments, shape + shape_increment, format_expression);

                // HACK: if there was more than one newline before each argument, then it will be incorrectly preserved
                // leading to weird formatting (https://github.com/JohnnyMorganz/StyLua/issues/290#issuecomment-964428535)
                // We get around this (badly) by reformatting each argument to remove leading newlines from them.
                // TODO(#169): once a proper fix to https://github.com/JohnnyMorganz/StyLua/issues/169 is solved
                // this can be removed.
                for argument in arguments.pairs_mut() {
                    let expression = argument.value_mut();
                    let trivia = trivia_util::get_expression_leading_trivia(expression)
                        .iter()
                        .skip_while(|trivia| trivia_util::trivia_is_whitespace(trivia))
                        .map(|x| x.to_owned())
                        .collect();
                    *expression =
                        expression.update_leading_trivia(FormatTriviaType::Replace(trivia));
                }

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments,
                }
            }
        }

        FunctionArgs::String(token_reference) => {
            if ctx.should_omit_string_parens()
                && !matches!(call_next_node, FunctionCallNextNode::ObscureWithoutParens)
            {
                let token_reference = format_token_reference(ctx, token_reference, shape)
                    .update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                        TokenType::spaces(1),
                    )])); // Single space before the token reference

                return FunctionArgs::String(token_reference);
            }

            let mut arguments = Punctuated::new();
            let new_expression = format_expression(
                ctx,
                &Expression::Value {
                    value: Box::new(Value::String(token_reference.to_owned())),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                },
                shape + 1, // 1 = opening parentheses
            );

            // Remove any trailing comments from the expression, and move them into a buffer
            let (new_expression, comments_buffer) =
                trivia_util::take_expression_trailing_comments(&new_expression);

            // Create parentheses, and add the trailing comments to the end of the parentheses
            let parentheses = ContainedSpan::new(
                TokenReference::symbol("(").unwrap(),
                TokenReference::symbol(")").unwrap(),
            )
            .update_trailing_trivia(FormatTriviaType::Append(comments_buffer));

            arguments.push(Pair::new(new_expression, None)); // Only single argument, so no trailing comma

            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            }
        }

        FunctionArgs::TableConstructor(table_constructor) => {
            if ctx.should_omit_table_parens()
                && !matches!(call_next_node, FunctionCallNextNode::ObscureWithoutParens)
            {
                let table_constructor = format_table_constructor(ctx, table_constructor, shape)
                    .update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                        TokenType::spaces(1),
                    )])); // Single space before the table constructor

                return FunctionArgs::TableConstructor(table_constructor);
            }

            let mut arguments = Punctuated::new();
            let new_expression = format_expression(
                ctx,
                &Expression::Value {
                    value: Box::new(Value::TableConstructor(table_constructor.to_owned())),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                },
                shape + 1, // 1 = opening parentheses
            );

            // Remove any trailing comments from the expression, and move them into a buffer
            let (new_expression, comments_buffer) =
                trivia_util::take_expression_trailing_comments(&new_expression);

            // Create parentheses, and add the trailing comments to the end of the parentheses
            let parentheses = ContainedSpan::new(
                TokenReference::symbol("(").unwrap(),
                TokenReference::symbol(")").unwrap(),
            )
            .update_trailing_trivia(FormatTriviaType::Append(comments_buffer));

            arguments.push(Pair::new(new_expression, None)); // Only single argument, so no trailing comma

            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

fn should_parameters_format_multiline(
    ctx: &Context,
    function_body: &FunctionBody,
    shape: Shape,
    block_empty: bool,
) -> bool {
    // Check the length of the parameters. We need to format them first onto a single line to check if required
    let mut line_length = format_singleline_parameters(ctx, function_body, shape)
        .to_string()
        .len()
        + 2; // Account for the parentheses around the parameters

    // If we are in Luau mode, take into account the types
    // If a type specifier is multiline, the whole parameters should be formatted multiline UNLESS there is only a single parameter.
    // Otherwise, include them in the total length
    #[cfg(feature = "luau")]
    {
        let (extra_line_length, multiline_specifier_present) = function_body
            .type_specifiers()
            .chain(std::iter::once(function_body.return_type())) // Include optional return type
            .map(|x| {
                x.map_or((0, false), |specifier| {
                    let formatted = format_type_specifier(ctx, specifier, shape).to_string();

                    (formatted.len(), formatted.lines().count() > 1)
                })
            })
            .fold(
                (0, false),
                |(acc_length, acc_multiline), (length, multiline)| {
                    (
                        acc_length + length,
                        if multiline { true } else { acc_multiline },
                    )
                },
            );

        // One of the type specifiers is multiline, and we have more than one parameter
        if multiline_specifier_present && function_body.parameters().len() > 1 {
            return true;
        }

        // Add the extra length
        line_length += extra_line_length
    }

    // If the block is empty, then the `end` will be inlined. We should include this in our line length check
    if block_empty {
        line_length += 4 // 4 = " end"
    }

    let singleline_shape = shape + line_length;
    singleline_shape.over_budget()
}

/// Formats a FunctionBody node
pub fn format_function_body(
    ctx: &Context,
    function_body: &FunctionBody,
    add_trivia_after_end: bool,
    shape: Shape,
) -> FunctionBody {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    // If the FunctionBody block is empty, then don't add a newline after the parameters, but add a space:
    // `function() end`
    let block_empty = trivia_util::is_function_empty(function_body);

    #[cfg(feature = "luau")]
    let generics = function_body
        .generics()
        .map(|generic_declaration| format_generic_declaration(ctx, generic_declaration, shape));
    #[cfg(feature = "luau")]
    let shape = shape + generics.as_ref().map_or(0, |x| x.to_string().len());

    // Check if the parameters should be placed across multiple lines
    let multiline_params = {
        #[cfg(feature = "luau")]
        let mut type_specifiers = function_body.type_specifiers();

        // Check whether they contain comments
        let contains_comments = function_body.parameters().pairs().any(|pair| {
            let contains_comments = pair
                .punctuation()
                .map_or(false, trivia_util::token_contains_comments)
                || trivia_util::contains_comments(pair.value());
            #[cfg(feature = "luau")]
            let type_specifier_comments = type_specifiers
                .next()
                .flatten()
                .map_or(false, |type_specifier| {
                    trivia_util::contains_comments(type_specifier)
                });
            #[cfg(not(feature = "luau"))]
            let type_specifier_comments = false;
            contains_comments || type_specifier_comments
        });

        contains_comments
            || should_parameters_format_multiline(ctx, function_body, shape, block_empty)
    };

    let (formatted_parameters, mut parameters_parentheses) = match multiline_params {
        true => {
            // TODO: This is similar to multiline in FunctionArgs, can we resolve?
            // Format start and end brace properly with correct trivia
            let (start_parens, end_parens) = function_body.parameters_parentheses().tokens();

            // Calculate to see if the end parentheses requires any additional indentation
            let end_parens_leading_trivia =
                vec![create_newline_trivia(ctx), create_indent_trivia(ctx, shape)];

            // Add new_line trivia to start_parens
            let start_parens_token = fmt_symbol!(ctx, start_parens, "(", shape)
                .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]));

            let end_parens_token = TokenReference::new(
                end_parens_leading_trivia,
                Token::new(TokenType::Symbol {
                    symbol: Symbol::RightParen,
                }),
                vec![],
            );

            (
                format_multiline_parameters(ctx, function_body, shape),
                ContainedSpan::new(
                    start_parens_token,
                    format_symbol(ctx, end_parens, &end_parens_token, shape),
                ),
            )
        }
        false => (
            format_singleline_parameters(ctx, function_body, shape),
            format_contained_span(ctx, function_body.parameters_parentheses(), shape),
        ),
    };

    #[cfg(feature = "luau")]
    let type_specifiers;
    #[cfg(feature = "luau")]
    let return_type;
    #[allow(unused_mut)]
    let mut added_trailing_trivia = false;

    #[cfg(feature = "luau")]
    {
        let parameters_shape = if multiline_params {
            shape.increment_additional_indent()
        } else {
            shape
        };

        type_specifiers = function_body
            .type_specifiers()
            .map(|x| x.map(|specifier| format_type_specifier(ctx, specifier, parameters_shape)))
            .collect();

        return_type = function_body.return_type().map(|return_type| {
            let formatted = format_type_specifier(ctx, return_type, shape);
            added_trailing_trivia = true;
            let trivia = if block_empty {
                vec![Token::new(TokenType::spaces(1))]
            } else {
                trailing_trivia.to_owned()
            };
            formatted.update_trailing_trivia(FormatTriviaType::Append(trivia))
        });
    }

    if !added_trailing_trivia {
        parameters_parentheses = parameters_parentheses.update_trailing_trivia(
            FormatTriviaType::Append(if block_empty {
                vec![Token::new(TokenType::spaces(1))]
            } else {
                trailing_trivia.to_owned()
            }),
        )
    }

    let block_shape = shape.reset().increment_block_indent();
    let block = format_block(ctx, function_body.block(), block_shape);

    let (end_token_leading_trivia, end_token_trailing_trivia) = (
        match block_empty {
            true => FormatTriviaType::NoChange,
            false => FormatTriviaType::Append(leading_trivia),
        },
        match add_trivia_after_end {
            true => FormatTriviaType::Append(trailing_trivia),
            false => FormatTriviaType::NoChange,
        },
    );

    let end_token = format_end_token(
        ctx,
        function_body.end_token(),
        EndTokenType::BlockEnd,
        shape,
    )
    .update_trivia(end_token_leading_trivia, end_token_trailing_trivia);

    let function_body = function_body.to_owned();
    #[cfg(feature = "luau")]
    let function_body = function_body
        .with_generics(generics)
        .with_type_specifiers(type_specifiers)
        .with_return_type(return_type);

    function_body
        .with_parameters_parentheses(parameters_parentheses)
        .with_parameters(formatted_parameters)
        .with_block(block)
        .with_end_token(end_token)
}

/// Formats a FunctionCall node
pub fn format_function_call(
    ctx: &Context,
    function_call: &FunctionCall,
    shape: Shape,
) -> FunctionCall {
    let formatted_prefix = format_prefix(ctx, function_call.prefix(), shape);

    let num_suffixes = function_call.suffixes().count();
    let should_hang = {
        // Hang if there is atleast more than one method call suffix
        if function_call
            .suffixes()
            .filter(|x| matches!(x, Suffix::Call(Call::MethodCall(_))))
            .count()
            > 1
        {
            // Check if either a), we are surpassing the column width
            // Or b), one of the INTERNAL (not the last call) method call's arguments is multiline [function/table]

            // Create a temporary formatted version of suffixes to use for this check
            let formatted_suffixes = function_call
                .suffixes()
                .map(|x| format_suffix(ctx, x, shape, FunctionCallNextNode::None)) // TODO: is this the right shape to use?
                .collect();
            let preliminary_function_call =
                FunctionCall::new(formatted_prefix.to_owned()).with_suffixes(formatted_suffixes);

            let outcome = if shape
                .take_first_line(&strip_trivia(&preliminary_function_call))
                .over_budget()
            {
                true
            } else {
                let suffixes = preliminary_function_call.suffixes().enumerate();
                let mut contains_newline = false;
                for (idx, suffix) in suffixes {
                    // Check to see whether this suffix is an "internal" method call suffix
                    // i.e. we are not at the last MethodCall suffix
                    let mut remaining_suffixes = preliminary_function_call.suffixes().skip(idx + 1);
                    if remaining_suffixes.any(|x| matches!(x, Suffix::Call(Call::MethodCall(_))))
                        && matches!(suffix, Suffix::Call(Call::MethodCall(_)))
                        && strip_trivia(suffix).to_string().contains('\n')
                    {
                        contains_newline = true;
                        break;
                    }
                }

                contains_newline
            };

            outcome
        } else {
            false
        }
    };

    let mut shape = shape.take_last_line(&strip_leading_trivia(&formatted_prefix));
    let mut formatted_suffixes = Vec::with_capacity(num_suffixes);
    let mut suffixes = function_call.suffixes().peekable();
    while let Some(suffix) = suffixes.next() {
        // Only hang if this is a method call
        let should_hang = should_hang && matches!(suffix, Suffix::Call(Call::MethodCall(_)));
        let current_shape = if should_hang {
            // Reset the shape as the call will be on a newline
            shape = shape.reset();
            // Increment the additional indent level for this current suffix
            shape.increment_additional_indent()
        } else {
            shape
        };

        // If the suffix after this one is something like `.foo` or `:foo` - this affects removing parentheses
        let ambiguous_next_suffix = if matches!(
            suffixes.peek(),
            Some(Suffix::Index(_)) | Some(Suffix::Call(Call::MethodCall(_)))
        ) {
            FunctionCallNextNode::ObscureWithoutParens
        } else {
            FunctionCallNextNode::None
        };

        let mut suffix = format_suffix(ctx, suffix, current_shape, ambiguous_next_suffix);

        if should_hang {
            suffix = suffix.update_leading_trivia(FormatTriviaType::Append(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, current_shape),
            ]));
        }

        shape = shape.take_last_line(&suffix);
        formatted_suffixes.push(suffix);
    }

    FunctionCall::new(formatted_prefix).with_suffixes(formatted_suffixes)
}

/// Formats a FunctionName node
pub fn format_function_name(
    ctx: &Context,
    function_name: &FunctionName,
    shape: Shape,
) -> FunctionName {
    // TODO: This is based off formatters::format_punctuated - can we merge them into one?
    let mut formatted_names = Punctuated::new();
    for pair in function_name.names().to_owned().into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = fmt_symbol!(ctx, &punctuation, ".", shape);
                let formatted_value = format_token_reference(ctx, &value, shape);
                formatted_names.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = format_token_reference(ctx, &value, shape);
                formatted_names.push(Pair::new(formatted_value, None));
            }
        }
    }

    let mut formatted_method: Option<(TokenReference, TokenReference)> = None;

    if let Some(method_colon) = function_name.method_colon() {
        if let Some(token_reference) = function_name.method_name() {
            formatted_method = Some((
                fmt_symbol!(ctx, method_colon, ":", shape),
                format_token_reference(ctx, token_reference, shape),
            ));
        }
    };

    FunctionName::new(formatted_names).with_method(formatted_method)
}

/// Formats a FunctionDeclaration node
pub fn format_function_declaration(
    ctx: &Context,
    function_declaration: &FunctionDeclaration,
    shape: Shape,
) -> FunctionDeclaration {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];

    let function_token = fmt_symbol!(
        ctx,
        function_declaration.function_token(),
        "function ",
        shape
    )
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let formatted_function_name = format_function_name(ctx, function_declaration.name(), shape);

    let shape = shape + (9 + strip_trivia(&formatted_function_name).to_string().len()); // 9 = "function "
    let function_body = format_function_body(ctx, function_declaration.body(), true, shape);

    FunctionDeclaration::new(formatted_function_name)
        .with_function_token(function_token)
        .with_body(function_body)
}

/// Formats a LocalFunction node
pub fn format_local_function(
    ctx: &Context,
    local_function: &LocalFunction,
    shape: Shape,
) -> LocalFunction {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];

    let local_token = fmt_symbol!(ctx, local_function.local_token(), "local ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let function_token = fmt_symbol!(ctx, local_function.function_token(), "function ", shape);
    let formatted_name = format_token_reference(ctx, local_function.name(), shape);

    let shape = shape + (6 + 9 + strip_trivia(&formatted_name).to_string().len()); // 6 = "local ", 9 = "function "
    let function_body = format_function_body(ctx, local_function.body(), true, shape);

    LocalFunction::new(formatted_name)
        .with_local_token(local_token)
        .with_function_token(function_token)
        .with_body(function_body)
}

/// Formats a MethodCall node
pub fn format_method_call(
    ctx: &Context,
    method_call: &MethodCall,
    shape: Shape,
    call_next_node: FunctionCallNextNode,
) -> MethodCall {
    let formatted_colon_token = format_token_reference(ctx, method_call.colon_token(), shape);
    let formatted_name = format_token_reference(ctx, method_call.name(), shape);
    let shape =
        shape + (formatted_colon_token.to_string().len() + formatted_name.to_string().len());
    let formatted_function_args =
        format_function_args(ctx, method_call.args(), shape, call_next_node);

    MethodCall::new(formatted_name, formatted_function_args).with_colon_token(formatted_colon_token)
}

/// Formats a single Parameter node
pub fn format_parameter(ctx: &Context, parameter: &Parameter, shape: Shape) -> Parameter {
    match parameter {
        Parameter::Ellipse(token) => Parameter::Ellipse(fmt_symbol!(ctx, token, "...", shape)),
        Parameter::Name(token_reference) => {
            Parameter::Name(format_token_reference(ctx, token_reference, shape))
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats the [`Parameters`] in the provided [`FunctionBody`] onto a single line.
fn format_singleline_parameters(
    ctx: &Context,
    function_body: &FunctionBody,
    shape: Shape,
) -> Punctuated<Parameter> {
    let mut formatted_parameters = Punctuated::new();

    for pair in function_body.parameters().pairs() {
        let parameter = format_parameter(ctx, pair.value(), shape);
        let punctuation = pair
            .punctuation()
            .map(|punctuation| fmt_symbol!(ctx, punctuation, ", ", shape));

        formatted_parameters.push(Pair::new(parameter, punctuation));
    }

    formatted_parameters
}

/// Formats the [`Parameters`] in the provided [`FunctionBody`], split across multiple lines.
fn format_multiline_parameters(
    ctx: &Context,
    function_body: &FunctionBody,
    shape: Shape,
) -> Punctuated<Parameter> {
    let mut formatted_parameters = Punctuated::new();

    for pair in function_body.parameters().pairs() {
        // Reset the shape (as the parameter is on a newline), and increment the additional indent level
        let shape = shape.reset().increment_additional_indent();

        let mut parameter = format_parameter(ctx, pair.value(), shape).update_leading_trivia(
            FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
        );

        let punctuation = match pair.punctuation() {
            Some(punctuation) => {
                // Remove any trailing comments from the parameter if present
                let mut trailing_comments: Vec<Token> = match &parameter {
                    Parameter::Name(token) | Parameter::Ellipse(token) => token.trailing_trivia(),
                    other => panic!("unknown node {:?}", other),
                }
                .filter(|token| trivia_util::trivia_is_comment(token))
                .flat_map(|x| {
                    // Prepend a single space beforehand
                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                })
                .collect();

                parameter = parameter.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

                // Add a newline to the end of the trailing comments, then append them all to the end of the comma
                trailing_comments.push(create_newline_trivia(ctx));
                Some(
                    fmt_symbol!(ctx, punctuation, ",", shape)
                        .update_trailing_trivia(FormatTriviaType::Append(trailing_comments)),
                )
            }
            None => None,
        };

        formatted_parameters.push(Pair::new(parameter, punctuation))
    }
    formatted_parameters
}
