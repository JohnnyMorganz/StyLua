use full_moon::ast::{Field, TableConstructor};
use pretty::docs;

use super::Formatter;

impl Formatter for Field {
    fn to_doc<'a, D, A>(
        &'a self,
        ctx: &crate::context::Context,
        allocator: &'a D,
    ) -> pretty::DocBuilder<'a, D, A>
    where
        D: pretty::DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            } => docs![
                allocator,
                brackets.tokens().0.to_doc(ctx, allocator),
                key.to_doc(ctx, allocator),
                brackets.tokens().1.to_doc(ctx, allocator),
                allocator.space(),
                equal.to_doc(ctx, allocator),
                allocator.softline(),
                value.to_doc(ctx, allocator),
            ]
            .group(),
            Field::NameKey { key, equal, value } => docs![
                allocator,
                key.to_doc(ctx, allocator),
                allocator.space(),
                equal.to_doc(ctx, allocator),
                allocator.softline(),
                value.to_doc(ctx, allocator),
            ]
            .group(),
            Field::NoKey(expression) => expression.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for TableConstructor {
    fn to_doc<'a, D, A>(
        &'a self,
        ctx: &crate::context::Context,
        allocator: &'a D,
    ) -> pretty::DocBuilder<'a, D, A>
    where
        D: pretty::DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let (left_brace, right_brace) = self.braces().tokens();
        if self.fields().is_empty() {
            docs![
                allocator,
                left_brace.to_doc(ctx, allocator),
                right_brace.to_doc(ctx, allocator)
            ]
        } else {
            docs![
                allocator,
                left_brace.to_doc(ctx, allocator),
                allocator.line(),
                self.fields().to_doc(ctx, allocator),
                allocator.line(),
                right_brace.to_doc(ctx, allocator),
            ]
            .group()
        }
    }
}
