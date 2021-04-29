use full_moon::ast::{
    span::ContainedSpan, BinOp, Expression, Index, Prefix, Suffix, UnOp, Value, Var, VarExpression,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_assertion;
use crate::{
    context::{create_newline_trivia, create_plain_indent_trivia, Context},
    fmt_symbol,
    formatters::{
        functions::{format_anonymous_function, format_call, format_function_call},
        general::{format_contained_span, format_token_reference},
        table::format_table_constructor,
        trivia::{
            strip_leading_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia,
            UpdateTrivia,
        },
        trivia_util,
    },
    shape::Shape,
};

#[macro_export]
macro_rules! fmt_op {
    ($ctx:expr, $enum:ident, $value:ident, { $($operator:ident = $output:expr,)+ }) => {
        match $value {
            $(
                $enum::$operator(token) => $enum::$operator(fmt_symbol!($ctx, token, $output)),
            )+
            other => panic!("unknown node {:?}", other),
        }
    };
}

enum ExpressionContext {
    /// Standard expression, with no special context
    Standard,
    /// The expression originates from a [`Prefix`] node. The special context here is that the expression will
    /// always be wrapped in parentheses.
    Prefix,
}

pub fn format_binop<'ast>(ctx: &Context, binop: &BinOp<'ast>) -> BinOp<'ast> {
    fmt_op!(ctx, BinOp, binop, {
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
    })
}

/// Check to determine whether expression parentheses are required, depending on the provided
/// internal expression contained within the parentheses
fn check_excess_parentheses(internal_expression: &Expression) -> bool {
    match internal_expression {
        // Parentheses inside parentheses, not necessary
        Expression::Parentheses { .. } => true,
        // Check whether the expression relating to the UnOp is safe
        Expression::UnaryOperator { expression, .. } => check_excess_parentheses(expression),
        // Don't bother removing them if there is a binop, as they may be needed. TODO: can we be more intelligent here?
        Expression::BinaryOperator { .. } => false,
        Expression::Value { value, .. } => {
            match &**value {
                // Internal expression is a function call
                // We could potentially be culling values, so we should not remove parentheses
                Value::FunctionCall(_) => false,
                Value::Symbol(token_ref) => {
                    match token_ref.token_type() {
                        // If we have an ellipse inside of parentheses, we may also be culling values
                        // Therefore, we don't remove parentheses
                        TokenType::Symbol { symbol } => !matches!(symbol, Symbol::Ellipse),
                        _ => true,
                    }
                }
                _ => true,
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats an Expression node
pub fn format_expression<'ast>(
    ctx: &mut Context,
    expression: &Expression<'ast>,
    shape: Shape,
) -> Expression<'ast> {
    format_expression_internal(ctx, expression, ExpressionContext::Standard, shape)
}

/// Internal expression formatter, with access to expression context
fn format_expression_internal<'ast>(
    ctx: &mut Context,
    expression: &Expression<'ast>,
    context: ExpressionContext,
    shape: Shape,
) -> Expression<'ast> {
    match expression {
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => Expression::Value {
            value: Box::new(format_value(ctx, value, shape)),
            #[cfg(feature = "luau")]
            type_assertion: match type_assertion {
                Some(assertion) => Some(format_type_assertion(ctx, assertion)),
                None => None,
            },
        },
        Expression::Parentheses {
            contained,
            expression,
        } => {
            // Examine whether the internal expression requires parentheses
            // If not, just format and return the internal expression. Otherwise, format the parentheses
            let use_internal_expression = check_excess_parentheses(expression);

            // If the context is for a prefix, we should always keep the parentheses, as they are always required
            if use_internal_expression && !matches!(context, ExpressionContext::Prefix) {
                format_expression(ctx, expression, shape)
            } else {
                Expression::Parentheses {
                    contained: format_contained_span(ctx, &contained),
                    expression: Box::new(format_expression(ctx, expression, shape + 1)), // 1 = opening parentheses
                }
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            let unop = format_unop(ctx, unop);
            let shape = shape + strip_leading_trivia(&unop).to_string().len();
            Expression::UnaryOperator {
                unop,
                expression: Box::new(format_expression(ctx, expression, shape)),
            }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let lhs = format_expression(ctx, lhs, shape);
            let binop = format_binop(ctx, binop);
            let shape = shape.take_last_line(&lhs) + binop.to_string().len();
            Expression::BinaryOperator {
                lhs: Box::new(lhs),
                binop,
                rhs: Box::new(format_expression(ctx, rhs, shape)),
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats an Index Node
pub fn format_index<'ast>(ctx: &mut Context, index: &Index<'ast>, shape: Shape) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: format_contained_span(ctx, &brackets),
            expression: format_expression(ctx, expression, shape + 1), // 1 = opening bracket
        },

        Index::Dot { dot, name } => Index::Dot {
            dot: format_token_reference(ctx, dot),
            name: format_token_reference(ctx, name),
        },
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Prefix Node
pub fn format_prefix<'ast>(ctx: &mut Context, prefix: &Prefix<'ast>, shape: Shape) -> Prefix<'ast> {
    match prefix {
        Prefix::Expression(expression) => Prefix::Expression(format_expression_internal(
            ctx,
            expression,
            ExpressionContext::Prefix,
            shape,
        )),
        Prefix::Name(token_reference) => Prefix::Name(format_token_reference(ctx, token_reference)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Suffix Node
pub fn format_suffix<'ast>(ctx: &mut Context, suffix: &Suffix<'ast>, shape: Shape) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => Suffix::Call(format_call(ctx, call, shape)),
        Suffix::Index(index) => Suffix::Index(format_index(ctx, index, shape)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Value Node
pub fn format_value<'ast>(ctx: &mut Context, value: &Value<'ast>, shape: Shape) -> Value<'ast> {
    match value {
        Value::Function((token_reference, function_body)) => Value::Function(
            format_anonymous_function(ctx, token_reference, function_body),
        ),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(format_function_call(ctx, function_call, shape))
        }
        Value::Number(token_reference) => {
            Value::Number(format_token_reference(ctx, token_reference))
        }
        Value::ParenthesesExpression(expression) => {
            Value::ParenthesesExpression(format_expression(ctx, expression, shape))
        }
        Value::String(token_reference) => {
            Value::String(format_token_reference(ctx, token_reference))
        }
        Value::Symbol(token_reference) => {
            Value::Symbol(format_token_reference(ctx, token_reference))
        }
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(format_table_constructor(ctx, table_constructor, shape))
        }
        Value::Var(var) => Value::Var(format_var(ctx, var, shape)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Var Node
pub fn format_var<'ast>(ctx: &mut Context, var: &Var<'ast>, shape: Shape) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(format_token_reference(ctx, token_reference)),
        Var::Expression(var_expression) => {
            Var::Expression(format_var_expression(ctx, var_expression, shape))
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_var_expression<'ast>(
    ctx: &mut Context,
    var_expression: &VarExpression<'ast>,
    shape: Shape,
) -> VarExpression<'ast> {
    let formatted_prefix = format_prefix(ctx, var_expression.prefix(), shape);
    let mut shape = shape + strip_leading_trivia(&formatted_prefix).to_string().len();

    let formatted_suffixes = var_expression
        .suffixes()
        .map(|x| {
            let suffix = format_suffix(ctx, x, shape);
            shape = shape + suffix.to_string().len();
            suffix
        })
        .collect();

    VarExpression::new(formatted_prefix).with_suffixes(formatted_suffixes)
}

/// Formats an UnOp Node
pub fn format_unop<'ast>(ctx: &Context, unop: &UnOp<'ast>) -> UnOp<'ast> {
    fmt_op!(ctx, UnOp, unop, {
        Minus = "-",
        Not = "not ",
        Hash = "#",
    })
}

