use crate::context::Context;
use crate::formatters::{
    trivia::{FormatTriviaType, UpdateTrivia},
    trivia_util::trivia_is_comment,
};
use full_moon::node::Node;
use std::fmt::Display;
use std::ops::Add;

/// A struct representing indentation level of the current code
#[derive(Clone, Copy, Debug)]
pub struct Indent {
    /// How many characters a single indent level represents. This is inferred from the configuration
    indent_width: usize,
    /// The current block indentation level. The base indentation level is 0. Note: this is not the indentation width
    block_indent: usize,
    /// Any additional indent level that we are in, excluding the block indent. For example, within a multiline table.
    additional_indent: usize,
}

impl Indent {
    /// Creates a new indentation at the base indent level, inferring indent_width from context.
    pub fn new(ctx: &Context) -> Self {
        Self {
            block_indent: 0,
            additional_indent: 0,
            indent_width: ctx.config().indent_width,
        }
    }

    /// The current block indentation level
    pub fn block_indent(&self) -> usize {
        self.block_indent
    }

    /// The current additional indentation level
    pub fn additional_indent(&self) -> usize {
        self.additional_indent
    }

    /// The configured width of a single indent
    pub fn configured_indent_width(&self) -> usize {
        self.indent_width
    }

    /// The current width (characters) taken up by indentation
    pub fn indent_width(&self) -> usize {
        (self.block_indent + self.additional_indent) * self.indent_width
    }

    /// Recreates an Indent struct with the given additional indent level
    pub fn with_additional_indent(&self, additional_indent: usize) -> Self {
        Self {
            additional_indent,
            ..*self
        }
    }

    /// Increments the block indentation level by one
    pub fn increment_block_indent(&self) -> Self {
        Self {
            block_indent: self.block_indent.saturating_add(1),
            ..*self
        }
    }

    // Decrements the block indentation level by one
    // pub fn decrement_block_indent(&self) -> Self {
    //     Self {
    //         block_indent: self.block_indent.saturating_sub(1),
    //         ..*self
    //     }
    // }

    /// Increments the additional indentation level by one
    pub fn increment_additional_indent(&self) -> Self {
        Self {
            additional_indent: self.additional_indent.saturating_add(1),
            ..*self
        }
    }

    // Decrements the additional indentation level by one
    // pub fn decrement_additional_indent(&self) -> Self {
    //     Self {
    //         additional_indent: self.additional_indent.saturating_sub(1),
    //         ..*self
    //     }
    // }

