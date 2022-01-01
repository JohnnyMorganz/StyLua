use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, hang_expression, is_brackets_string},
        general::{format_contained_span, format_end_token, format_token_reference, EndTokenType},
        trivia::{strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        trivia_util,
    },
    shape::Shape,
};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Expression, Field, TableConstructor, Value,
    },
    node::Node,
    tokenizer::{Token, TokenReference, TokenType},
};

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

/// Formats an Expression value part of a k,v field pair
fn format_field_expression_value(
    ctx: &Context,
    expression: &Expression,
    shape: Shape,
) -> Expression {
    let singleline_value = format_expression(ctx, expression, shape)
        .update_trailing_trivia(FormatTriviaType::Replace(vec![])); // We will remove all the trivia from this value, and place it after the comma

    if trivia_util::can_hang_expression(expression)
        && shape.take_first_line(&singleline_value).over_budget()
    {
        hang_expression(ctx, expression, shape, Some(1))
            .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
    } else {
        singleline_value
    }
}

/// Handles the formatting of the comments around a key and the equals sign in a field of a table.
/// Takes in the key as a node (so that we can handle both expression key brackets and name keys)
/// as well as the equals sign, then outputs the new leading trivia of the key + new equals token. The trailing trivia of the key should be emptied.
fn handle_field_key_equals_comments<T: Node>(
    ctx: &Context,
    key: &T,
    equal: &TokenReference,
    shape: Shape,
) -> (Vec<Token>, TokenReference) {
    // Get the current leading and trailing trivia around the key
    let (key_leading_trivia, key_trailing_trivia) = key.surrounding_trivia();

    let key_leading_trivia_iter = key_leading_trivia.iter();

    // Preverse any leading whitespace from the key, so that we can keep it
    let key_leading_whitespace = key_leading_trivia_iter
        .take_while(|token| trivia_util::trivia_is_whitespace(token))
        .map(|x| x.to_owned().to_owned());

    // Retrieve the old leading comments from the key, so that we can append onto them
    // We also chain on any trailing comments of the key, so that we can move them to before the key
    let key_comments = key_leading_trivia
        .iter() // We recreate the iterator since the previous one was consumed
        .chain(key_trailing_trivia.iter())
        .filter(|token| trivia_util::trivia_is_comment(token))
        .map(|x| x.to_owned().to_owned());

    // Take leading and trailing comments from the equal sign, and put it before the key
    let equal_sign_comments = equal
        .leading_trivia()
        .chain(equal.trailing_trivia())
        .filter(|token| trivia_util::trivia_is_comment(token))
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    // Join together the comments, and intersperse a newline between them
    let key_leading_comments = key_comments.chain(equal_sign_comments).flat_map(|comment| {
        // Prepend an indent before the comment, and append a newline after the comments
        vec![
            create_indent_trivia(ctx, shape),
            comment.to_owned(),
            create_newline_trivia(ctx),
        ]
    });

    // Join together the leading whitespace and the comments, and collect into into a Vec
    let key_leading_comments = key_leading_whitespace
        .chain(key_leading_comments)
        .collect::<Vec<_>>();

    // Create the new equals token
    let equal = TokenReference::symbol("=")
        .unwrap()
        .update_leading_trivia(FormatTriviaType::Replace(vec![Token::new(
            TokenType::spaces(1),
        )]))
        .update_trailing_trivia(FormatTriviaType::Replace(vec![Token::new(
            TokenType::spaces(1),
        )]));

    (key_leading_comments, equal)
}

