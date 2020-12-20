#[cfg(feature = "luau")]
use crate::formatters::luau_formatter;
use crate::formatters::{
    assignment_formatter, expression_formatter, functions_formatter, trivia_formatter,
    CodeFormatter,
};
use full_moon::ast::{
    punctuated::Pair, Block, Do, ElseIf, GenericFor, If, LastStmt, NumericFor, Repeat, Return,
    Stmt, While, Prefix, Expression
};
use full_moon::tokenizer::{Token, TokenReference};
#[cfg(feature = "luau")]
use full_moon::tokenizer::TokenType;
use std::borrow::Cow;

/// Format a Do node
pub fn format_do_block<'ast>(code_formatter: &CodeFormatter, do_block: Do<'ast>) -> Do<'ast> {
    let do_token = code_formatter.format_symbol(
        do_block.do_token().to_owned(),
        TokenReference::symbol("do").unwrap(),
    );
    let end_token = code_formatter.format_symbol(
        do_block.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    do_block.with_do_token(do_token).with_end_token(end_token)
}

/// Format a GenericFor node
pub fn format_generic_for<'ast>(
    code_formatter: &mut CodeFormatter,
    generic_for: GenericFor<'ast>,
) -> GenericFor<'ast> {
    let for_token = code_formatter.format_symbol(
        generic_for.for_token().to_owned(),
        TokenReference::symbol("for ").unwrap(),
    );
    let formatted_names = code_formatter.format_punctuated(
        generic_for.names().to_owned(),
        &CodeFormatter::format_token_reference_mut,
    );

    #[cfg(feature = "luau")]
    let type_specifiers = generic_for
        .type_specifiers()
        .map(|x| match x {
            Some(type_specifier) => Some(luau_formatter::format_type_specifier(
                code_formatter,
                type_specifier.to_owned(),
            )),
            None => None,
        })
        .collect();

    #[cfg(feature = "luau")]
    let generic_for = generic_for.with_type_specifiers(type_specifiers);

    let in_token = code_formatter.format_symbol(
        generic_for.in_token().to_owned(),
        TokenReference::symbol(" in ").unwrap(),
    );
    let formatted_expr_list = code_formatter.format_punctuated(
        generic_for.expr_list().to_owned(),
        &expression_formatter::format_expression,
    );
    let do_token = code_formatter.format_symbol(
        generic_for.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = code_formatter.format_symbol(
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
fn format_else_if<'ast>(
    code_formatter: &mut CodeFormatter,
    else_if_node: ElseIf<'ast>,
) -> ElseIf<'ast> {
    let formatted_else_if_token = code_formatter.format_symbol(
        else_if_node.else_if_token().to_owned(),
        TokenReference::symbol("elseif ").unwrap(),
    );
    let formatted_condition = expression_formatter::format_expression(
        code_formatter,
        else_if_node.condition().to_owned(),
    );
    let formatted_then_token = code_formatter.format_symbol(
        else_if_node.then_token().to_owned(),
        TokenReference::symbol(" then").unwrap(),
    );

    else_if_node
        .with_else_if_token(formatted_else_if_token)
        .with_condition(formatted_condition)
        .with_then_token(formatted_then_token)
}