    /// Increases the additional indentation level by amount specified
    pub fn add_indent_level(&self, amount: usize) -> Self {
        Self {
            additional_indent: self.additional_indent.saturating_add(amount),
            ..*self
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Shape {
    /// The current indentation level
    indent: Indent,
    /// The current width we have taken on the line, excluding any indentation.
    offset: usize,
    /// The maximum number of characters we want to fit on a line. This is inferred from the configuration
    column_width: usize,
    /// Whether we should use simple heuristic checking.
    /// This is enabled when we are calling within a heuristic itself, to reduce the exponential blowup
    simple_heuristics: bool,
}

impl Shape {
    /// Creates a new shape at the base indentation level
    #[must_use]
    pub fn new(ctx: &Context) -> Self {
        Self {
            indent: Indent::new(ctx),
            offset: 0,
            column_width: ctx.config().column_width,
            simple_heuristics: false,
        }
    }

    /// Sets the column width to the provided width. Normally only used to set an infinite width when testing layouts
    #[must_use]
    pub fn with_column_width(&self, column_width: usize) -> Self {
        Self {
            column_width,
            ..*self
        }
    }

    /// Recreates the shape with the provided indentation
    #[must_use]
    pub fn with_indent(&self, indent: Indent) -> Self {
        Self { indent, ..*self }
    }

    /// Recreates the shape with an infinite width. Useful when testing layouts and want to force code onto a single line
    #[must_use]
    pub fn with_infinite_width(&self) -> Self {
        self.with_column_width(usize::MAX)
    }

    /// The current indentation of the shape
    #[must_use]
    pub fn indent(&self) -> Indent {
        self.indent
    }

    /// Increments the block indentation level by one. Alias for `shape.with_indent(shape.indent().increment_block_indent())`
    #[must_use]
    pub fn increment_block_indent(&self) -> Self {
        Self {
            indent: self.indent.increment_block_indent(),
            ..*self
        }
    }

    /// Increments the additional indentation level by one. Alias for `shape.with_indent(shape.indent().increment_additional_indent())`
    #[must_use]
    pub fn increment_additional_indent(&self) -> Self {
        Self {
            indent: self.indent.increment_additional_indent(),
            ..*self
        }
    }

    /// The width currently taken up for this line
    #[must_use]
    pub fn used_width(&self) -> usize {
        self.indent.indent_width() + self.offset
    }

    /// Check to see whether our current width is above the budget available
    #[must_use]
    pub fn over_budget(&self) -> bool {
        self.used_width() > self.column_width
    }

    /// Adds a width offset to the current width total
    #[must_use]
    pub fn add_width(&self, width: usize) -> Shape {
        Self {
            offset: self.offset + width,
            ..*self
        }
    }

    /// Whether simple heuristics should be used when calculating formatting shape
    /// This is to reduce the expontential blowup of discarded test formatting
    #[must_use]
    pub fn using_simple_heuristics(&self) -> bool {
        self.simple_heuristics
    }

    #[must_use]
    pub fn with_simple_heuristics(&self) -> Shape {
        Self {
            simple_heuristics: true,
            ..*self
        }
    }

    /// Resets the offset for the shape
    #[must_use]
    pub fn reset(&self) -> Shape {
        Self { offset: 0, ..*self }
    }

    /// Takes the first line from an item which can be converted into a string, and sets that to the shape
    #[must_use]
    pub fn take_first_line<T: Display>(&self, item: &T) -> Shape {
        let string = format!("{item}");
        let mut lines = string.lines();
        let width = lines.next().unwrap_or("").len();
        self.add_width(width)
    }

    /// Takes an item which could possibly span multiple lines. If it spans multiple lines, the shape is reset
    /// and the last line is added to the width. If it only takes a single line, we just continue adding to the current
    /// width
    #[must_use]
    pub fn take_last_line<T: Display>(&self, item: &T) -> Shape {
        let string = format!("{item}");
        let mut lines = string.lines();
        let last_item = lines.next_back().unwrap_or("");

        // Check if we have any more lines remaining
        if lines.count() > 0 {
            // Reset the shape and add the last line
            self.reset().add_width(last_item.len())
        } else {
            // Continue adding to the current shape
            self.add_width(last_item.len())
        }
    }

    /// Takes in a new node, and tests whether adding it in will force any lines over the budget.
    /// This function attempts to ignore the impact of comments by removing them, which makes this function more expensive.
    /// NOTE: This function does not update state/return a new shape
    #[must_use]
    pub fn test_over_budget<T: Node>(&self, item: &T) -> bool {
        // Converts the node into a string, removing any comments present
        // We strip leading/trailing comments of each token present, but keep whitespace
        let string = item
            .tokens()
            .map(|token| {
                token
                    .update_trivia(
                        FormatTriviaType::Replace(
                            token
                                .leading_trivia()
                                .filter(|token| !trivia_is_comment(token))
                                .map(|x| x.to_owned())
                                .collect(),
                        ),
                        FormatTriviaType::Replace(
                            token
                                .trailing_trivia()
                                .filter(|token| !trivia_is_comment(token))
                                .map(|x| x.to_owned())
                                .collect(),
                        ),
                    )
                    .to_string()
            })
            .collect::<String>();

        let lines = string.lines();

        lines.enumerate().any(|(idx, line)| {
            let shape = if idx == 0 { *self } else { self.reset() };
            shape.add_width(line.len()).over_budget()
        })
    }
}

impl Add<usize> for Shape {
    type Output = Shape;

    fn add(self, rhs: usize) -> Shape {
        self.add_width(rhs)
    }
}
