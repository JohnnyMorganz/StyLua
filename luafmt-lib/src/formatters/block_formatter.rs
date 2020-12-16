use crate::formatters::{
    self, assignment_formatter, expression_formatter, functions_formatter, trivia_formatter,
};
use full_moon::ast::{Block, If, LastStmt, Return, Stmt};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

pub fn format_stmt<'ast>(stmt: Stmt<'ast>) -> Stmt<'ast> {
    match stmt {
        Stmt::Assignment(assignment) => {
            Stmt::Assignment(assignment_formatter::format_assignment(assignment))
        }
        Stmt::FunctionCall(function_call) => {
            Stmt::FunctionCall(functions_formatter::format_function_call(function_call))
        }
        Stmt::FunctionDeclaration(function_declaration) => Stmt::FunctionDeclaration(
            functions_formatter::format_function_declaration(function_declaration),
        ),
        Stmt::If(if_node) => Stmt::If(format_if(if_node)),
        Stmt::LocalAssignment(local_assignment) => Stmt::LocalAssignment(
            assignment_formatter::format_local_assignment(local_assignment),
        ),
        Stmt::LocalFunction(local_function) => {
            Stmt::LocalFunction(functions_formatter::format_local_function(local_function))
        }
        // TODO: Handle remainder
        _ => stmt,
        // Stmt::Do(do_node) => stmt,
        // Stmt::GenericFor(generic_for) => stmt,
        // Stmt::NumericFor(numeric_for) => stmt,
        // Stmt::Repeat(repeat) => stmt,
        // Stmt::While(while_node) => stmt
    }
}

pub fn stmt_add_trivia<'ast>(stmt: Stmt<'ast>, indent_level: &usize) -> Stmt<'ast> {
    let leading_trivia = vec![formatters::create_indent_trivia(indent_level)];
    let trailing_trivia = vec![formatters::create_newline_trivia()];

    match stmt {
        Stmt::Assignment(assignment) => Stmt::Assignment(trivia_formatter::assignment_add_trivia(
            assignment,
            leading_trivia,
            trailing_trivia,
        )),
        Stmt::FunctionCall(function_call) => {
            Stmt::FunctionCall(trivia_formatter::function_call_add_trivia(
                function_call,
                leading_trivia,
                trailing_trivia,
            ))
        }
        Stmt::LocalAssignment(local_assignment) => {
            Stmt::LocalAssignment(trivia_formatter::local_assignment_add_trivia(
                local_assignment,
                leading_trivia,
                trailing_trivia,
            ))
        }
        // TODO: Handle remainder
        _ => stmt,
    }
}

pub fn format_return<'ast>(return_node: Return<'ast>) -> Return<'ast> {
    let formatted_returns = formatters::format_punctuated(
        return_node.returns().to_owned(),
        &expression_formatter::format_expression,
    );
    let formatted_token: Cow<'ast, TokenReference<'ast>> =
        Cow::Owned(if formatted_returns.is_empty() {
            TokenReference::symbol("return").unwrap()
        } else {
            TokenReference::symbol("return ").unwrap()
        });
    return_node
        .with_token(formatted_token)
        .with_returns(formatted_returns)
}

pub fn format_last_stmt<'ast>(last_stmt: LastStmt<'ast>) -> LastStmt<'ast> {
    match last_stmt {
        LastStmt::Break(_) => LastStmt::Break(Cow::Owned(TokenReference::symbol("break").unwrap())),
        LastStmt::Return(return_node) => LastStmt::Return(format_return(return_node)),
    }
}

// pub fn last_stmt_add_trivia<'ast>(last_stmt: LastStmt<'ast>, indent_level: &usize) -> LastStmt<'ast> {
//     match last_stmt {
//         LastStmt::Break(break_node) => {
//             // TODO: Add indents
//             LastStmt::Break(Cow::Owned(trivia_formatter::token_reference_add_trailing_trivia(break_node.into_owned(), vec![formatters::create_newline_trivia()])))
//         }
//         LastStmt::Return(return_node) =>
//     }
// }

pub fn format_block<'ast>(block: Block<'ast>, indent_level: &usize) -> Block<'ast> {
    let formatted_statements: Vec<(Stmt<'ast>, Option<Cow<'ast, TokenReference<'ast>>>)> = block
        .iter_stmts()
        .map(|stmt| {
            (
                stmt_add_trivia(format_stmt(stmt.to_owned()), indent_level),
                None,
            )
        }) // The second parameter in the tuple is for semicolons - we do not want any semi-colons
        .collect();

    let formatted_last_stmt = match block.last_stmt() {
        Some(last_stmt) => Some((format_last_stmt(last_stmt.to_owned()), None)),
        None => None,
    }; // TODO: Add trivia

    block
        .with_stmts(formatted_statements)
        .with_last_stmt(formatted_last_stmt)
}

pub fn format_if<'ast>(if_node: If<'ast>) -> If<'ast> {
    let formatted_if_token = Cow::Owned(TokenReference::symbol("if ").unwrap());
    let formatted_condition =
        expression_formatter::format_expression(if_node.condition().to_owned());
    let formatted_then_token = Cow::Owned(TokenReference::symbol(" then\n").unwrap());
    let formatted_end_token = Cow::Owned(TokenReference::symbol("end").unwrap());

    if_node
        .with_if_token(formatted_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
        .with_end_token(formatted_end_token)
}