/// Format an If node
pub fn format_if<'ast>(code_formatter: &mut CodeFormatter, if_node: If<'ast>) -> If<'ast> {
    let formatted_if_token = code_formatter.format_symbol(
        if_node.if_token().to_owned(),
        TokenReference::symbol("if ").unwrap(),
    );
    let formatted_condition =
        expression_formatter::format_expression(code_formatter, if_node.condition().to_owned());
    let formatted_then_token = code_formatter.format_symbol(
        if_node.then_token().to_owned(),
        TokenReference::symbol(" then").unwrap(),
    );
    let formatted_end_token = code_formatter.format_symbol(
        if_node.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    let formatted_else_if = match if_node.else_if() {
        Some(else_if) => Some(
            else_if
                .iter()
                .map(|else_if| format_else_if(code_formatter, else_if.to_owned()))
                .collect(),
        ),
        None => None,
    };

    let formatted_else_token = match if_node.else_token() {
        Some(token) => Some(
            code_formatter.format_symbol(token.to_owned(), TokenReference::symbol("else").unwrap()),
        ),
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
pub fn format_numeric_for<'ast>(
    code_formatter: &mut CodeFormatter,
    numeric_for: NumericFor<'ast>,
) -> NumericFor<'ast> {
    let for_token = code_formatter.format_symbol(
        numeric_for.for_token().to_owned(),
        TokenReference::symbol("for ").unwrap(),
    );
    let formatted_index_variable = Cow::Owned(
        code_formatter.format_plain_token_reference(numeric_for.index_variable().to_owned()),
    );
    #[cfg(feature = "luau")]
    let type_specifier = match numeric_for.type_specifier() {
        Some(type_specifier) => Some(luau_formatter::format_type_specifier(
            code_formatter,
            type_specifier.to_owned(),
        )),
        None => None,
    };

    #[cfg(feature = "luau")]
    let numeric_for = numeric_for.with_type_specifier(type_specifier);

    let equal_token = code_formatter.format_symbol(
        numeric_for.equal_token().to_owned(),
        TokenReference::symbol(" = ").unwrap(),
    );
    let formatted_start_expression =
        expression_formatter::format_expression(code_formatter, numeric_for.start().to_owned());
    let start_end_comma = code_formatter.format_symbol(
        numeric_for.start_end_comma().to_owned(),
        TokenReference::symbol(", ").unwrap(),
    );
    let formatted_end_expression =
        expression_formatter::format_expression(code_formatter, numeric_for.end().to_owned());

    let (end_step_comma, formatted_step_expression) = match numeric_for.step() {
        Some(step) => (
            Some(code_formatter.format_symbol(
                numeric_for.end_step_comma().unwrap().to_owned(),
                TokenReference::symbol(", ").unwrap(),
            )),
            Some(expression_formatter::format_expression(
                code_formatter,
                step.to_owned(),
            )),
        ),
        None => (None, None),
    };

    let do_token = code_formatter.format_symbol(
        numeric_for.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = code_formatter.format_symbol(
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
pub fn format_repeat_block<'ast>(
    code_formatter: &mut CodeFormatter,
    repeat_block: Repeat<'ast>,
) -> Repeat<'ast> {
    let repeat_token = code_formatter.format_symbol(
        repeat_block.repeat_token().to_owned(),
        TokenReference::symbol("repeat").unwrap(),
    );
    let until_token = code_formatter.format_symbol(
        repeat_block.until_token().to_owned(),
        TokenReference::symbol("until ").unwrap(),
    );
    let formatted_until =
        expression_formatter::format_expression(code_formatter, repeat_block.until().to_owned());

    repeat_block
        .with_repeat_token(repeat_token)
        .with_until_token(until_token)
        .with_until(formatted_until)
}

/// Format a While node
pub fn format_while_block<'ast>(
    code_formatter: &mut CodeFormatter,
    while_block: While<'ast>,
) -> While<'ast> {
    let while_token = code_formatter.format_symbol(
        while_block.while_token().to_owned(),
        TokenReference::symbol("while ").unwrap(),
    );
    let formatted_condition =
        expression_formatter::format_expression(code_formatter, while_block.condition().to_owned());
    let do_token = code_formatter.format_symbol(
        while_block.do_token().to_owned(),
        TokenReference::symbol(" do").unwrap(),
    );
    let end_token = code_formatter.format_symbol(
        while_block.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    while_block
        .with_while_token(while_token)
        .with_condition(formatted_condition)
        .with_do_token(do_token)
        .with_end_token(end_token)
}

