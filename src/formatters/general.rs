use crate::{
    check_should_format,
    context::{create_indent_trivia, create_newline_trivia, Context},
    formatters::{
        trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        trivia_util,
    },
    shape::Shape,
    QuoteStyle,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
};
use full_moon::node::Node;
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenKind, TokenReference, TokenType};

#[derive(Debug)]
enum FormatTokenType {
    Token,
    LeadingTrivia,
    TrailingTrivia,
}

/// The type of end token being used to format
#[derive(Debug)]
pub enum EndTokenType {
    /// A token ending a block, i.e. the `end` symbol
    /// This means that the indent block it was closing is at the current block indent level
    BlockEnd,
    /// A closing brace at the end of a table.
    /// This means that the indent block that it was closing is formed from an indent range, rather than the current block indent level.
    ClosingBrace,
    /// A closing parentheses at the end of e.g. a function call
    /// This means that the indent block that it was closing is formed from an indent range, rather than the current block indent level.
    ClosingParens,
}

#[macro_export]
macro_rules! fmt_symbol {
    ($ctx:expr, $token:expr, $x:expr, $shape:expr) => {
        crate::formatters::general::format_symbol(
            $ctx,
            $token,
            &TokenReference::symbol($x).unwrap(),
            $shape,
        )
    };
}

fn get_quote_to_use(ctx: &Context, literal: &str) -> StringLiteralQuoteType {
    match ctx.config().quote_style {
        QuoteStyle::ForceDouble => StringLiteralQuoteType::Double,
        QuoteStyle::ForceSingle => StringLiteralQuoteType::Single,
        _ => {
            let preferred = match ctx.config().quote_style {
                QuoteStyle::AutoPreferDouble => StringLiteralQuoteType::Double,
                QuoteStyle::AutoPreferSingle => StringLiteralQuoteType::Single,
                _ => unreachable!("have other quote styles we haven't looked into yet"),
            };

            // Check to see if there is a quote within it
            if literal.contains('\'') || literal.contains('"') {
                let num_single_quotes = literal.matches('\'').count();
                let num_double_quotes = literal.matches('"').count();

                match num_single_quotes.cmp(&num_double_quotes) {
                    std::cmp::Ordering::Equal => preferred,
                    std::cmp::Ordering::Greater => StringLiteralQuoteType::Double,
                    std::cmp::Ordering::Less => StringLiteralQuoteType::Single,
                }
            } else {
                preferred
            }
        }
    }
}

fn format_single_line_comment_string(comment: &str) -> &str {
    // Trim any trailing whitespace
    comment.trim_end()
}

