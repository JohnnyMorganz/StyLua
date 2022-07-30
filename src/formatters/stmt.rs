use full_moon::{
    ast::{
        Assignment, Do, ElseIf, GenericFor, If, LastStmt, LocalAssignment, NumericFor, Repeat,
        Return, Stmt, While,
    },
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
        allocator.space()
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

impl Formatter for LocalAssignment {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if let Some(equal_token) = self.equal_token() {
            docs![
                allocator,
                self.local_token().to_doc(ctx, allocator),
                " ",
                self.names().to_doc(ctx, allocator).group(),
                equals_token(ctx, allocator, equal_token),
                self.expressions().to_doc(ctx, allocator).group(),
            ]
            .group()
        } else {
            docs![
                allocator,
                self.local_token().to_doc(ctx, allocator),
                " ",
                self.names().to_doc(ctx, allocator).group()
            ]
        }
    }
}

impl Formatter for Do {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.do_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.end_token().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for GenericFor {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.for_token().to_doc(ctx, allocator),
            docs![
                allocator,
                allocator.line(),
                self.names().to_doc(ctx, allocator),
                allocator.line()
            ]
            .group(),
            self.in_token().to_doc(ctx, allocator),
            docs![
                allocator,
                allocator.line(),
                self.expressions().to_doc(ctx, allocator),
                allocator.line()
            ],
            self.do_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.end_token().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for NumericFor {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.for_token().to_doc(ctx, allocator),
            self.index_variable().to_doc(ctx, allocator),
            allocator.space(),
            self.equal_token().to_doc(ctx, allocator),
            allocator.space(),
            self.start().to_doc(ctx, allocator),
            self.start_end_comma().to_doc(ctx, allocator),
            self.end().to_doc(ctx, allocator),
            self.end_step_comma()
                .map_or_else(|| allocator.nil(), |token| token.to_doc(ctx, allocator)),
            self.step()
                .map_or_else(|| allocator.nil(), |expr| expr.to_doc(ctx, allocator)),
            self.do_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.end_token().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for Repeat {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.repeat_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.until_token().to_doc(ctx, allocator),
            allocator.space(),
            self.until().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for While {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.while_token().to_doc(ctx, allocator),
            docs![
                allocator,
                allocator.line(),
                self.condition().to_doc(ctx, allocator),
                allocator.line()
            ]
            .group(),
            self.do_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.end_token().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for If {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        let else_body = self.else_block().map_or_else(
            || allocator.nil(),
            |block| {
                allocator
                    .hardline()
                    .append(block.to_doc(ctx, allocator))
                    .nest(ctx.config().indent_width_signed())
                    .append(allocator.hardline())
            },
        );

        docs![
            allocator,
            self.if_token().to_doc(ctx, allocator),
            docs![
                allocator,
                allocator.line(),
                self.condition().to_doc(ctx, allocator),
                allocator.line()
            ]
            .group(),
            self.then_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
            self.else_if().map_or_else(
                || allocator.nil(),
                |else_ifs| allocator.concat(
                    else_ifs
                        .iter()
                        .map(|else_if| else_if.to_doc(ctx, allocator))
                )
            ),
            self.else_token()
                .map_or_else(|| allocator.nil(), |token| token.to_doc(ctx, allocator)),
            else_body,
            self.end_token().to_doc(ctx, allocator),
        ]
    }
}

impl Formatter for ElseIf {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let body = allocator
            .hardline()
            .append(self.block().to_doc(ctx, allocator))
            .nest(ctx.config().indent_width_signed());

        docs![
            allocator,
            self.else_if_token().to_doc(ctx, allocator),
            docs![
                allocator,
                allocator.line(),
                self.condition().to_doc(ctx, allocator),
                allocator.line()
            ]
            .group(),
            self.then_token().to_doc(ctx, allocator),
            body,
            allocator.hardline(),
        ]
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
            Stmt::Do(r#do) => r#do.to_doc(ctx, allocator),
            Stmt::FunctionCall(function_call) => function_call.to_doc(ctx, allocator),
            Stmt::FunctionDeclaration(function_declaration) => {
                function_declaration.to_doc(ctx, allocator)
            }
            Stmt::GenericFor(generic_for) => generic_for.to_doc(ctx, allocator),
            Stmt::If(r#if) => r#if.to_doc(ctx, allocator),
            Stmt::LocalAssignment(assignment) => assignment.to_doc(ctx, allocator),
            Stmt::LocalFunction(local_function) => local_function.to_doc(ctx, allocator),
            Stmt::NumericFor(numeric_for) => numeric_for.to_doc(ctx, allocator),
            Stmt::Repeat(repeat) => repeat.to_doc(ctx, allocator),
            Stmt::While(r#while) => r#while.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for Return {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if self.returns().is_empty() {
            self.token().to_doc(ctx, allocator)
        } else {
            docs![
                allocator,
                self.token().to_doc(ctx, allocator),
                allocator.space(),
                self.returns().to_doc(ctx, allocator).nest(4).group(),
            ]
        }
    }
}

impl Formatter for LastStmt {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            LastStmt::Break(r#break) => r#break.to_doc(ctx, allocator),
            LastStmt::Return(r#return) => r#return.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}
