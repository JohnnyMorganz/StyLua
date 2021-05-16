use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, hang_expression},
        general::{format_contained_span, format_end_token, format_token_reference, EndTokenType},
        trivia::{strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        trivia_util,
    },
    shape::Shape,
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

fn format_field<'ast>(
    ctx: &Context,
    field: &Field<'ast>,
    table_type: TableType,
    shape: Shape,
) -> (Field<'ast>, Vec<Token<'ast>>) {
    let leading_trivia = match table_type {
        TableType::MultiLine => FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
        _ => FormatTriviaType::NoChange,
    };

    let trailing_trivia;
    let field = match field {
        Field::ExpressionKey {
            brackets,
            key,
            equal,
            value,
        } => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
            let brackets =
                format_contained_span(ctx, brackets, shape).update_leading_trivia(leading_trivia);
            let key = format_expression(ctx, key, shape + 1); // 1 = opening bracket
            let equal = fmt_symbol!(ctx, equal, " = ", shape);
            let shape = shape.take_last_line(&key) + (2 + 3); // 2 = brackets, 3 = " = "

            let singleline_value = format_expression(ctx, value, shape)
                .update_trailing_trivia(FormatTriviaType::Replace(vec![])); // We will remove all the trivia from this value, and place it after the comma

            let value = if trivia_util::can_hang_expression(value)
                && shape.take_first_line(&singleline_value).over_budget()
            {
                hang_expression(ctx, value, shape, Some(1))
                    .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
            } else {
                singleline_value
            };

            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            }
        }
        Field::NameKey { key, equal, value } => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
            let key = format_token_reference(ctx, key, shape).update_leading_trivia(leading_trivia);
            let equal = fmt_symbol!(ctx, equal, " = ", shape);
            let shape = shape + (strip_trivia(&key).to_string().len() + 3); // 3 = " = "

            let singleline_value = format_expression(ctx, value, shape)
                .update_trailing_trivia(FormatTriviaType::Replace(vec![])); // We will remove all the trivia from this value, and place it after the comma

            let value = if trivia_util::can_hang_expression(value)
                && shape.take_first_line(&singleline_value).over_budget()
            {
                hang_expression(ctx, value, shape, Some(1))
                    .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
            } else {
                singleline_value
            };

            Field::NameKey { key, equal, value }
        }
        Field::NoKey(expression) => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(expression);
            let formatted_expression = format_expression(ctx, expression, shape);

            if let TableType::MultiLine = table_type {
                // If still over budget, hang the expression
                let formatted_expression = if trivia_util::can_hang_expression(expression)
                    && shape.take_first_line(&formatted_expression).over_budget()
                {
                    hang_expression(ctx, expression, shape, Some(1))
                } else {
                    formatted_expression
                };

                Field::NoKey(
                    formatted_expression
                        .update_leading_trivia(leading_trivia)
                        .update_trailing_trivia(FormatTriviaType::Replace(vec![])),
                )
            } else {
                Field::NoKey(formatted_expression)
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
    shape: Shape,
) -> ContainedSpan<'ast> {
    match table_type {
        TableType::MultiLine => {
            // Format start and end brace properly with correct trivia
            let end_brace_leading_trivia = vec![create_indent_trivia(ctx, shape)];

            // Add new_line trivia to start_brace
            let start_brace_token = fmt_symbol!(ctx, start_brace, "{", shape)
                .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]));

            let end_brace_token =
                format_end_token(ctx, end_brace, EndTokenType::ClosingBrace, shape)
                    .update_leading_trivia(FormatTriviaType::Append(end_brace_leading_trivia));

            ContainedSpan::new(start_brace_token, end_brace_token)
        }

        TableType::SingleLine => ContainedSpan::new(
            fmt_symbol!(ctx, start_brace, "{ ", shape),
            fmt_symbol!(ctx, end_brace, " }", shape),
        ),

        TableType::Empty => {
            let start_brace = fmt_symbol!(ctx, start_brace, "{", shape);
            let end_brace = fmt_symbol!(ctx, end_brace, "}", shape);
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
    ctx: &Context,
    table_constructor: &TableConstructor<'ast>,
    shape: Shape,
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

    // Determine if there are any comments within the table. If so, we should force, multiline
    let contains_comments = {
        let braces_contain_comments = start_brace.trailing_trivia().any(|trivia| {
            trivia.token_kind() == TokenKind::SingleLineComment
                || trivia.token_kind() == TokenKind::MultiLineComment
        }) || end_brace.leading_trivia().any(|trivia| {
            trivia.token_kind() == TokenKind::SingleLineComment
                || trivia.token_kind() == TokenKind::MultiLineComment
        });

        braces_contain_comments || trivia_util::table_fields_contains_comments(table_constructor)
    };

    // Use input shape to determine if we are over budget
    // TODO: should we format the table onto a single line first?
    let singleline_shape = shape + (braces_range.1 - braces_range.0);

    let table_type = match (contains_comments, current_fields.peek()) {
        // We have comments, so force multiline
        (true, _) => TableType::MultiLine,

        (false, Some(_)) => match singleline_shape.over_budget() {
            true => TableType::MultiLine,
            false => {
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
        },
        (false, None) => TableType::Empty,
    };

    let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
    let mut shape = match table_type {
        TableType::SingleLine => shape + 2, // 1 = opening brace, 1 = space
        TableType::MultiLine => shape.reset().increment_additional_indent(), // Will take new line, and additional indentation
        TableType::Empty => shape,
    };

    while let Some(pair) = current_fields.next() {
        let (field, punctuation) = pair.into_tuple();

        // Reset the shape onto a newline if multiline
        if let TableType::MultiLine = table_type {
            shape = shape.reset()
        };

        let (formatted_field, mut trailing_trivia) = format_field(ctx, &field, table_type, shape);

        // If trivia is just whitespace, ignore it completely
        if trailing_trivia
            .iter()
            .all(trivia_util::trivia_is_whitespace)
        {
            trailing_trivia = Vec::new();
        } else {
            // Filter trailing trivia for any newlines
            trailing_trivia = trailing_trivia
                .iter()
                .filter(|x| !trivia_util::trivia_is_newline(x))
                .map(|x| x.to_owned())
                .collect();
        }

        let mut formatted_punctuation = None;

        match table_type {
            TableType::MultiLine => {
                // Continue adding a comma and a new line for multiline tables
                // Add newline trivia to the end of the symbol
                trailing_trivia.push(create_newline_trivia(ctx));
                let symbol = match punctuation {
                    Some(punctuation) => fmt_symbol!(ctx, &punctuation, ",", shape),
                    None => TokenReference::symbol(",").unwrap(),
                }
                .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));
                formatted_punctuation = Some(symbol)
            }
            _ => {
                if current_fields.peek().is_some() {
                    // Have more elements still to go
                    shape = shape + (formatted_field.to_string().len() + 2); // 2 = ", "
                    formatted_punctuation = match punctuation {
                        Some(punctuation) => Some(fmt_symbol!(ctx, &punctuation, ", ", shape)),
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
