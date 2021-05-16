use crate::{context::Context, shape::Shape, Config};
use full_moon::ast::{Ast, Block};
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

    /// Runs the formatter over the given AST
    pub fn format<'ast>(&self, ast: Ast<'ast>) -> Ast<'ast> {
        let shape = Shape::new(&self.context);
        let new_block = format_block(&mut self.context, ast.nodes(), shape);
        let new_eof = format_eof(&mut self.context, ast.eof());

        ast.with_nodes(new_block).with_eof(new_eof)
    }
}
