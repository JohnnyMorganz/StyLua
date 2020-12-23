use crate::formatters::{get_line_ending_character, trivia_formatter, CodeFormatter};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Field, TableConstructor,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::borrow::Cow;

impl CodeFormatter {
    pub fn format_field<'ast>(
        &mut self,
        field: &Field<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
                    FormatTriviaType::NoChange,
            },
            Field::NameKey { key, equal, value } => Field::NameKey {
                key: Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    self.format_token_reference(key.to_owned()).into_owned(),
                    leading_trivia,
                    FormatTriviaType::NoChange,
            },
            Field::NoKey(expression) => {
                let formatted_expression = self.format_expression(expression.to_owned());
                if let FormatTriviaType::NoChange = leading_trivia {
                    Field::NoKey(formatted_expression)
                } else {
                    Field::NoKey(trivia_formatter::expression_add_leading_trivia(
                        formatted_expression,
                        leading_trivia,
                    ))
                }
            }
        }
    }

    pub fn format_table_constructor<'ast>(
        &mut self,
        table_constructor: TableConstructor<'ast>,
    ) -> TableConstructor<'ast> {
        let mut fields = Punctuated::new();
        let mut current_fields = table_constructor
            .fields()
            .to_owned()
            .into_pairs()
            .peekable();

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
                    let additional_indent_level = self.get_range_indent_increase(braces_range);
                    let end_brace_leading_trivia =
                        vec![self.create_indent_trivia(additional_indent_level)];

                    // Add new_line trivia to start_brace
                    let start_brace_token = TokenReference::symbol(
                        &(String::from("{")
                            + &get_line_ending_character(&self.config.line_endings)),
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
                        self.format_symbol(start_brace.to_owned(), start_brace_token),
                        self.format_symbol(end_brace.to_owned(), end_brace_token),
                    )
                }
                false => ContainedSpan::new(
                    self.format_symbol(
                        start_brace.to_owned(),
                        TokenReference::symbol("{ ").unwrap(),
                    ),
                    self.format_symbol(end_brace.to_owned(), TokenReference::symbol(" }").unwrap()),
                ),
            },
            None => ContainedSpan::new(
                self.format_symbol(start_brace.to_owned(), TokenReference::symbol("{").unwrap()),
                self.format_symbol(end_brace.to_owned(), TokenReference::symbol("}").unwrap()),
            ),
        };

        if is_multiline {
            self.add_indent_range(braces_range);
        }

        while let Some(pair) = current_fields.next() {
            let (field, punctuation) = pair.into_tuple();

            let leading_trivia = match is_multiline {
                true => {
                    let range = match field.to_owned() {
                        Field::ExpressionKey { brackets, .. } => {
                            CodeFormatter::get_token_range(brackets.tokens().0.token())
                        }
                        Field::NameKey { key, .. } => CodeFormatter::get_token_range(key.token()),
                        Field::NoKey(expr) => CodeFormatter::get_range_in_expression(&expr),
                    };
                    let additional_indent_level = self.get_range_indent_increase(range);
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ])
                }
                false => FormatTriviaType::NoChange,
            };

            let formatted_field = self.format_field(&field, leading_trivia);
            let mut formatted_punctuation = None;

            match is_multiline {
                true => {
                    // Continue adding a comma and a new line for multiline tables
                    let mut symbol = TokenReference::symbol(",").unwrap();
                    if let Some(punctuation) = punctuation {
                        symbol = self
                            .format_symbol(punctuation.into_owned(), symbol)
                            .into_owned();
                    }
                    // Add newline trivia to the end of the symbol
                    let symbol = trivia_formatter::token_reference_add_trivia(
                        symbol,
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(trailing_trivia),
                    );
                    formatted_punctuation = Some(Cow::Owned(symbol))
                }
                false => {
                    if current_fields.peek().is_some() {
                        // Have more elements still to go
                        formatted_punctuation = match punctuation {
                            Some(punctuation) => Some(self.format_symbol(
                                punctuation.into_owned(),
                                TokenReference::symbol(", ").unwrap(),
                            )),
                            None => Some(Cow::Owned(TokenReference::symbol(", ").unwrap())),
                        }
                    };
                }
            }

            fields.push(Pair::new(formatted_field, formatted_punctuation))
        }

        TableConstructor::new()
            .with_braces(braces)
            .with_fields(fields)
    }
}
