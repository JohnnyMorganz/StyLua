use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Block, FunctionBody,
};
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenKind, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;

pub mod assignment_formatter;
pub mod block_formatter;
pub mod expression_formatter;
pub mod functions_formatter;
pub mod table_formatter;
pub mod trivia_formatter;

#[derive(Debug)]
pub enum IndentType {
    Tabs,
    Spaces,
}

impl Default for IndentType {
    fn default() -> Self {
        IndentType::Tabs
    }
}

#[derive(Debug)]
pub enum LineEndings {
    // Auto,
    Unix,
    Windows,
}

impl Default for LineEndings {
    fn default() -> Self {
        LineEndings::Unix
    }
}

#[derive(Default, Debug)]
pub struct Config {
    line_endings: LineEndings,
    indent_type: IndentType,
}

#[derive(Default)]
pub struct CodeFormatter {
    indent_level: usize,
    config: Config,
}

enum FormatTokenType {
    Token,
    LeadingTrivia,
    TrailingTrivia,
}

fn get_line_ending_character(line_endings: &LineEndings) -> String {
    match line_endings {
        LineEndings::Unix => String::from("\n"),
        LineEndings::Windows => String::from("\r\n"),
    }
}

impl CodeFormatter {
    /// Creates a new CodeFormatter, with the given configuration
    pub fn new(config: Config) -> Self {
        CodeFormatter {
            indent_level: 0,
            config,
        }
    }

    /// Creates a new Token containing whitespace for indents, used for trivia
    pub fn create_indent_trivia<'ast>(&self, indent_level: Option<usize>) -> Token<'ast> {
        // self.indent_level starts at 1
        let indent_level = match indent_level {
            Some(level) => level,
            None => self.indent_level - 1,
        };

        match self.config.indent_type {
            IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
            IndentType::Spaces => Token::new(TokenType::spaces(indent_level)),
        }
    }

    /// Creates a new Token containing new line whitespace, used for trivia
    pub fn create_newline_trivia<'ast>(&self) -> Token<'ast> {
        Token::new(TokenType::Whitespace {
            characters: Cow::Owned(get_line_ending_character(&self.config.line_endings)),
        })
    }

