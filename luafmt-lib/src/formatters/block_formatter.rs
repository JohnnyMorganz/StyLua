use crate::formatters::{
    self, assignment_formatter, expression_formatter, format_symbol, functions_formatter,
    trivia_formatter,
};
use full_moon::ast::{
    punctuated::Pair, Block, Do, ElseIf, GenericFor, If, LastStmt, NumericFor, Repeat, Return,
    Stmt, While,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

/// Format a Do node
pub fn format_do_block<'ast>(do_block: Do<'ast>) -> Do<'ast> {
    let do_token = format_symbol(
        do_block.do_token().to_owned(),
        TokenReference::symbol("do").unwrap(),
    );
    let end_token = format_symbol(
        do_block.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    do_block.with_do_token(do_token).with_end_token(end_token)
}

/// Format a GenericFor node
pub fn format_generic_for<'ast>(generic_for: GenericFor<'ast>) -> GenericFor<'ast> {
    let for_token = format_symbol(
        generic_for.for_token().to_owned(),
        TokenReference::symbol("for ").unwrap(),
    );
    let formatted_names = formatters::format_punctuated(
        generic_for.names().to_owned(),
        &formatters::format_token_reference,
    );
    let in_token = format_symbol(
        generic_for.in_token().to_owned(),
        TokenReference::symbol(" in ").unwrap(),
    );
    let formatted_expr_list = formatters::format_punctuated(
        generic_for.expr_list().to_owned(),
        &expression_formatter::format_expression,
    );
    let do_token = format_symbol(
        generic_for.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = format_symbol(
        generic_for.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    generic_for
        .with_for_token(for_token)
        .with_names(formatted_names)
        .with_in_token(in_token)
        .with_expr_list(formatted_expr_list)
        .with_do_token(do_token)
        .with_end_token(end_token)
}

/// Formats an ElseIf node - This must always reside within format_if
fn format_else_if<'ast>(else_if_node: ElseIf<'ast>) -> ElseIf<'ast> {
    let formatted_else_if_token = format_symbol(
        else_if_node.else_if_token().to_owned(),
        TokenReference::symbol("elseif ").unwrap(),
    );
    let formatted_condition =
        expression_formatter::format_expression(else_if_node.condition().to_owned());
    let formatted_then_token = format_symbol(
        else_if_node.then_token().to_owned(),
        TokenReference::symbol(" then").unwrap(),
    );

    else_if_node
        .with_else_if_token(formatted_else_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
}

