#[cfg(feature = "luau")]
use full_moon::ast::types::{
    ElseIfExpression, IfExpression, InterpolatedString, InterpolatedStringSegment,
};
use full_moon::{
    ast::{
        span::ContainedSpan, BinOp, Expression, FunctionCall, Index, Prefix, Suffix, UnOp, Var,
        VarExpression,
    },
    node::Node,
    tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
};
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::{
    assignment::calculate_hang_level, luau::format_type_assertion,
    stmt::remove_condition_parentheses, trivia_util::HasInlineComments,
};
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        functions::{
            format_anonymous_function, format_call, format_function_call, FunctionCallNextNode,
        },
        general::{format_contained_span, format_end_token, format_token_reference, EndTokenType},
        table::format_table_constructor,
        trivia::{
            strip_leading_trivia, strip_trivia, FormatTriviaType, UpdateLeadingTrivia,
            UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{
            self, contains_comments, prepend_newline_indent, take_leading_comments,
            take_trailing_comments, trivia_is_newline, CommentSearch, GetLeadingTrivia,
            GetTrailingTrivia,
        },
    },
    shape::Shape,
};

#[macro_export]
macro_rules! fmt_op {
    ($ctx:expr, $enum:ident, $value:ident, $shape:expr, { $($(#[$inner:meta])* $operator:ident = $output:expr,)+ }, $other:expr) => {
        match $value {
            $(
                $(#[$inner])*
                $enum::$operator(token) => $enum::$operator(fmt_symbol!($ctx, token, $output, $shape)),
            )+
            #[allow(clippy::redundant_closure_call)]
            other => $other(other),
        }
    };
}

#[derive(Clone, Copy)]
enum ExpressionContext {
    /// Standard expression, with no special context
    Standard,
    /// The expression originates from a [`Prefix`] node. The special context here is that the expression will
    /// always be wrapped in parentheses.
    Prefix,
    /// The internal expression is being asserted by a type: the `expr` part of `(expr) :: type`.
    /// If this occurs and `expr` is wrapped in parentheses, we keep the parentheses, such
    /// as for cases like `(expr) :: any) :: type`
    #[cfg(feature = "luau")]
    TypeAssertion,

    /// The internal expression is on the RHS of a binary operation
    /// e.g. `(not X) and Y` or `(not X) == Y`, where internal_expression = `not X`
    /// We should keep parentheses in this case to highlight precedence
    BinaryLHS,

    /// The internal expression is on the LHS of a binary expression involving ^
    /// e.g. `(-X) ^ Y`
    /// We need to keep parentheses here because ^ has higher precedence and is right associative
    /// and removing parentheses changes meaning
    BinaryLHSExponent,

    /// The internal expression is having a unary operation applied to it: the `expr` part of #expr.
    /// If this occurs, and `expr` is a type assertion, then we need to keep the parentheses
    UnaryOrBinary,
}

pub fn format_binop(ctx: &Context, binop: &BinOp, shape: Shape) -> BinOp {
    fmt_op!(ctx, BinOp, binop, shape, {
        And = " and ",
        Caret = " ^ ",
        GreaterThan = " > ",
        GreaterThanEqual = " >= ",
        LessThan = " < ",
        LessThanEqual = " <= ",
        Minus = " - ",
        Or = " or ",
        Percent = " % ",
        Plus = " + ",
        Slash = " / ",
        Star = " * ",
        TildeEqual = " ~= ",
        TwoDots = " .. ",
        TwoEqual = " == ",
        #[cfg(feature = "lua53")]
        Ampersand = " & ",
        #[cfg(any(feature = "luau", feature = "lua53"))]
        DoubleSlash = " // ",
        #[cfg(feature = "lua53")]
        DoubleLessThan = " << ",
        #[cfg(feature = "lua53")]
        Pipe = " | ",
        #[cfg(feature = "lua53")]
        Tilde = " ~ ",
    }, |other: &BinOp| match other {
        #[cfg(feature = "lua53")]
        BinOp::DoubleGreaterThan(token) => BinOp::DoubleGreaterThan(
            format_token_reference(ctx, token, shape)
                .update_trivia(
                    FormatTriviaType::Append(vec![Token::new(TokenType::spaces(1))]),
                    FormatTriviaType::Append(vec![Token::new(TokenType::spaces(1))])
                )
        ),
        other => panic!("unknown node {:?}", other)
    })
}

/// Check to determine whether expression parentheses are required, depending on the provided
/// internal expression contained within the parentheses
fn check_excess_parentheses(internal_expression: &Expression, context: ExpressionContext) -> bool {
    match internal_expression {
        // Parentheses inside parentheses, not necessary
        Expression::Parentheses { .. } => true,
        // Check whether the expression relating to the UnOp is safe
        Expression::UnaryOperator {
            expression, unop, ..
        } => {
            // If the expression is of the format `(not X) and Y` or `(not X) == Y` etc.
            // Where internal_expression = not X, we should keep the parentheses
            if let ExpressionContext::BinaryLHSExponent = context {
                return false;
            } else if let ExpressionContext::BinaryLHS = context {
                if let UnOp::Not(_) = unop {
                    return false;
                }
            }

            check_excess_parentheses(expression, context)
        }
        // Don't bother removing them if there is a binop, as they may be needed. TODO: can we be more intelligent here?
        Expression::BinaryOperator { .. } => false,

        // If we have a type assertion, and the context is a unary or binary operation
        // we should always keep parentheses
        // [e.g. #(value :: Array<string>) or -(value :: number)]
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { .. }
            if matches!(
                context,
                ExpressionContext::UnaryOrBinary
                    | ExpressionContext::BinaryLHS
                    | ExpressionContext::BinaryLHSExponent
            ) =>
        {
            false
        }

        // Internal expression is a function call
        // We could potentially be culling values, so we should not remove parentheses
        Expression::FunctionCall(_) => false,
        Expression::Symbol(token_ref) => {
            match token_ref.token_type() {
                // If we have an ellipse inside of parentheses, we may also be culling values
                // Therefore, we don't remove parentheses
                TokenType::Symbol { symbol } => !matches!(symbol, Symbol::Ellipse),
                _ => true,
            }
        }
        // If the internal expression is an if expression, we need to keep the parentheses
        // as modifying it can lead to issues [e.g. (if <x> then <expr> else <expr>) + 1 is different without parens]
        #[cfg(feature = "luau")]
        Expression::IfExpression(_) => false,
        _ => true,
    }
}

/// Formats an Expression node
pub fn format_expression(ctx: &Context, expression: &Expression, shape: Shape) -> Expression {
    format_expression_internal(ctx, expression, ExpressionContext::Standard, shape)
}

/// Internal expression formatter, with access to expression context
fn format_expression_internal(
    ctx: &Context,
    expression: &Expression,
    context: ExpressionContext,
    shape: Shape,
) -> Expression {
    match expression {
        Expression::Function((token_reference, function_body)) => Expression::Function(
            format_anonymous_function(ctx, token_reference, function_body, shape),
        ),
        Expression::FunctionCall(function_call) => {
            Expression::FunctionCall(format_function_call(ctx, function_call, shape))
        }
        #[cfg(feature = "luau")]
        Expression::IfExpression(if_expression) => {
            Expression::IfExpression(format_if_expression(ctx, if_expression, shape))
        }
        Expression::Number(token_reference) => {
            Expression::Number(format_token_reference(ctx, token_reference, shape))
        }
        Expression::String(token_reference) => {
            Expression::String(format_token_reference(ctx, token_reference, shape))
        }
        #[cfg(feature = "luau")]
        Expression::InterpolatedString(interpolated_string) => Expression::InterpolatedString(
            format_interpolated_string(ctx, interpolated_string, shape),
        ),
        Expression::Symbol(token_reference) => {
            Expression::Symbol(format_token_reference(ctx, token_reference, shape))
        }
        Expression::TableConstructor(table_constructor) => {
            Expression::TableConstructor(format_table_constructor(ctx, table_constructor, shape))
        }
        Expression::Var(var) => Expression::Var(format_var(ctx, var, shape)),

        #[cfg(feature = "luau")]
        Expression::TypeAssertion {
            expression,
            type_assertion,
        } => Expression::TypeAssertion {
            expression: Box::new(format_expression_internal(
                ctx,
                expression,
                ExpressionContext::TypeAssertion,
                shape,
            )),
            type_assertion: format_type_assertion(ctx, type_assertion, shape),
        },
        Expression::Parentheses {
            contained,
            expression,
        } => {
            #[cfg(feature = "luau")]
            let keep_parentheses = matches!(
                context,
                ExpressionContext::Prefix | ExpressionContext::TypeAssertion
            );
            #[cfg(not(feature = "luau"))]
            let keep_parentheses = matches!(context, ExpressionContext::Prefix);

            // Examine whether the internal expression requires parentheses
            // If not, just format and return the internal expression. Otherwise, format the parentheses
            let use_internal_expression = check_excess_parentheses(expression, context);

            // If the context is for a prefix, we should always keep the parentheses, as they are always required
            if use_internal_expression && !keep_parentheses {
                // Get the leading and trailing comments from contained span and append them onto the expression
                let (start_parens, end_parens) = contained.tokens();
                let leading_comments = start_parens
                    .leading_trivia()
                    .filter(|token| trivia_util::trivia_is_comment(token))
                    .flat_map(|x| {
                        vec![
                            create_indent_trivia(ctx, shape),
                            x.to_owned(),
                            create_newline_trivia(ctx),
                        ]
                    })
                    // .chain(std::iter::once(create_indent_trivia(ctx, shape)))
                    .collect();

                let trailing_comments = end_parens
                    .trailing_trivia()
                    .filter(|token| trivia_util::trivia_is_comment(token))
                    .flat_map(|x| {
                        // Prepend a single space beforehand
                        vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                    })
                    .collect();

                format_expression(ctx, expression, shape)
                    .update_leading_trivia(FormatTriviaType::Append(leading_comments))
                    .update_trailing_trivia(FormatTriviaType::Append(trailing_comments))
            } else {
                Expression::Parentheses {
                    contained: format_contained_span(ctx, contained, shape),
                    expression: Box::new(format_expression(ctx, expression, shape + 1)), // 1 = opening parentheses
                }
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            let unop = format_unop(ctx, unop, shape);
            let shape = shape + strip_leading_trivia(&unop).to_string().len();
            let mut expression = format_expression_internal(
                ctx,
                expression,
                ExpressionContext::UnaryOrBinary,
                shape,
            );

            // Special case: if we have `- -foo`, or `-(-foo)` where we have already removed the parentheses, then
            // it will lead to `--foo`, which is invalid syntax. We must explicitly add/keep the parentheses `-(-foo)`.
            if let UnOp::Minus(_) = unop {
                let require_parentheses = match expression {
                    Expression::UnaryOperator {
                        unop: UnOp::Minus(_),
                        ..
                    } => true,
                    Expression::Parentheses { ref expression, .. } => matches!(
                        &**expression,
                        Expression::UnaryOperator {
                            unop: UnOp::Minus(_),
                            ..
                        }
                    ),
                    _ => false,
                };

                if require_parentheses {
                    let (new_expression, trailing_comments) =
                        trivia_util::take_trailing_comments(&expression);
                    expression = Expression::Parentheses {
                        contained: ContainedSpan::new(
                            TokenReference::symbol("(").unwrap(),
                            TokenReference::symbol(")").unwrap(),
                        )
                        .update_trailing_trivia(FormatTriviaType::Append(trailing_comments)),
                        expression: Box::new(new_expression),
                    }
                }
            }

            Expression::UnaryOperator {
                unop,
                expression: Box::new(expression),
            }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let context = if let BinOp::Caret(_) = binop {
                ExpressionContext::BinaryLHSExponent
            } else {
                ExpressionContext::BinaryLHS
            };
            let lhs = format_expression_internal(ctx, lhs, context, shape);
            let binop = format_binop(ctx, binop, shape);
            let shape = shape.take_last_line(&lhs) + binop.to_string().len();
            Expression::BinaryOperator {
                lhs: Box::new(lhs),
                binop,
                rhs: Box::new(format_expression_internal(
                    ctx,
                    rhs,
                    ExpressionContext::UnaryOrBinary,
                    shape,
                )),
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Determines whether the provided [`Expression`] is a brackets string, i.e. `[[string]]`
/// We care about this because `[ [[string] ]` is invalid syntax if we remove the whitespace
pub fn is_brackets_string(expression: &Expression) -> bool {
    match expression {
        Expression::String(token_reference) => matches!(
            token_reference.token_type(),
            TokenType::StringLiteral {
                quote_type: StringLiteralQuoteType::Brackets,
                ..
            }
        ),
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { expression, .. } => is_brackets_string(expression),
        _ => false,
    }
}

/// Formats an Index Node
pub fn format_index(ctx: &Context, index: &Index, shape: Shape) -> Index {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => {
            if brackets
                .tokens()
                .0
                .has_trailing_comments(CommentSearch::All)
                || contains_comments(expression)
                || brackets.tokens().1.has_leading_comments(CommentSearch::All)
            {
                let (start_bracket, end_bracket) = brackets.tokens();

                let indent_shape = shape.reset().increment_additional_indent();

                // Format the brackets multiline
                let brackets = ContainedSpan::new(
                    fmt_symbol!(ctx, start_bracket, "[", shape).update_trailing_trivia(
                        FormatTriviaType::Append(vec![
                            create_newline_trivia(ctx),
                            create_indent_trivia(ctx, indent_shape),
                        ]),
                    ),
                    format_end_token(ctx, end_bracket, EndTokenType::IndentComments, shape)
                        .update_leading_trivia(FormatTriviaType::Append(vec![
                            create_indent_trivia(ctx, shape),
                        ])),
                );

                let expression = format_expression(ctx, expression, indent_shape)
                    .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(
                        ctx,
                    )]));

                Index::Brackets {
                    brackets,
                    expression,
                }
            } else if is_brackets_string(expression) {
                Index::Brackets {
                    brackets: format_contained_span(ctx, brackets, shape),
                    expression: format_expression(ctx, expression, shape + 2) // 2 = "[ "
                        .update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                            TokenType::spaces(1),
                        )]))
                        .update_trailing_trivia(FormatTriviaType::Append(vec![Token::new(
                            TokenType::spaces(1),
                        )])),
                }
            } else {
                Index::Brackets {
                    brackets: format_contained_span(ctx, brackets, shape),
                    expression: format_expression(ctx, expression, shape + 1), // 1 = opening bracket
                }
            }
        }

        Index::Dot { dot, name } => {
            // If there are any comments in between the dot and name,
            // then taken them out and put them before the dot
            let (mut dot, mut dot_comments) =
                take_trailing_comments(&format_token_reference(ctx, dot, shape));
            let (name, name_comments) =
                take_leading_comments(&format_token_reference(ctx, name, shape));

            dot_comments.extend(name_comments);

            if !dot_comments.is_empty() {
                dot = prepend_newline_indent(
                    ctx,
                    &dot.update_leading_trivia(FormatTriviaType::Append(dot_comments)),
                    shape,
                );
            }

            Index::Dot { dot, name }
        }
        other => panic!("unknown node {:?}", other),
    }
}

// Checks if this is a string (allows strings wrapped in parentheses)
fn is_string(expression: &Expression) -> bool {
    match expression {
        Expression::String(_) => true,
        #[cfg(feature = "luau")]
        Expression::InterpolatedString(_) => true,
        Expression::Parentheses { expression, .. } => is_string(expression),
        _ => false,
    }
}

/// Formats a Prefix Node
pub fn format_prefix(ctx: &Context, prefix: &Prefix, shape: Shape) -> Prefix {
    match prefix {
        Prefix::Expression(expression) => {
            let singleline_format =
                format_expression_internal(ctx, expression, ExpressionContext::Prefix, shape);
            let singeline_shape = shape.take_first_line(&strip_trivia(&singleline_format));

            if singeline_shape.over_budget() && !is_string(expression) {
                Prefix::Expression(Box::new(format_hanging_expression_(
                    ctx,
                    expression,
                    shape,
                    ExpressionContext::Prefix,
                    None,
                )))
            } else {
                Prefix::Expression(Box::new(singleline_format))
            }
        }
        Prefix::Name(token_reference) => {
            Prefix::Name(format_token_reference(ctx, token_reference, shape))
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Suffix Node
pub fn format_suffix(
    ctx: &Context,
    suffix: &Suffix,
    shape: Shape,
    call_next_node: FunctionCallNextNode,
) -> Suffix {
    match suffix {
        Suffix::Call(call) => Suffix::Call(format_call(ctx, call, shape, call_next_node)),
        Suffix::Index(index) => Suffix::Index(format_index(ctx, index, shape)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats and else if expression onto a single line.
/// This function does not take into account for comments
#[cfg(feature = "luau")]
fn format_else_if_expression_singleline(
    ctx: &Context,
    else_if_expression: &ElseIfExpression,
    shape: Shape,
) -> ElseIfExpression {
    let else_if_token = fmt_symbol!(ctx, else_if_expression.else_if_token(), "elseif ", shape);
    let else_if_condition = remove_condition_parentheses(else_if_expression.condition().to_owned());
    let else_if_condition = format_expression(ctx, &else_if_condition, shape + 7); // 7 = "elseif "
    let (then_token, expression) = format_token_expression_sequence(
        ctx,
        else_if_expression.then_token(),
        else_if_expression.expression(),
        shape.take_first_line(&else_if_condition) + 13, // 13 = "elseif " + " then ",
    );

    // Add a space before the then token
    let then_token = then_token.update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
        TokenType::spaces(1),
    )]));

    ElseIfExpression::new(else_if_condition, expression)
        .with_else_if_token(else_if_token)
        .with_then_token(then_token)
}

/// Formats a `<token> <expr>` sequence, such as `then <expr>` or `else <expr>`.
/// In particular, this handles when the <expr> has to be formatted onto multiple lines (either due to comments, or going over width)
#[cfg(feature = "luau")]
fn format_token_expression_sequence(
    ctx: &Context,
    token: &TokenReference,
    expression: &Expression,
    shape: Shape,
) -> (TokenReference, Expression) {
    const SPACE_LEN: usize = " ".len();
    let formatted_token = format_token_reference(ctx, token, shape);
    let token_width = strip_trivia(&formatted_token).to_string().len();

    let formatted_expression =
        format_expression(ctx, expression, shape.add_width(token_width + SPACE_LEN));

    let requires_multiline_expression = shape.take_first_line(&formatted_expression).over_budget()
        || token.has_trailing_comments(CommentSearch::All)
        || trivia_util::contains_comments(
            expression.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
        ); // Remove trailing trivia (comments) before checking, as they shouldn't have an impact

    let newline_after_token = token.has_trailing_comments(CommentSearch::Single)
        || expression.has_leading_comments(CommentSearch::Single);

    let token = match newline_after_token {
        // `<token>\n`
        true => formatted_token
            .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)])),
        // `<token> `
        false => {
            formatted_token.update_trailing_trivia(FormatTriviaType::Append(vec![Token::new(
                TokenType::spaces(1),
            )]))
        }
    };

    let expression = match requires_multiline_expression {
        true => match newline_after_token {
            true => {
                let shape = shape.reset().increment_additional_indent();
                hang_expression(ctx, expression, shape, calculate_hang_level(expression))
                    .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                        ctx, shape,
                    )]))
            }
            false => hang_expression(
                ctx,
                expression,
                shape.add_width(token_width + SPACE_LEN),
                calculate_hang_level(expression),
            ),
        },
        false => formatted_expression,
    };

    (token, expression)
}

