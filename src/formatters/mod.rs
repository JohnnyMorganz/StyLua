use full_moon::ast::{Ast, Block};

use pretty::{DocAllocator, DocBuilder};

use crate::context::Context;

mod base;
mod expression;
mod stmt;

pub trait Formatter {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone;
}

impl Formatter for Block {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let mut doc = allocator.nil();

        for (stmt, semicolon) in self.stmts_with_semicolon() {
            doc = doc.append(stmt.to_doc(ctx, allocator));

            if semicolon.is_some() {
                todo!()
            }
        }

        if self.last_stmt().is_some() {
            todo!()
        }

        doc
    }
}

impl Formatter for Ast {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let block = self.nodes().to_doc(ctx, allocator);

        // TODO: handle eof comments
        let eof = allocator.hardline();

        block.append(eof)
    }
}
