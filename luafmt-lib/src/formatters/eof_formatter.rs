use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::borrow::Cow;

fn format_eof<'ast>(_eof: TokenReference<'ast>) -> TokenReference<'ast> {
    TokenReference::new(
        Vec::new(),
        Token::new(TokenType::Eof),
        vec![Token::new(TokenType::Whitespace {
            characters: Cow::Owned(String::from("\n")),
        })],
    )
}
