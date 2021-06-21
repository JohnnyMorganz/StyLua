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
    Expression, Field, TableConstructor, Value,
};
use full_moon::tokenizer::{Token, TokenReference};

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

/// Formats a table constructor onto a single line.
/// This function does not perform any length checking, or checking whether comments are present.
fn format_singleline_table<'ast>(
    ctx: &Context,
    table_constructor: &TableConstructor<'ast>,
    shape: Shape,
) -> TableConstructor<'ast> {
    let table_type = TableType::SingleLine;

    let (start_brace, end_brace) = table_constructor.braces().tokens();
    let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
    let mut shape = shape + 2; // 2 = "{ "

    let mut fields = Punctuated::new();
    let mut current_fields = table_constructor.fields().pairs().peekable();

    while let Some(pair) = current_fields.next() {
        let (field, punctuation) = (pair.value(), pair.punctuation());

        // Format the field. We will ignore the taken trailing trivia, as we do not need it.
        // (If there were any comments present, this function should never have been called)
        let formatted_field = format_field(ctx, field, table_type, shape).0;

        let formatted_punctuation = match current_fields.peek() {
            Some(_) => {
                // Have more elements still to go
                shape = shape + (formatted_field.to_string().len() + 2); // 2 = ", "
                match punctuation {
                    Some(punctuation) => Some(fmt_symbol!(ctx, &punctuation, ", ", shape)),
                    None => Some(TokenReference::symbol(", ").unwrap()),
                }
            }
            None => None,
        };

        fields.push(Pair::new(formatted_field, formatted_punctuation))
    }

    TableConstructor::new()
        .with_braces(braces)
        .with_fields(fields)
}

/// Expands a table constructor to format it onto multiple lines
/// This function does not perform any length checking.
fn format_multiline_table<'ast>(
    ctx: &Context,
    table_constructor: &TableConstructor<'ast>,
    shape: Shape,
) -> TableConstructor<'ast> {
    let table_type = TableType::MultiLine;

    let (start_brace, end_brace) = table_constructor.braces().tokens();
    let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
    let mut shape = shape.reset().increment_additional_indent(); // Will take new line, and additional indentation

    let mut fields = Punctuated::new();
    let current_fields = table_constructor.fields().pairs();

    for pair in current_fields {
        let (field, punctuation) = (pair.value(), pair.punctuation());

        // Reset the shape onto a new line, as we are a new field
        shape = shape.reset();

        // Format the field
        let (formatted_field, mut trailing_trivia) = format_field(ctx, field, table_type, shape);

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

        // Continue adding a comma and a new line for multiline tables
        // Add newline trivia to the end of the symbol
        trailing_trivia.push(create_newline_trivia(ctx));

        let symbol = match punctuation {
            Some(punctuation) => fmt_symbol!(ctx, &punctuation, ",", shape),
            None => TokenReference::symbol(",").unwrap(),
        }
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));
        let formatted_punctuation = Some(symbol);

        fields.push(Pair::new(formatted_field, formatted_punctuation))
    }

    TableConstructor::new()
        .with_braces(braces)
        .with_fields(fields)
}

fn expression_is_multiline_function(expression: &Expression) -> bool {
    if let Expression::Value { value, .. } = expression {
        if let Value::Function((_, function_body)) = &**value {
            return !trivia_util::is_function_empty(function_body);
        }
    }
    false
}

/// Examines the fields of a table constructor to see if we should force the table constructor multiline.
/// This will only happen if either:
///  1) There are comments within the table
///  2) There are anonymous functions defined within the table [As these will expand multiline]
fn should_expand(table_constructor: &TableConstructor) -> bool {
    let (start_brace, end_brace) = table_constructor.braces().tokens();
    let contains_comments = start_brace
        .trailing_trivia()
        .any(trivia_util::trivia_is_comment)
        || end_brace
            .leading_trivia()
            .any(trivia_util::trivia_is_comment)
        || trivia_util::table_fields_contains_comments(table_constructor);

    if contains_comments {
        true
    } else {
        for field in table_constructor.fields() {
            let should_expand = match field {
                Field::ExpressionKey { key, value, .. } => {
                    expression_is_multiline_function(key) || expression_is_multiline_function(value)
                }
                Field::NameKey { value, .. } => expression_is_multiline_function(value),
                Field::NoKey(expression) => expression_is_multiline_function(expression),
                other => panic!("unknown node {:?}", other),
            };

            if should_expand {
                return true;
            }
        }
        false
    }
}

/// A fail-fast check to determine whether the formatted fields are going over the budget.
/// We format each field one at a time, then add its width to the shape, and check to see if the shape is over budget.
/// Originally, we used `format_singleline_table()` to check this, which had exponential time complexity for nested tables.
/// We don't need to format the whole table to see if we are going over budget.
fn check_table_over_budget<'ast>(
    ctx: &Context,
    fields: &Punctuated<'ast, Field<'ast>>,
    shape: Shape,
) -> bool {
    // Use an infinite width shape to force everything onto a single line as much as possible
    // + 2 = opening brace plus space
    let mut shape = shape.with_infinite_width() + 2;

    for field in fields {
        let formatted_field = format_field(ctx, field, TableType::SingleLine, shape).0;
        shape = shape + (formatted_field.to_string().len() + 2); // 2 = ", "
        if shape.over_budget() {
            return true;
        }
    }

    false
}

pub fn format_table_constructor<'ast>(
    ctx: &Context,
    table_constructor: &TableConstructor<'ast>,
    shape: Shape,
) -> TableConstructor<'ast> {
    let (start_brace, end_brace) = table_constructor.braces().tokens();

    // Determine if we need to force the table multiline
    let should_expand = should_expand(table_constructor);

    let table_type = match (should_expand, table_constructor.fields().iter().next()) {
        // We should expand, so force multiline
        (true, _) => TableType::MultiLine,

        (false, Some(_)) => {
            // Format the table onto a single line, then take the shape to determine if we are over budget
            // let singleline_table =
            //     format_singleline_table(ctx, table_constructor, shape.with_infinite_width());
            // let singleline_shape = shape.take_first_line(&strip_trivia(&singleline_table));
            // let braces_range = (
            //     start_brace.token().end_position().bytes(),
            //     end_brace.token().start_position().bytes(),
            // );
            // let singleline_shape = shape + (braces_range.1 - braces_range.0);

            match check_table_over_budget(ctx, table_constructor.fields(), shape) {
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
            }
        }

        (false, None) => TableType::Empty,
    };

    match table_type {
        TableType::Empty => {
            let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
            TableConstructor::new().with_braces(braces)
        }
        TableType::SingleLine => format_singleline_table(ctx, table_constructor, shape),
        TableType::MultiLine => format_multiline_table(ctx, table_constructor, shape),
    }
}