pub fn format_stmt<'ast>(code_formatter: &mut CodeFormatter, stmt: Stmt<'ast>) -> Stmt<'ast> {
    match stmt {
        Stmt::Assignment(assignment) => Stmt::Assignment(assignment_formatter::format_assignment(
            code_formatter,
            assignment,
        )),
        Stmt::Do(do_block) => Stmt::Do(format_do_block(code_formatter, do_block)),
        Stmt::FunctionCall(function_call) => Stmt::FunctionCall(
            functions_formatter::format_function_call(code_formatter, function_call),
        ),
        Stmt::FunctionDeclaration(function_declaration) => Stmt::FunctionDeclaration(
            functions_formatter::format_function_declaration(code_formatter, function_declaration),
        ),
        Stmt::GenericFor(generic_for) => {
            Stmt::GenericFor(format_generic_for(code_formatter, generic_for))
        }
        Stmt::If(if_node) => Stmt::If(format_if(code_formatter, if_node)),
        Stmt::LocalAssignment(local_assignment) => Stmt::LocalAssignment(
            assignment_formatter::format_local_assignment(code_formatter, local_assignment),
        ),
        Stmt::LocalFunction(local_function) => Stmt::LocalFunction(
            functions_formatter::format_local_function(code_formatter, local_function),
        ),
        Stmt::NumericFor(numeric_for) => {
            Stmt::NumericFor(format_numeric_for(code_formatter, numeric_for))
        }
        Stmt::Repeat(repeat) => Stmt::Repeat(format_repeat_block(code_formatter, repeat)),
        Stmt::While(while_block) => Stmt::While(format_while_block(code_formatter, while_block)),
        #[cfg(feature = "luau")]
        Stmt::CompoundAssignment(compound_assignment) => Stmt::CompoundAssignment(
            luau_formatter::format_compound_assignment(code_formatter, compound_assignment),
        ),
        #[cfg(feature = "luau")]
        Stmt::ExportedTypeDeclaration(exported_type_declaration) => {
            Stmt::ExportedTypeDeclaration(luau_formatter::format_exported_type_declaration(
                code_formatter,
                exported_type_declaration,
            ))
        }
        #[cfg(feature = "luau")]
        Stmt::TypeDeclaration(type_declaration) => Stmt::TypeDeclaration(
            luau_formatter::format_type_declaration(code_formatter, type_declaration),
        ),
    }
}

pub fn get_token_range<'ast>(token: &Token<'ast>) -> (usize, usize) {
    (token.start_position().bytes(), token.end_position().bytes())
}

pub fn get_range_in_expression<'ast>(expression: &Expression<'ast>) -> (usize, usize) {
    match expression {
        Expression::Parentheses { contained, .. } => {
            get_token_range(contained.tokens().0)
        }
        Expression::UnaryOperator { unop, .. } => {
            match unop {
                full_moon::ast::UnOp::Minus(token_reference) => get_token_range(token_reference.token()),
                full_moon::ast::UnOp::Not(token_reference) => get_token_range(token_reference.token()),
                full_moon::ast::UnOp::Hash(token_reference) => get_token_range(token_reference.token()),
            }
        }
        Expression::Value { value, .. } => {
            let value = &**value;
            match value {
                full_moon::ast::Value::Function((token_ref, _)) => get_token_range(token_ref.token()),
                full_moon::ast::Value::FunctionCall(function_call) => get_range_in_prefix(function_call.prefix()),
                full_moon::ast::Value::TableConstructor(table_constructor) => get_token_range(table_constructor.braces().tokens().0.token()),
                full_moon::ast::Value::Number(token_ref) => get_token_range(token_ref.token()),
                full_moon::ast::Value::ParseExpression(expr) => get_range_in_expression(&expr),
                full_moon::ast::Value::String(token_ref) => get_token_range(token_ref.token()),
                full_moon::ast::Value::Symbol(token_ref) => get_token_range(token_ref.token()),
                full_moon::ast::Value::Var(var) => match var {
                    full_moon::ast::Var::Name(token_ref) => get_token_range(token_ref.token()),
                    full_moon::ast::Var::Expression(var_expr) => get_range_in_prefix(var_expr.prefix()),
                }
            }
        }
    }
}

pub fn get_range_in_prefix<'ast>(prefix: &Prefix) -> (usize, usize) {
    match prefix {    
        Prefix::Name(token) => get_token_range(token.token()),
        Prefix::Expression(expression) => get_range_in_expression(expression),
    }
}

