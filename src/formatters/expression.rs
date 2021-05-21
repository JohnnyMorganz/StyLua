use full_moon::{
    ast::{
        span::ContainedSpan, BinOp, Call, Expression, Index, Prefix, Suffix, UnOp, Value, Var,
        VarExpression,
    },
    node::Node,
    tokenizer::{Symbol, Token, TokenReference, TokenType},
};
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_assertion;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        functions::{format_anonymous_function, format_call, format_function_call},
        general::{format_contained_span, format_token_reference},
        table::format_table_constructor,
        trivia::{
            strip_leading_trivia, strip_trivia, FormatTriviaType, UpdateLeadingTrivia,
            UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{self, contains_comments, get_expression_trailing_trivia},
    },
    shape::Shape,
};

#[macro_export]
macro_rules! fmt_op {
    ($ctx:expr, $enum:ident, $value:ident, $shape:expr, { $($operator:ident = $output:expr,)+ }) => {
        match $value {
            $(
                $enum::$operator(token) => $enum::$operator(fmt_symbol!($ctx, token, $output, $shape)),
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

pub fn format_binop<'ast>(ctx: &Context, binop: &BinOp<'ast>, shape: Shape) -> BinOp<'ast> {
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
    ctx: &Context,
    expression: &Expression<'ast>,
    shape: Shape,
) -> Expression<'ast> {
    format_expression_internal(ctx, expression, ExpressionContext::Standard, shape)
}

/// Internal expression formatter, with access to expression context
fn format_expression_internal<'ast>(
    ctx: &Context,
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
                Some(assertion) => Some(format_type_assertion(ctx, assertion, shape)),
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
                    contained: format_contained_span(ctx, &contained, shape),
                    expression: Box::new(format_expression(ctx, expression, shape + 1)), // 1 = opening parentheses
                }
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            let unop = format_unop(ctx, unop, shape);
            let shape = shape + strip_leading_trivia(&unop).to_string().len();
            Expression::UnaryOperator {
                unop,
                expression: Box::new(format_expression(ctx, expression, shape)),
            }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            let lhs = format_expression(ctx, lhs, shape);
            let binop = format_binop(ctx, binop, shape);
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
pub fn format_index<'ast>(ctx: &Context, index: &Index<'ast>, shape: Shape) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: format_contained_span(ctx, &brackets, shape),
            expression: format_expression(ctx, expression, shape + 1), // 1 = opening bracket
        },

        Index::Dot { dot, name } => Index::Dot {
            dot: format_token_reference(ctx, dot, shape),
            name: format_token_reference(ctx, name, shape),
        },
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Prefix Node
pub fn format_prefix<'ast>(ctx: &Context, prefix: &Prefix<'ast>, shape: Shape) -> Prefix<'ast> {
    match prefix {
        Prefix::Expression(expression) => {
            let singleline_format =
                format_expression_internal(ctx, expression, ExpressionContext::Prefix, shape);
            let singeline_shape = shape.take_first_line(&strip_trivia(&singleline_format));

            if singeline_shape.over_budget() {
                Prefix::Expression(format_hanging_expression_(
                    ctx,
                    expression,
                    shape,
                    ExpressionContext::Prefix,
                    None,
                ))
            } else {
                Prefix::Expression(singleline_format)
            }
        }
        Prefix::Name(token_reference) => {
            Prefix::Name(format_token_reference(ctx, token_reference, shape))
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Suffix Node
pub fn format_suffix<'ast>(
    ctx: &Context,
    suffix: &Suffix<'ast>,
    shape: Shape,
    no_parens_ambiguous_next_node: bool,
) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => {
            Suffix::Call(format_call(ctx, call, shape, no_parens_ambiguous_next_node))
        }
        Suffix::Index(index) => Suffix::Index(format_index(ctx, index, shape)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Value Node
pub fn format_value<'ast>(ctx: &Context, value: &Value<'ast>, shape: Shape) -> Value<'ast> {
    match value {
        Value::Function((token_reference, function_body)) => Value::Function(
            format_anonymous_function(ctx, token_reference, function_body, shape),
        ),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(format_function_call(ctx, function_call, shape))
        }
        Value::Number(token_reference) => {
            Value::Number(format_token_reference(ctx, token_reference, shape))
        }
        Value::ParenthesesExpression(expression) => {
            Value::ParenthesesExpression(format_expression(ctx, expression, shape))
        }
        Value::String(token_reference) => {
            Value::String(format_token_reference(ctx, token_reference, shape))
        }
        Value::Symbol(token_reference) => {
            Value::Symbol(format_token_reference(ctx, token_reference, shape))
        }
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(format_table_constructor(ctx, table_constructor, shape))
        }
        Value::Var(var) => Value::Var(format_var(ctx, var, shape)),
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Var Node
pub fn format_var<'ast>(ctx: &Context, var: &Var<'ast>, shape: Shape) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => {
            Var::Name(format_token_reference(ctx, token_reference, shape))
        }
        Var::Expression(var_expression) => {
            Var::Expression(format_var_expression(ctx, var_expression, shape))
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_var_expression<'ast>(
    ctx: &Context,
    var_expression: &VarExpression<'ast>,
    shape: Shape,
) -> VarExpression<'ast> {
    let formatted_prefix = format_prefix(ctx, var_expression.prefix(), shape);
    let mut shape = shape + strip_leading_trivia(&formatted_prefix).to_string().len();

    let mut formatted_suffixes = Vec::new();
    let mut suffixes = var_expression.suffixes().peekable();

    while let Some(suffix) = suffixes.next() {
        // If the suffix after this one is something like `.foo` or `:foo` - this affects removing parentheses
        let ambiguous_next_suffix = matches!(
            suffixes.peek(),
            Some(Suffix::Index(_)) | Some(Suffix::Call(Call::MethodCall(_)))
        );

        let suffix = format_suffix(ctx, suffix, shape, ambiguous_next_suffix);
        shape = shape + suffix.to_string().len();
        formatted_suffixes.push(suffix);
    }

    VarExpression::new(formatted_prefix).with_suffixes(formatted_suffixes)
}

