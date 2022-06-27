use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
    LocalFunction, MethodCall, Parameter, Suffix, Value,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
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
            format_contained_punctuated_multiline, format_contained_span, format_end_token,
            format_punctuated, format_token_reference, EndTokenType,
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

/// Determines whether a parenthesised function call contains comments, forcing it to go multiline
fn function_args_contains_comments(
    parentheses: &ContainedSpan,
    arguments: &Punctuated<Expression>,
) -> bool {
    let (start_parens, end_parens) = parentheses.tokens();

    if trivia_util::trivia_contains_comments(
        start_parens.trailing_trivia(),
        trivia_util::CommentSearch::Single,
    ) || trivia_util::token_trivia_contains_comments(end_parens.leading_trivia())
    {
        true
    } else {
        arguments.pairs().any(|argument| {
            // Leading / Trailing trivia of expression (ignore inline comments)
            trivia_util::get_expression_leading_trivia(argument.value())
                .iter()
                .chain(trivia_util::get_expression_trailing_trivia(argument.value()).iter())
                .any(trivia_util::trivia_is_comment)
            // Punctuation contains comments
            || argument
                .punctuation()
                .map_or(false, trivia_util::token_contains_comments)
        })
    }
}

#[derive(Clone, Copy)]
enum ArgumentState {
    /// No special arguments have been seen
    None,
    /// Whether a multiline table/function has been seen
    SeenMultilineArguments,
    /// Whether a normal argument has been seen after a multiline table/function
    SeenNonMultilineArgumentAfterMultiline,
}

impl ArgumentState {
    fn new() -> ArgumentState {
        ArgumentState::None
    }

    /// Record that a multiline argument has been seen.
    #[must_use]
    fn record_multiline_arg(self) -> ArgumentState {
        match self {
            // Move from standard state to "seen multiline arguments state"
            ArgumentState::None => ArgumentState::SeenMultilineArguments,
            _ => self,
        }
    }

    /// Records that a normal (non-multiline) argument has been seen
    #[must_use]
    fn record_standard_arg(self) -> ArgumentState {
        match self {
            // If we have already seen a multiline argument, move from that state
            ArgumentState::SeenMultilineArguments => {
                ArgumentState::SeenNonMultilineArgumentAfterMultiline
            }
            _ => self,
        }
    }

    /// If we are in the [`ArgumentState::SeenNonMultilineArgumentAfterMultiline`] state, then we need to hang.
    fn should_hang(self) -> bool {
        matches!(self, ArgumentState::SeenNonMultilineArgumentAfterMultiline)
    }
}

