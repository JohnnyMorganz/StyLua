use crate::formatters::{
    expression_formatter::format_expression, format_contained_span, format_symbol,
    format_token_reference,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Field, TableConstructor,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

pub fn format_field<'ast>(field: &Field<'ast>) -> Field<'ast> {
    match field {
        Field::ExpressionKey {
            brackets,
            key,
            equal,
            value,
        } => Field::ExpressionKey {
            brackets: format_contained_span(brackets.to_owned()),
            key: format_expression(key.to_owned()),
            equal: format_symbol(
                equal.to_owned().into_owned(),
                TokenReference::symbol(" = ").unwrap(),
            ),
            value: format_expression(value.to_owned()),
        },
        Field::NameKey { key, equal, value } => Field::NameKey {
            key: format_token_reference(key.to_owned()),
            equal: format_symbol(
                equal.to_owned().into_owned(),
                TokenReference::symbol(" = ").unwrap(),
            ),
            value: format_expression(value.to_owned()),
        },
        Field::NoKey(expression) => Field::NoKey(format_expression(expression.to_owned())),
    }
}

pub fn format_table_constructor<'ast>(
    table_constructor: TableConstructor<'ast>,
) -> TableConstructor<'ast> {
    let mut fields = Punctuated::new();
    let mut current_fields = table_constructor.iter_fields().peekable();

    let (start_brace, end_brace) = table_constructor.braces().tokens();
    let is_multiline =
        (end_brace.start_position().bytes() - start_brace.end_position().bytes()) > 30;

    let braces = match current_fields.peek() {
        Some(_) => ContainedSpan::new(
            format_symbol(
                start_brace.to_owned(),
                TokenReference::symbol(if is_multiline { "{\n" } else { "{ " }).unwrap(),
            ), // TODO: use proper newline character
            format_symbol(
                end_brace.to_owned(),
                TokenReference::symbol(if is_multiline { "\n}" } else { " }" }).unwrap(),
            ),
        ),
        None => ContainedSpan::new(
            format_symbol(start_brace.to_owned(), TokenReference::symbol("{").unwrap()),
            format_symbol(end_brace.to_owned(), TokenReference::symbol("}").unwrap()),
        ),
    };

    // TODO: Should we sort NameKey/ExpressionKey tables?
    loop {
        match current_fields.next() {
            Some(field) => {
                let formatted_field = format_field(field);
                let mut punctuation = None;

                if let Some(_) = current_fields.peek() {
                    // Have more elements still to go
                    punctuation = Some(match is_multiline {
                        true => Cow::Owned(TokenReference::symbol(",\n").unwrap()), // TODO: use proper newline character
                        false => Cow::Owned(TokenReference::symbol(", ").unwrap()),
                    })
                }
                fields.push(Pair::new(formatted_field, punctuation))
            }
            None => break,
        }
    }

    TableConstructor::new()
        .with_braces(braces)
        .with_fields(fields)
}
