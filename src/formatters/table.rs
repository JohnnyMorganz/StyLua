use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::format_expression,
        general::{
            format_contained_span, format_end_token, format_symbol, format_token_reference,
            EndTokenType,
        },
        trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        trivia_util,
        util::{expression_range, token_range},
    },
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

pub fn format_field<'ast>(
    ctx: &mut Context,
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
                brackets: format_contained_span(ctx, brackets)
                    .update_leading_trivia(leading_trivia),
                key: format_expression(ctx, key),
                equal: fmt_symbol!(ctx, equal, " = "),
                // We will remove all the trivia from this value, and place it after the comma
                value: format_expression(ctx, value)
                    .update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            }
        }
        Field::NameKey { key, equal, value } => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
            Field::NameKey {
                key: format_token_reference(ctx, key).update_leading_trivia(leading_trivia),
                equal: fmt_symbol!(ctx, equal, " = "),
                value: format_expression(ctx, value)
                    .update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            }
        }
        Field::NoKey(expression) => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(expression);
            let formatted_expression = format_expression(ctx, expression);
            if let FormatTriviaType::NoChange = leading_trivia {
                Field::NoKey(formatted_expression)
            } else {
                Field::NoKey(
                    formatted_expression
                        .update_leading_trivia(leading_trivia)
                        .update_trailing_trivia(FormatTriviaType::Replace(vec![])),
                )
            }
        }

        other => panic!("unknown node {:?}", other),
    };

    (field, trailing_trivia)
}

pub fn create_table_braces<'ast>(
    ctx: &Context,
    start_brace: &TokenReference<'ast>,
    end_brace: &TokenReference<'ast>,
    table_type: TableType,
    additional_indent_level: Option<usize>,
) -> ContainedSpan<'ast> {
    match table_type {
        TableType::MultiLine => {
            // Format start and end brace properly with correct trivia
            let end_brace_leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];

            // Add new_line trivia to start_brace
            let start_brace_token = fmt_symbol!(ctx, start_brace, "{")
                .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]));

            let end_brace_token = format_end_token(ctx, end_brace, EndTokenType::ClosingBrace)
                .update_leading_trivia(FormatTriviaType::Append(end_brace_leading_trivia));

            ContainedSpan::new(start_brace_token, end_brace_token)
        }

        TableType::SingleLine => ContainedSpan::new(
            fmt_symbol!(ctx, start_brace, "{ "),
            fmt_symbol!(ctx, end_brace, " }"),
        ),

        TableType::Empty => {
            let start_brace = fmt_symbol!(ctx, start_brace, "{");
            let end_brace = fmt_symbol!(ctx, end_brace, "}");
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
                start_brace
                    .update_trailing_trivia(FormatTriviaType::Replace(start_brace_trailing_trivia)),
                end_brace
                    .update_leading_trivia(FormatTriviaType::Replace(end_brace_leading_trivia)),
            )
        }
    }
}

pub fn format_table_constructor<'ast>(
    ctx: &mut Context,
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
        start_brace.token().end_position().bytes(),
        end_brace.token().start_position().bytes(),
    );

    // We subtract 20 as we don't have full information about what preceded this table constructor (e.g. the assignment).
    // This is used as a general estimate. TODO: see if we can improve this calculation
    let mut is_multiline =
        (braces_range.1 - braces_range.0) + ctx.indent_width() > ctx.config().column_width - 20;

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

    if let TableType::MultiLine = table_type {
        // Need to take the inner portion of the braces, not including the braces themselves
        ctx.add_indent_range(braces_range);
    }

    let additional_indent_level = ctx.get_range_indent_increase(token_range(end_brace.token()));
    let braces = create_table_braces(
        ctx,
        start_brace,
        end_brace,
        table_type,
        additional_indent_level,
    );

    while let Some(pair) = current_fields.next() {
        let (field, punctuation) = pair.into_tuple();

        let leading_trivia = match table_type {
            TableType::MultiLine => {
                let range = match field.to_owned() {
                    Field::ExpressionKey { brackets, .. } => {
                        token_range(brackets.tokens().0.token())
                    }
                    Field::NameKey { key, .. } => token_range(key.token()),
                    Field::NoKey(expr) => expression_range(&expr),
                    other => panic!("unknown node {:?}", other),
                };
                let additional_indent_level = ctx.get_range_indent_increase(range);
                FormatTriviaType::Append(vec![create_indent_trivia(ctx, additional_indent_level)])
            }
            _ => FormatTriviaType::NoChange,
        };

        let (formatted_field, mut trailing_trivia) = format_field(ctx, &field, leading_trivia);
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
                // Add newline trivia to the end of the symbol
                trailing_trivia.push(create_newline_trivia(ctx));
                let symbol = match punctuation {
                    Some(punctuation) => fmt_symbol!(ctx, &punctuation, ","),
                    None => TokenReference::symbol(",").unwrap(),
                }
                .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));
                formatted_punctuation = Some(symbol)
            }
            _ => {
                if current_fields.peek().is_some() {
                    // Have more elements still to go
                    formatted_punctuation = match punctuation {
                        Some(punctuation) => Some(format_symbol(
                            ctx,
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