/// Formats a Token Node
/// Also returns any extra leading or trailing trivia to add for the Token node
/// This should only ever be called from format_token_reference
fn format_token(
    ctx: &Context,
    token: &Token,
    format_type: &FormatTokenType,
    shape: Shape,
) -> (Token, Option<Vec<Token>>, Option<Vec<Token>>) {
    let mut leading_trivia: Option<Vec<Token>> = None;
    let mut trailing_trivia: Option<Vec<Token>> = None;

    let token_type = match token.token_type() {
        TokenType::Number { text } => {
            let text = if text.starts_with('.') {
                String::from("0") + text.as_str()
            } else if text.starts_with("-.") {
                String::from("-0") + text.get(1..).expect("unknown number literal")
            } else {
                text.to_string()
            }
            .into();

            TokenType::Number { text }
        }
        TokenType::StringLiteral {
            literal,
            multi_line,
            quote_type,
        } => {
            // If we have a brackets string, don't mess with it
            if let StringLiteralQuoteType::Brackets = quote_type {
                TokenType::StringLiteral {
                    literal: literal.to_owned(),
                    multi_line: *multi_line,
                    quote_type: StringLiteralQuoteType::Brackets,
                }
            } else {
                // Match all escapes within the the string
                // Based off https://github.com/prettier/prettier/blob/181a325c1c07f1a4f3738665b7b28288dfb960bc/src/common/util.js#L439
                lazy_static::lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new(r#"\\?(["'])|\\([\S\s])"#).unwrap();
                    static ref UNNECESSARY_ESCAPES: regex::Regex = regex::Regex::new(r#"^[^\n\r"'0-9\\abfnrtuvxz]$"#).unwrap();
                }
                let quote_to_use = get_quote_to_use(ctx, literal);
                let literal = RE
                    .replace_all(literal, |caps: &regex::Captures| {
                        let quote = caps.get(1);
                        let escaped = caps.get(2);

                        match quote {
                            Some(quote) => {
                                // We have a quote, find what type it is, and see if we need to escape it
                                // then return the output string
                                match quote.as_str() {
                                    "'" => {
                                        // Check whether to escape the quote
                                        if let StringLiteralQuoteType::Single = quote_to_use {
                                            String::from("\\'")
                                        } else {
                                            String::from("'")
                                        }
                                    }
                                    "\"" => {
                                        // Check whether to escape the quote
                                        if let StringLiteralQuoteType::Double = quote_to_use {
                                            String::from("\\\"")
                                        } else {
                                            String::from("\"")
                                        }
                                    }
                                    other => unreachable!("unknown quote type {:?}", other),
                                }
                            }
                            None => {
                                // We have a normal escape
                                // Test to see if it is necessary, and if not, then unescape it
                                let text = escaped
                                    .expect("have a match which was neither an escape or a quote")
                                    .as_str();
                                if UNNECESSARY_ESCAPES.is_match(text) {
                                    text.to_owned()
                                } else {
                                    format!("\\{}", text.to_owned())
                                }
                            }
                        }
                    })
                    .into();
                TokenType::StringLiteral {
                    literal,
                    multi_line: None,
                    quote_type: quote_to_use,
                }
            }
        }
        TokenType::SingleLineComment { comment } => {
            let comment = format_single_line_comment_string(comment).into();

            match format_type {
                FormatTokenType::LeadingTrivia => {
                    leading_trivia = Some(vec![create_indent_trivia(ctx, shape)]);
                    trailing_trivia = Some(vec![create_newline_trivia(ctx)]);
                }
                FormatTokenType::TrailingTrivia => {
                    // Add a space before the comment
                    leading_trivia = Some(vec![Token::new(TokenType::spaces(1))]);
                }
                _ => (),
            }

            TokenType::SingleLineComment { comment }
        }
        TokenType::MultiLineComment { blocks, comment } => {
            if let FormatTokenType::LeadingTrivia = format_type {
                leading_trivia = Some(vec![create_indent_trivia(ctx, shape)]);
                // Add a new line once the comment is completed
                trailing_trivia = Some(vec![create_newline_trivia(ctx)]);
            }

            TokenType::MultiLineComment {
                blocks: *blocks,
                comment: comment.to_owned(),
            }
        }
        TokenType::Whitespace { characters } => TokenType::Whitespace {
            characters: characters.to_owned(),
        }, // TODO
        _ => token.token_type().to_owned(),
    };

    (Token::new(token_type), leading_trivia, trailing_trivia)
}

/// Wraps around the format_token function to create a complete list of trivia to add to a node.
/// Handles any leading/trailing trivia provided by format_token, and appends it accordingly in relation to the formatted token.
/// Mainly useful for comments
/// Additional indent level will indent any trivia by the further level - useful for comments on the `end` token
fn load_token_trivia(
    ctx: &Context,
    current_trivia: Vec<&Token>,
    format_token_type: FormatTokenType,
    shape: Shape,
) -> Vec<Token> {
    let mut token_trivia = Vec::new();

    let mut newline_count_in_succession = 0;
    let mut trivia_iter = current_trivia.iter().peekable();

    while let Some(trivia) = trivia_iter.next() {
        match trivia.token_type() {
            TokenType::Whitespace { characters } => {
                // Handle cases where the user has left a newline gap in between e.g. two statements
                // If we are formatting trailing trivia, this can be ignored, as all trailing newlines will have already
                // been handled by the formatter.
                // If we are formatting leading trivia, we will allow a single newline to be kept in succession, if we
                // find one.
                match format_token_type {
                    FormatTokenType::LeadingTrivia => {
                        if characters.contains('\n') {
                            newline_count_in_succession += 1;
                            if newline_count_in_succession == 1 {
                                // We have a case where we will allow a single newline to be kept
                                token_trivia.push(create_newline_trivia(ctx));
                            }
                        }
                    }
                    FormatTokenType::TrailingTrivia => {
                        // If the next trivia is a MultiLineComment, and this whitespace is just spacing, then we
                        // will preserve a single space
                        if let Some(next_trivia) = trivia_iter.peek() {
                            if let TokenType::MultiLineComment { .. } = next_trivia.token_type() {
                                if !characters.contains('\n') {
                                    token_trivia.push(Token::new(TokenType::spaces(1)))
                                }
                            }
                        }
                    }
                    _ => (),
                }

                // Move to next trivia
                continue;
            }
            TokenType::SingleLineComment { .. } | TokenType::MultiLineComment { .. } => {
                // If we have a comment, when `format_token` is called, it will put a newline at the end
                // If this happens, we want to skip the next iteration if its a newline, as that has already been covered here
                if let FormatTokenType::LeadingTrivia = format_token_type {
                    if let Some(next_trivia) = trivia_iter.peek() {
                        if let TokenType::Whitespace { characters } = next_trivia.token_type() {
                            if characters.contains('\n') {
                                // Consume iterator once to skip the next iteration
                                trivia_iter.next();
                            }
                        }
                    }
                }
                // We will reset the counter as well, because the newline above is only to terminate the comment
                newline_count_in_succession = 0;
            }
            _ => {
                // Reset new line counter, as we only want two new lines in a row
                newline_count_in_succession = 0;
            }
        }

        let (token, leading_trivia, trailing_trivia) =
            format_token(ctx, trivia.to_owned(), &format_token_type, shape);
        if let Some(mut trivia) = leading_trivia {
            token_trivia.append(&mut trivia);
        }

        token_trivia.push(token);

        if let Some(mut trivia) = trailing_trivia {
            token_trivia.append(&mut trivia)
        }
    }

    token_trivia
}

pub fn format_token_reference(
    ctx: &Context,
    token_reference: &TokenReference,
    shape: Shape,
) -> TokenReference {
    // Preserve comments in leading/trailing trivia
    let formatted_leading_trivia: Vec<Token> = load_token_trivia(
        ctx,
        token_reference.leading_trivia().collect(),
        FormatTokenType::LeadingTrivia,
        shape,
    );
    let formatted_trailing_trivia: Vec<Token> = load_token_trivia(
        ctx,
        token_reference.trailing_trivia().collect(),
        FormatTokenType::TrailingTrivia,
        shape,
    );

    let (token, _leading_trivia, _trailing_trivia) =
        format_token(ctx, token_reference.token(), &FormatTokenType::Token, shape);

    TokenReference::new(formatted_leading_trivia, token, formatted_trailing_trivia)
}

/// Formats a Punctuated sequence with correct punctuated values.
/// This function assumes that there are no comments present which would lead to a syntax error if the list was collapsed.
/// If not sure about comments, [`try_format_punctuated`] should be used instead.
pub fn format_punctuated<T, F>(
    ctx: &Context,
    old: &Punctuated<T>,
    shape: Shape,
    value_formatter: F,
) -> Punctuated<T>
where
    T: std::fmt::Display,
    F: Fn(&Context, &T, Shape) -> T,
{
    let mut list: Punctuated<T> = Punctuated::new();
    let mut shape = shape;

    for pair in old.pairs() {
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let value = value_formatter(ctx, value, shape);
                let punctuation = fmt_symbol!(ctx, punctuation, ", ", shape);
                shape = shape.take_last_line(&value) + 2; // 2 = ", "

                list.push(Pair::new(value, Some(punctuation)));
            }
            Pair::End(value) => {
                let value = value_formatter(ctx, value, shape);
                list.push(Pair::new(value, None));
            }
        }
    }

    list
}

