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

use std::collections::{HashMap, HashSet};

use full_moon::{
    ast::{
        Ast, Block, Call, Expression, FunctionArgs, LocalAssignment, Prefix, Stmt, Suffix, Value,
        Var,
    },
    tokenizer::{TokenReference, TokenType},
};
use serde::Deserialize;

type StmtSemicolon = (Stmt, Option<TokenReference>);

#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SortRequiresConfig {
    /// Whether the sort requires codemod is enabled.
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

/// Takes in an input AST, and applies the sort requires codemod to output a new AST
pub fn sort_requires(input_ast: Ast) -> Ast {
    let block = input_ast.nodes();

    // Find all the requires in the code
    let (mut dependencies, mut requires, remainders) = find_requires(block);

    // Exit early if no requires present
    if requires.is_empty() {
        return input_ast;
    }

    // TODO: find dependents and place them in a section beforehand
    // TODO: perform a topo sort on the requires

    // Sort all requires
    requires.sort_by_key(|assignment| {
        name_from_token(
            extract_local_assignment(&assignment.0)
                .names()
                .iter()
                .next()
                .expect("local assignment with no names")
                .token_type(),
        )
    });

    // TODO: this is wrong
    dependencies.extend(requires);

    // Rewrite the requires into the AST, at the beginning of the block
    let new_stmts = rewrite_requires(dependencies, remainders);
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
                            match function_args {
                                FunctionArgs::String(_) => return Some(function_args),
                                FunctionArgs::Parentheses { arguments, .. }
                                    if arguments.len() == 1 =>
                                {
                                    return Some(function_args);
                                }
                                // TODO: require of a table? is this possible?
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Extracts the [`Expression`] inside a require assignment.
/// NOTE: this function EXPECTS only one expression in the local assignemnt
fn extract_assignment_expression(assignment: &LocalAssignment) -> &Expression {
    assert!(assignment.expressions().len() == 1);
    assignment.expressions().iter().next().unwrap()
}

/// Determines whether the provided [`LocalAssignment`] is a require assignment.
/// i.e., of the form `local NAME = require(ARG)`
fn is_require_assignment(assignment: &LocalAssignment) -> bool {
    assignment.expressions().len() == 1
        && parse_require_function(extract_assignment_expression(assignment)).is_some()
}

fn create_map_from_assignments(block: &Block) -> HashMap<String, &StmtSemicolon> {
    let mut map = HashMap::new();

    for stmt_semicolon in block.stmts_with_semicolon() {
        match &stmt_semicolon.0 {
            Stmt::LocalAssignment(assignment) => {
                // TODO: should we map multi assignments?
                for name in assignment.names() {
                    map.insert(name_from_token(name.token_type()), stmt_semicolon);
                }
            }
            _ => continue,
        }
    }

    map
}

fn find_dependency_name(stmt: &StmtSemicolon) -> Option<String> {
    match &stmt.0 {
        Stmt::LocalAssignment(assignment) => {
            // TODO: handle multi expression?
            if let Some(expression) = assignment.expressions().iter().next() {
                // Handle if its a require function
                let expression = match parse_require_function(expression) {
                    Some(FunctionArgs::String(_)) => return None, // A string require cannot have any dependencies
                    Some(FunctionArgs::TableConstructor(_)) => todo!("require of a table"),
                    Some(FunctionArgs::Parentheses { arguments, .. }) => {
                        arguments.iter().next().unwrap()
                    }
                    Some(other) => unreachable!("unknown node: {:?}", other),
                    None => expression,
                };

                let dependency = match expression {
                    Expression::Value { value, .. } => {
                        match &**value {
                            Value::String(_) => return None, // A string require cannot have any dependencies
                            Value::Var(var) => match var {
                                Var::Name(token) => name_from_token(token.token_type()),
                                Var::Expression(var_expression) => match var_expression.prefix() {
                                    Prefix::Name(token) => name_from_token(token.token_type()),
                                    _ => todo!(
                                        "non-standard dependency [PREFIX]: {}",
                                        var_expression.to_string()
                                    ),
                                },
                                other => unreachable!("unknown node: {:?}", other),
                            },
                            Value::FunctionCall(function_call) => match function_call.prefix() {
                                Prefix::Name(token) => name_from_token(token.token_type()),
                                _ => todo!(
                                    "non-standard dependency [PREFIX]: {}",
                                    function_call.to_string()
                                ),
                            },
                            _ => todo!("non-standard dependency [VALUE]: {}", value.to_string()),
                        }
                    }
                    _ => todo!("non-standard dependency [EXPR]: {}", expression.to_string()),
                };

                Some(dependency)
            } else {
                None
            }
        }

        _ => todo!("non-local assignment dependency"),
    }
}

fn is_dependency(stmt: &Stmt, dependency_names: &HashSet<String>) -> bool {
    match stmt {
        Stmt::LocalAssignment(local_assignment) => local_assignment
            .names()
            .iter()
            .any(|name| dependency_names.contains(&name_from_token(name.token_type()))),
        _ => false,
    }
}

/// Partitions all the statements within a block into requires and remainder statements
fn find_requires(
    block: &Block,
) -> (
    Vec<&StmtSemicolon>,
    Vec<&StmtSemicolon>,
    Vec<&StmtSemicolon>,
) {
    let assignment_map = create_map_from_assignments(block);

    // Partition into require statements and the remaining statements
    let (requires, remainders): (Vec<_>, _) = block.stmts_with_semicolon().partition(|stmt| matches!(&stmt.0, Stmt::LocalAssignment(assignment) if is_require_assignment(assignment)));

    // Find all the dependency variables from the list of remaining statements
    // We need to continuously search the list to find all the statements (i.e., local a = __, local b = a.x, local c = require(b.y) : both a and b are dependencies)
    let mut dependency_names = HashSet::new();
    let mut search_stack = requires.clone();
    while let Some(element) = search_stack.pop() {
        let dependency_name = find_dependency_name(element);
        if let Some(dependency_name) = dependency_name {
            dependency_names.insert(dependency_name.clone());

            // Climb up to look for any more dependencies
            // TODO: we should structure this so that we can topo-sort
            let dependency_stmt = assignment_map.get(&dependency_name);
            if let Some(dependency_stmt) = dependency_stmt {
                search_stack.push(dependency_stmt);
            }
        }
    }

    // From the remaining statements, find all the assignments which a require is dependent on
    let (dependencies, remainders) = remainders
        .iter()
        .partition(|stmt| is_dependency(&stmt.0, &dependency_names));

    (dependencies, requires, remainders)
}

/// Constructs a new set of Stmts from a sorted list of requires and any remainder statements
fn rewrite_requires(
    sorted_requires: Vec<&StmtSemicolon>,
    remainder_stmts: Vec<&StmtSemicolon>,
) -> Vec<StmtSemicolon> {
    // TODO: remove any excessive newlines at the end of requires (in case they were removed)
    // TODO: place a single newline between the sorted requires and the remainder statements

    sorted_requires
        .into_iter()
        .chain(remainder_stmts.into_iter())
        .cloned()
        .collect()
}