/// Formats an [`IfExpression`] node
#[cfg(feature = "luau")]
fn format_if_expression(ctx: &Context, if_expression: &IfExpression, shape: Shape) -> IfExpression {
    // Remove parentheses around the condition
    let condition = remove_condition_parentheses(if_expression.condition().to_owned());
    let if_token = fmt_symbol!(ctx, if_expression.if_token(), "if ", shape);

    // Initially format the remainder on a single line
    let singleline_condition = format_expression(ctx, &condition, shape.with_infinite_width());
    let then_token = fmt_symbol!(ctx, if_expression.then_token(), " then ", shape);
    let singleline_expression = format_expression(
        ctx,
        if_expression.if_expression(),
        shape.with_infinite_width(),
    );
    let else_ifs = if_expression
        .else_if_expressions()
        .map(|else_if_expressions| {
            else_if_expressions
                .iter()
                .map(|else_if_expression| {
                    format_else_if_expression_singleline(
                        ctx,
                        else_if_expression,
                        shape.with_infinite_width(),
                    )
                })
                .collect::<Vec<_>>()
        });
    let else_token = fmt_symbol!(ctx, if_expression.else_token(), " else ", shape);
    let singleline_else_expression = format_expression(
        ctx,
        if_expression.else_expression(),
        shape.with_infinite_width(),
    );

    const IF_LENGTH: usize = 3; // "if "
    const THEN_LENGTH: usize = 6; // " then "
    const ELSE_LENGTH: usize = 6; // " else "

    // Determine if we need to hang the expression
    let singleline_shape = (shape + IF_LENGTH + THEN_LENGTH + ELSE_LENGTH)
        .take_first_line(&strip_trivia(&singleline_condition))
        .take_first_line(&strip_trivia(&singleline_expression))
        .take_first_line(&else_ifs.as_ref().map_or(String::new(), |x| {
            x.iter().map(|x| x.to_string()).collect::<String>()
        }))
        .take_first_line(&strip_trivia(&singleline_else_expression));

    let require_multiline_expression = singleline_shape.over_budget()
        || if_expression
            .if_token()
            .has_trailing_comments(CommentSearch::All)
        || trivia_util::contains_comments(if_expression.condition())
        || trivia_util::contains_comments(if_expression.then_token())
        || trivia_util::contains_comments(if_expression.if_expression())
        || trivia_util::contains_comments(if_expression.else_token())
        || if_expression
            .else_if_expressions()
            .map_or(false, |else_ifs| {
                else_ifs.iter().any(trivia_util::contains_comments)
            })
        || if_expression.else_expression().has_inline_comments()
        || trivia_util::spans_multiple_lines(&singleline_condition)
        || trivia_util::spans_multiple_lines(&singleline_expression)
        || else_ifs.as_ref().map_or(false, |else_ifs| {
            else_ifs.iter().any(trivia_util::spans_multiple_lines)
        })
        || trivia_util::spans_multiple_lines(&singleline_else_expression);

    if require_multiline_expression {
        let condition = hang_expression_trailing_newline(
            ctx,
            if_expression.condition(),
            shape.increment_additional_indent(),
            Some(1),
        );
        let hanging_shape = shape.reset().increment_additional_indent();

        // then <expr>
        let (then_token, expression) = format_token_expression_sequence(
            ctx,
            if_expression.then_token(),
            if_expression.if_expression(),
            hanging_shape,
        );

        // Indent the then token
        let then_token =
            then_token.update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                ctx,
                hanging_shape,
            )]));

        // elseif <condition> then <expr>
        let else_ifs = if_expression
            .else_if_expressions()
            .map(|else_if_expressions| {
                else_if_expressions
                    .iter()
                    .map(|else_if_expression| {
                        let singleline_else_if = format_else_if_expression_singleline(
                            ctx,
                            else_if_expression,
                            hanging_shape,
                        );
                        let singleline_shape = hanging_shape.take_first_line(&singleline_else_if);

                        if singleline_shape.over_budget()
                            || else_if_expression
                                .else_if_token()
                                .has_trailing_comments(CommentSearch::All)
                            || trivia_util::contains_comments(else_if_expression.condition())
                            || trivia_util::contains_comments(else_if_expression.then_token())
                        {
                            let else_if_token = fmt_symbol!(
                                ctx,
                                else_if_expression.else_if_token(),
                                "elseif",
                                shape
                            )
                            .update_leading_trivia(FormatTriviaType::Append(vec![
                                create_newline_trivia(ctx),
                                create_indent_trivia(ctx, hanging_shape),
                            ]));

                            let condiiton_shape =
                                hanging_shape.reset().increment_additional_indent();
                            let else_if_condition = hang_expression(
                                ctx,
                                &remove_condition_parentheses(
                                    else_if_expression.condition().to_owned(),
                                ),
                                condiiton_shape,
                                None,
                            )
                            .update_leading_trivia(FormatTriviaType::Append(vec![
                                create_newline_trivia(ctx),
                                create_indent_trivia(ctx, condiiton_shape),
                            ]));

                            let hanging_shape =
                                hanging_shape.take_first_line(&else_if_condition) + 13; // 13 = "elseif " + " then "

                            let (then_token, expression) = format_token_expression_sequence(
                                ctx,
                                else_if_expression.then_token(),
                                else_if_expression.expression(),
                                hanging_shape,
                            );

                            let then_token =
                                then_token.update_leading_trivia(FormatTriviaType::Append(vec![
                                    create_newline_trivia(ctx),
                                    create_indent_trivia(ctx, hanging_shape),
                                ]));

                            ElseIfExpression::new(else_if_condition, expression)
                                .with_else_if_token(else_if_token)
                                .with_then_token(then_token)
                        } else {
                            singleline_else_if.update_leading_trivia(FormatTriviaType::Append(
                                vec![
                                    create_newline_trivia(ctx),
                                    create_indent_trivia(ctx, hanging_shape),
                                ],
                            ))
                        }
                    })
                    .collect::<Vec<_>>()
            });

        // else <expr>
        let (else_token, else_expression) = format_token_expression_sequence(
            ctx,
            if_expression.else_token(),
            if_expression.else_expression(),
            hanging_shape + 5, // 5 = "else "
        );

        // Put the else on a new line
        let else_token = trivia_util::prepend_newline_indent(ctx, &else_token, hanging_shape);

        IfExpression::new(condition, expression, else_expression)
            .with_if_token(if_token)
            .with_then_token(then_token)
            .with_else_if(else_ifs)
            .with_else_token(else_token)
    } else {
        // Prepend a space before each else if
        let else_ifs = else_ifs.map(|x| {
            x.iter()
                .map(|x| {
                    x.update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                        TokenType::spaces(1),
                    )]))
                })
                .collect()
        });

        IfExpression::new(
            singleline_condition,
            singleline_expression,
            singleline_else_expression,
        )
        .with_if_token(if_token)
        .with_then_token(then_token)
        .with_else_if(else_ifs)
        .with_else_token(else_token)
    }
}