// Formats a Punctuated sequence across multiple lines. Also indents each item by hang_level
pub fn format_punctuated_multiline<T, F>(
    ctx: &Context,
    old: &Punctuated<T>,
    shape: Shape,
    value_formatter: F,
    hang_level: Option<usize>,
) -> Punctuated<T>
where
    T: Node,
    F: Fn(&Context, &T, Shape) -> T,
{
    let mut formatted: Punctuated<T> = Punctuated::new();

    // Include hang level if required
    let hanging_shape = match hang_level {
        Some(hang_level) => shape.with_indent(shape.indent().add_indent_level(hang_level)),
        None => shape,
    };

    for (idx, pair) in old.pairs().enumerate() {
        // Indent the pair (unless its the first item)
        let shape = if idx == 0 {
            shape
        } else {
            hanging_shape.reset()
        };

        match pair {
            Pair::Punctuated(value, punctuation) => {
                let value = value_formatter(ctx, value, shape);
                let punctuation = fmt_symbol!(ctx, punctuation, ",", shape).update_trailing_trivia(
                    FormatTriviaType::Append(vec![
                        create_newline_trivia(ctx),
                        create_indent_trivia(ctx, hanging_shape), // Use hanging_shape here as we want to have a hanging indent for the next item, which may not be present in the shape of the first item.
                    ]),
                );
                formatted.push(Pair::new(value, Some(punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = value_formatter(ctx, value, shape);
                formatted.push(Pair::new(formatted_value, None));
            }
        }
    }

    formatted
}

/// Formats a Punctuated sequence, depending on its layout. If the sequence contains comments, we will format
/// across multiple lines
pub fn try_format_punctuated<T, F>(
    ctx: &Context,
    old: &Punctuated<T>,
    shape: Shape,
    value_formatter: F,
    hang_level: Option<usize>,
) -> Punctuated<T>
where
    T: Node + std::fmt::Display,
    F: Fn(&Context, &T, Shape) -> T,
{
    let mut format_multiline = false;

    for pair in old.pairs() {
        if let Pair::Punctuated(_, punctuation) = pair {
            if trivia_util::contains_comments(punctuation) {
                format_multiline = true;
                break;
            }
        }
    }

    if format_multiline {
        format_punctuated_multiline(ctx, old, shape, value_formatter, hang_level)
    } else {
        format_punctuated(ctx, old, shape, value_formatter)
    }
}

/// Formats a Punctuated sequence contained within parentheses onto multiple lines
/// In particular, it handles the indentation of the sequence within the parentheses, and comments
pub fn format_contained_punctuated_multiline<T, F1, F2>(
    ctx: &Context,
    parentheses: &ContainedSpan,
    arguments: &Punctuated<T>,
    argument_formatter: F1,              // Function to format the argument
    argument_take_trailing_comments: F2, // Function to separate trailing comments from the argument - so they can be placed after the comma
    shape: Shape,
) -> (ContainedSpan, Punctuated<T>)
where
    T: UpdateLeadingTrivia,
    F1: Fn(&Context, &T, Shape) -> T,
    F2: Fn(&T) -> (T, Vec<Token>),
{
    // Format start and end brace properly with correct trivia
    let (start_parens, end_parens) = parentheses.tokens();
    let start_parens = fmt_symbol!(ctx, start_parens, "(", shape)
        .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]));

    let end_parens = format_end_token(ctx, end_parens, EndTokenType::ClosingParens, shape)
        .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
            ctx, shape,
        )]));

    let parentheses = ContainedSpan::new(start_parens, end_parens);

    let mut formatted_arguments = Punctuated::new();
    let shape = shape.increment_additional_indent();

    for argument in arguments.pairs() {
        let shape = shape.reset(); // Argument is on a new line, so reset the shape

        let formatted_argument = argument_formatter(ctx, argument.value(), shape)
            .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
                ctx, shape,
            )]));

        // Take any trailing trivia (i.e. comments) from the argument, and append it to the end of the punctuation
        let (formatted_argument, mut trailing_comments) =
            argument_take_trailing_comments(&formatted_argument);

        let punctuation = match argument.punctuation() {
            Some(punctuation) => {
                // Continue adding a comma and a new line for multiline function args
                // Also add any trailing comments we have taken from the expression
                trailing_comments.push(create_newline_trivia(ctx));
                let symbol = fmt_symbol!(ctx, punctuation, ",", shape)
                    .update_trailing_trivia(FormatTriviaType::Append(trailing_comments));

                Some(symbol)
            }
            // TODO/HACK: we create a phantom comma which is just actually a new line
            // We need to do this because in function declarations, we format parameters but if they have a type
            // specifier we don't have access to put it after the type specifier
            None => Some(TokenReference::new(
                trailing_comments,
                create_newline_trivia(ctx),
                vec![],
            )),
        };

        formatted_arguments.push(Pair::new(formatted_argument, punctuation))
    }

    (parentheses, formatted_arguments)
}

