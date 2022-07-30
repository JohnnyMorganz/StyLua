use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
    },
    tokenizer::{StringLiteralQuoteType, TokenReference, TokenType},
};
use pretty::{docs, DocAllocator, DocBuilder};

use crate::{context::Context, QuoteStyle};

use super::Formatter;

fn infer_quotes_from_string(ctx: &Context, literal: &str) -> StringLiteralQuoteType {
    match ctx.config().quote_style {
        QuoteStyle::ForceDouble => StringLiteralQuoteType::Double,
        QuoteStyle::ForceSingle => StringLiteralQuoteType::Single,
        _ => {
            let preferred = match ctx.config().quote_style {
                QuoteStyle::AutoPreferDouble => StringLiteralQuoteType::Double,
                QuoteStyle::AutoPreferSingle => StringLiteralQuoteType::Single,
                _ => unreachable!("have other quote styles we haven't looked into yet"),
            };

            // Check to see if there is a quote within it
            if literal.contains('\'') || literal.contains('"') {
                let num_single_quotes = literal.matches('\'').count();
                let num_double_quotes = literal.matches('"').count();

                match num_single_quotes.cmp(&num_double_quotes) {
                    std::cmp::Ordering::Equal => preferred,
                    std::cmp::Ordering::Greater => StringLiteralQuoteType::Double,
                    std::cmp::Ordering::Less => StringLiteralQuoteType::Single,
                }
            } else {
                preferred
            }
        }
    }
}

fn normalise_string_escapes(literal: &str, quote_type: StringLiteralQuoteType) -> String {
    // Match all escapes within the string
    // Based off https://github.com/prettier/prettier/blob/181a325c1c07f1a4f3738665b7b28288dfb960bc/src/common/util.js#L439
    lazy_static::lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r#"\\?(["'])|\\([\S\s])"#).unwrap();
        static ref UNNECESSARY_ESCAPES: regex::Regex = regex::Regex::new(r#"^[^\n\r"'0-9\\abfnrtuvxz]$"#).unwrap();
    }

    RE.replace_all(literal, |caps: &regex::Captures| {
        let quote = caps.get(1);
        let escaped = caps.get(2);

        match quote {
            Some(quote) => {
                // Given a quote, determine if it needs to be escaped
                match quote.as_str() {
                    "'" => match quote_type {
                        StringLiteralQuoteType::Single => "\\'".to_string(),
                        _ => "'".to_string(),
                    },
                    "\"" => match quote_type {
                        StringLiteralQuoteType::Double => "\\\"".to_string(),
                        _ => "\"".to_string(),
                    },
                    other => unreachable!("unknown quote type {:?}", other),
                }
            }
            None => {
                // We have a normal escape
                // Test to see if it is necessary, and if not, then unescape it
                let text = escaped
                    .expect("have a match which was neither an escape or a quote")
                    .as_str();
                if UNNECESSARY_ESCAPES.is_match(text) {
                    text.to_owned()
                } else {
                    format!("\\{}", text.to_owned())
                }
            }
        }
    })
    .into()
}

impl Formatter for TokenType {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            TokenType::Number { text } => {
                let text = if text.starts_with('.') {
                    String::from("0") + text.as_str()
                } else if text.starts_with("-.") {
                    String::from("-0") + text.get(1..).expect("unknown number literal")
                } else {
                    text.to_string()
                };

                allocator.text(text)
            }
            TokenType::Identifier { identifier } => allocator.text(identifier.to_string()),
            TokenType::Symbol { symbol } => allocator.text(symbol.to_string()),
            TokenType::Shebang { line } => allocator.text(line.to_string()),
            TokenType::StringLiteral {
                literal,
                multi_line,
                quote_type,
            } => {
                // Normalise strings, don't touch bracket strings
                let quotes = match quote_type {
                    StringLiteralQuoteType::Brackets => StringLiteralQuoteType::Brackets,
                    _ => infer_quotes_from_string(ctx, literal),
                };

                let literal = match quotes {
                    StringLiteralQuoteType::Brackets => literal.to_string(),
                    _ => normalise_string_escapes(literal, quotes),
                };

                let literal_doc = allocator.text(literal);

                match quotes {
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

    begin
        .to_doc(ctx, allocator)
        .append(
            allocator
                .line_()
                .append(item.to_doc(ctx, allocator))
                .nest(ctx.config().indent_width_signed()),
        )
        .append(allocator.line_())
        .append(end.to_doc(ctx, allocator))
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