#[cfg(feature = "luau")]
fn format_interpolated_string(
    ctx: &Context,
    interpolated_string: &InterpolatedString,
    shape: Shape,
) -> InterpolatedString {
    let mut shape = shape;

    let mut segments = Vec::new();
    for segment in interpolated_string.segments() {
        let literal = format_token_reference(ctx, &segment.literal, shape);
        shape = shape + literal.to_string().len();

        let mut expression = format_expression(ctx, &segment.expression, shape);
        shape = shape.take_last_line(&expression);

        // If expression is a table constructor, then ensure a space is added beforehand
        // since `{{` syntax is not permitted
        if let Expression::TableConstructor { .. } = expression {
            expression =
                expression.update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                    TokenType::spaces(1),
                )]))
        }

        segments.push(InterpolatedStringSegment {
            literal,
            expression,
        })
    }

    interpolated_string
        .to_owned()
        .with_segments(segments)
        .with_last_string(format_token_reference(
            ctx,
            interpolated_string.last_string(),
            shape,
        ))
}

/// Formats a Var Node
pub fn format_var(ctx: &Context, var: &Var, shape: Shape) -> Var {
    match var {
        Var::Name(token_reference) => {
            Var::Name(format_token_reference(ctx, token_reference, shape))
        }
        Var::Expression(var_expression) => {
            Var::Expression(Box::new(format_var_expression(ctx, var_expression, shape)))
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_var_expression(
    ctx: &Context,
    var_expression: &VarExpression,
    shape: Shape,
) -> VarExpression {
    // A VarExpression is pretty much exactly the same as FunctionCall, so we repurpose
    // format_function_call for that, and reuse its output
    let function_call = format_function_call(
        ctx,
        &FunctionCall::new(var_expression.prefix().clone())
            .with_suffixes(var_expression.suffixes().cloned().collect()),
        shape,
    );
    VarExpression::new(function_call.prefix().clone())
        .with_suffixes(function_call.suffixes().cloned().collect())
}

/// Formats an UnOp Node
pub fn format_unop(ctx: &Context, unop: &UnOp, shape: Shape) -> UnOp {
    fmt_op!(ctx, UnOp, unop, shape, {
        Minus = "-",
        Not = "not ",
        Hash = "#",
        #[cfg(feature = "lua53")]
        Tilde = "~",
    }, |other| panic!("unknown node {:?}", other))
}

/// Pushes a [`BinOp`] onto a newline, and indent its depending on indent_level.
/// Preserves any leading comments, and moves trailing comments to before the BinOp.
/// Also takes in the [`Expression`] present on the RHS of the BinOp - this is needed so that we can take any
/// leading comments from the expression, and place them before the BinOp.
fn hang_binop(ctx: &Context, binop: BinOp, shape: Shape, rhs: &Expression) -> BinOp {
    // Get the leading comments of a binop, as we need to preserve them
    // Intersperse a newline and indent trivia between them
    // iter_intersperse is currently not available, so we need to do something different. Tracking issue: https://github.com/rust-lang/rust/issues/79524
    let mut leading_comments = binop
        .leading_comments()
        .iter()
        .flat_map(|x| {
            vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
                x.to_owned(),
            ]
        })
        .collect::<Vec<_>>();

    // If there are any comments trailing the BinOp, we need to move them to before the BinOp
    let mut trailing_comments = binop.trailing_comments();
    leading_comments.append(&mut trailing_comments);

    // If there are any leading comments to the RHS expression, we need to move them to before the BinOp
    let mut expression_leading_comments = rhs
        .leading_comments()
        .iter()
        .flat_map(|x| {
            vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
                x.to_owned(),
            ]
        })
        .collect::<Vec<_>>();
    leading_comments.append(&mut expression_leading_comments);

    // Create a newline just before the BinOp, and preserve the indentation
    leading_comments.push(create_newline_trivia(ctx));
    leading_comments.push(create_indent_trivia(ctx, shape));

    binop.update_trivia(
        FormatTriviaType::Replace(leading_comments),
        FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]),
    )
}

