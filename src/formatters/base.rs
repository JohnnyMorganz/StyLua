use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
    },
    tokenizer::{StringLiteralQuoteType, TokenReference, TokenType},
};
use pretty::{docs, DocAllocator, DocBuilder};

use crate::context::Context;

use super::Formatter;

impl Formatter for TokenType {
    fn to_doc<'a, D, A>(&'a self, _ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            TokenType::Number { text } => {
                // TODO: number formatting
                docs![allocator, text.to_string()]
            }
            TokenType::Identifier { identifier } => allocator.text(identifier.to_string()),
            TokenType::Symbol { symbol } => allocator.text(symbol.to_string()),
            TokenType::Shebang { line } => allocator.text(line.to_string()),
            TokenType::StringLiteral {
                literal,
                multi_line,
                quote_type,
            } => {
                // TODO: string formatting
                let literal_doc = allocator.text(literal.to_string());
                match quote_type {
                    StringLiteralQuoteType::Single => docs![allocator, "'", literal_doc, "'"],
                    StringLiteralQuoteType::Double => docs![allocator, "\"", literal_doc, "\'"],
                    StringLiteralQuoteType::Brackets => docs![
                        allocator,
                        format!(
                            "[{}[",
                            multi_line.map_or(String::new(), |count| "=".repeat(count))
                        ),
                        literal_doc,
                        format!(
                            "]{}]",
                            multi_line.map_or(String::new(), |count| "=".repeat(count))
                        )
                    ],
                    other => unreachable!("unknown node: {:?}", other),
                }
            }
            TokenType::SingleLineComment { comment: _ } => todo!(),
            TokenType::MultiLineComment {
                blocks: _,
                comment: _,
            } => todo!(),
            TokenType::Whitespace { .. } => todo!(),
            TokenType::Eof => todo!(),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for TokenReference {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        // TODO: handle trivia
        self.token_type().to_doc(ctx, allocator)
    }
}

pub fn contained_span<'a, D, A, T: Formatter>(
    ctx: &Context,
    allocator: &'a D,
    contained: &'a ContainedSpan,
    item: &'a T,
) -> DocBuilder<'a, D, A>
where
    D: DocAllocator<'a, A>,
    D::Doc: Clone,
    A: Clone,
{
    let (begin, end) = contained.tokens();
    let doc = item.to_doc(ctx, allocator);

    docs![
        allocator,
        begin.to_doc(ctx, allocator),
        allocator.line_(),
        doc,
        allocator.line_(),
        end.to_doc(ctx, allocator)
    ]
    .group()
}

impl<T: Formatter> Formatter for Pair<T> {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Pair::Punctuated(node, punctuation) => docs![
                allocator,
                node.to_doc(ctx, allocator),
                punctuation.to_doc(ctx, allocator)
            ],
            Pair::End(node) => node.to_doc(ctx, allocator),
        }
    }
}

impl<T: Formatter> Formatter for Punctuated<T> {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator.intersperse(
            self.pairs().into_iter().map(|x| x.to_doc(ctx, allocator)),
            allocator.line(),
        )
    }
}