/// Applies heuristics to determine whether a parenthesised function call should be expanded onto multiple lines.
/// These heuristics are subject to change.
fn function_args_multiline_heuristic(
    ctx: &Context,
    arguments: &Punctuated<Expression>,
    shape: Shape,
) -> bool {
    const PAREN_LEN: usize = "(".len();
    const COMMA_SPACE_LEN: usize = ", ".len();
    const SPACE_LEN: usize = " ".len();
    const BRACKET_LEN: usize = "}".len();
    const END_LEN: usize = "end".len();

    // If we have no arguments, then we don't need to do anything
    if arguments.is_empty() {
        return false;
    }

    // Format all the arguments on an infinite width, so that we can prepare them and check to see whether they
    // need expanding. We will ignore punctuation for now
    let first_iter_formatted_arguments = arguments.iter().map(|argument| {
        if shape.using_simple_heuristics() {
            argument.to_owned()
        } else {
            format_expression(
                ctx,
                argument,
                shape.with_simple_heuristics().with_infinite_width(),
            )
        }
    });

    // Apply some heuristics to determine whether we should expand the function call
    let mut singleline_shape = shape + PAREN_LEN;

    // Find how far we are currently indented, we can use this to determine when to expand
    // We will expand on two occasions:
    // 1) If a group of arguments fall on a single line, and they surpass the column width setting
    // 2) If we have a mixture of multiline (tables/anonymous functions) and other values. For
    //    example, call({ ... }, foo, { ... }), should be expanded, but
    //    call(foo, { ... }) or call(foo, { ... }, foo) can stay on one line, provided the
    //    single line arguments don't surpass the column width setting

    // Use state values to determine the type of arguments we have seen so far
    let mut current_state = ArgumentState::new();

    for argument in first_iter_formatted_arguments {
        match argument {
            Expression::Value { ref value, .. } => {
                match &**value {
                    // Check to see if we have a table constructor, or anonymous function
                    Value::Function((_, function_body)) => {
                        // Check to see whether it has been expanded
                        let is_expanded = !trivia_util::is_function_empty(function_body);
                        if is_expanded {
                            // If we have a mixture of multiline args, and other arguments
                            // Then the function args should be expanded
                            if current_state.should_hang() {
                                return true;
                            }

                            current_state = current_state.record_multiline_arg();

                            // First check the top line of the anonymous function (i.e. the function token and any parameters)
                            // If this is over budget, then we should expand
                            singleline_shape = singleline_shape.take_first_line(&value);
                            if singleline_shape.over_budget() {
                                return true;
                            }

                            // Reset the shape onto a new line, and include the `end` token
                            singleline_shape = singleline_shape.reset() + END_LEN;
                        } else {
                            // Update the width with the collapsed function (normally indicative of a noop function)
                            singleline_shape = singleline_shape + argument.to_string().len();
                        }
                    }
                    Value::TableConstructor(table) => {
                        // Check to see whether it has been expanded (there is a newline after the start brace)
                        let is_expanded = trivia_util::trivia_contains_newline(
                            table.braces().tokens().0.trailing_trivia(),
                        );

                        if is_expanded {
                            // If we have a mixture of multiline args, and other arguments
                            // Then the function args should be expanded
                            if current_state.should_hang() {
                                return true;
                            }

                            current_state = current_state.record_multiline_arg();

                            // Reset the shape onto a new line
                            singleline_shape = singleline_shape.reset() + BRACKET_LEN;
                        } else {
                            // Update the shape with the size of the collapsed table constructor
                            singleline_shape = singleline_shape + argument.to_string().len();
                        }
                    }

                    _ => {
                        current_state = current_state.record_standard_arg();

                        // If the argument is complex (spans multiple lines), then we will immediately
                        // exit and span multiline - it is most likely too complex to keep going forward.
                        if is_complex_arg(value) && arguments.len() > 1 {
                            return true;
                        }

                        // Take the first line to see if we are over budget
                        if singleline_shape.take_first_line(&argument).over_budget() {
                            return true;
                        }

                        // Update the shape with the last line (which may be different from the first)
                        singleline_shape = singleline_shape.take_last_line(&argument);
                    }
                }
            }
            // TODO: Parentheses/UnOp, do we need to do more checking?
            // We will continue counting on the width_passed
            _ => {
                current_state = current_state.record_standard_arg();

                // Take the first line to see if we are over budget
                if singleline_shape.take_first_line(&argument).over_budget() {
                    return true;
                }

                // Update the shape with the last line (which may be different from the first)
                singleline_shape = singleline_shape.take_last_line(&argument);
            }
        }

        // Check the current shape to see if it has fallen over budget.
        // If it has, we can bail out and force the arguments onto multiple lines.
        if singleline_shape.over_budget() {
            return true;
        }

        // Add width which would be taken up by comma and space
        singleline_shape = singleline_shape + COMMA_SPACE_LEN;
    }

    // Check the final shape to see if its over budget, if it isn't, then we can leave it
    // -1 because we added +2 for ", " in the last iteration, but we don't want a trailing
    // space and the comma is replaced with a parentheses
    singleline_shape.sub_width(SPACE_LEN).over_budget()
}

