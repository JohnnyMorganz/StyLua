use full_moon::ast::{Expression, Value, Var};
use pretty::{DocAllocator, DocBuilder};

use crate::context::Context;

use super::{base::contained_span, Formatter};

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

impl Formatter for Expression {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Expression::Value { value } => match &**value {
                Value::Number(token) => token.to_doc(ctx, allocator),
                Value::Function(_) => todo!(),
                Value::FunctionCall(_) => todo!(),
                Value::TableConstructor(_) => todo!(),
                Value::ParenthesesExpression(expression) => expression.to_doc(ctx, allocator),
                Value::String(token) => token.to_doc(ctx, allocator),
                Value::Symbol(token) => token.to_doc(ctx, allocator),
                Value::Var(var) => var.to_doc(ctx, allocator),
                other => unreachable!("unknown node: {:?}", other),
            },
            Expression::BinaryOperator { lhs, binop, rhs } => todo!(),
            Expression::Parentheses {
                contained,
                expression,
            } => contained_span(ctx, allocator, contained, &**expression),
            Expression::UnaryOperator { unop, expression } => todo!(),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}