/// Formats an UnOp Node
pub fn format_unop<'ast>(ctx: &Context, unop: &UnOp<'ast>, shape: Shape) -> UnOp<'ast> {
    fmt_op!(ctx, UnOp, unop, shape, {
        Minus = "-",
        Not = "not ",
        Hash = "#",
    })
}

/// Pushes a [`BinOp`] onto a newline, and indent its depending on indent_level.
/// Preserves any leading comments, and moves trailing comments to before the BinOp.
/// Also takes in the [`Expression`] present on the RHS of the BinOp - this is needed so that we can take any
/// leading comments from the expression, and place them before the BinOp.
fn hang_binop<'ast>(
    ctx: &Context,
    binop: BinOp<'ast>,
    shape: Shape,
    rhs: &Expression<'ast>,
) -> BinOp<'ast> {
    // Get the leading comments of a binop, as we need to preserve them
    // Intersperse a newline and indent trivia between them
    // iter_intersperse is currently not available, so we need to do something different. Tracking issue: https://github.com/rust-lang/rust/issues/79524
    let mut leading_comments = trivia_util::binop_leading_comments(&binop)
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
    let mut trailing_comments = trivia_util::binop_trailing_comments(&binop);
    leading_comments.append(&mut trailing_comments);

    // If there are any leading comments to the RHS expression, we need to move them to before the BinOp
    let mut expression_leading_comments = trivia_util::expression_leading_comments(rhs)
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

/// Converts an item to a range
trait ToRange {
    fn to_range(&self) -> (usize, usize);
}

impl ToRange for (usize, usize) {
    fn to_range(&self) -> (usize, usize) {
        *self
    }
}

impl ToRange for Expression<'_> {
    fn to_range(&self) -> (usize, usize) {
        let (start, end) = self.range().unwrap();
        (start.bytes(), end.bytes())
    }
}

/// This struct encompasses information about the leftmost-expression in a BinaryExpression tree.
/// It holds the range of the leftmost binary expression, and the original additional indent level of this range.
/// This struct is only used when the hanging binary expression involves a hang level, for example:
/// ```lua
/// foooo
///    + bar
///    + baz
/// ```
/// or in a larger context:
/// ```lua
/// local someVariable = foooo
///    + bar
///    + baz
/// ```
/// As seen, the first item (`foooo`) is inlined, and has an indent level one lower than the rest of the binary
/// expressions. We want to ensure that whenever we have `foooo` in our expression, we use the original indentation level
/// because the expression is (at this current point in time) inlined - otherwise, it will be over-indented.
/// We hold the original indentation level incase we are deep down in the recursivecalls:
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

enum ExpressionSide {
    Left,
    Right,
}

