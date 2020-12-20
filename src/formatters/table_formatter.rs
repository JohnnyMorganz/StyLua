use crate::formatters::{
    block_formatter::get_range_in_expression, block_formatter::get_token_range,
    expression_formatter::format_expression, get_line_ending_character, trivia_formatter,
    CodeFormatter,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Field, TableConstructor,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::borrow::Cow;

pub fn format_field<'ast>(
    code_formatter: &mut CodeFormatter,
    field: &Field<'ast>,
    leading_trivia: Option<Vec<Token<'ast>>>,
) -> Field<'ast> {
    match field {
        Field::ExpressionKey {
            brackets,
            key,
            equal,
            value,
        } => Field::ExpressionKey {
            brackets: trivia_formatter::contained_span_add_trivia(
                code_formatter.format_contained_span(brackets.to_owned()),
                leading_trivia,
                None,
            ),
            key: format_expression(code_formatter, key.to_owned()),
            equal: code_formatter.format_symbol(
                equal.to_owned().into_owned(),
                TokenReference::symbol(" = ").unwrap(),
            ),
            value: format_expression(code_formatter, value.to_owned()),
        },
        Field::NameKey { key, equal, value } => Field::NameKey {
            key: Cow::Owned(trivia_formatter::token_reference_add_trivia(
                code_formatter
                    .format_token_reference(key.to_owned())
                    .into_owned(),
                leading_trivia,
                None,
            )),
            equal: code_formatter.format_symbol(
                equal.to_owned().into_owned(),
                TokenReference::symbol(" = ").unwrap(),
            ),
            value: format_expression(code_formatter, value.to_owned()),
        },
        Field::NoKey(expression) => {
            let formatted_expression = format_expression(code_formatter, expression.to_owned());
            match leading_trivia {
                Some(trivia) => Field::NoKey(trivia_formatter::expression_add_leading_trivia(
                    formatted_expression,
                    trivia,
                )),
                None => Field::NoKey(formatted_expression),
            }
        }
    }
}

pub fn format_table_constructor<'ast>(
    code_formatter: &mut CodeFormatter,
    table_constructor: TableConstructor<'ast>,
) -> TableConstructor<'ast> {
    let mut fields = Punctuated::new();
    let mut current_fields = table_constructor.iter_fields().peekable();

    let (start_brace, end_brace) = table_constructor.braces().tokens();
    let braces_range = (
        start_brace.end_position().bytes(),
        end_brace.start_position().bytes(),
    );
    let is_multiline = (braces_range.1 - braces_range.0) > 30; // TODO: Properly determine this arbitrary number, and see if other factors should come into play

    let braces = match current_fields.peek() {
        Some(_) => match is_multiline {
            true => {
                // Format start and end brace properly with correct trivia
                let additional_indent_level =
                    code_formatter.get_range_indent_increase(braces_range);
                let end_brace_leading_trivia =
                    vec![code_formatter.create_indent_trivia(additional_indent_level)];

                // Add new_line trivia to start_brace
                let start_brace_token = TokenReference::symbol(
                    &(String::from("{")
                        + &get_line_ending_character(&code_formatter.config.line_endings)),
                )
                .unwrap();
                let end_brace_token = TokenReference::new(
                    end_brace_leading_trivia,
                    Token::new(TokenType::Symbol {
                        symbol: Symbol::RightBrace,
                    }),
                    vec![],
                );
                ContainedSpan::new(
                    code_formatter.format_symbol(start_brace.to_owned(), start_brace_token),
                    code_formatter.format_symbol(end_brace.to_owned(), end_brace_token),
                )
            }
            false => ContainedSpan::new(
                code_formatter.format_symbol(
                    start_brace.to_owned(),
                    TokenReference::symbol("{ ").unwrap(),
                ),
                code_formatter
                    .format_symbol(end_brace.to_owned(), TokenReference::symbol(" }").unwrap()),
            ),
        },
        None => ContainedSpan::new(
            code_formatter
                .format_symbol(start_brace.to_owned(), TokenReference::symbol("{").unwrap()),
            code_formatter
                .format_symbol(end_brace.to_owned(), TokenReference::symbol("}").unwrap()),
        ),
    };

    if is_multiline {
        code_formatter.add_indent_range(braces_range);
    }

    // TODO: Should we sort NameKey/ExpressionKey tables?
    loop {
        match current_fields.next() {
            Some(field) => {
                let formatted_field = format_field(
                    code_formatter,
                    field,
                    if is_multiline {
                        let range = match field {
                            Field::ExpressionKey { brackets, .. } => {
                                get_token_range(brackets.tokens().0.token())
                            }
                            Field::NameKey { key, .. } => get_token_range(key.token()),
                            Field::NoKey(expr) => get_range_in_expression(expr),
                        };
                        let additional_indent_level =
                            code_formatter.get_range_indent_increase(range);
                        Some(vec![
                            code_formatter.create_indent_trivia(additional_indent_level)
                        ])
                    } else {
                        None
                    },
                );
                let mut punctuation = None;

                match is_multiline {
                    true => {
                        // Continue adding a comma and a new line for multiline tables
                        let symbol = String::from(",")
                            + &get_line_ending_character(&code_formatter.config.line_endings);
                        punctuation = Some(Cow::Owned(TokenReference::symbol(&symbol).unwrap()));
                    }
                    false => {
                        if let Some(_) = current_fields.peek() {
                            // Have more elements still to go
                            punctuation = Some(Cow::Owned(TokenReference::symbol(", ").unwrap()));
                        };
                    }
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
