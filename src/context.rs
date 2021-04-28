use crate::{Config, IndentType, LineEndings, Range as FormatRange};
use full_moon::{
    node::Node,
    tokenizer::{Token, TokenType},
};
use std::borrow::Cow;
use std::collections::HashSet;

/// A Range, from a Start Position to an End Position
pub type Range = (usize, usize);

pub struct Context {
    /// The configuration passed to the formatter
    config: Config,
    /// An optional range of values to format within the file.
    range: Option<FormatRange>,
    /// The current indent level
    indent_level: usize,
    /// A link of specific ranges to indent increases. The indent increases are added ontop of indent_level
    indent_ranges: HashSet<Range>,
}

impl Context {
    /// Creates a new Context, with the given configuration
    pub fn new(config: Config, range: Option<FormatRange>) -> Self {
        Self {
            indent_level: 0,
            config,
            range,
            indent_ranges: HashSet::new(),
        }
    }

    /// Get the configuration for this context
    pub fn config(&self) -> Config {
        self.config
    }

    /// Get the current indent level
    pub fn indent_level(&self) -> usize {
        self.indent_level
    }

    /// Returns the size of the current indent level in characters
    pub fn indent_width(&self) -> usize {
        (self.indent_level() - 1) * self.config().indent_width
    }

    /// Returns the size of the current indent level in characters, including any additional indent level
    pub fn indent_width_additional(&self, additional_indent_level: Option<usize>) -> usize {
        (self.indent_level() - 1 + additional_indent_level.unwrap_or(0))
            * self.config().indent_width
    }

    /// Increase the level of indention at the current position of the formatter
    pub fn increment_indent_level(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease the level of indentation at the current position of the formatter
    pub fn decrement_indent_level(&mut self) {
        self.indent_level -= 1;
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
}

#[macro_export]
macro_rules! check_should_format {
    ($ctx:expr, $token:expr) => {
        if !$ctx.should_format_node($token) {
            return $token.to_owned();
        }
    };
}

/// Returns the relevant line ending string from the [`LineEndings`] enum
fn line_ending_character(line_endings: LineEndings) -> String {
    match line_endings {
        LineEndings::Unix => String::from("\n"),
        LineEndings::Windows => String::from("\r\n"),
    }
}

/// Creates a new Token containing whitespace for indents, used for trivia
pub fn create_indent_trivia<'ast>(
    ctx: &Context,
    additional_indent_level: Option<usize>,
) -> Token<'ast> {
    // ctx.indent_level starts at 1
    let indent_level = match additional_indent_level {
        Some(level) => ctx.indent_level() - 1 + level,
        None => ctx.indent_level() - 1,
    };

    create_plain_indent_trivia(ctx, indent_level)
}

/// Creates indent trivia without including `ctx.indent_level()`.
/// You should pass the exact amount of indent you require to this function
pub fn create_plain_indent_trivia<'ast>(ctx: &Context, indent_level: usize) -> Token<'ast> {
    match ctx.config().indent_type {
        IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
        IndentType::Spaces => {
            Token::new(TokenType::spaces(indent_level * ctx.config().indent_width))
        }
    }
}

/// Creates a new Token containing new line whitespace, used for trivia
pub fn create_newline_trivia<'ast>(ctx: &Context) -> Token<'ast> {
    Token::new(TokenType::Whitespace {
        characters: Cow::Owned(line_ending_character(ctx.config().line_endings)),
    })
}
