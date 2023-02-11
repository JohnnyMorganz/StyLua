//! Sort Requires CodeMod
//! This is an optional extension which will firstly sort all requires within a file before formatting the file
//!
//! The following assumptions are made when using this codemod:
//! - All requires are pure and have no side effects: resorting the requires is not an issue
//! - Only requires at the top level block are to be sorted
//! - Requires are of the form `local NAME = require(REQUIRE)`, with only a single require per local assignment
//!
//! Requires sorting works in the following way:
//! - We group consecutive requires into a "block".
//!   If we hit a line which is a non-require or empty, we close the old block and start a new one.
//! - Requires are sorted only within their block.
//!   This allows us to solve the assumption of depending on local variables
//!   (if there is a local variable in between requires, it would split them into two separate blocks,
//!   so a require will always be after any local variable it uses)
//! - Blocks remain in-place in the file.

use full_moon::{
    ast::{Ast, Block, Expression, Prefix, Stmt, Value},
    node::Node,
    tokenizer::{TokenReference, TokenType},
};
use serde::Deserialize;

use crate::formatters::trivia::{FormatTriviaType, UpdateLeadingTrivia};

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct SortRequiresConfig {
    /// Whether the sort requires codemod is enabled
    enabled: bool,
}

impl SortRequiresConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_enabled(&self, enabled: bool) -> Self {
        Self { enabled }
    }
}

fn extract_identifier_from_token(token: &TokenReference) -> Option<String> {
    match token.token_type() {
        TokenType::Identifier { identifier } => Some(identifier.to_string()),
        _ => None,
    }
}

fn is_require_expression(expression: &Expression) -> bool {
    if let Expression::Value { value, .. } = expression {
        if let Value::FunctionCall(function_call) = &**value {
            if let Prefix::Name(token) = function_call.prefix() {
                if let Some(name) = extract_identifier_from_token(token) {
                    if name == "require" {
                        return true;
                    }
                }
            }
        }
    }

    false
}

type StmtSemicolon = (Stmt, Option<TokenReference>);

enum BlockPartition {
    RequiresGroup(Vec<(String, StmtSemicolon)>),
    Other(Vec<StmtSemicolon>),
}

fn partition_nodes_into_groups(block: &Block) -> Vec<BlockPartition> {
    let mut parts = Vec::new();

    for stmt in block.stmts_with_semicolon() {
        if let Stmt::LocalAssignment(node) = &stmt.0 {
            if node.names().len() == 1 && node.expressions().len() == 1 {
                let name = node.names().iter().next().unwrap();
                let expression = node.expressions().iter().next().unwrap();

                let current_line = name.start_position().unwrap().line();

                if is_require_expression(expression) {
                    let require_name = extract_identifier_from_token(name)
                        .expect("require is stored as non-identifier");

                    // Check if we need to start a new block:
                    // Either, the parts list is empty, the last part was a BlockPartition::Other
                    // or, there is > 1 line in between the previous require and this one
                    let create_new_block = match parts.last() {
                        None => true,
                        Some(BlockPartition::Other(_)) => true,
                        Some(BlockPartition::RequiresGroup(list)) => {
                            if let Some(previous_require) = list.last() {
                                if let Some(position) = previous_require.1.end_position() {
                                    let previous_require_line = position.line();
                                    current_line - previous_require_line > 1
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    };

                    if create_new_block {
                        parts.push(BlockPartition::RequiresGroup(Vec::new()))
                    }

                    match parts.last_mut() {
                        Some(BlockPartition::RequiresGroup(map)) => {
                            map.push((require_name, stmt.clone()))
                        }
                        _ => unreachable!(),
                    };

                    continue;
                }
            }
        }

        // Handle as a non-require
        if parts.is_empty() {
            parts.push(BlockPartition::Other(Vec::new()))
        } else if let Some(BlockPartition::RequiresGroup(_)) = parts.last() {
            parts.push(BlockPartition::Other(Vec::new()))
        }

        match parts.last_mut() {
            Some(BlockPartition::Other(list)) => list.push(stmt.clone()),
            _ => unreachable!(),
        }
    }

    parts
}

pub(crate) fn sort_requires(input_ast: Ast) -> Ast {
    let block = input_ast.nodes();

    // Find all `local NAME = require(EXPR)` lines
    let parts = partition_nodes_into_groups(block);

    // If there is only one non-require partition, or no partitions at all
    // then just return the original AST
    match parts.last() {
        Some(BlockPartition::Other(_)) if parts.len() == 1 => return input_ast,
        None => return input_ast,
        _ => (),
    };

    // Reconstruct the AST with sorted require groups
    let mut stmts: Vec<StmtSemicolon> = Vec::new();
    for part in parts {
        match part {
            BlockPartition::RequiresGroup(mut list) => {
                // Get the leading trivia of the first statement in the list, as that will be what
                // is appended to the new statement
                let leading_trivia = match list.first_mut() {
                    Some((_, (Stmt::LocalAssignment(local_assignment), _))) => {
                        let trivia = local_assignment
                            .local_token()
                            .leading_trivia()
                            .cloned()
                            .collect();

                        // Replace the trivia
                        *local_assignment = local_assignment
                            .update_leading_trivia(FormatTriviaType::Replace(vec![]));

                        trivia
                    }
                    _ => unreachable!(),
                };

                // Sort our list of requires
                list.sort_by_key(|key| key.0.clone());

                // Mutate the first element with our leading trivia
                match list.first_mut() {
                    Some((_, (Stmt::LocalAssignment(local_assignment), _))) => {
                        *local_assignment = local_assignment
                            .update_leading_trivia(FormatTriviaType::Replace(leading_trivia))
                    }
                    _ => unreachable!(),
                };

                // Add to the list of stmts
                stmts.extend(list.iter().map(|x| x.1.clone()))
            }
            BlockPartition::Other(mut list) => stmts.append(&mut list),
        };
    }

    let block = block.clone().with_stmts(stmts);
    input_ast.with_nodes(block).update_positions()
}