/// Pushes a BinOp onto a newline, and indent its depending on indent_level. Moves trailing comments to before the BinOp.
/// Does not hang if the BinOp is a relational operator.
fn hang_binop<'ast>(ctx: &Context, binop: BinOp<'ast>, indent_level: usize) -> BinOp<'ast> {
    match binop {
        // Don't add the trivia if the binop is binding
        BinOp::GreaterThan(_)
        | BinOp::GreaterThanEqual(_)
        | BinOp::LessThan(_)
        | BinOp::LessThanEqual(_)
        | BinOp::TildeEqual(_)
        | BinOp::TwoEqual(_) => {
            // Return original binop
            binop
        }
        _ => {
            // If there are any comments trailing the BinOp, we need to move them to before the BinOp
            let mut trailing_comments = trivia_util::binop_trailing_comments(&binop);
            // Create a newline just before the BinOp, and preserve the indentation
            trailing_comments.push(create_newline_trivia(ctx));
            trailing_comments.push(create_plain_indent_trivia(ctx, indent_level));

            binop.update_trivia(
                FormatTriviaType::Replace(trailing_comments),
                FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]),
            )
        }
    }
}

/// Finds the length of the expression which matches the precedence level of the provided binop
fn binop_expression_length<'ast>(expression: &Expression<'ast>, top_binop: &BinOp<'ast>) -> usize {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            if binop.precedence() == top_binop.precedence()
                && binop.is_right_associative() == top_binop.is_right_associative()
            {
                if binop.is_right_associative() {
                    binop_expression_length(rhs, top_binop)
                        + binop.to_string().len()
                        + lhs.to_string().len()
                } else {
                    binop_expression_length(lhs, top_binop)
                        + binop.to_string().len()
                        + rhs.to_string().len()
                }
            } else {
                0
            }
        }
        _ => expression.to_string().len(),
    }
}

