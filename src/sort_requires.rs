//! Sort Requires CodeMod
//! This is an optional extension which will firstly sort all requires within a file before formatting the file
//!
//! The following assumptions are made when using this codemod:
//! - All requires are pure and have no side effects: resorting the requires is not an issue
//! - Only requires at the top level block are to be sorted
//! - Requires are of the form `local NAME = require(REQUIRE)`, with only a single require per local assignment
//!
//! Particular cases to consider:
//! - Requires based on other variables (e.g., `require(ReplicatedStorage.Module)`)
//!     - We firstly find all variables that have an impact on the require
//!     - We then list all these variables beforehand, in a sorted order
//!     - We then leave one line gap, and include all the requires (in a sorted order)
//!     - Requires are sectioned by the variables that they rely on

use full_moon::{
    ast::{
        Ast, Block, Call, Expression, FunctionArgs, LocalAssignment, Prefix, Stmt, Suffix, Value,
    },
    tokenizer::{TokenReference, TokenType},
};

type StmtSemicolon = (Stmt, Option<TokenReference>);

/// Takes in an input AST, and applies the sort requires codemod to output a new AST
pub fn sort_requires(input_ast: Ast) -> Ast {
    let block = input_ast.nodes();

    // Find all the requires in the code
    let mut requires = find_requires(block);

    // Exit early if no requires present
    if requires.0.is_empty() {
        return input_ast;
    }

    // TODO: find dependents and place them in a section beforehand

    // Sort all requires
    requires.0.sort_by_key(|assignment| {
        name_from_token(
            extract_local_assignment(&assignment.0)
                .names()
                .iter()
                .next()
                .expect("local assignment with no names")
                .token_type(),
        )
    });

    // Rewrite the requires into the AST, at the beginning of the block
    let new_stmts = rewrite_requires(requires.0, requires.1);
    let new_block = block.to_owned().with_stmts(new_stmts);

    input_ast.with_nodes(new_block)
}

/// Extracts a name from an identifier.
/// Errors if a non identifier is provided
fn name_from_token(token: &TokenType) -> String {
    match token {
        TokenType::Identifier { identifier } => identifier.to_string(),
        _ => unreachable!("attempted to get a name from non-identifier token"),
    }
}

/// Extracts a [`LocalAssignment`] from a [`Stmt`].
/// Errors if a provided [`Stmt`] is not a [`LocalAssignment`]
fn extract_local_assignment(stmt: &Stmt) -> &LocalAssignment {
    match stmt {
        Stmt::LocalAssignment(assignment) => assignment,
        _ => unreachable!("attempt to extract non-localassignment from stmt"),
    }
}

/// Verifies whether the provided [`Expression`] is in require form (i.e. `require(arg)`).
/// Returns the `arg` part of the require expression.
fn parse_require_function(expression: &Expression) -> Option<&FunctionArgs> {
    if let Expression::Value { value, .. } = expression {
        if let Value::FunctionCall(function_call) = &**value {
            if let Prefix::Name(name) = function_call.prefix() {
                if name.to_string() == "require" {
                    if let Some(Suffix::Call(Call::AnonymousCall(function_args))) =
                        function_call.suffixes().next()
                    {
                        if function_call.suffixes().count() == 1 {
                            return Some(function_args);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Determines whether the provided [`LocalAssignment`] is a require assignment.
/// i.e., of the form `local NAME = require(ARG)`
fn is_require_assignment(assignment: &LocalAssignment) -> bool {
    let expressions = assignment.expressions();
    expressions.len() == 1 && parse_require_function(expressions.iter().next().unwrap()).is_some()
}

/// Partitions all the statements within a block into requires and remainder statements
fn find_requires(block: &Block) -> (Vec<&StmtSemicolon>, Vec<&StmtSemicolon>) {
    block.stmts_with_semicolon().partition(|stmt| matches!(&stmt.0, Stmt::LocalAssignment(assignment) if is_require_assignment(assignment)))
}

/// Constructs a new set of Stmts from a sorted list of requires and any remainder statements
fn rewrite_requires(
    sorted_requires: Vec<&StmtSemicolon>,
    remainder_stmts: Vec<&StmtSemicolon>,
) -> Vec<StmtSemicolon> {
    sorted_requires
        .into_iter()
        .chain(remainder_stmts.into_iter())
        .cloned()
        .collect()
}
