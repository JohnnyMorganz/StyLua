use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Block,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;

pub mod assignment_formatter;
pub mod block_formatter;
pub mod eof_formatter;
pub mod expression_formatter;
pub mod functions_formatter;
pub mod table_formatter;
pub mod trivia_formatter;

#[derive(Default)]
pub struct CodeFormatter {
    indent_level: usize,
}

pub fn create_indent_trivia<'ast>(indent_level: &usize) -> Token<'ast> {
    Token::new(TokenType::tabs(*indent_level - 1)) // indent_level starts at 1
}

pub fn create_newline_trivia<'ast>() -> Token<'ast> {
    Token::new(TokenType::Whitespace {
        characters: Cow::Owned(String::from("\n")), // TODO: Support CRLF line endings
    })
}

pub fn format_plain_token_reference<'a>(token_reference: TokenReference<'a>) -> TokenReference<'a> {
    TokenReference::new(Vec::new(), token_reference.token().to_owned(), Vec::new())
}

pub fn format_token_reference<'a>(
    token_reference: Cow<'a, TokenReference<'a>>,
) -> Cow<'a, TokenReference<'a>> {
    Cow::Owned(format_plain_token_reference(token_reference.into_owned()))
}

pub fn format_punctuation<'ast>(
    punctuation: Cow<'ast, TokenReference<'ast>>,
) -> Cow<'ast, TokenReference<'ast>> {
    Cow::Owned(TokenReference::new(
        Vec::new(),
        punctuation.token().to_owned(),
        vec![Token::new(TokenType::spaces(1))], // Single space whitespace
    ))
}

pub fn format_punctuated<'a, T>(
    old: Punctuated<'a, T>,
    value_formatter: &dyn Fn(T) -> T,
    // wanted_trailing_trivia: Vec<Token<'a>>,
) -> Punctuated<'a, T> {
    let mut formatted: Punctuated<T> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = format_punctuation(punctuation);
                let formatted_value = value_formatter(value);
                formatted.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = value_formatter(value);
                formatted.push(Pair::new(formatted_value, None));
            }
        }
    }

    formatted
}

impl<'ast> VisitorMut<'ast> for CodeFormatter {
    fn visit_block(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level += 1;
        block_formatter::format_block(node, &self.indent_level)
    }

    fn visit_block_end(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level -= 1;
        node
    }
}