    /// Formats a Token Node
    /// This should only ever be called from format_token_reference
    fn format_token<'ast>(&self, token: Token<'ast>, format_type: FormatTokenType) -> Token<'ast> {
        let token_type = match token.token_type() {
            TokenType::StringLiteral {
                literal,
                multi_line,
                quote_type: _,
            } => TokenType::StringLiteral {
                literal: literal.to_owned(),
                multi_line: match multi_line {
                    Some(size) => Some(*size),
                    None => None,
                },
                quote_type: StringLiteralQuoteType::Double,
            },
            TokenType::SingleLineComment { comment } => {
                let mut new_str = comment.to_owned().into_owned();
                match format_type {
                    FormatTokenType::LeadingTrivia => {
                        // TODO: We need to add trivia to indent the comment, how can we pass this along?
                        // Add a newline character
                        new_str.push_str(&get_line_ending_character(&self.config.line_endings));
                    }
                    FormatTokenType::TrailingTrivia => {
                        // TODO: We need to add a single space trivia to separate the comment and the code, how can we pass this along?
                    }
                    _ => (),
                }
                TokenType::SingleLineComment {
                    comment: Cow::Owned(new_str),
                }
            }
            TokenType::MultiLineComment { blocks, comment } => {
                // TODO: This is copied from above, can we abstract it out?
                let mut new_str = comment.to_owned().into_owned();
                match format_type {
                    FormatTokenType::LeadingTrivia => {
                        // TODO: We need to add trivia to indent the comment, how can we pass this along?
                        // Add a newline character
                        new_str.push_str(&get_line_ending_character(&self.config.line_endings));
                    }
                    FormatTokenType::TrailingTrivia => {
                        // TODO: We need to add a single space trivia to separate the comment and the code, how can we pass this along?
                    }
                    _ => (),
                }
                TokenType::MultiLineComment {
                    blocks: *blocks,
                    comment: Cow::Owned(new_str),
                }
            }
            TokenType::Whitespace { characters } => TokenType::Whitespace {
                characters: characters.to_owned(),
            }, // TODO
            _ => token.token_type().to_owned(),
        };

        Token::new(token_type)
    }

    fn format_plain_token_reference<'a>(
        &self,
        token_reference: TokenReference<'a>,
    ) -> TokenReference<'a> {
        // Preserve comments in leading/trailing trivia
        let formatted_leading_trivia: Vec<Token<'a>> = token_reference
            .leading_trivia()
            .filter(|trivia| trivia.token_kind() != TokenKind::Whitespace)
            .map(|x| self.format_token(x.to_owned(), FormatTokenType::LeadingTrivia))
            .collect();
        let formatted_trailing_trivia: Vec<Token<'a>> = token_reference
            .trailing_trivia()
            .filter(|trivia| trivia.token_kind() != TokenKind::Whitespace)
            .map(|x| self.format_token(x.to_owned(), FormatTokenType::TrailingTrivia))
            .collect();

        TokenReference::new(
            formatted_leading_trivia,
            self.format_token(token_reference.token().to_owned(), FormatTokenType::Token),
            formatted_trailing_trivia,
        )
    }

    pub fn format_token_reference<'a>(
        &self,
        token_reference: Cow<'a, TokenReference<'a>>,
    ) -> Cow<'a, TokenReference<'a>> {
        Cow::Owned(self.format_plain_token_reference(token_reference.into_owned()))
    }

    pub fn format_punctuation<'ast>(
        &self,
        punctuation: Cow<'ast, TokenReference<'ast>>,
    ) -> Cow<'ast, TokenReference<'ast>> {
        Cow::Owned(TokenReference::new(
            Vec::new(),
            punctuation.token().to_owned(),
            vec![Token::new(TokenType::spaces(1))], // Single space whitespace
        ))
    }

    pub fn format_punctuated<'a, T>(
        &self,
        old: Punctuated<'a, T>,
        value_formatter: &dyn Fn(&Self, T) -> T,
        // wanted_trailing_trivia: Vec<Token<'a>>,
    ) -> Punctuated<'a, T> {
        let mut formatted: Punctuated<T> = Punctuated::new();
        for pair in old.into_pairs() {
            // Format Punctuation
            match pair {
                Pair::Punctuated(value, punctuation) => {
                    let formatted_punctuation = self.format_punctuation(punctuation);
                    let formatted_value = value_formatter(self, value);
                    formatted.push(Pair::new(formatted_value, Some(formatted_punctuation)));
                }
                Pair::End(value) => {
                    let formatted_value = value_formatter(self, value);
                    formatted.push(Pair::new(formatted_value, None));
                }
            }
        }

        formatted
    }

    pub fn format_contained_span<'ast>(
        &self,
        contained_span: ContainedSpan<'ast>,
    ) -> ContainedSpan<'ast> {
        let (start_token, end_token) = contained_span.tokens();

        ContainedSpan::new(
            Cow::Owned(self.format_plain_token_reference(start_token.to_owned())),
            Cow::Owned(self.format_plain_token_reference(end_token.to_owned())),
        )
    }

    /// Formats a special TokenReference which is a symbol
    /// Used to preserve the comments around the symbol
    pub fn format_symbol<'ast>(
        &self,
        current_symbol: TokenReference<'ast>,
        wanted_symbol: TokenReference<'ast>,
    ) -> Cow<'ast, TokenReference<'ast>> {
        // TODO: This is copied from format_token_reference, can we simplify this?
        // Preserve comments in leading/trailing trivia
        let mut formatted_leading_trivia: Vec<Token<'ast>> = current_symbol
            .leading_trivia()
            .filter(|trivia| trivia.token_kind() != TokenKind::Whitespace)
            .map(|x| self.format_token(x.to_owned(), FormatTokenType::LeadingTrivia))
            .collect();
        let mut formatted_trailing_trivia: Vec<Token<'ast>> = current_symbol
            .trailing_trivia()
            .filter(|trivia| trivia.token_kind() != TokenKind::Whitespace)
            .map(|x| self.format_token(x.to_owned(), FormatTokenType::TrailingTrivia))
            .collect();

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

        Cow::Owned(TokenReference::new(
            formatted_leading_trivia,
            wanted_symbol.token().to_owned(),
            wanted_trailing_trivia,
        ))
    }
}

impl<'ast> VisitorMut<'ast> for CodeFormatter {
    fn visit_block(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level += 1;
        block_formatter::format_block(&self, node)
    }

    fn visit_block_end(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level -= 1;
        node
    }

    // Special case where trivia needs to be added, and it isn't handled elsewhere
    fn visit_function_body_end(&mut self, function_body: FunctionBody<'ast>) -> FunctionBody<'ast> {
        let parameters_parentheses = trivia_formatter::contained_span_add_trivia(
            function_body.parameters_parentheses().to_owned(),
            None,
            Some(vec![self.create_newline_trivia()]),
        );
        let end_token = Cow::Owned(trivia_formatter::token_reference_add_trivia(
            function_body.end_token().to_owned(),
            Some(vec![self.create_indent_trivia(None)]),
            None,
        ));

        function_body
            .with_parameters_parentheses(parameters_parentheses)
            .with_end_token(end_token)
    }
}
