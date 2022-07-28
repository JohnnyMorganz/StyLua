use full_moon::ast::{Ast, Block, Expression, Prefix, Stmt, Var};

use pretty::{DocAllocator, DocBuilder};

use crate::context::Context;

mod base;
mod expression;
mod functions;
mod stmt;
mod table;

pub trait Formatter {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone;
}

fn requires_semicolon(current_stmt: &Stmt, next_stmt: &Stmt) -> bool {
    matches!(
        current_stmt,
        Stmt::Assignment(_) | Stmt::LocalAssignment(_) | Stmt::FunctionCall(_) | Stmt::Repeat(_)
    ) && match next_stmt {
        Stmt::FunctionCall(call)
            if matches!(
                call.prefix(),
                Prefix::Expression(Expression::Parentheses { .. })
            ) =>
        {
            true
        }
        Stmt::Assignment(assignment) => match assignment.variables().iter().next() {
            Some(Var::Expression(var_expr))
                if matches!(
                    var_expr.prefix(),
                    Prefix::Expression(Expression::Parentheses { .. })
                ) =>
            {
                true
            }
            _ => false,
        },
        _ => false,
    }
}

impl Formatter for Block {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let mut doc = allocator.nil();

        let mut stmt_iterator = self.stmts_with_semicolon().peekable();

        while let (Some(current_stmt), next_stmt) = (stmt_iterator.next(), stmt_iterator.peek()) {
            let (stmt, semicolon) = current_stmt;
            doc = doc.append(stmt.to_doc(ctx, allocator));

            if let Some(semicolon) = semicolon {
                if let Some(next_stmt) = next_stmt && requires_semicolon(stmt, &next_stmt.0) {
                    doc = doc.append(semicolon.to_doc(ctx, allocator))
                } else {
                    todo!("keep trivia on semicolon");
                }
            }

            doc = doc.append(allocator.hardline());
        }

        if let Some((last_stmt, semicolon)) = self.last_stmt_with_semicolon() {
            doc = doc.append(last_stmt.to_doc(ctx, allocator));

            if let Some(_semicolon) = semicolon {
                todo!("keep trivia on semicolon")
            }

            doc = doc.append(allocator.hardline());
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
        let eof = allocator.nil();

        block.append(eof)
    }
}