pub fn format_contained_span(
    ctx: &Context,
    contained_span: &ContainedSpan,
    shape: Shape,
) -> ContainedSpan {
    let (start_token, end_token) = contained_span.tokens();

    ContainedSpan::new(
        format_token_reference(ctx, start_token, shape),
        format_token_reference(ctx, end_token, shape),
    )
}

/// Formats a special TokenReference which is a symbol
/// Used to preserve the comments around the symbol
pub fn format_symbol(
    ctx: &Context,
    current_symbol: &TokenReference,
    wanted_symbol: &TokenReference,
    shape: Shape,
) -> TokenReference {
    // Preserve comments in leading/trailing trivia
    let mut formatted_leading_trivia: Vec<Token> = load_token_trivia(
        ctx,
        current_symbol.leading_trivia().collect(),
        FormatTokenType::LeadingTrivia,
        shape,
    );
    let mut formatted_trailing_trivia: Vec<Token> = load_token_trivia(
        ctx,
        current_symbol.trailing_trivia().collect(),
        FormatTokenType::TrailingTrivia,
        shape,
    );

    // Add on any whitespace created in the new symbol
    // The wanted leading trivia should be added to the end of formatted_leading_trivia
    // whilst the wanted trailing trivia should be added to the start of formatted_trailing_trivia
    // so that the token is "wrapped" around
    let mut wanted_leading_trivia: Vec<Token> = wanted_symbol
        .leading_trivia()
        .map(|x| x.to_owned())
        .collect();
    let mut wanted_trailing_trivia: Vec<Token> = wanted_symbol
        .trailing_trivia()
        .map(|x| x.to_owned())
        .collect();
    wanted_trailing_trivia.append(&mut formatted_trailing_trivia);
    formatted_leading_trivia.append(&mut wanted_leading_trivia);

    TokenReference::new(
        formatted_leading_trivia,
        wanted_symbol.token().to_owned(),
        wanted_trailing_trivia,
    )
}