/// Returns an arbitrary token inside of the stmt, to see if it falls inside of an indent range.
/// The token returned does not matter, as we will be using the position of it, and if this token falls within the range, then the whole statement must do
fn get_range_in_stmt<'ast>(stmt: Stmt<'ast>) -> (usize, usize) {
    match stmt {
        Stmt::Assignment(assignment) => get_token_range(assignment.equal_token().token()),
        Stmt::Do(do_block) => get_token_range(do_block.do_token().token()),
        Stmt::FunctionCall(function_call) => get_range_in_prefix(function_call.prefix()),
        Stmt::FunctionDeclaration(function_declaration) => get_token_range(function_declaration.function_token().token()),
        Stmt::GenericFor(generic_for) => get_token_range(generic_for.for_token().token()),
        Stmt::If(if_block) => get_token_range(if_block.if_token().token()),
        Stmt::LocalAssignment(local_assignment) => get_token_range(local_assignment.local_token().token()),
        Stmt::LocalFunction(local_function) => get_token_range(local_function.local_token().token()),
        Stmt::NumericFor(numeric_for) => get_token_range(numeric_for.for_token().token()),
        Stmt::Repeat(repeat_block) => get_token_range(repeat_block.repeat_token().token()),
        Stmt::While(while_block) => get_token_range(while_block.while_token().token()),
        #[cfg(feature = "luau")]
        Stmt::CompoundAssignment(compound_assignment) => get_range_in_expression(compound_assignment.rhs()),
        #[cfg(feature = "luau")]
        Stmt::ExportedTypeDeclaration(exported_type_declaration) => get_token_range(exported_type_declaration.export_token().token()),
        #[cfg(feature = "luau")]
        Stmt::TypeDeclaration(type_declaration) => get_token_range(type_declaration.type_token().token())
    }
}

pub fn stmt_add_trivia<'ast>(code_formatter: &CodeFormatter, stmt: Stmt<'ast>, additional_indent_level: Option<usize>) -> Stmt<'ast> {
    let leading_trivia = vec![code_formatter.create_indent_trivia(additional_indent_level)];
    let trailing_trivia = vec![code_formatter.create_newline_trivia()];

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

        #[cfg(feature = "luau")]
        Stmt::CompoundAssignment(compound_assignment) => {
            Stmt::CompoundAssignment(trivia_formatter::compound_assignment_add_trivia(
                compound_assignment,
                leading_trivia,
                trailing_trivia,
            ))
        }

        #[cfg(feature = "luau")]
        Stmt::ExportedTypeDeclaration(exported_type_declaration) => {
            Stmt::ExportedTypeDeclaration(trivia_formatter::exported_type_declaration_add_trivia(
                exported_type_declaration,
                leading_trivia,
                trailing_trivia,
            ))
        }

        #[cfg(feature = "luau")]
        Stmt::TypeDeclaration(type_declaration) => {
            Stmt::TypeDeclaration(trivia_formatter::type_declaration_add_trivia(
                type_declaration,
                Some(leading_trivia),
                Some(trailing_trivia),
            ))
        }
    }
}

pub fn format_return<'ast>(
    code_formatter: &mut CodeFormatter,
    return_node: Return<'ast>,
) -> Return<'ast> {
    let formatted_returns = code_formatter.format_punctuated(
        return_node.returns().to_owned(),
        &expression_formatter::format_expression,
    );
    let wanted_token: TokenReference<'ast> = if formatted_returns.is_empty() {
        TokenReference::symbol("return").unwrap()
    } else {
        TokenReference::symbol("return ").unwrap()
    };
    let formatted_token =
        code_formatter.format_symbol(return_node.token().to_owned(), wanted_token);
    return_node
        .with_token(formatted_token)
        .with_returns(formatted_returns)
}

