use crate::{context::Context, Config};
use full_moon::ast::Block;
use full_moon::tokenizer::TokenReference;
use full_moon::visitors::VisitorMut;

pub mod assignment;
pub mod block;
pub mod general;
#[macro_use]
pub mod expression;
pub mod functions;
#[cfg(feature = "lua52")]
pub mod lua52;
#[cfg(feature = "luau")]
pub mod luau;
pub mod stmt;
pub mod table;
pub mod trivia;
pub mod trivia_util;
pub mod util;

use block::format_block;
use general::format_eof;

pub struct CodeFormatter {
    /// The formatting context
    context: Context,
}

impl CodeFormatter {
    /// Creates a new CodeFormatter, with the given configuration
    pub fn new(config: Config, range: Option<crate::Range>) -> Self {
        CodeFormatter {
            context: Context::new(config, range),
        }
    }
}

impl<'ast> VisitorMut<'ast> for CodeFormatter {
    fn visit_block(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.context.increment_indent_level();
        format_block(&mut self.context, node)
    }

    fn visit_block_end(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.context.decrement_indent_level();
        node
    }

    // Remove any extra whitespace at the end of the file
    fn visit_eof(&mut self, node: TokenReference<'ast>) -> TokenReference<'ast> {
        format_eof(&mut self.context, &node)
    }
}
