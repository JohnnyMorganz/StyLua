use crate::{shape::Shape, CallParenType, Config, IndentType, LineEndings, Range as FormatRange};
use full_moon::{
    node::Node,
    tokenizer::{Token, TokenType},
};

#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The configuration passed to the formatter
    config: Config,
    /// An optional range of values to format within the file.
    range: Option<FormatRange>,
    /// Whether the formatting has currently been disabled. This should occur when we see the relevant comment.
    formatting_disabled: bool,
}

impl Context {
    /// Creates a new Context, with the given configuration
    pub fn new(config: Config, range: Option<FormatRange>) -> Self {
        Self {
            config,
            range,
            formatting_disabled: false,
        }
    }

    /// Get the configuration for this context
    pub fn config(&self) -> Config {
        self.config
    }

    /// Determines whether we need to toggle whether formatting is enabled or disabled.
    /// Formatting is toggled on/off whenever we see a `-- stylua: ignore start` or `-- stylua: ignore end` comment respectively.
    // To preserve immutability of Context, we return a new Context with the `formatting_disabled` field toggled or left the same
    // where necessary. Context is cheap so this is reasonable to do.
    pub fn check_toggle_formatting(&self, node: &impl Node) -> Self {
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
                if line == "stylua: ignore start" && !self.formatting_disabled {
                    return Self {
                        formatting_disabled: true,
                        ..*self
                    };
                } else if line == "stylua: ignore end" && self.formatting_disabled {
                    return Self {
                        formatting_disabled: false,
                        ..*self
                    };
                }
            }
        }

        *self
    }

    /// Checks whether we should format the given node.
    /// Firstly determine if formatting is disabled (due to the relevant comment)
    /// If not, determine whether the node has an ignore comment present.
    /// If not, checks whether the provided node is outside the formatting range.
    /// If not, the node should be formatted.
    pub fn should_format_node(&self, node: &impl Node) -> bool {
        // If formatting is disabled we should immediately bailed out.
        if self.formatting_disabled {
            return false;
        }

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

    pub fn should_omit_string_parens(&self) -> bool {
        self.config().no_call_parentheses
            || self.config().call_parentheses == CallParenType::None
            || self.config().call_parentheses == CallParenType::NoString
    }

    pub fn should_omit_table_parens(&self) -> bool {
        self.config().no_call_parentheses
            || self.config().call_parentheses == CallParenType::None
            || self.config().call_parentheses == CallParenType::NoTable
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
pub fn create_indent_trivia(ctx: &Context, shape: Shape) -> Token {
    let indent_level = shape.indent().block_indent() + shape.indent().additional_indent();
    create_plain_indent_trivia(ctx, indent_level)
}

/// Creates indent trivia without including `ctx.indent_level()`.
/// You should pass the exact amount of indent you require to this function
pub fn create_plain_indent_trivia(ctx: &Context, indent_level: usize) -> Token {
    match ctx.config().indent_type {
        IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
        IndentType::Spaces => {
            Token::new(TokenType::spaces(indent_level * ctx.config().indent_width))
        }
    }
}

/// Creates a new Token containing new line whitespace, used for trivia
pub fn create_newline_trivia(ctx: &Context) -> Token {
    Token::new(TokenType::Whitespace {
        characters: line_ending_character(ctx.config().line_endings).into(),
    })
}
