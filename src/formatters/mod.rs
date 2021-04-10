use crate::{Config, IndentType, LineEndings, QuoteStyle};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Block,
};
use full_moon::node::Node;
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenKind, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;
use std::collections::HashSet;

pub mod assignment_formatter;
pub mod block_formatter;
#[macro_use]
pub mod expression_formatter;
pub mod functions_formatter;
#[cfg(feature = "lua52")]
pub mod lua52_formatter;
#[cfg(feature = "luau")]
pub mod luau_formatter;
pub mod stmt_formatter;
pub mod table_formatter;
pub mod trivia_formatter;
pub mod trivia_util;

/// A Range, from a Start Position to an End Position
pub type Range = (usize, usize);

#[derive(Default)]
pub struct CodeFormatter {
    /// The configuration passed to the formatter
    config: Config,
    /// An optional range of values to format within the file.
    range: Option<crate::Range>,
    /// The current indent level
    indent_level: usize,
    /// A link of specific ranges to indent increases. The indent increases are added ontop of indent_level
    indent_ranges: HashSet<Range>,
}

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
}

/// Returns the relevant line ending string from the [`LineEndings`] enum
fn get_line_ending_character(line_endings: &LineEndings) -> String {
    match line_endings {
        LineEndings::Unix => String::from("\n"),
        LineEndings::Windows => String::from("\r\n"),
    }
}

#[macro_export]
macro_rules! fmt_symbol {
    ($fmter:expr, $token:expr, $x:expr) => {
        $fmter.format_symbol($token, &TokenReference::symbol($x).unwrap())
    };
}

#[macro_export]
macro_rules! check_should_format {
    ($fmter:expr, $token:expr) => {
        if !$fmter.should_format_node($token) {
            return $token.to_owned();
        }
    };
}

impl CodeFormatter {
    /// Creates a new CodeFormatter, with the given configuration
    pub fn new(config: Config, range: Option<crate::Range>) -> Self {
        CodeFormatter {
            indent_level: 0,
            config,
            range,
            indent_ranges: HashSet::new(),
        }
    }

