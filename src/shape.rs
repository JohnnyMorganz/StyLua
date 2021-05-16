use crate::context::Context;
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

    /// The current width (characters) taken up by indentation
    pub fn indent_width(&self) -> usize {
        (self.block_indent + self.additional_indent) * self.indent_width
    }

    /// Increments the block indentation level by one
    pub fn increment_block_indent(&self) -> Self {
        Self {
            block_indent: self.block_indent.saturating_add(1),
            ..*self
        }
    }

    /// Decrements the block indentation level by one
    pub fn decrement_block_indent(&self) -> Self {
        Self {
            block_indent: self.block_indent.saturating_sub(1),
            ..*self
        }
    }

    /// Increments the additional indentation level by one
    pub fn increment_additional_indent(&self) -> Self {
        Self {
            additional_indent: self.block_indent.saturating_add(1),
            ..*self
        }
    }

    /// Decrements the additional indentation level by one
    pub fn decrement_additional_indent(&self) -> Self {
        Self {
            additional_indent: self.block_indent.saturating_sub(1),
            ..*self
        }
    }

    /// Increases the additional indentation level by amount specified
    pub fn add_indent_level(&self, amount: usize) -> Self {
        Self {
            additional_indent: self.block_indent.saturating_add(amount),
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
}

impl Shape {
    /// Creates a new shape at the base indentation level
    pub fn new(ctx: &Context) -> Self {
        Self {
            indent: Indent::new(ctx),
            offset: 0,
            column_width: ctx.config().column_width,
        }
    }

    /// Create a shape from the given Indent
    pub fn from_indent(ctx: &Context, indent: Indent) -> Self {
        Self {
            indent,
            offset: 0,
            column_width: ctx.config().column_width,
        }
    }

    /// Sets the column width to the provided width. Normally only used to set an infinite width when testing layouts
    pub fn with_column_width(&self, column_width: usize) -> Self {
        Self {
            column_width,
            ..*self
        }
    }

    /// Recreates the shape with the provided indentation
    pub fn with_indent(&self, indent: Indent) -> Self {
        Self { indent, ..*self }
    }

    /// Recreates the shape with an infinite width. Useful when testing layouts and want to force code onto a single line
    pub fn with_infinite_width(&self) -> Self {
        self.with_column_width(usize::MAX)
    }

    /// The current indentation of the shape
    pub fn indent(&self) -> Indent {
        self.indent
    }

    /// Increments the block indentation level by one. Alias for `shape.with_indent(shape.indent().increment_block_indent())`
    pub fn increment_block_indent(&self) -> Self {
        Self {
            indent: self.indent.increment_block_indent(),
            ..*self
        }
    }

    /// Decrements the block indentation level by one. Alias for `shape.with_indent(shape.indent().decrement_block_indent())`
    pub fn decrement_block_indent(&self) -> Self {
        Self {
            indent: self.indent.decrement_block_indent(),
            ..*self
        }
    }

    /// Increments the additional indentation level by one. Alias for `shape.with_indent(shape.indent().increment_additional_indent())`
    pub fn increment_additional_indent(&self) -> Self {
        Self {
            indent: self.indent.increment_additional_indent(),
            ..*self
        }
    }

    /// Decrements the additional indentation level by one. Alias for `shape.with_indent(shape.indent().decrement_additional_indent())`
    pub fn decrement_additional_indent(&self) -> Self {
        Self {
            indent: self.indent.decrement_additional_indent(),
            ..*self
        }
    }

    /// The width currently taken up for this line
    pub fn used_width(&self) -> usize {
        self.indent.indent_width() + self.offset
    }

    /// Check to see whether our current width is above the budget available
    pub fn over_budget(&self) -> bool {
        self.used_width() > self.column_width
    }

    /// Adds a width offset to the current width total
    pub fn add_width(&self, width: usize) -> Shape {
        Self {
            offset: self.offset + width,
            ..*self
        }
    }

    /// Subtracts a width offset from the current width total
    pub fn sub_width(&self, width: usize) -> Shape {
        Self {
            offset: self.offset.saturating_sub(width),
            ..*self
        }
    }

    /// Resets the offset for the shape
    pub fn reset(&self) -> Shape {
        Self { offset: 0, ..*self }
    }

    /// Takes the first line from an item which can be converted into a string, and sets that to the the shape
    pub fn take_first_line<T: Display>(&self, item: &T) -> Shape {
        let string = format!("{}", item);
        let mut lines = string.lines();
        let width = lines.next().expect("no lines").len();
        self.add_width(width)
    }

    /// Takes an item which could possibly span multiple lines. If it spans multiple lines, the shape is reset
    /// and the last line is added to the width. If it only takes a single line, we just continue adding to the current
    /// width
    pub fn take_last_line<T: Display>(&self, item: &T) -> Shape {
        let string = format!("{}", item);
        let mut lines = string.lines();
        let last_item = lines.next_back().expect("no lines");

        // Check if we have any more lines remaining
        if lines.count() > 0 {
            // Reset the shape and add the last line
            self.reset().add_width(last_item.len())
        } else {
            // Continue adding to the current shape
            self.add_width(last_item.len())
        }
    }
}

impl Add<usize> for Shape {
    type Output = Shape;

    fn add(self, rhs: usize) -> Shape {
        self.add_width(rhs)
    }
}
