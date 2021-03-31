use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    trivia_util, CodeFormatter,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Field, TableConstructor,
};
use full_moon::tokenizer::{Token, TokenKind, TokenReference};

/// Used to provide information about the table
#[derive(Debug, Clone, Copy)]
pub enum TableType {
    /// The table will have multline fields
    MultiLine,
    /// The table will be on a single line
    SingleLine,
    /// The table has no fields
    Empty,
}

impl CodeFormatter {
    pub fn format_field<'ast>(
        &mut self,
        field: &Field<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> (Field<'ast>, Vec<Token<'ast>>) {
        let trailing_trivia;
        let field = match field {
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            } => {
                trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
                Field::ExpressionKey {
                    brackets: trivia_formatter::contained_span_add_trivia(
                        self.format_contained_span(brackets),
                        leading_trivia,
                        FormatTriviaType::NoChange,
                    ),
                    key: self.format_expression(key),
                    equal: crate::fmt_symbol!(self, equal, " = "),
                    // We will remove all the trivia from this value, and place it after the comma
                    value: trivia_formatter::expression_add_trailing_trivia(
                        self.format_expression(value),
                        FormatTriviaType::Replace(vec![]),
                    ),
                }
            }
            Field::NameKey { key, equal, value } => {
                trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
                Field::NameKey {
                    key: trivia_formatter::token_reference_add_trivia(
                        self.format_token_reference(key),
                        leading_trivia,
                        FormatTriviaType::NoChange,
                    ),
                    equal: crate::fmt_symbol!(self, equal, " = "),
                    value: trivia_formatter::expression_add_trailing_trivia(
                        self.format_expression(value),
                        FormatTriviaType::Replace(vec![]),
                    ),
                }
            }
            Field::NoKey(expression) => {
                trailing_trivia = trivia_util::get_expression_trailing_trivia(expression);
                let formatted_expression = self.format_expression(expression);
                if let FormatTriviaType::NoChange = leading_trivia {
                    Field::NoKey(formatted_expression)
                } else {
                    Field::NoKey(trivia_formatter::expression_add_trailing_trivia(
                        trivia_formatter::expression_add_leading_trivia(
                            formatted_expression,
                            leading_trivia,
                        ),
                        FormatTriviaType::Replace(vec![]),
                    ))
                }
            }

            other => panic!("unknown node {:?}", other),
        };