fn hang_binop_expression<'ast>(
    ctx: &Context,
    expression: Expression<'ast>,
    top_binop: BinOp<'ast>,
    shape: Shape,
    indent_level: usize,
) -> Expression<'ast> {
    let full_expression = expression.to_owned();

    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            // Keep grouping together all operators with the same precedence level as the main BinOp
            // They should also have the same associativity
            let same_op_level = binop.precedence() == top_binop.precedence()
                && binop.is_right_associative() == top_binop.is_right_associative();
            let is_right_associative = top_binop.is_right_associative();

            let indent_level = if same_op_level {
                indent_level
            } else {
                indent_level + 1
            };

            let side_to_use = if is_right_associative {
                rhs.to_owned()
            } else {
                lhs.to_owned()
            };

            let over_column_width = shape
                .add_width(binop_expression_length(&full_expression, &binop))
                .over_budget();

            let (binop, updated_side) = if same_op_level || over_column_width {
                let op = hang_binop(ctx, binop.to_owned(), indent_level);

                let side = hang_binop_expression(
                    ctx,
                    *side_to_use,
                    if same_op_level { top_binop } else { binop },
                    Shape::with_indent_level(ctx, indent_level)
                        + strip_leading_trivia(&op).to_string().len(),
                    indent_level,
                );

                (op, side)
            } else {
                (binop, *side_to_use)
            };

            if is_right_associative {
                Expression::BinaryOperator {
                    lhs,
                    binop,
                    rhs: Box::new(updated_side),
                }
            } else {
                Expression::BinaryOperator {
                    lhs: Box::new(updated_side),
                    binop,
                    rhs,
                }
            }
        }
        // Base case: no more binary operators - just return to normal splitting
        _ => expression_split_binop(ctx, expression, shape, indent_level),
    }
}

fn expression_split_binop<'ast>(
    ctx: &Context,
    expression: Expression<'ast>,
    shape: Shape,
    indent_increase: usize,
) -> Expression<'ast> {
    match expression {
        Expression::Parentheses {
            contained,
            expression,
        } => {
            // Examine the expression itself to see if needs to be split onto multiple lines
            let expression_str = expression.to_string();
            if !shape.add_width(2 + expression_str.len()).over_budget() {
                // The expression inside the parentheses is small, we do not need to break it down further
                return Expression::Parentheses {
                    contained,
                    expression,
                };
            }

            // Modify the parentheses to hang the expression
            let (start_token, end_token) = contained.tokens();
            // Create a newline after the start brace and before the end brace
            // Also, indent enough for the first expression in the start brace
            let contained = ContainedSpan::new(
                start_token.update_trailing_trivia(FormatTriviaType::Append(vec![
                    create_newline_trivia(ctx),
                    create_plain_indent_trivia(ctx, indent_increase + 1),
                ])),
                end_token.update_leading_trivia(FormatTriviaType::Append(vec![
                    create_newline_trivia(ctx),
                    create_plain_indent_trivia(ctx, indent_increase),
                ])),
            );

            Expression::Parentheses {
                contained,
                expression: Box::new(expression_split_binop(
                    ctx,
                    *expression,
                    Shape::with_indent_level(ctx, indent_increase + 1),
                    indent_increase + 1, // Apply indent increase
                )),
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            let expression = Box::new(expression_split_binop(
                ctx,
                *expression,
                shape + strip_leading_trivia(&unop).to_string().len(),
                indent_increase,
            ));
            Expression::UnaryOperator { unop, expression }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let lhs = Box::new(hang_binop_expression(
                ctx,
                *lhs,
                binop.to_owned(),
                shape,
                indent_increase,
            ));
            let rhs = Box::new(hang_binop_expression(
                ctx,
                *rhs,
                binop.to_owned(),
                shape,
                indent_increase,
            ));
            let binop = hang_binop(ctx, binop, indent_increase);

            Expression::BinaryOperator { lhs, binop, rhs }
        }

        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => Expression::Value {
            value: match *value {
                Value::ParenthesesExpression(expression) => Box::new(Value::ParenthesesExpression(
                    expression_split_binop(ctx, expression, shape, indent_increase),
                )),
                _ => value,
            },
            #[cfg(feature = "luau")]
            type_assertion,
        },

        // Can't hang anything else, so just return the original expression
        _ => expression,
    }
}

pub fn hang_expression<'ast>(
    ctx: &Context,
    expression: Expression<'ast>,
    additional_indent_level: Option<usize>,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression<'ast> {
    let additional_indent_level = additional_indent_level.unwrap_or(0) + hang_level.unwrap_or(0);
    let hang_level = ctx.indent_level() + additional_indent_level;

    expression_split_binop(ctx, expression, shape, hang_level)
}

pub fn hang_expression_trailing_newline<'ast>(
    ctx: &Context,
    expression: Expression<'ast>,
    additional_indent_level: Option<usize>,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression<'ast> {
    hang_expression(ctx, expression, additional_indent_level, shape, hang_level)
        .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]))
}
