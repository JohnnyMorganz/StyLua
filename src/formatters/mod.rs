use crate::{context::Context, shape::Shape};
use full_moon::ast::Ast;

pub mod assignment;
pub mod block;
pub mod general;
#[macro_use]
pub mod expression;
#[cfg(any(feature = "luau", feature = "cfxlua"))]
pub mod compound_assignment;
pub mod functions;
#[cfg(any(feature = "lua52", feature = "luajit"))]
pub mod goto;
#[cfg(feature = "lua54")]
pub mod lua54;
#[cfg(feature = "luau")]
pub mod luau;
pub mod stmt;
pub mod table;
pub mod trivia;
pub mod trivia_util;

use block::format_block;
use general::format_eof;

pub struct CodeFormatter {
    /// The formatting context
    context: Context,
}

impl CodeFormatter {
    /// Creates a new CodeFormatter, with the given configuration
    pub fn new(ctx: Context) -> Self {
        CodeFormatter { context: ctx }
    }

    /// Runs the formatter over the given AST
    pub fn format(&self, ast: Ast) -> Ast {
        let shape = Shape::new(&self.context);
        let new_block = format_block(&self.context, ast.nodes(), shape);
        let new_eof = format_eof(&self.context, ast.eof(), shape);

        ast.with_nodes(new_block).with_eof(new_eof)
    }
}
