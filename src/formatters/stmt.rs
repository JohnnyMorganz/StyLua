use full_moon::{
    ast::{Assignment, Stmt},
    tokenizer::TokenReference,
};
use pretty::{docs, DocAllocator, DocBuilder};

use crate::context::Context;

use super::Formatter;

fn equals_token<'a, D, A>(
    ctx: &Context,
    allocator: &'a D,
    token: &'a TokenReference,
) -> DocBuilder<'a, D, A>
where
    D: DocAllocator<'a, A>,
    D::Doc: Clone,
    A: Clone,
{
    docs![
        allocator,
        allocator.space(),
        token.to_doc(ctx, allocator),
        allocator.line()
    ]
}

impl Formatter for Assignment {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        docs![
            allocator,
            self.variables().to_doc(ctx, allocator).group(),
            equals_token(ctx, allocator, self.equal_token()),
            self.expressions().to_doc(ctx, allocator).group(),
        ]
        .group()
    }
}

impl Formatter for Stmt {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Stmt::Assignment(assignment) => assignment.to_doc(ctx, allocator),
            Stmt::Do(_) => todo!(),
            Stmt::FunctionCall(_) => todo!(),
            Stmt::FunctionDeclaration(_) => todo!(),
            Stmt::GenericFor(_) => todo!(),
            Stmt::If(_) => todo!(),
            Stmt::LocalAssignment(_) => todo!(),
            Stmt::LocalFunction(_) => todo!(),
            Stmt::NumericFor(_) => todo!(),
            Stmt::Repeat(_) => todo!(),
            Stmt::While(_) => todo!(),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}
