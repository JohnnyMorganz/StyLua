use crate::context::Context;
use std::fmt::Display;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Shape {
    /// The current block indentation level. The base indentation level is 0. Note: this is not the indentation width
    indent_level: usize,
    /// How many characters a single indent level represents. This is inferred from the configuration
    indent_width: usize,
    /// Any additional indent level that we are currently in.
    additional_indent_level: usize,
    /// The current width we have taken on the line, excluding any indentation.
    offset: usize,
    /// The maximum number of characters we want to fit on a line. This is inferred from the configuration
    column_width: usize,
}

impl Shape {
    pub fn from_context(ctx: &Context) -> Self {
        Self {
            indent_level: ctx.indent_level().saturating_sub(1),
            indent_width: ctx.config().indent_width,
            additional_indent_level: 0,
            offset: 0,
            column_width: ctx.config().column_width,
        }
    }

    /// Create a shape with a given block indent level
    pub fn with_indent_level(ctx: &Context, indent_level: usize) -> Self {
        Self {
            indent_level,
            indent_width: ctx.config().indent_width,
            additional_indent_level: 0,
            offset: 0,
            column_width: ctx.config().column_width,
        }
    }

    /// Create a new shape containing any additional indent level
    pub fn with_additional_indent(&self, additional_indent_level: Option<usize>) -> Shape {
        match additional_indent_level {
            Some(t) => Self {
                additional_indent_level: t,
                ..*self
            },
            None => *self,
        }
    }

    /// Sets the column width to the provided width. Normally only used to set an infinite width when testing layouts
    pub fn with_column_width(&self, column_width: usize) -> Self {
        Self {
            column_width,
            ..*self
        }
    }

    /// Recreates the shape with an infinite width. Useful when testing layouts and want to force code onto a single line
    pub fn with_infinite_width(&self) -> Self {
        self.with_column_width(usize::MAX)
    }

    pub fn indent_width(&self) -> usize {
        (self.indent_level + self.additional_indent_level) * self.indent_width
    }

    /// The width currently taken up for this line
    pub fn used_width(&self) -> usize {
        self.indent_width() + self.offset
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