    /// Checks whether we should format the given node.
    /// Firstly determines whether the node has an ignore comment present.
    /// If not, checks whether the provided node is within the formatting range.
    /// If not, the node should not be formatted.
    pub fn should_format_node<'ast>(&self, node: &impl Node<'ast>) -> bool {
        // Check comments
        let leading_trivia = node.surrounding_trivia().0;
        for trivia in leading_trivia {
            let comment_lines = match trivia.token_type() {
                TokenType::SingleLineComment { comment } => comment,
                TokenType::MultiLineComment { comment, .. } => comment,
                _ => continue,
            }
            .lines()
            .map(|line| line.trim());

            for line in comment_lines {
                if line == "stylua: ignore" {
                    return false;
                }
            }
        }

        if let Some(range) = self.range {
            let mut in_range = true;

            if let Some(start_bound) = range.start {
                if let Some(node_start) = node.start_position() {
                    if node_start.bytes() < start_bound {
                        in_range = false;
                    }
                }
            }

            if let Some(end_bound) = range.end {
                if let Some(node_end) = node.end_position() {
                    if node_end.bytes() > end_bound {
                        in_range = false;
                    }
                }
            }

            in_range
        } else {
            // No range provided, therefore always in formatting range
            true
        }
    }

    /// Increase the level of indention at the current position of the formatter
    pub fn increment_indent_level(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease the level of indentation at the current position of the formatter
    pub fn decrement_indent_level(&mut self) {
        self.indent_level -= 1;
    }

    /// Returns the size of the current indent level in characters
    pub fn get_indent_width(&self) -> usize {
        (self.indent_level - 1) * self.config.indent_width
    }

    /// Adds a Position Range of locations where indents should be increased on top of the current indent level.
    /// This is used mainly within tables, where the values may be an anonymous function but the indent level not being
    /// high enough
    pub fn add_indent_range(&mut self, range: Range) {
        self.indent_ranges.insert(range);
    }

    /// Determines the amount of increase in indentation for the current range
    /// This is used in conjunction with `add_indent_range` to see if we need to increase the indentation at a
    /// given location
    pub fn get_range_indent_increase(&self, range: Range) -> Option<usize> {
        // TODO: Do we need to pass a "Range" parameter here? Can it just be a single value?
        let indent_increase = self
            .indent_ranges
            .iter()
            .filter(|x| range.0 >= x.0 && range.1 <= x.1);
        let count = indent_increase.count();
        if count > 0 {
            Some(count)
        } else {
            None
        }
    }

    /// Creates a new Token containing whitespace for indents, used for trivia
    pub fn create_indent_trivia<'ast>(
        &self,
        additional_indent_level: Option<usize>,
    ) -> Token<'ast> {
        // self.indent_level starts at 1
        let indent_level = match additional_indent_level {
            Some(level) => self.indent_level - 1 + level,
            None => self.indent_level - 1,
        };

        match self.config.indent_type {
            IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
            IndentType::Spaces => {
                Token::new(TokenType::spaces(indent_level * self.config.indent_width))
            }
        }
    }

    /// Creates a new Token containing new line whitespace, used for trivia
    pub fn create_newline_trivia<'ast>(&self) -> Token<'ast> {
        Token::new(TokenType::Whitespace {
            characters: Cow::Owned(get_line_ending_character(&self.config.line_endings)),
        })
    }

    fn format_single_line_comment_string(&self, comment: String) -> String {
        // Trim any trailing whitespace
        comment.trim_end().to_string()
    }

    fn get_quote_to_use(&self, literal: &str) -> StringLiteralQuoteType {
        match self.config.quote_style {
            QuoteStyle::ForceDouble => StringLiteralQuoteType::Double,
            QuoteStyle::ForceSingle => StringLiteralQuoteType::Single,
            _ => {
                let preferred = match self.config.quote_style {
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

    /// Formats a Token Node
    /// Also returns any extra leading or trailing trivia to add for the Token node
    /// This should only ever be called from format_token_reference
    fn format_token<'ast>(
        &self,
        token: &Token<'ast>,
        format_type: &FormatTokenType,
        additional_indent_level: Option<usize>,
    ) -> (
        Token<'ast>,
        Option<Vec<Token<'ast>>>,
        Option<Vec<Token<'ast>>>,
    ) {
        let mut leading_trivia: Option<Vec<Token<'ast>>> = None;
        let mut trailing_trivia: Option<Vec<Token<'ast>>> = None;

        let token_type = match token.token_type() {
            TokenType::Number { text } => TokenType::Number {
                text: Cow::Owned(if text.starts_with('.') {
                    String::from("0")
                        + match text {
                            Cow::Owned(text) => text.as_str(),
                            Cow::Borrowed(text) => text,
                        }
                } else if text.starts_with("-.") {
                    String::from("-0") + text.get(1..).expect("unknown number literal")
                } else {
                    text.to_owned().into_owned()
                }),
            },
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
                        static ref UNNECESSARY_ESCAPES: regex::Regex = regex::Regex::new(r#"^[^\n\r"'0-7\\abfnrtuvxz]$"#).unwrap();
                    }
                    let quote_to_use = self.get_quote_to_use(literal);
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
                                        .expect(
                                            "have a match which was neither an escape or a quote",
                                        )
                                        .as_str();
                                    if UNNECESSARY_ESCAPES.is_match(text) {
                                        text.to_owned()
                                    } else {
                                        format!("\\{}", text.to_owned())
                                    }
                                }
                            }
                        })
                        .into_owned();
                    TokenType::StringLiteral {
                        literal: Cow::Owned(literal),
                        multi_line: None,
                        quote_type: quote_to_use,
                    }
                }
            }
            TokenType::SingleLineComment { comment } => {
                let comment =
                    self.format_single_line_comment_string(comment.to_owned().into_owned());

                match format_type {
                    FormatTokenType::LeadingTrivia => {
                        let additional_indent_level = additional_indent_level.unwrap_or(0)
                            + self
                                .get_range_indent_increase((
                                    token.start_position().bytes(),
                                    token.end_position().bytes(),
                                ))
                                .unwrap_or(0);
                        leading_trivia = Some(vec![
                            self.create_indent_trivia(Some(additional_indent_level))
                        ]);
                        trailing_trivia = Some(vec![self.create_newline_trivia()]);
                    }
                    FormatTokenType::TrailingTrivia => {
                        // Add a space before the comment
                        leading_trivia = Some(vec![Token::new(TokenType::spaces(1))]);
                    }
                    _ => (),
                }

                TokenType::SingleLineComment {
                    comment: Cow::Owned(comment),
                }
            }
            TokenType::MultiLineComment { blocks, comment } => {
                // let comment =
                //     self.format_multi_line_comment_string(comment.to_owned().into_owned());

                if let FormatTokenType::LeadingTrivia = format_type {
                    let additional_indent_level = additional_indent_level.unwrap_or(0)
                        + self
                            .get_range_indent_increase((
                                token.start_position().bytes(),
                                token.end_position().bytes(),
                            ))
                            .unwrap_or(0);
                    leading_trivia = Some(vec![
                        self.create_indent_trivia(Some(additional_indent_level))
                    ]);
                    // Add a new line once the comment is completed
                    trailing_trivia = Some(vec![self.create_newline_trivia()]);
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
    fn load_token_trivia<'ast>(
        &self,
        current_trivia: Vec<&Token<'ast>>,
        format_token_type: FormatTokenType,
        additional_indent_level: Option<usize>,
    ) -> Vec<Token<'ast>> {
        let mut token_trivia = Vec::new();

        let mut newline_count_in_succession = 0;
        let mut trivia_iter = current_trivia.iter().peekable();

        while let Some(trivia) = trivia_iter.next() {
            // Handle cases where the user has left a newline gap in between e.g. two statements
            // If we are formatting trailing trivia, this can be ignored, as all trailing newlines will have already
            // been handled by the formatter.
            // If we are formatting leading trivia, we will allow a single newline to be kept in succession, if we
            // find one.
            match trivia.token_type() {
                TokenType::Whitespace { characters } => {
                    if let FormatTokenType::LeadingTrivia = format_token_type {
                        if characters.contains('\n') {
                            newline_count_in_succession += 1;
                            if newline_count_in_succession == 1 {
                                // We have a case where we will allow a single newline to be kept
                                token_trivia.push(self.create_newline_trivia());
                            }
                        }
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

            let (token, leading_trivia, trailing_trivia) = self.format_token(
                trivia.to_owned(),
                &format_token_type,
                additional_indent_level,
            );
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

    fn format_token_reference<'a>(
        &self,
        token_reference: &TokenReference<'a>,
    ) -> TokenReference<'a> {
        // Preserve comments in leading/trailing trivia
        let formatted_leading_trivia: Vec<Token<'a>> = self.load_token_trivia(
            token_reference.leading_trivia().collect(),
            FormatTokenType::LeadingTrivia,
            None,
        );
        let formatted_trailing_trivia: Vec<Token<'a>> = self.load_token_trivia(
            token_reference.trailing_trivia().collect(),
            FormatTokenType::TrailingTrivia,
            None,
        );

        let (token, _leading_trivia, _trailing_trivia) =
            self.format_token(token_reference.token(), &FormatTokenType::Token, None);
        // TODO: is it possible for leading/trailing trivia to be present here?
        // if let Some(trivia) = leading_trivia {
        //     formatted_leading_trivia.append(trivia);
        // }

        // if let Some(trivia) = trailing_trivia {
        //     formatted_leading_trivia.append(trivia);
        // }

        TokenReference::new(formatted_leading_trivia, token, formatted_trailing_trivia)
    }

    pub fn format_token_reference_mut<'ast>(
        &mut self,
        token_reference: &TokenReference<'ast>,
    ) -> TokenReference<'ast> {
        self.format_token_reference(token_reference)
    }

    // Formats a punctuation for a Punctuated sequence
    // Removes any trailing comments to be stored in a comments buffer
    pub fn format_punctuation<'ast>(
        &self,
        punctuation: &TokenReference<'ast>,
    ) -> (TokenReference<'ast>, Vec<Token<'ast>>) {
        let trailing_comments = punctuation
            .trailing_trivia()
            .filter(|token| {
                token.token_kind() == TokenKind::SingleLineComment
                    || token.token_kind() == TokenKind::MultiLineComment
            })
            .map(|x| {
                // Prepend a single space beforehand
                vec![Token::new(TokenType::spaces(1)), x.to_owned()]
            })
            .flatten()
            .collect();

        (
            TokenReference::new(
                Vec::new(),
                punctuation.token().to_owned(),
                vec![Token::new(TokenType::spaces(1))], // Single space whitespace
            ),
            trailing_comments,
        )
    }

    // Formats a Punctuated sequence with correct punctuated values
    // If there are any comments in between tied to the punctuation, they will be removed and stored in a returned comments buffer
    pub fn format_punctuated<'a, T>(
        &mut self,
        old: &Punctuated<'a, T>,
        value_formatter: &dyn Fn(&mut Self, &T) -> T,
    ) -> (Punctuated<'a, T>, Vec<Token<'a>>) {
        let mut formatted: Punctuated<T> = Punctuated::new();
        let mut comments_buffer = Vec::new();

        for pair in old.pairs() {
            match pair {
                Pair::Punctuated(value, punctuation) => {
                    // Format punctuation and store any comments into buffer
                    let (formatted_punctuation, mut comments) =
                        self.format_punctuation(punctuation);
                    comments_buffer.append(&mut comments);

                    let formatted_value = value_formatter(self, value);

                    formatted.push(Pair::new(formatted_value, Some(formatted_punctuation)));
                }
                Pair::End(value) => {
                    let formatted_value = value_formatter(self, value);
                    formatted.push(Pair::new(formatted_value, None));
                }
            }
        }

        (formatted, comments_buffer)
    }

    pub fn format_contained_span<'ast>(
        &self,
        contained_span: &ContainedSpan<'ast>,
    ) -> ContainedSpan<'ast> {
        let (start_token, end_token) = contained_span.tokens();

        ContainedSpan::new(
            self.format_token_reference(start_token),
            self.format_token_reference(end_token),
        )
    }

    /// Formats a special TokenReference which is a symbol
    /// Used to preserve the comments around the symbol
    pub fn format_symbol<'ast>(
        &self,
        current_symbol: &TokenReference<'ast>,
        wanted_symbol: &TokenReference<'ast>,
    ) -> TokenReference<'ast> {
        // Preserve comments in leading/trailing trivia
        let mut formatted_leading_trivia: Vec<Token<'ast>> = self.load_token_trivia(
            current_symbol.leading_trivia().collect(),
            FormatTokenType::LeadingTrivia,
            None,
        );
        let mut formatted_trailing_trivia: Vec<Token<'ast>> = self.load_token_trivia(
            current_symbol.trailing_trivia().collect(),
            FormatTokenType::TrailingTrivia,
            None,
        );

        // Add on any whitespace created in the new symbol
        // The wanted leading trivia should be added to the end of formatted_leading_trivia
        // whilst the wanted trailing trivia should be added to the start of formatted_trailing_trivia
        // so that the token is "wrapped" around
        let mut wanted_leading_trivia: Vec<Token<'ast>> = wanted_symbol
            .leading_trivia()
            .map(|x| x.to_owned())
            .collect();
        let mut wanted_trailing_trivia: Vec<Token<'ast>> = wanted_symbol
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
    pub fn format_end_token<'ast>(
        &self,
        current_token: &TokenReference<'ast>,
        token_type: EndTokenType,
    ) -> TokenReference<'ast> {
        // Indent any comments leading a token, as these comments are technically part of the function body block
        let formatted_leading_trivia: Vec<Token<'ast>> = self.load_token_trivia(
            current_token.leading_trivia().collect(),
            crate::formatters::FormatTokenType::LeadingTrivia,
            match token_type {
                // The indent level we are currently at is one less (as we are at the end token, not the indented block).
                // The comment is present inside the indented block
                EndTokenType::BlockEnd => Some(1),
                // We are closing an "indent range". The leading comments will still be inside this range, so we don't need an extra indent level
                EndTokenType::ClosingBrace => None,
            },
        );
        let formatted_trailing_trivia: Vec<Token<'ast>> = self.load_token_trivia(
            current_token.trailing_trivia().collect(),
            crate::formatters::FormatTokenType::TrailingTrivia,
            None,
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

impl<'ast> VisitorMut<'ast> for CodeFormatter {
    fn visit_block(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.increment_indent_level();
        self.format_block(node)
    }

    fn visit_block_end(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.decrement_indent_level();
        node
    }

    // Remove any extra whitespace at the end of the file
    fn visit_eof(&mut self, node: TokenReference<'ast>) -> TokenReference<'ast> {
        check_should_format!(self, &node);

        // Need to preserve any comments in leading_trivia if present
        // The indent level will be 0 at this point, as we have finished the whole file, so we need to one-index it again
        self.indent_level += 1;
        let mut formatted_leading_trivia: Vec<Token<'ast>> = self.load_token_trivia(
            node.leading_trivia().collect(),
            FormatTokenType::LeadingTrivia,
            None,
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
            formatted_leading_trivia.push(self.create_newline_trivia());

            TokenReference::new(
                formatted_leading_trivia,
                Token::new(TokenType::Eof),
                Vec::new(),
            )
        }
    }
}