fn format_field(
    ctx: &Context,
    field: &Field,
    table_type: TableType,
    shape: Shape,
) -> (Field, Vec<Token>) {
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
            let brackets = format_contained_span(ctx, brackets, shape);

            let space_brackets = is_brackets_string(key);
            let key = if space_brackets {
                format_expression(ctx, key, shape + 2) // 2 = "[ "
                    .update_leading_trivia(FormatTriviaType::Append(vec![Token::new(
                        TokenType::spaces(1),
                    )]))
                    .update_trailing_trivia(FormatTriviaType::Append(vec![Token::new(
                        TokenType::spaces(1),
                    )]))
            } else {
                format_expression(ctx, key, shape + 1) // 1 = "["
            };

            // Get the new leading comments to add before the key, and the equal token
            let (key_leading_comments, equal) =
                handle_field_key_equals_comments(ctx, &brackets, equal, shape);

            // Update the key to contain the leading comments, remove the trailing comments,
            // and also add the extra leading_trivia we add in general to all fields
            let brackets = brackets
                .update_leading_trivia(FormatTriviaType::Replace(key_leading_comments))
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
                .update_leading_trivia(leading_trivia);

            let shape = shape.take_last_line(&key) + (2 + 3 + if space_brackets { 2 } else { 0 }); // 2 = brackets, 3 = " = ", 2 = spaces around brackets if necessary
            let value = format_field_expression_value(ctx, value, shape);

            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            }
        }
        Field::NameKey { key, equal, value } => {
            trailing_trivia = trivia_util::get_expression_trailing_trivia(value);
            let key = format_token_reference(ctx, key, shape);

            // Get the new leading comments to add before the key, and the equal token
            let (key_leading_comments, equal) =
                handle_field_key_equals_comments(ctx, &key, equal, shape);

            // Update the key to contain the leading comments, remove the trailing comments,
            // and also add the extra leading_trivia we add in general to all fields
            let key = key
                .update_leading_trivia(FormatTriviaType::Replace(key_leading_comments))
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
                .update_leading_trivia(leading_trivia);

            let shape = shape + (strip_trivia(&key).to_string().len() + 3); // 3 = " = "
            let value = format_field_expression_value(ctx, value, shape);

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

pub fn create_table_braces(
    ctx: &Context,
    start_brace: &TokenReference,
    end_brace: &TokenReference,
    table_type: TableType,
    shape: Shape,
) -> ContainedSpan {
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

/// Formats a table onto a single line.
/// Takes in a [`ContainedSpan`] representing the braces, and the fields within the table.
/// This function is generic to support [`TableConstructor`] and [`TypeInfo::Table`] in Luau.
/// This function does not perform any length checking, or checking whether comments are present.
pub fn format_singleline_table<T, U>(
    ctx: &Context,
    braces: &ContainedSpan,
    fields: &Punctuated<T>,
    formatter: U,
    shape: Shape,
) -> (ContainedSpan, Punctuated<T>)
where
    T: std::fmt::Display,
    U: Fn(&Context, &T, TableType, Shape) -> (T, Vec<Token>),
{
    let table_type = TableType::SingleLine;

    let (start_brace, end_brace) = braces.tokens();
    let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
    let mut shape = shape + 2; // 2 = "{ "

    let mut current_fields = fields.pairs().peekable();
    let mut fields = Punctuated::new();

    while let Some(pair) = current_fields.next() {
        let (field, punctuation) = (pair.value(), pair.punctuation());

        // Format the field. We will ignore the taken trailing trivia, as we do not need it.
        // (If there were any comments present, this function should never have been called)
        let formatted_field = formatter(ctx, field, table_type, shape).0;

        let formatted_punctuation = match current_fields.peek() {
            Some(_) => {
                // Have more elements still to go
                shape = shape + (formatted_field.to_string().len() + 2); // 2 = ", "
                match punctuation {
                    Some(punctuation) => Some(fmt_symbol!(ctx, punctuation, ", ", shape)),
                    None => Some(TokenReference::symbol(", ").unwrap()),
                }
            }
            None => None,
        };

        fields.push(Pair::new(formatted_field, formatted_punctuation))
    }

    (braces, fields)
}

/// Expands a table's fields to format it onto multiple lines
/// Takes in a [`ContainedSpan`] representing the braces, and the fields within the table.
/// This function is generic to support [`TableConstructor`] and [`TypeInfo::Table`] in Luau.
/// This function does not perform any length checking.
pub fn format_multiline_table<T, U>(
    ctx: &Context,
    braces: &ContainedSpan,
    fields: &Punctuated<T>,
    formatter: U,
    shape: Shape,
) -> (ContainedSpan, Punctuated<T>)
where
    T: std::fmt::Display,
    U: Fn(&Context, &T, TableType, Shape) -> (T, Vec<Token>),
{
    let table_type = TableType::MultiLine;

    let (start_brace, end_brace) = braces.tokens();
    let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
    let mut shape = shape.reset().increment_additional_indent(); // Will take new line, and additional indentation

    let current_fields = fields.pairs();
    let mut fields = Punctuated::new();

    for pair in current_fields {
        let (field, punctuation) = (pair.value(), pair.punctuation());

        // Reset the shape onto a new line, as we are a new field
        shape = shape.reset().add_width(1); // Add 1 to include the trailing comma at the end

        // Format the field
        let (formatted_field, mut trailing_trivia) = formatter(ctx, field, table_type, shape);

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
            Some(punctuation) => fmt_symbol!(ctx, punctuation, ",", shape),
            None => TokenReference::symbol(",").unwrap(),
        }
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));
        let formatted_punctuation = Some(symbol);

        fields.push(Pair::new(formatted_field, formatted_punctuation))
    }

    (braces, fields)
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

pub fn format_table_constructor(
    ctx: &Context,
    table_constructor: &TableConstructor,
    shape: Shape,
) -> TableConstructor {
    let (start_brace, end_brace) = table_constructor.braces().tokens();

    // Determine if we need to force the table multiline
    let should_expand = should_expand(table_constructor);

    let table_type = match (should_expand, table_constructor.fields().iter().next()) {
        // We should expand, so force multiline
        (true, _) => TableType::MultiLine,

        (false, Some(_)) => {
            // Compare the difference between the position of the start brace and the end brace to
            // guess how long the table is. This heuristic is very naiive, since it relies on the input.
            // If the input is badly formatted (e.g. lots of spaces in the table), then it would flag this over width.
            // However, this is currently our best solution: attempting to format the input onto a single line to
            // see if we are over width (both completely and in a fail-fast shape.over_budget() check) leads to
            // exponential time complexity with respect to how deep the table is.
            // TODO: find an improved heuristic whilst comparing against benchmarks
            let braces_range = (
                // Use the position of the last trivia in case there is some present (e.g. whitespace)
                // So that we don't include an extra space
                if let Some(token) = start_brace.leading_trivia().last() {
                    token.end_position().bytes()
                } else {
                    start_brace.token().end_position().bytes()
                },
                end_brace.token().start_position().bytes(),
            );
            let singleline_shape = shape + (braces_range.1 - braces_range.0) + 3; // 4 = two braces + single space before last brace

            match singleline_shape.over_budget() {
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

    let (braces, fields) = match table_type {
        TableType::Empty => {
            let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);
            (braces, Punctuated::new())
        }
        TableType::SingleLine => format_singleline_table(
            ctx,
            table_constructor.braces(),
            table_constructor.fields(),
            format_field,
            shape,
        ),
        TableType::MultiLine => format_multiline_table(
            ctx,
            table_constructor.braces(),
            table_constructor.fields(),
            format_field,
            shape,
        ),
    };

    TableConstructor::new()
        .with_braces(braces)
        .with_fields(fields)
}