/// Format an If node
pub fn format_if<'ast>(if_node: If<'ast>) -> If<'ast> {
    let formatted_if_token = format_symbol(
        if_node.if_token().to_owned(),
        TokenReference::symbol("if ").unwrap(),
    );
    let formatted_condition =
        expression_formatter::format_expression(if_node.condition().to_owned());
    let formatted_then_token = format_symbol(
        if_node.then_token().to_owned(),
        TokenReference::symbol(" then").unwrap(),
    );
    let formatted_end_token = format_symbol(
        if_node.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    let formatted_else_if = match if_node.else_if() {
        Some(else_if) => Some(
            else_if
                .iter()
                .map(|else_if| format_else_if(else_if.to_owned()))
                .collect(),
        ),
        None => None,
    };

    let formatted_else_token = match if_node.else_token() {
        Some(token) => Some(format_symbol(
            token.to_owned(),
            TokenReference::symbol("else").unwrap(),
        )),
        None => None,
    };

    if_node
        .with_if_token(formatted_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
        .with_else_if(formatted_else_if)
        .with_else_token(formatted_else_token)
        .with_end_token(formatted_end_token)
}

/// Format a NumericFor node
pub fn format_numeric_for<'ast>(numeric_for: NumericFor<'ast>) -> NumericFor<'ast> {
    let for_token = format_symbol(
        numeric_for.for_token().to_owned(),
        TokenReference::symbol("for ").unwrap(),
    );
    let formatted_index_variable = Cow::Owned(formatters::format_plain_token_reference(
        numeric_for.index_variable().to_owned(),
    ));
    let equal_token = format_symbol(
        numeric_for.equal_token().to_owned(),
        TokenReference::symbol(" = ").unwrap(),
    );
    let formatted_start_expression =
        expression_formatter::format_expression(numeric_for.start().to_owned());
    let start_end_comma = format_symbol(
        numeric_for.start_end_comma().to_owned(),
        TokenReference::symbol(", ").unwrap(),
    );
    let formatted_end_expression =
        expression_formatter::format_expression(numeric_for.end().to_owned());

    let (end_step_comma, formatted_step_expression) = match numeric_for.step() {
        Some(step) => (
            Some(format_symbol(
                numeric_for.end_step_comma().unwrap().to_owned(),
                TokenReference::symbol(", ").unwrap(),
            )),
            Some(expression_formatter::format_expression(step.to_owned())),
        ),
        None => (None, None),
    };

    let do_token = format_symbol(
        numeric_for.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = format_symbol(
        numeric_for.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    numeric_for
        .with_for_token(for_token)
        .with_index_variable(formatted_index_variable)
        .with_equal_token(equal_token)
        .with_start(formatted_start_expression)
        .with_start_end_comma(start_end_comma)
        .with_end(formatted_end_expression)
        .with_end_step_comma(end_step_comma)
        .with_step(formatted_step_expression)
        .with_do_token(do_token)
        .with_end_token(end_token)
}

/// Format a Repeat node
pub fn format_repeat_block<'ast>(repeat_block: Repeat<'ast>) -> Repeat<'ast> {
    let repeat_token = format_symbol(
        repeat_block.repeat_token().to_owned(),
        TokenReference::symbol("repeat").unwrap(),
    );
    let until_token = format_symbol(
        repeat_block.until_token().to_owned(),
        TokenReference::symbol("until ").unwrap(),
    );
    let formatted_until = expression_formatter::format_expression(repeat_block.until().to_owned());

    repeat_block
        .with_repeat_token(repeat_token)
        .with_until_token(until_token)
        .with_until(formatted_until)
}

/// Format a While node
pub fn format_while_block<'ast>(while_block: While<'ast>) -> While<'ast> {
    let while_token = format_symbol(
        while_block.while_token().to_owned(),
        TokenReference::symbol("while ").unwrap(),
    );
    let formatted_condition =
        expression_formatter::format_expression(while_block.condition().to_owned());
    let do_token = format_symbol(
        while_block.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = format_symbol(
        while_block.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    while_block
        .with_while_token(while_token)
        .with_condition(formatted_condition)
        .with_do_token(do_token)
        .with_end_token(end_token)
}

pub fn format_stmt<'ast>(stmt: Stmt<'ast>) -> Stmt<'ast> {
    match stmt {
        Stmt::Assignment(assignment) => {
            Stmt::Assignment(assignment_formatter::format_assignment(assignment))
        }
        Stmt::Do(do_block) => Stmt::Do(format_do_block(do_block)),
        Stmt::FunctionCall(function_call) => {
            Stmt::FunctionCall(functions_formatter::format_function_call(function_call))
        }
        Stmt::FunctionDeclaration(function_declaration) => Stmt::FunctionDeclaration(
            functions_formatter::format_function_declaration(function_declaration),
        ),
        Stmt::GenericFor(generic_for) => Stmt::GenericFor(format_generic_for(generic_for)),
        Stmt::If(if_node) => Stmt::If(format_if(if_node)),
        Stmt::LocalAssignment(local_assignment) => Stmt::LocalAssignment(
            assignment_formatter::format_local_assignment(local_assignment),
        ),
        Stmt::LocalFunction(local_function) => {
            Stmt::LocalFunction(functions_formatter::format_local_function(local_function))
        }
        Stmt::NumericFor(numeric_for) => Stmt::NumericFor(format_numeric_for(numeric_for)),
        Stmt::Repeat(repeat) => Stmt::Repeat(format_repeat_block(repeat)),
        Stmt::While(while_block) => Stmt::While(format_while_block(while_block)),
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
        Stmt::Do(do_block) => Stmt::Do(trivia_formatter::do_block_add_trivia(
            do_block,
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
        Stmt::FunctionDeclaration(function_declaration) => {
            Stmt::FunctionDeclaration(trivia_formatter::function_declaration_add_trivia(
                function_declaration,
                leading_trivia,
                trailing_trivia,
            ))
        }
        Stmt::GenericFor(generic_for) => Stmt::GenericFor(
            trivia_formatter::generic_for_add_trivia(generic_for, leading_trivia, trailing_trivia),
        ),
        Stmt::If(if_block) => Stmt::If(trivia_formatter::if_block_add_trivia(
            if_block,
            leading_trivia,
            trailing_trivia,
        )),
        Stmt::LocalAssignment(local_assignment) => {
            Stmt::LocalAssignment(trivia_formatter::local_assignment_add_trivia(
                local_assignment,
                leading_trivia,
                trailing_trivia,
            ))
        }
        Stmt::LocalFunction(local_function) => {
            Stmt::LocalFunction(trivia_formatter::local_function_add_trivia(
                local_function,
                leading_trivia,
                trailing_trivia,
            ))
        }
        Stmt::NumericFor(numeric_for) => Stmt::NumericFor(
            trivia_formatter::numeric_for_add_trivia(numeric_for, leading_trivia, trailing_trivia),
        ),
        Stmt::Repeat(repeat_block) => Stmt::Repeat(trivia_formatter::repeat_block_add_trivia(
            repeat_block,
            leading_trivia,
            trailing_trivia,
        )),
        Stmt::While(while_block) => Stmt::While(trivia_formatter::while_block_add_trivia(
            while_block,
            leading_trivia,
            trailing_trivia,
        )),
    }
}

pub fn format_return<'ast>(return_node: Return<'ast>) -> Return<'ast> {
    let formatted_returns = formatters::format_punctuated(
        return_node.returns().to_owned(),
        &expression_formatter::format_expression,
    );
    let wanted_token: TokenReference<'ast> = if formatted_returns.is_empty() {
        TokenReference::symbol("return").unwrap()
    } else {
        TokenReference::symbol("return ").unwrap()
    };
    let formatted_token = format_symbol(return_node.token().to_owned(), wanted_token);
    return_node
        .with_token(formatted_token)
        .with_returns(formatted_returns)
}