fn hang_binop_expression<'ast>(
    ctx: &Context,
    expression: Expression<'ast>,
    top_binop: BinOp<'ast>,
    shape: Shape,
    lhs_range: Option<LeftmostRangeHang>,
) -> Expression<'ast> {
    let full_expression = expression.to_owned();

    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            // Keep grouping together all operators with the same precedence level as the main BinOp
            // They should also have the same associativity
            let same_op_level = binop.precedence() == top_binop.precedence()
                && binop.is_right_associative() == top_binop.is_right_associative();
            let is_right_associative = top_binop.is_right_associative();

            let shape = if same_op_level {
                shape
            } else {
                shape.increment_additional_indent()
            };

            let side_to_hang = if is_right_associative {
                ExpressionSide::Right
            } else {
                ExpressionSide::Left
            };

            let over_column_width =
                is_hang_binop_over_width(shape, &full_expression, &binop, lhs_range);
            let should_hang = same_op_level || over_column_width;

            let mut new_binop = format_binop(ctx, &binop, shape);
            if should_hang {
                new_binop = hang_binop(ctx, binop.to_owned(), shape, &rhs);
            }

            let (lhs, rhs) = match should_hang {
                true => {
                    let lhs_shape = shape;
                    let rhs_shape = shape + strip_trivia(&new_binop).to_string().len() + 1;

                    let (lhs, rhs) = match side_to_hang {
                        ExpressionSide::Left => (
                            hang_binop_expression(
                                ctx,
                                *lhs,
                                if same_op_level { top_binop } else { binop },
                                lhs_shape,
                                lhs_range,
                            ),
                            format_expression(ctx, &*rhs, rhs_shape),
                        ),
                        ExpressionSide::Right => (
                            format_expression(ctx, &*lhs, lhs_shape),
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
                false => (
                    format_expression(ctx, &*lhs, shape),
                    format_expression(ctx, &*rhs, shape),
                ),
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
fn format_hanging_expression_<'ast>(
    ctx: &Context,
    expression: &Expression<'ast>,
    shape: Shape,
    expression_context: ExpressionContext,
    lhs_range: Option<LeftmostRangeHang>,
) -> Expression<'ast> {
    let expression_range = expression.to_range();

    match expression {
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => {
            let value = Box::new(match &**value {
                Value::ParenthesesExpression(expression) => {
                    Value::ParenthesesExpression(format_hanging_expression_(
                        ctx,
                        expression,
                        shape,
                        expression_context,
                        lhs_range,
                    ))
                }
                _ => {
                    let shape = if let Some(lhs_hang) = lhs_range {
                        lhs_hang.required_shape(shape, &expression_range)
                    } else {
                        shape
                    };
                    format_value(ctx, value, shape)
                }
            });
            Expression::Value {
                value,
                #[cfg(feature = "luau")]
                type_assertion: match type_assertion {
                    Some(assertion) => Some(format_type_assertion(ctx, assertion, shape)),
                    None => None,
                },
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

            // Examine whether the internal expression requires parentheses
            // If not, just format and return the internal expression. Otherwise, format the parentheses
            let use_internal_expression = check_excess_parentheses(expression);

            // If the context is for a prefix, we should always keep the parentheses, as they are always required
            if use_internal_expression && !matches!(expression_context, ExpressionContext::Prefix) {
                format_hanging_expression_(
                    ctx,
                    expression,
                    lhs_shape,
                    expression_context,
                    lhs_range,
                )
            } else {
                let contained = format_contained_span(ctx, &contained, lhs_shape);

                // Provide a sample formatting to see how large it is
                // Examine the expression itself to see if needs to be split onto multiple lines
                let formatted_expression = format_expression(ctx, expression, lhs_shape + 1); // 1 = opening parentheses

                let expression_str = formatted_expression.to_string();
                if !lhs_shape.add_width(2 + expression_str.len()).over_budget() {
                    // The expression inside the parentheses is small, we do not need to break it down further
                    return Expression::Parentheses {
                        contained,
                        expression: Box::new(formatted_expression),
                    };
                }

                // Update the expression shape to be used inside the parentheses, applying the indent increase
                // Use the original `shape` rather than the LeftmostRangeHang-determined shape, because we are now
                // indenting the internal expression, which is not part of the hang
                let expression_shape = shape.reset().increment_additional_indent();

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
                        create_indent_trivia(ctx, shape),
                    ])),
                );

                Expression::Parentheses {
                    contained,
                    expression: Box::new(format_hanging_expression_(
                        ctx,
                        &expression,
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
            let expression =
                format_hanging_expression_(ctx, &expression, shape, expression_context, lhs_range);

            Expression::UnaryOperator {
                unop,
                expression: Box::new(expression),
            }
        }
        Expression::BinaryOperator { lhs, binop, rhs } => {
            // Don't format the lhs and rhs here, because it will be handled later when hang_binop_expression calls back for a Value
            let lhs =
                hang_binop_expression(ctx, *lhs.to_owned(), binop.to_owned(), shape, lhs_range);

            let mut new_binop = format_binop(ctx, binop, shape);
            let singleline_shape =
                shape.take_last_line(&lhs) + strip_trivia(binop).to_string().len() + 1;

            let mut new_rhs = hang_binop_expression(
                ctx,
                *rhs.to_owned(),
                binop.to_owned(),
                singleline_shape,
                None,
            );

            // Examine the last line to see if we need to hang this binop, or if the precedence levels match
            if binop_precedence_level(&lhs) >= binop.precedence()
                || contains_comments(binop)
                || get_expression_trailing_trivia(&lhs)
                    .iter()
                    .any(trivia_util::trivia_is_comment)
                || (shape.take_last_line(&lhs) + format!("{}{}", binop, rhs).len()).over_budget()
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
        other => panic!("unknown node {:?}", other),
    }
}

pub fn hang_expression<'ast>(
    ctx: &Context,
    expression: &Expression<'ast>,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression<'ast> {
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

pub fn hang_expression_trailing_newline<'ast>(
    ctx: &Context,
    expression: &Expression<'ast>,
    shape: Shape,
    hang_level: Option<usize>,
) -> Expression<'ast> {
    hang_expression(ctx, expression, shape, hang_level)
        .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]))
}
