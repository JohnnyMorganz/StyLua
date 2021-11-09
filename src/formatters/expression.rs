#[cfg(feature = "luau")]
use full_moon::ast::types::IfExpression;
use full_moon::{
    ast::{
        span::ContainedSpan, BinOp, Call, Expression, Index, Prefix, Suffix, UnOp, Value, Var,
        VarExpression,
    },
    node::Node,
    tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
};
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::luau::format_type_assertion;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        functions::{
            format_anonymous_function, format_call, format_function_call, FunctionCallNextNode,
        },
        general::{format_contained_span, format_token_reference},
        table::format_table_constructor,
        trivia::{
            strip_leading_trivia, strip_trivia, FormatTriviaType, UpdateLeadingTrivia,
            UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{
            self, contains_comments, expression_leading_comments, get_expression_trailing_trivia,
            trivia_is_newline,
        },
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
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => {
            // If we have a type assertion, we should always keep parentheses
            #[cfg(feature = "luau")]
            if type_assertion.is_some() {
                return false;
            }

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
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => Expression::Value {
            value: Box::new(format_value(ctx, value, shape)),
            #[cfg(feature = "luau")]
            type_assertion: type_assertion
                .as_ref()
                .map(|assertion| format_type_assertion(ctx, assertion, shape)),
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
                // Get the trailing comments from contained span and append them onto the expression
                let trailing_comments = contained
                    .tokens()
                    .1
                    .trailing_trivia()
                    .filter(|token| trivia_util::trivia_is_comment(token))
                    .flat_map(|x| {
                        // Prepend a single space beforehand
                        vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                    })
                    .collect();
                format_expression(ctx, expression, shape)
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
            let mut expression = format_expression(ctx, expression, shape);

            // Special case: if we have `- -foo`, or `-(-foo)` where we have already removed the parentheses, then
            // it will lead to `--foo`, which is invalid syntax. We must explicitly add/keep the parentheses `-(-foo)`.
            if let UnOp::Minus(_) = unop {
                let require_parentheses = match expression {
                    Expression::UnaryOperator {
                        unop: UnOp::Minus(_),
                        ..
                    } => true,

                    Expression::Value { ref value, .. } => matches!(
                        **value,
                        Value::ParenthesesExpression(Expression::UnaryOperator {
                            unop: UnOp::Minus(_),
                            ..
                        })
                    ),

                    _ => false,
                };

                if require_parentheses {
                    let (new_expression, trailing_comments) =
                        trivia_util::take_expression_trailing_comments(&expression);
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

/// Determines whether the provided [`Expression`] is a brackets string, i.e. `[[string]]`
pub fn is_brackets_string(expression: &Expression) -> bool {
    if let Expression::Value { value, .. } = expression {
        if let Value::String(token_reference) = &**value {
            return matches!(
                token_reference.token_type(),
                TokenType::StringLiteral {
                    quote_type: StringLiteralQuoteType::Brackets,
                    ..
                }
            );
        }
    }
    false
}

/// Formats an Index Node
pub fn format_index(ctx: &Context, index: &Index, shape: Shape) -> Index {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => {
            if is_brackets_string(expression) {
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

        Index::Dot { dot, name } => Index::Dot {
            dot: format_token_reference(ctx, dot, shape),
            name: format_token_reference(ctx, name, shape),
        },
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a Prefix Node
pub fn format_prefix(ctx: &Context, prefix: &Prefix, shape: Shape) -> Prefix {
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

/// Formats an [`IfExpression`] node
#[cfg(feature = "luau")]
fn format_if_expression(
    _ctx: &Context,
    if_expression: &IfExpression,
    _shape: Shape,
) -> IfExpression {
    // TODO: Apply actual formatting here
    if_expression.to_owned()
}

/// Formats a Value Node
pub fn format_value(ctx: &Context, value: &Value, shape: Shape) -> Value {
    match value {
        Value::Function((token_reference, function_body)) => Value::Function(
            format_anonymous_function(ctx, token_reference, function_body, shape),
        ),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(format_function_call(ctx, function_call, shape))
        }
        #[cfg(feature = "luau")]
        Value::IfExpression(if_expression) => {
            Value::IfExpression(format_if_expression(ctx, if_expression, shape))
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
pub fn format_var(ctx: &Context, var: &Var, shape: Shape) -> Var {
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

pub fn format_var_expression(
    ctx: &Context,
    var_expression: &VarExpression,
    shape: Shape,
) -> VarExpression {
    let formatted_prefix = format_prefix(ctx, var_expression.prefix(), shape);
    let mut shape = shape + strip_leading_trivia(&formatted_prefix).to_string().len();

    let mut formatted_suffixes = Vec::new();
    let mut suffixes = var_expression.suffixes().peekable();

    while let Some(suffix) = suffixes.next() {
        // If the suffix after this one is something like `.foo` or `:foo` - this affects removing parentheses
        let ambiguous_next_suffix = if matches!(
            suffixes.peek(),
            Some(Suffix::Index(_)) | Some(Suffix::Call(Call::MethodCall(_)))
        ) {
            FunctionCallNextNode::ObscureWithoutParens
        } else {
            FunctionCallNextNode::None
        };

        let suffix = format_suffix(ctx, suffix, shape, ambiguous_next_suffix);
        shape = shape + suffix.to_string().len();
        formatted_suffixes.push(suffix);
    }

    VarExpression::new(formatted_prefix).with_suffixes(formatted_suffixes)
}

/// Formats an UnOp Node
pub fn format_unop(ctx: &Context, unop: &UnOp, shape: Shape) -> UnOp {
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
fn hang_binop(ctx: &Context, binop: BinOp, shape: Shape, rhs: &Expression) -> BinOp {
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
                    || !expression_leading_comments(rhs).is_empty()
                    || get_expression_trailing_trivia(lhs)
                        .iter()
                        .any(trivia_util::trivia_is_comment)
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
                false => {
                    // Check if the chain still has comments deeper inside of it.
                    // If it does, we need to hang that part of the chain still, otherwise the comments will mess it up
                    let lhs = if contains_comments(&*lhs) {
                        hang_binop_expression(ctx, *lhs, binop.to_owned(), shape, lhs_range)
                    } else {
                        format_expression(ctx, &*lhs, shape)
                    };

                    let rhs = if contains_comments(&*rhs) {
                        hang_binop_expression(ctx, *rhs, binop, shape, lhs_range)
                    } else {
                        format_expression(ctx, &*rhs, shape)
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
                type_assertion: type_assertion
                    .as_ref()
                    .map(|assertion| format_type_assertion(ctx, assertion, shape)),
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
                let contained = format_contained_span(ctx, contained, lhs_shape);

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
            let expression =
                format_hanging_expression_(ctx, expression, shape, expression_context, lhs_range);

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