/// Formats a singular argument in a [`FunctionArgs`] node, in a multiline fashion
fn format_argument_multiline(ctx: &Context, argument: &Expression, shape: Shape) -> Expression {
    // First format the argument assuming infinite width
    let infinite_width_argument = format_expression(ctx, argument, shape.with_infinite_width());

    // If the argument fits, great! Otherwise, see if we can hang the expression
    // If we can, use that instead (as it provides a nicer output). If not, format normally without infinite width
    if shape
        .add_width(strip_trivia(&infinite_width_argument).to_string().len())
        .over_budget()
    {
        if trivia_util::can_hang_expression(argument) {
            hang_expression(ctx, argument, shape, Some(1))
        } else {
            format_expression(ctx, argument, shape)
        }
    } else {
        infinite_width_argument
    }
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

                // Take any trailing trivia from the end parentheses, in case we need to keep it
                let trailing_comments = parentheses.tokens().1.trailing_trivia().cloned().collect();

                if let Expression::Value { value, .. } = argument {
                    match &**value {
                        Value::String(token_reference) => {
                            if ctx.should_omit_string_parens() {
                                return format_function_args(
                                    ctx,
                                    &FunctionArgs::String(token_reference.update_trailing_trivia(
                                        FormatTriviaType::Append(trailing_comments),
                                    )),
                                    shape,
                                    call_next_node,
                                );
                            }
                        }
                        Value::TableConstructor(table_constructor) => {
                            if ctx.should_omit_table_parens() {
                                return format_function_args(
                                    ctx,
                                    &FunctionArgs::TableConstructor(
                                        table_constructor.update_trailing_trivia(
                                            FormatTriviaType::Append(trailing_comments),
                                        ),
                                    ),
                                    shape,
                                    call_next_node,
                                );
                            }
                        }
                        _ => (),
                    }
                }
            }

            // Determine whether we should format the function call onto multiple lines

            // If there is a comment present anywhere in between the start parentheses and end parentheses, we should keep it multiline
            let force_mutliline = function_args_contains_comments(parentheses, arguments);

            let is_multiline =
                force_mutliline || function_args_multiline_heuristic(ctx, arguments, shape);

            // Handle special case: we want to go multiline, but we have a single argument which is a table constructor
            // In this case, we want to hug the table braces with the parentheses.
            // To do this, we format single line, but include the closing parentheses in the shape
            let hug_table_constructor = is_multiline
                && !force_mutliline
                && arguments.len() == 1
                && is_table_constructor(arguments.iter().next().unwrap());

            if is_multiline && !hug_table_constructor {
                let (parentheses, arguments) = format_contained_punctuated_multiline(
                    ctx,
                    parentheses,
                    arguments,
                    format_argument_multiline,
                    trivia_util::take_expression_trailing_comments,
                    shape,
                );

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments,
                }
            } else {
                // We don't need to worry about comments here, as if there were comments present, we would have
                // multiline function args

                // If we are hugging a table constructor with the parentheses, we use a shape increment of 2 to include the closing
                // parentheses as well. Otherwise, we just use 1 = opening parentheses.
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

    let (mut parameters_parentheses, formatted_parameters) = match multiline_params {
        true => format_contained_punctuated_multiline(
            ctx,
            function_body.parameters_parentheses(),
            function_body.parameters(),
            format_parameter,
            trivia_util::take_parameter_trailing_comments,
            shape,
        ),
        false => (
            format_contained_span(ctx, function_body.parameters_parentheses(), shape),
            format_singleline_parameters(ctx, function_body, shape),
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
        EndTokenType::IndentComments,
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

    // If there are comments within the chain, then we must hang the chain otherwise it can lead to an issue
    let must_hang = {
        let mut peekable_suffixes = function_call.suffixes().peekable();
        let mut must_hang = false;
        while let Some(suffix) = peekable_suffixes.next() {
            must_hang =
                // Check for a leading comment
                trivia_util::suffix_leading_trivia(suffix).any(trivia_util::trivia_is_comment)
                // Check for a trailing comment (iff there is still a suffix after this)
                || (peekable_suffixes.peek().is_some()
                    && trivia_util::suffix_trailing_trivia(suffix)
                        .iter()
                        .any(trivia_util::trivia_is_comment));

            if must_hang {
                break;
            }
        }

        must_hang
    };

    let should_hang = {
        // Hang if there is at least more than one method call suffix
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
    let mut previous_suffix_was_index = true; // The index is a name, so we treat that as an index so `A()` doesn't hang

    while let Some(suffix) = suffixes.next() {
        // Only hang if this is a method call
        let should_hang =
            must_hang || (should_hang && matches!(suffix, Suffix::Call(Call::MethodCall(_))));
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

        // Hang the call, but don't hang if the previous suffix was an index and this is an anonymous call, i.e. `.foo()`
        if should_hang
            && !(previous_suffix_was_index
                && matches!(suffix, Suffix::Call(Call::AnonymousCall(_))))
        {
            suffix = trivia_util::prepend_newline_indent(
                ctx,
                &suffix,
                trivia_util::suffix_leading_trivia(&suffix),
                current_shape,
            );
        }

        previous_suffix_was_index = matches!(suffix, Suffix::Index(_));
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