/// Formats a token present at the end of an indented block, such as the `end` token or closing brace in a multiline table.
/// This is required due to leading comments bound to the last token - they need to have one level higher indentation
pub fn format_end_token(
    ctx: &Context,
    current_token: &TokenReference,
    _token_type: EndTokenType,
    shape: Shape,
) -> TokenReference {
    // Indent any comments leading a token, as these comments are technically part of the function body block
    let formatted_leading_trivia: Vec<Token> = load_token_trivia(
        ctx,
        current_token.leading_trivia().collect(),
        FormatTokenType::LeadingTrivia,
        // The indent level we are currently at is one less (as we are at the block closing token, not the indented block).
        // The comment is present inside the indented block
        shape.increment_additional_indent(),
    );
    let formatted_trailing_trivia: Vec<Token> = load_token_trivia(
        ctx,
        current_token.trailing_trivia().collect(),
        FormatTokenType::TrailingTrivia,
        shape,
    );

    // Special case for block end tokens:
    // We will reverse the leading trivia, and keep removing any newlines we find, until we find something else, then we stop.
    // This is to remove unnecessary newlines at the end of the block.
    let mut iter = formatted_leading_trivia.iter().rev().peekable();

    let mut formatted_leading_trivia = Vec::new();
    let mut stop_removal = false;
    while let Some(x) = iter.next() {
        match x.token_type() {
            TokenType::Whitespace { ref characters } => {
                if !stop_removal
                    && characters.contains('\n')
                    && !matches!(
                        iter.peek().map(|x| x.token_kind()),
                        Some(TokenKind::SingleLineComment) | Some(TokenKind::MultiLineComment)
                    )
                {
                    continue;
                } else {
                    formatted_leading_trivia.push(x.to_owned());
                }
            }
            _ => {
                formatted_leading_trivia.push(x.to_owned());
                stop_removal = true; // Stop removing newlines once we have seen some sort of comment
            }
        }
    }

    // Need to reverse the vector since we reversed the iterator
    formatted_leading_trivia.reverse();

    TokenReference::new(
        formatted_leading_trivia,
        Token::new(current_token.token_type().to_owned()),
        formatted_trailing_trivia,
    )
}

/// Continues mutating a Vec of Tokens until there is no more trailing whitespace present
fn pop_until_no_whitespace(trivia: &mut Vec<Token>) {
    if let Some(t) = trivia.pop() {
        match t.token_kind() {
            TokenKind::Whitespace => pop_until_no_whitespace(trivia), // Keep popping until no more whitespace
            _ => trivia.push(t), // Its not whitespace, so add it back and stop popping
        }
    }
}

/// Format the EOF token.
/// This is done by removing any leading whitespace, whilst preserving leading comments.
/// An EOF token has no trailing trivia
pub fn format_eof(ctx: &Context, eof: &TokenReference, shape: Shape) -> TokenReference {
    check_should_format!(ctx, eof);

    // Need to preserve any comments in leading_trivia if present
    let mut formatted_leading_trivia: Vec<Token> = load_token_trivia(
        ctx,
        eof.leading_trivia().collect(),
        FormatTokenType::LeadingTrivia,
        shape,
    );

    let only_whitespace = formatted_leading_trivia
        .iter()
        .all(|x| x.token_kind() == TokenKind::Whitespace);
    if only_whitespace {
        // Remove all the whitespace, and return an empty EOF
        TokenReference::new(Vec::new(), Token::new(TokenType::Eof), Vec::new())
    } else {
        // We have some comments in here, so we need to remove any trailing whitespace then add a single new line
        pop_until_no_whitespace(&mut formatted_leading_trivia);
        formatted_leading_trivia.push(create_newline_trivia(ctx));

        TokenReference::new(
            formatted_leading_trivia,
            Token::new(TokenType::Eof),
            Vec::new(),
        )
    }
}