/// Finds the length of the expression which matches the precedence level of the provided binop
fn binop_expression_length(expression: &Expression, top_binop: &BinOp) -> usize {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            if binop.precedence() >= top_binop.precedence()
                && binop.is_right_associative() == top_binop.is_right_associative()
            {
                if binop.is_right_associative() {
                    binop_expression_length(rhs, top_binop)
                        + strip_trivia(binop).to_string().len() + 2 // 2 = space before and after binop
                        + strip_trivia(&**lhs).to_string().len()
                } else {
                    binop_expression_length(lhs, top_binop)
                        + strip_trivia(binop).to_string().len() + 2 // 2 = space before and after binop
                        + strip_trivia(&**rhs).to_string().len()
                }
            } else {
                0
            }
        }
        _ => strip_trivia(expression).to_string().len(),
    }
}

fn binop_expression_contains_comments(expression: &Expression, top_binop: &BinOp) -> bool {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            if binop.precedence() == top_binop.precedence() {
                contains_comments(binop)
                    || rhs.has_leading_comments(CommentSearch::All)
                    || lhs.has_trailing_comments(CommentSearch::All)
                    || binop_expression_contains_comments(lhs, top_binop)
                    || binop_expression_contains_comments(rhs, top_binop)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Converts an item to a range
trait ToRange {
    fn to_range(&self) -> (usize, usize);
}

impl ToRange for (usize, usize) {
    fn to_range(&self) -> (usize, usize) {
        *self
    }
}

impl ToRange for Expression {
    fn to_range(&self) -> (usize, usize) {
        let (start, end) = self.range().unwrap();
        (start.bytes(), end.bytes())
    }
}

/// This struct encompasses information about the leftmost-expression in a BinaryExpression tree.
/// It holds the range of the leftmost binary expression, and the original additional indent level of this range.
/// This struct is only used when the hanging binary expression involves a hang level, for example:
/// ```lua
/// foo
///    + bar
///    + baz
/// ```
/// or in a larger context:
/// ```lua
/// local someVariable = foo
///    + bar
///    + baz
/// ```
/// As seen, the first item (`foo`) is inlined, and has an indent level one lower than the rest of the binary
/// expressions. We want to ensure that whenever we have `foo` in our expression, we use the original indentation level
/// because the expression is (at this current point in time) inlined - otherwise, it will be over-indented.
/// We hold the original indentation level in case we are deep down in the recursive calls:
/// ```lua
/// local ratio = (minAxis - minAxisSize) / delta * (self.props.maxScaleRatio - self.props.minScaleRatio)
///     + self.props.minScaleRatio
/// ```
/// Since the first line contains binary operators at a different precedence level to the `+`, then the indentation
/// level has been increased even further. But we want to use the original indentation level, because as it stands,
/// the expression is currently inlined on the original line.
#[derive(Clone, Copy, Debug)]
struct LeftmostRangeHang {
    range: (usize, usize),
    original_additional_indent_level: usize,
}

impl LeftmostRangeHang {
    /// Finds the leftmost expression from the given (full) expression, and then creates a [`LeftmostRangeHang`]
    /// to represent it
    fn find(expression: &Expression, original_additional_indent_level: usize) -> Self {
        match expression {
            Expression::BinaryOperator { lhs, .. } => {
                Self::find(lhs, original_additional_indent_level)
            }
            _ => Self {
                range: expression.to_range(),
                original_additional_indent_level,
            },
        }
    }

    /// Given an [`Expression`], returns the [`Shape`] to use for this expression.
    /// This function checks the provided expression to see if the LeftmostRange falls inside of it.
    /// If so, then we need to use the original indentation level shape, as (so far) the expression is inlined.
    fn required_shape<T: ToRange>(&self, shape: Shape, item: &T) -> Shape {
        let (expression_start, expression_end) = item.to_range();
        let (lhs_start, lhs_end) = self.range;

        if lhs_start >= expression_start && lhs_end <= expression_end {
            shape.with_indent(
                shape
                    .indent()
                    .with_additional_indent(self.original_additional_indent_level),
            )
        } else {
            shape
        }
    }
}

fn is_hang_binop_over_width(
    shape: Shape,
    expression: &Expression,
    top_binop: &BinOp,
    lhs_range: Option<LeftmostRangeHang>,
) -> bool {
    let shape = if let Some(lhs_hang) = lhs_range {
        lhs_hang.required_shape(shape, expression)
    } else {
        shape
    };

    shape
        .add_width(binop_expression_length(expression, top_binop))
        .over_budget()
}

/// If present, finds the precedence level of the provided binop in the BinOp expression. Otherwise, returns 0
fn binop_precedence_level(expression: &Expression) -> u8 {
    match expression {
        Expression::BinaryOperator { binop, .. } => binop.precedence(),
        _ => 0,
    }
}

fn did_hang_expression(expression: &Expression) -> bool {
    if let Expression::BinaryOperator { binop, .. } = expression {
        // Examine the binop's leading trivia for a newline
        // TODO: this works..., but is it the right solution?
        binop
            .surrounding_trivia()
            .0
            .iter()
            .any(|x| trivia_is_newline(x))
    } else {
        false
    }
}

#[derive(Debug)]
enum ExpressionSide {
    Left,
    Right,
}

fn hang_binop_expression(
    ctx: &Context,
    expression: Expression,
    top_binop: BinOp,
    shape: Shape,
    lhs_range: Option<LeftmostRangeHang>,
) -> Expression {
    const SPACE_LEN: usize = " ".len();

    let full_expression = expression.to_owned();

    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            // Keep grouping together all operators with the same precedence level as the main BinOp
            // They should also have the same associativity
            let same_op_level = binop.precedence() == top_binop.precedence()
                && binop.is_right_associative() == top_binop.is_right_associative();
            let is_right_associative = binop.is_right_associative();

            let test_shape = if same_op_level {
                shape
            } else {
                shape.increment_additional_indent()
            };

            let side_to_hang = if is_right_associative {
                ExpressionSide::Right
            } else {
                ExpressionSide::Left
            };

            // TODO/FIXME: using test_shape here leads to too high of an indent level, causing the expression to hang unnecessarily
            let over_column_width =
                is_hang_binop_over_width(test_shape, &full_expression, &binop, lhs_range);
            let should_hang = same_op_level
                || over_column_width
                || binop_expression_contains_comments(&full_expression, &binop);

            // Only use the indented shape if we are planning to hang
            let shape = if should_hang { test_shape } else { shape };

            let mut new_binop = format_binop(ctx, &binop, shape);
            if should_hang {
                new_binop = hang_binop(ctx, binop.to_owned(), shape, &rhs);
            }

            let (lhs, rhs) = match should_hang {
                true => {
                    let lhs_shape = shape;
                    let rhs_shape =
                        shape.reset() + strip_trivia(&new_binop).to_string().len() + SPACE_LEN;

                    let (lhs, rhs) = match side_to_hang {
                        ExpressionSide::Left => (
                            hang_binop_expression(
                                ctx,
                                *lhs,
                                if same_op_level {
                                    top_binop
                                } else {
                                    binop.clone()
                                },
                                lhs_shape,
                                lhs_range,
                            ),
                            if contains_comments(&*rhs) {
                                hang_binop_expression(ctx, *rhs, binop, shape, lhs_range)
                            } else {
                                format_expression_internal(
                                    ctx,
                                    &rhs,
                                    ExpressionContext::UnaryOrBinary,
                                    rhs_shape,
                                )
                            },
                        ),
                        ExpressionSide::Right => (
                            if contains_comments(&*lhs) {
                                hang_binop_expression(ctx, *lhs, binop.clone(), shape, lhs_range)
                            } else {
                                let context = if let BinOp::Caret(_) = binop {
                                    ExpressionContext::BinaryLHSExponent
                                } else {
                                    ExpressionContext::BinaryLHS
                                };
                                format_expression_internal(ctx, &lhs, context, lhs_shape)
                            },
                            hang_binop_expression(
                                ctx,
                                *rhs,
                                if same_op_level { top_binop } else { binop },
                                rhs_shape,
                                lhs_range,
                            ),
                        ),
                    };
                    (
                        lhs,
                        rhs.update_leading_trivia(FormatTriviaType::Replace(Vec::new())),
                    )
                }
                false => {
                    // Check if the chain still has comments deeper inside of it.
                    // If it does, we need to hang that part of the chain still, otherwise the comments will mess it up
                    let lhs = if contains_comments(&*lhs) {
                        hang_binop_expression(ctx, *lhs, binop.to_owned(), shape, lhs_range)
                    } else {
                        let context = if let BinOp::Caret(_) = binop {
                            ExpressionContext::BinaryLHSExponent
                        } else {
                            ExpressionContext::BinaryLHS
                        };
                        format_expression_internal(ctx, &lhs, context, shape)
                    };

                    let rhs = if contains_comments(&*rhs) {
                        hang_binop_expression(ctx, *rhs, binop, shape, lhs_range)
                    } else {
                        format_expression_internal(
                            ctx,
                            &rhs,
                            ExpressionContext::UnaryOrBinary,
                            shape,
                        )
                    };

                    (lhs, rhs)
                }
            };

            Expression::BinaryOperator {
                lhs: Box::new(lhs),
                binop: new_binop,
                rhs: Box::new(rhs),
            }
        }
        // Base case: no more binary operators - just return to normal splitting
        _ => format_hanging_expression_(
            ctx,
            &expression,
            shape,
            ExpressionContext::Standard,
            lhs_range,
        ),
    }
}

