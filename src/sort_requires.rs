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
    ast::{Ast, Block, Call, Expression, Prefix, Stmt, Suffix},
    node::Node,
    tokenizer::{TokenReference, TokenType},
};

use crate::{
    context::{Context, FormatNode},
    formatters::trivia::{FormatTriviaType, UpdateLeadingTrivia},
};

fn extract_identifier_from_token(token: &TokenReference) -> Option<String> {
    match token.token_type() {
        TokenType::Identifier { identifier } => Some(identifier.to_string()),
        _ => None,
    }
}

fn get_expression_kind(expression: &Expression) -> Option<GroupKind> {
    match expression {
        Expression::FunctionCall(function_call) => {
            let Prefix::Name(token) = function_call.prefix() else {
                return None;
            };
            let Some(name) = extract_identifier_from_token(token) else {
                return None;
            };

            if name == "require" {
                Some(GroupKind::Require)
            } else if name == "game" {
                let Some(Suffix::Call(Call::MethodCall(method_call))) =
                    function_call.suffixes().next()
                else {
                    return None;
                };
                let Some(name) = extract_identifier_from_token(method_call.name()) else {
                    return None;
                };
                if name == "GetService" {
                    Some(GroupKind::GetService)
                } else {
                    None
                }
            } else {
                None
            }
        }
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { expression, .. } => get_expression_kind(expression),
        _ => None,
    }
}

type StmtSemicolon = (Stmt, Option<TokenReference>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GroupKind {
    Require,
    GetService,
}

enum BlockPartition {
    RequiresGroup(GroupKind, Vec<(String, StmtSemicolon)>),
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

                let expression_kind = get_expression_kind(expression);
                if let Some(expression_kind) = expression_kind {
                    let variable_name = extract_identifier_from_token(name)
                        .expect("require is stored as non-identifier");

                    // Check if we need to start a new block:
                    // Either, the parts list is empty, the last part was a BlockPartition::Other,
                    // the last part group was a different kind, or,
                    // there is > 1 line in between the previous require and this one
                    let create_new_block = match parts.last() {
                        None => true,
                        Some(BlockPartition::Other(_)) => true,
                        Some(BlockPartition::RequiresGroup(other_kind, _))
                            if *other_kind != expression_kind =>
                        {
                            true
                        }
                        Some(BlockPartition::RequiresGroup(_, list)) => {
                            let previous_require =
                                list.last().expect("unreachable!: empty require group");
                            let position = previous_require
                                .1
                                .end_position()
                                .expect("unreachable!: previous require stmt has no end position");

                            let previous_require_line = position.line();
                            current_line - previous_require_line > 1
                        }
                    };

                    if create_new_block {
                        parts.push(BlockPartition::RequiresGroup(expression_kind, Vec::new()))
                    }

                    match parts.last_mut() {
                        Some(BlockPartition::RequiresGroup(_, map)) => {
                            map.push((variable_name, stmt.clone()))
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
        } else if let Some(BlockPartition::RequiresGroup(_, _)) = parts.last() {
            parts.push(BlockPartition::Other(Vec::new()))
        }

        match parts.last_mut() {
            Some(BlockPartition::Other(list)) => list.push(stmt.clone()),
            _ => unreachable!(),
        }
    }

    parts
}

pub(crate) fn sort_requires(ctx: &Context, input_ast: Ast) -> Ast {
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
            BlockPartition::RequiresGroup(_, mut list) => {
                // If any of the block is ignored, then ignore the whole thing
                if list
                    .iter()
                    .any(|(_, stmt)| !matches!(ctx.should_format_node(stmt), FormatNode::Normal))
                {
                    stmts.extend(list.iter().map(|x| x.1.clone()));
                    continue;
                }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_test_expression(ast: &Ast) -> &Expression {
        let stmt = ast.nodes().stmts().next().unwrap();
        match stmt {
            Stmt::LocalAssignment(local_assignment) => {
                local_assignment.expressions().iter().next().unwrap()
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn fail_extracting_non_identifier_token() {
        let token = TokenReference::symbol("return").unwrap();
        assert!(extract_identifier_from_token(&token).is_none());
    }

    #[test]
    fn get_expression_kind_require_stmt() {
        let ast = full_moon::parse("local NAME = require(EXPR)").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), Some(GroupKind::Require));
    }

    #[test]
    fn get_expression_kind_get_service() {
        let ast = full_moon::parse("local NAME = game:GetService(EXPR)").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), Some(GroupKind::GetService));
    }

    #[test]
    fn get_expression_kind_other_1() {
        let ast = full_moon::parse("local NAME = testing").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    fn get_expression_kind_other_2() {
        let ast = full_moon::parse("local NAME = game").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    fn get_expression_kind_other_3() {
        let ast = full_moon::parse("local NAME = game:FindFirstChild(EXPR)").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    fn get_expression_kind_other_4() {
        let ast = full_moon::parse("local NAME = game.Name").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    fn get_expression_kind_other_5() {
        let ast = full_moon::parse("local NAME = ('game'):GetService(EXPR)").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    fn get_expression_kind_other_6() {
        let ast = full_moon::parse("local NAME = 1 + 2").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), None);
    }

    #[test]
    #[cfg(feature = "luau")]
    fn get_expression_kind_require_type_assertion() {
        let ast = full_moon::parse("local NAME = require(path) :: any").unwrap();
        let expression = extract_test_expression(&ast);

        assert_eq!(get_expression_kind(expression), Some(GroupKind::Require));
    }
}
