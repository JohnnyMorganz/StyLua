use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Field, TableConstructor,
};
use pretty::{docs, DocAllocator, DocBuilder, RcDoc};

use crate::context::Context;

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
                allocator.line(),
                value.to_doc(ctx, allocator),
            ]
            .group(),
            Field::NameKey { key, equal, value } => docs![
                allocator,
                key.to_doc(ctx, allocator),
                allocator.space(),
                equal.to_doc(ctx, allocator),
                allocator.line(),
                value.to_doc(ctx, allocator),
            ]
            .group(),
            Field::NoKey(expression) => expression.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

// fn fields<'a, D, A>(
//     ctx: &Context,
//     allocator: &'a D,
//     fields: Punctuated<&'a Field>,
// ) -> DocBuilder<'a, D, A>
// where
//     D: DocAllocator<'a, A>,
//     D::Doc: Clone,
//     A: Clone,
// {
//     let mut doc = allocator.nil();
//     let fields = fields.pairs();
//     let mut peekable_fields = fields.peekable();

//     while let Some(field) = peekable_fields.next() {
//         let d = match field {
//             Pair::Punctuated(node, punctuation) => {
//                 let comma = punctuation.to_doc(ctx, allocator);
//                 docs![
//                     allocator,
//                     node.to_doc(ctx, allocator),
//                     if peekable_fields.peek().is_some() {
//                         comma
//                     } else {
//                         comma.flat_alt(allocator.nil())
//                     }
//                 ]
//             }
//             Pair::End(node) => node
//                 .to_doc(ctx, allocator)
//                 .append(allocator.text(",").flat_alt(allocator.nil())),
//         };

//         doc = doc.append(d);
//     }

//     doc
// }

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
            left_brace
                .to_doc(ctx, allocator)
                .append(allocator.line())
                .append(self.fields().to_doc(ctx, allocator))
                .nest(4)
                .append(allocator.line())
                .append(right_brace.to_doc(ctx, allocator))
                .group()
        }
    }
}

#[test]
fn test() {
    let fields = [
        "laaaaaaaaaaaarge",
        "laaaaaaaaaaaarge",
        "laaaaaaaaaaaarge",
        "laaaaaaaaaaaarge",
        "laaaaaaaaaaaarge",
    ];
    println!(
        "{}",
        RcDoc::<()>::text("{")
            .append(RcDoc::line())
            .append(RcDoc::intersperse(
                fields,
                RcDoc::text(",").append(RcDoc::line())
            ))
            .nest(4)
            .append(RcDoc::line())
            .append(RcDoc::text("}"))
            .group()
            .pretty(200)
    )
}
