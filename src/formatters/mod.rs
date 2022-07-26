use crate::{context::Context, shape::Shape, Config};
use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        Assignment, Ast, Block, Expression, Stmt, Value, Var,
    },
    tokenizer::TokenReference,
};

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

use block::format_block;
use general::format_eof;
use pretty::{DocAllocator, DocBuilder, RcDoc};

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
    pub fn format(&self, ast: Ast) -> Ast {
        let shape = Shape::new(&self.context);
        let new_block = format_block(&self.context, ast.nodes(), shape);
        let new_eof = format_eof(&self.context, ast.eof(), shape);

        ast.with_nodes(new_block).with_eof(new_eof)
    }
}

pub trait Formatter {
    // fn pretty<'a, D, A>(&self, allocator: &D) -> DocBuilder<'a, D, A>
    // where
    //     D: DocAllocator<'a, A>,
    //     D::Doc: Clone,
    //     A: Clone;

    fn to_doc(&self) -> RcDoc<()>;
}

impl Formatter for TokenReference {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text(self.to_string()) // TODO
    }
}

impl Formatter for Var {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Var::Name(token) => token.to_doc(),
            _ => todo!(),
        }
    }
}

impl Formatter for Expression {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Expression::Value { value } => match &**value {
                Value::Number(token) => token.to_doc(),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl<T: Formatter> Formatter for Pair<T> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Pair::Punctuated(_, _) => todo!(),
            Pair::End(node) => node.to_doc(),
        }
    }
}

impl<T: Formatter> Formatter for Punctuated<T> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(self.pairs().into_iter().map(|x| x.to_doc()), RcDoc::line())
    }
}

impl Formatter for Assignment {
    fn to_doc(&self) -> RcDoc<()> {
        self.variables()
            .to_doc()
            .append(RcDoc::text(" = "))
            .append(self.expressions().to_doc())
    }
}

impl Formatter for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Assignment(assignment) => assignment.to_doc(),
            _ => todo!(),
        }
    }
}

impl Formatter for Block {
    fn to_doc(&self) -> RcDoc<()> {
        let mut doc = RcDoc::nil();

        for (stmt, semicolon) in self.stmts_with_semicolon() {
            doc = doc.append(stmt.to_doc());

            if semicolon.is_some() {
                todo!()
            }
        }

        if self.last_stmt().is_some() {
            todo!()
        }

        doc
    }
}

impl Formatter for Ast {
    fn to_doc(&self) -> RcDoc<()> {
        self.nodes().to_doc()
    }
}
