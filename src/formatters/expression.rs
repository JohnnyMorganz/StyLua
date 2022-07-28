use full_moon::ast::{BinOp, Expression, UnOp, Value, Var};
use pretty::{DocAllocator, DocBuilder};

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
            Var::Expression(_) => todo!(),
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
        todo!()
    }
}

impl Formatter for BinOp {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
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
            Expression::UnaryOperator { unop, expression } => todo!(),
            Expression::BinaryOperator { lhs, binop, rhs } => todo!(),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}