pub fn format_last_stmt<'ast>(
    code_formatter: &mut CodeFormatter,
    last_stmt: LastStmt<'ast>,
) -> LastStmt<'ast> {
    match last_stmt {
        LastStmt::Break(token) => LastStmt::Break(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol("break").unwrap()),
        ),
        LastStmt::Return(return_node) => {
            LastStmt::Return(format_return(code_formatter, return_node))
        }
        #[cfg(feature = "luau")]
        LastStmt::Continue(token) => LastStmt::Continue(code_formatter.format_symbol(
            token.into_owned(),
            TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("continue")),
                }),
                vec![],
            ),
        )),
    }
}

fn get_range_in_last_stmt<'ast>(last_stmt: &LastStmt<'ast>) -> (usize, usize) {
    match last_stmt {
        LastStmt::Break(token_ref) => get_token_range(token_ref.token()),
        LastStmt::Return(return_node) => get_token_range(return_node.token().token()),
        #[cfg(feature = "luau")]
        LastStmt::Continue(token_ref) => get_token_range(token_ref.token()),
    }
}

pub fn last_stmt_add_trivia<'ast>(
    code_formatter: &CodeFormatter,
    last_stmt: LastStmt<'ast>,
    additional_indent_level: Option<usize>,
) -> LastStmt<'ast> {
    match last_stmt {
        LastStmt::Break(break_node) => {
            LastStmt::Break(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                break_node.into_owned(),
                Some(vec![code_formatter.create_indent_trivia(additional_indent_level)]),
                Some(vec![code_formatter.create_newline_trivia()]),
            )))
        }
        LastStmt::Return(return_node) => {
            let mut token = return_node.token().to_owned();
            let mut returns = return_node.returns().to_owned();

            if return_node.returns().is_empty() {
                token = trivia_formatter::token_reference_add_trivia(
                    token,
                    Some(vec![code_formatter.create_indent_trivia(additional_indent_level)]),
                    Some(vec![code_formatter.create_newline_trivia()]),
                );
            } else {
                token = trivia_formatter::token_reference_add_trivia(
                    token,
                    Some(vec![code_formatter.create_indent_trivia(additional_indent_level)]),
                    None,
                );

                // TODO: This is copied from the Assignment/LocalAssignment formatters
                // Retrieve last item and add new line to it
                if let Some(last_pair) = returns.pop() {
                    match last_pair {
                        Pair::End(value) => {
                            let expression = trivia_formatter::expression_add_trailing_trivia(
                                value,
                                vec![code_formatter.create_newline_trivia()],
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
        #[cfg(feature = "luau")]
        LastStmt::Continue(continue_node) => {
            LastStmt::Continue(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                continue_node.into_owned(),
                Some(vec![code_formatter.create_indent_trivia(additional_indent_level)]),
                Some(vec![code_formatter.create_newline_trivia()]),
            )))
        }
    }
}

pub fn format_block<'ast>(code_formatter: &mut CodeFormatter, block: Block<'ast>) -> Block<'ast> {
    let formatted_statements: Vec<(Stmt<'ast>, Option<Cow<'ast, TokenReference<'ast>>>)> = block
        .iter_stmts()
        .map(|stmt| {
            let range_in_stmt = get_range_in_stmt(stmt.to_owned());
            let additional_indent_level = code_formatter.get_range_indent_increase(range_in_stmt);
            let stmt = format_stmt(code_formatter, stmt.to_owned());
            (
                stmt_add_trivia(code_formatter, stmt, additional_indent_level),
                None, // The second parameter in the tuple is for semicolons - we do not want any semi-colons
            )
        })
        .collect();

    let formatted_last_stmt = match block.last_stmt() {
        Some(last_stmt) => {
            let range_in_last_stmt = get_range_in_last_stmt(last_stmt);
            let additional_indent_level = code_formatter.get_range_indent_increase(range_in_last_stmt);
            let last_stmt = format_last_stmt(code_formatter, last_stmt.to_owned());
            Some((last_stmt_add_trivia(code_formatter, last_stmt, additional_indent_level), None))
        }
        None => None,
    };

    block
        .with_stmts(formatted_statements)
        .with_last_stmt(formatted_last_stmt)
}