pub fn format_last_stmt<'ast>(last_stmt: LastStmt<'ast>) -> LastStmt<'ast> {
    match last_stmt {
        LastStmt::Break(token) => LastStmt::Break(format_symbol(
            token.into_owned(),
            TokenReference::symbol("break").unwrap(),
        )),
        LastStmt::Return(return_node) => LastStmt::Return(format_return(return_node)),
    }
}

pub fn last_stmt_add_trivia<'ast>(
    last_stmt: LastStmt<'ast>,
    indent_level: &usize,
) -> LastStmt<'ast> {
    match last_stmt {
        LastStmt::Break(break_node) => {
            LastStmt::Break(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                break_node.into_owned(),
                Some(vec![formatters::create_indent_trivia(indent_level)]),
                Some(vec![formatters::create_newline_trivia()]),
            )))
        }
        LastStmt::Return(return_node) => {
            let mut token = return_node.token().to_owned();
            let mut returns = return_node.returns().to_owned();

            if return_node.returns().is_empty() {
                token = trivia_formatter::token_reference_add_trivia(
                    token,
                    Some(vec![formatters::create_indent_trivia(indent_level)]),
                    Some(vec![formatters::create_newline_trivia()]),
                );
            } else {
                token = trivia_formatter::token_reference_add_trivia(
                    token,
                    Some(vec![formatters::create_indent_trivia(indent_level)]),
                    None,
                );

                // TODO: This is copied from the Assignment/LocalAssignment formatters
                // Retrieve last item and add new line to it
                if let Some(last_pair) = returns.pop() {
                    match last_pair {
                        Pair::End(value) => {
                            let expression = trivia_formatter::expression_add_trailing_trivia(
                                value,
                                vec![formatters::create_newline_trivia()],
                            );
                            returns.push(Pair::End(expression));
                        }
                        Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
                    }
                }
            }

            LastStmt::Return(
                return_node
                    .with_token(Cow::Owned(token))
                    .with_returns(returns),
            )
        }
    }
}

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
        Some(last_stmt) => Some((
            last_stmt_add_trivia(format_last_stmt(last_stmt.to_owned()), indent_level),
            None,
        )),
        None => None,
    };

    block
        .with_stmts(formatted_statements)
        .with_last_stmt(formatted_last_stmt)
}
