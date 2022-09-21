use crate::{
    context::Context,
    formatters::{
        general::{format_contained_span, format_token_reference},
        trivia::{FormatTriviaType, UpdateLeadingTrivia},
    },
    shape::Shape,
};
use full_moon::{
    ast::lua54::Attribute,
    tokenizer::{Token, TokenType},
};

pub fn format_attribute(ctx: &Context, attribute: &Attribute, shape: Shape) -> Attribute {
    let brackets = format_contained_span(ctx, attribute.brackets(), shape).update_leading_trivia(
        FormatTriviaType::Append(vec![Token::new(TokenType::spaces(1))]),
    );
    let name = format_token_reference(ctx, attribute.name(), shape);

    Attribute::new(name).with_brackets(brackets)
}
