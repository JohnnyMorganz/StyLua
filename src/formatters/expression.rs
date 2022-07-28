use full_moon::ast::{
    BinOp, Call, Expression, Index, Prefix, Suffix, UnOp, Value, Var, VarExpression,
};
use pretty::{docs, DocAllocator, DocBuilder};

use crate::context::Context;

use super::{base::contained_span, functions::anonymous_function, Formatter};

impl Formatter for Var {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Var::Name(token) => token.to_doc(ctx, allocator),
            Var::Expression(var_expression) => var_expression.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for VarExpression {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        docs![
            allocator,
            self.prefix().to_doc(ctx, allocator),
            allocator.intersperse(
                self.suffixes().map(|suffix| suffix.to_doc(ctx, allocator)),
                allocator.line_()
            ),
        ]
    }
}

impl Formatter for Prefix {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Prefix::Name(token) => token.to_doc(ctx, allocator),
            Prefix::Expression(expression) => expression.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for Suffix {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Suffix::Call(call) => call.to_doc(ctx, allocator),
            Suffix::Index(index) => index.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for Index {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Index::Brackets {
                brackets,
                expression,
            } => contained_span(ctx, allocator, brackets, expression),
            Index::Dot { dot, name } => docs![
                allocator,
                dot.to_doc(ctx, allocator),
                name.to_doc(ctx, allocator)
            ],
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for Value {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Value::Number(token) => token.to_doc(ctx, allocator),
            Value::Function(function) => anonymous_function(ctx, allocator, function),
            Value::FunctionCall(function_call) => function_call.to_doc(ctx, allocator),
            Value::TableConstructor(table_constructor) => table_constructor.to_doc(ctx, allocator),
            Value::ParenthesesExpression(expression) => expression.to_doc(ctx, allocator),
            Value::String(token) => token.to_doc(ctx, allocator),
            Value::Symbol(token) => token.to_doc(ctx, allocator),
            Value::Var(var) => var.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for UnOp {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            UnOp::Hash(token) => token.to_doc(ctx, allocator),
            UnOp::Minus(token) | UnOp::Not(token) => {
                docs![allocator, token.to_doc(ctx, allocator), allocator.space()]
            }
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for BinOp {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            BinOp::And(t)
            | BinOp::Caret(t)
            | BinOp::GreaterThan(t)
            | BinOp::GreaterThanEqual(t)
            | BinOp::LessThan(t)
            | BinOp::LessThanEqual(t)
            | BinOp::Minus(t)
            | BinOp::Or(t)
            | BinOp::Percent(t)
            | BinOp::Plus(t)
            | BinOp::Slash(t)
            | BinOp::Star(t)
            | BinOp::TildeEqual(t)
            | BinOp::TwoDots(t)
            | BinOp::TwoEqual(t) => t.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for Expression {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Expression::Value { value } => value.to_doc(ctx, allocator),
            Expression::Parentheses {
                contained,
                expression,
            } => contained_span(ctx, allocator, contained, &**expression),
            Expression::UnaryOperator { unop, expression } => docs![
                allocator,
                unop.to_doc(ctx, allocator),
                expression.to_doc(ctx, allocator)
            ],
            Expression::BinaryOperator { lhs, binop, rhs } => docs![
                allocator,
                lhs.to_doc(ctx, allocator),
                allocator.line(),
                binop.to_doc(ctx, allocator),
                allocator.space(),
                rhs.to_doc(ctx, allocator),
            ],
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}