        (field, trailing_trivia)
    }

    pub fn create_table_braces<'ast>(
        &self,
        start_brace: &TokenReference<'ast>,
        end_brace: &TokenReference<'ast>,
        table_type: TableType,
        additional_indent_level: Option<usize>,
    ) -> ContainedSpan<'ast> {
        match table_type {
            TableType::MultiLine => {
                // Format start and end brace properly with correct trivia
                let end_brace_leading_trivia =
                    vec![self.create_indent_trivia(additional_indent_level)];

                // Add new_line trivia to start_brace
                let start_brace_token = trivia_formatter::token_reference_add_trivia(
                    crate::fmt_symbol!(self, start_brace, "{"),
                    FormatTriviaType::NoChange,
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                );

                let end_brace_token = trivia_formatter::token_reference_add_trivia(
                    self.format_end_token(end_brace),
                    FormatTriviaType::Append(end_brace_leading_trivia),
                    FormatTriviaType::NoChange,
                );

                ContainedSpan::new(start_brace_token, end_brace_token)
            }

            TableType::SingleLine => ContainedSpan::new(
                crate::fmt_symbol!(self, start_brace, "{ "),
                crate::fmt_symbol!(self, end_brace, " }"),
            ),

            TableType::Empty => {
                let start_brace = crate::fmt_symbol!(self, start_brace, "{");
                let end_brace = crate::fmt_symbol!(self, end_brace, "}");
                // Remove any newline trivia trailing the start brace, as it shouldn't be present
                let start_brace_trailing_trivia = start_brace
                    .trailing_trivia()
                    .filter(|t| !trivia_util::trivia_is_newline(t))
                    .map(|x| x.to_owned())
                    .collect();
                // Remove any newline trivia leading the end brace, as it shouldn't be present
                let end_brace_leading_trivia = end_brace
                    .leading_trivia()
                    .filter(|t| !trivia_util::trivia_is_newline(t))
                    .map(|x| x.to_owned())
                    .collect();

                ContainedSpan::new(
                    trivia_formatter::token_reference_add_trivia(
                        start_brace,
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Replace(start_brace_trailing_trivia),
                    ),
                    trivia_formatter::token_reference_add_trivia(
                        end_brace,
                        FormatTriviaType::Replace(end_brace_leading_trivia),
                        FormatTriviaType::NoChange,
                    ),
                )
            }
        }
    }

    pub fn format_table_constructor<'ast>(
        &mut self,
        table_constructor: &TableConstructor<'ast>,
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

        // We subtract 20 as we don't have full information about what preceded this table constructor (e.g. the assignment).
        // This is used as a general estimate. TODO: see if we can improve this calculation
        let mut is_multiline = (braces_range.1 - braces_range.0) + self.get_indent_width()
            > self.config.column_width - 20;

        // Determine if there are any comments within the table. If so, we should go multiline
        if !is_multiline {
            let braces_contain_comments = start_brace.trailing_trivia().any(|trivia| {
                trivia.token_kind() == TokenKind::SingleLineComment
                    || trivia.token_kind() == TokenKind::MultiLineComment
            }) || end_brace.leading_trivia().any(|trivia| {
                trivia.token_kind() == TokenKind::SingleLineComment
                    || trivia.token_kind() == TokenKind::MultiLineComment
            });

            is_multiline = braces_contain_comments
                || trivia_util::table_fields_contains_comments(table_constructor)
        };

        let table_type = match is_multiline {
            true => TableType::MultiLine,
            false => match current_fields.peek() {
                Some(_) => {
                    // Determine if there was a new line at the end of the start brace
                    // If so, then we should always be multiline
                    if start_brace
                        .trailing_trivia()
                        .any(trivia_util::trivia_is_newline)
                    {
                        TableType::MultiLine
                    } else {
                        TableType::SingleLine
                    }
                }
                None => TableType::Empty,
            },
        };

        let additional_indent_level =
            self.get_range_indent_increase(CodeFormatter::get_token_range(end_brace.token()));
        let braces =
            self.create_table_braces(start_brace, end_brace, table_type, additional_indent_level);

        if let TableType::MultiLine = table_type {
            // Need to take the inner portion of the braces, not including the braces themselves
            let braces_range = (braces_range.0, braces_range.1);
            self.add_indent_range(braces_range);
        }

        while let Some(pair) = current_fields.next() {
            let (field, punctuation) = pair.into_tuple();

            let leading_trivia = match table_type {
                TableType::MultiLine => {
                    let range = match field.to_owned() {
                        Field::ExpressionKey { brackets, .. } => {
                            CodeFormatter::get_token_range(brackets.tokens().0.token())
                        }
                        Field::NameKey { key, .. } => CodeFormatter::get_token_range(key.token()),
                        Field::NoKey(expr) => CodeFormatter::get_range_in_expression(&expr),
                        other => panic!("unknown node {:?}", other),
                    };
                    let additional_indent_level = self.get_range_indent_increase(range);
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ])
                }
                _ => FormatTriviaType::NoChange,
            };

            let (formatted_field, mut trailing_trivia) = self.format_field(&field, leading_trivia);
            // Filter trailing_trivia for any newlines
            trailing_trivia = trailing_trivia
                .iter()
                .filter(|x| !trivia_util::trivia_is_newline(x))
                .map(|x| x.to_owned())
                .collect();

            let mut formatted_punctuation = None;

            match table_type {
                TableType::MultiLine => {
                    // Continue adding a comma and a new line for multiline tables
                    let mut symbol = TokenReference::symbol(",").unwrap();
                    if let Some(punctuation) = punctuation {
                        symbol = self.format_symbol(&punctuation, &symbol);
                    }
                    // Add newline trivia to the end of the symbol, and preserve any comments
                    trailing_trivia.push(self.create_newline_trivia());
                    let symbol = trivia_formatter::token_reference_add_trivia(
                        symbol,
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(trailing_trivia),
                    );
                    formatted_punctuation = Some(symbol)
                }
                _ => {
                    if current_fields.peek().is_some() {
                        // Have more elements still to go
                        formatted_punctuation = match punctuation {
                            Some(punctuation) => Some(self.format_symbol(
                                &punctuation,
                                &TokenReference::symbol(", ").unwrap(),
                            )),
                            None => Some(TokenReference::symbol(", ").unwrap()),
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