/// Internal expression formatter, where the binop is also hung
fn format_hanging_expression_(
    ctx: &Context,
    expression: &Expression,
    shape: Shape,
    expression_context: ExpressionContext,
    lhs_range: Option<LeftmostRangeHang>,
) -> Expression {
    let expression_range = expression.to_range();

    match expression {
        #[cfg(feature = "luau")]
        Expression::TypeAssertion {
            expression,
            type_assertion,
        } => {
            // If we have a type assertion, we increment the current shape with the size of the assertion
            // to "force" the parentheses to hang if necessary
            let (expression_context, value_shape) = (
                ExpressionContext::TypeAssertion,
                shape.take_first_line(&strip_trivia(type_assertion)),
            );

            let expression = format_hanging_expression_(
                ctx,
                expression,
                value_shape,
                expression_context,
                lhs_range,
            );

            // Update the shape used to format the type assertion
            #[cfg(feature = "luau")]
            let assertion_shape = shape.take_last_line(&expression);

            Expression::TypeAssertion {
                expression: Box::new(expression),
                type_assertion: format_type_assertion(ctx, type_assertion, assertion_shape),
            }
        }
        Expression::Parentheses {
            contained,
            expression,
        } => {
            let lhs_shape = if let Some(lhs_hang) = lhs_range {
                lhs_hang.required_shape(shape, &expression_range)
            } else {
                shape
            };
            #[cfg(feature = "luau")]
            let keep_parentheses = matches!(
                expression_context,
                ExpressionContext::Prefix | ExpressionContext::TypeAssertion
            );
            #[cfg(not(feature = "luau"))]
            let keep_parentheses = matches!(expression_context, ExpressionContext::Prefix);

            // Examine whether the internal expression requires parentheses
            // If not, just format and return the internal expression. Otherwise, format the parentheses
            let use_internal_expression = check_excess_parentheses(expression, expression_context);

            // If the context is for a prefix, we should always keep the parentheses, as they are always required
            if use_internal_expression && !keep_parentheses {
                format_hanging_expression_(
                    ctx,
                    expression,
                    lhs_shape,
                    expression_context,
                    lhs_range,
                )
            } else {
                let contained = format_contained_span(ctx, contained, lhs_shape);

                // Provide a sample formatting to see how large it is
                // Examine the expression itself to see if needs to be split onto multiple lines
                let formatted_expression = format_expression(ctx, expression, lhs_shape + 1); // 1 = opening parentheses

                let expression_str = formatted_expression.to_string();
                if !contains_comments(expression)
                    && !lhs_shape.add_width(2 + expression_str.len()).over_budget()
                {
                    // The expression inside the parentheses is small, we do not need to break it down further
                    return Expression::Parentheses {
                        contained,
                        expression: Box::new(formatted_expression),
                    };
                }

                // Update the expression shape to be used inside the parentheses, applying the indent increase
                let expression_shape = lhs_shape.reset().increment_additional_indent();

                // Modify the parentheses to hang the expression
                let (start_token, end_token) = contained.tokens();

                // Create a newline after the start brace and before the end brace
                // Also, indent enough for the first expression in the start brace
                let contained = ContainedSpan::new(
                    start_token.update_trailing_trivia(FormatTriviaType::Append(vec![
                        create_newline_trivia(ctx),
                        create_indent_trivia(ctx, expression_shape),
                    ])),
                    end_token.update_leading_trivia(FormatTriviaType::Append(vec![
                        create_newline_trivia(ctx),
                        create_indent_trivia(ctx, lhs_shape),
                    ])),
                );

                Expression::Parentheses {
                    contained,
                    expression: Box::new(format_hanging_expression_(
                        ctx,
                        expression,
                        expression_shape,
                        ExpressionContext::Standard,
                        None,
                    )),
                }
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            let unop = format_unop(ctx, unop, shape);
            let shape = shape + strip_leading_trivia(&unop).to_string().len();
            let expression = format_hanging_expression_(
                ctx,
                expression,
                shape,
                ExpressionContext::UnaryOrBinary,
                lhs_range,
            );

            Expression::UnaryOperator {
                unop,
                expression: Box::new(expression),
            }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            // Don't format the lhs and rhs here, because it will be handled later when hang_binop_expression calls back for a Value
            let lhs =
                hang_binop_expression(ctx, *lhs.to_owned(), binop.to_owned(), shape, lhs_range);

            let current_shape = shape.take_last_line(&lhs) + 1; // 1 = space before binop
            let mut new_binop = format_binop(ctx, binop, current_shape);

            let singleline_shape = current_shape + strip_trivia(binop).to_string().len() + 1; // 1 = space after binop

            let mut new_rhs = hang_binop_expression(
                ctx,
                *rhs.to_owned(),
                binop.to_owned(),
                singleline_shape,
                None,
            );

            // Examine the last line to see if we need to hang this binop, or if the precedence levels match
            if (did_hang_expression(&lhs) && binop_precedence_level(&lhs) >= binop.precedence())
                || (did_hang_expression(&new_rhs)
                    && binop_precedence_level(&new_rhs) >= binop.precedence())
                || contains_comments(binop)
                || lhs.has_trailing_comments(CommentSearch::All)
                || (shape.take_last_line(&lhs) + format!("{binop}{rhs}").len()).over_budget()
            {
                let hanging_shape = shape.reset() + strip_trivia(binop).to_string().len() + 1;
                new_binop = hang_binop(ctx, binop.to_owned(), shape, rhs);
                new_rhs = hang_binop_expression(
                    ctx,
                    *rhs.to_owned(),
                    binop.to_owned(),
                    hanging_shape,
                    None,
                )
                .update_leading_trivia(FormatTriviaType::Replace(Vec::new()));
            }

            Expression::BinaryOperator {
                lhs: Box::new(lhs),
                binop: new_binop,
                rhs: Box::new(new_rhs),
            }
        }
        _ => {
            let value_shape = if let Some(lhs_hang) = lhs_range {
                lhs_hang.required_shape(shape, &expression_range)
            } else {
                shape
            };

            format_expression_internal(ctx, expression, expression_context, value_shape)
        }
    }
}

pub fn hang_expression(
    ctx: &Context,
    expression: &Expression,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression {
    let original_additional_indent_level = shape.indent().additional_indent();
    let shape = match hang_level {
        Some(hang_level) => shape.with_indent(shape.indent().add_indent_level(hang_level)),
        None => shape,
    };

    let lhs_range =
        hang_level.map(|_| LeftmostRangeHang::find(expression, original_additional_indent_level));

    format_hanging_expression_(
        ctx,
        expression,
        shape,
        ExpressionContext::Standard,
        lhs_range,
    )
}

pub fn hang_expression_trailing_newline(
    ctx: &Context,
    expression: &Expression,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression {
    hang_expression(ctx, expression, shape, hang_level)
        .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]))
}
