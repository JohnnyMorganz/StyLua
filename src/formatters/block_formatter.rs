use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    CodeFormatter, Range,
};
use full_moon::ast::{
    punctuated::Pair, Block, Expression, LastStmt, Prefix, Return, Stmt, UnOp, Value, Var,
};
#[cfg(feature = "luau")]
use full_moon::tokenizer::TokenType;
use full_moon::tokenizer::{Token, TokenReference};
use std::borrow::Cow;

impl CodeFormatter {
    pub fn get_token_range<'ast>(token: &Token<'ast>) -> Range {
        (token.start_position().bytes(), token.end_position().bytes())
    }

    pub fn get_range_in_expression<'ast>(expression: &Expression<'ast>) -> Range {
        match expression {
            Expression::Parentheses { contained, .. } => {
                CodeFormatter::get_token_range(contained.tokens().0)
            }
            Expression::UnaryOperator { unop, .. } => match unop {
                UnOp::Minus(token_reference) => {
                    CodeFormatter::get_token_range(token_reference.token())
                }
                UnOp::Not(token_reference) => {
                    CodeFormatter::get_token_range(token_reference.token())
                }
                UnOp::Hash(token_reference) => {
                    CodeFormatter::get_token_range(token_reference.token())
                }
            },
            Expression::Value { value, .. } => {
                let value = &**value;
                match value {
                    Value::Function((token_ref, _)) => {
                        CodeFormatter::get_token_range(token_ref.token())
                    }
                    Value::FunctionCall(function_call) => {
                        CodeFormatter::get_range_in_prefix(function_call.prefix())
                    }
                    Value::TableConstructor(table_constructor) => CodeFormatter::get_token_range(
                        table_constructor.braces().tokens().0.token(),
                    ),
                    Value::Number(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
                    Value::ParseExpression(expr) => CodeFormatter::get_range_in_expression(&expr),
                    Value::String(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
                    Value::Symbol(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
                    Value::Var(var) => match var {
                        Var::Name(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
                        Var::Expression(var_expr) => {
                            CodeFormatter::get_range_in_prefix(var_expr.prefix())
                        }
                    },
                }
            }
        }
    }

    pub fn get_range_in_prefix(prefix: &Prefix) -> Range {
        match prefix {
            Prefix::Name(token) => CodeFormatter::get_token_range(token.token()),
            Prefix::Expression(expression) => CodeFormatter::get_range_in_expression(expression),
        }
    }

    /// Returns an arbitrary token inside of the stmt, to see if it falls inside of an indent range.
    /// The token returned does not matter, as we will be using the position of it, and if this token falls within the range, then the whole statement must do
    fn get_range_in_stmt(stmt: Stmt) -> Range {
        match stmt {
            Stmt::Assignment(assignment) => {
                CodeFormatter::get_token_range(assignment.equal_token().token())
            }
            Stmt::Do(do_block) => CodeFormatter::get_token_range(do_block.do_token().token()),
            Stmt::FunctionCall(function_call) => {
                CodeFormatter::get_range_in_prefix(function_call.prefix())
            }
            Stmt::FunctionDeclaration(function_declaration) => {
                CodeFormatter::get_token_range(function_declaration.function_token().token())
            }
            Stmt::GenericFor(generic_for) => {
                CodeFormatter::get_token_range(generic_for.for_token().token())
            }
            Stmt::If(if_block) => CodeFormatter::get_token_range(if_block.if_token().token()),
            Stmt::LocalAssignment(local_assignment) => {
                CodeFormatter::get_token_range(local_assignment.local_token().token())
            }
            Stmt::LocalFunction(local_function) => {
                CodeFormatter::get_token_range(local_function.local_token().token())
            }
            Stmt::NumericFor(numeric_for) => {
                CodeFormatter::get_token_range(numeric_for.for_token().token())
            }
            Stmt::Repeat(repeat_block) => {
                CodeFormatter::get_token_range(repeat_block.repeat_token().token())
            }
            Stmt::While(while_block) => {
                CodeFormatter::get_token_range(while_block.while_token().token())
            }
            #[cfg(feature = "luau")]
            Stmt::CompoundAssignment(compound_assignment) => {
                CodeFormatter::get_range_in_expression(compound_assignment.rhs())
            }
            #[cfg(feature = "luau")]
            Stmt::ExportedTypeDeclaration(exported_type_declaration) => {
                CodeFormatter::get_token_range(exported_type_declaration.export_token().token())
            }
            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(type_declaration) => {
                CodeFormatter::get_token_range(type_declaration.type_token().token())
            }
        }
    }

    pub fn stmt_add_trivia<'ast>(
        &self,
        stmt: Stmt<'ast>,
        additional_indent_level: Option<usize>,
    ) -> Stmt<'ast> {
        let leading_trivia =
            FormatTriviaType::Append(vec![self.create_indent_trivia(additional_indent_level)]);
        let trailing_trivia = FormatTriviaType::Append(vec![self.create_newline_trivia()]);

        match stmt {
            Stmt::Assignment(assignment) => {
                Stmt::Assignment(trivia_formatter::assignment_add_trivia(
                    assignment,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
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
            Stmt::GenericFor(generic_for) => {
                Stmt::GenericFor(trivia_formatter::generic_for_add_trivia(
                    generic_for,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
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
            Stmt::NumericFor(numeric_for) => {
                Stmt::NumericFor(trivia_formatter::numeric_for_add_trivia(
                    numeric_for,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
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
                Stmt::ExportedTypeDeclaration(
                    trivia_formatter::exported_type_declaration_add_trivia(
                        exported_type_declaration,
                        leading_trivia,
                        trailing_trivia,
                    ),
                )
            }

            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(type_declaration) => {
                Stmt::TypeDeclaration(trivia_formatter::type_declaration_add_trivia(
                    type_declaration,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
        }
    }

    pub fn format_return<'ast>(&mut self, return_node: Return<'ast>) -> Return<'ast> {
        let formatted_returns = self.format_punctuated(
            return_node.returns().to_owned(),
            &CodeFormatter::format_expression,
        );
        let wanted_token: TokenReference<'ast> = if formatted_returns.is_empty() {
            TokenReference::symbol("return").unwrap()
        } else {
            TokenReference::symbol("return ").unwrap()
        };
        let formatted_token = self.format_symbol(return_node.token().to_owned(), wanted_token);
        return_node
            .with_token(formatted_token)
            .with_returns(formatted_returns)
    }

    pub fn format_last_stmt<'ast>(&mut self, last_stmt: LastStmt<'ast>) -> LastStmt<'ast> {
        match last_stmt {
            LastStmt::Break(token) => LastStmt::Break(
                self.format_symbol(token.into_owned(), TokenReference::symbol("break").unwrap()),
            ),
            LastStmt::Return(return_node) => LastStmt::Return(self.format_return(return_node)),
            #[cfg(feature = "luau")]
            LastStmt::Continue(token) => LastStmt::Continue(self.format_symbol(
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

    fn get_range_in_last_stmt<'ast>(last_stmt: &LastStmt<'ast>) -> Range {
        match last_stmt {
            LastStmt::Break(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
            LastStmt::Return(return_node) => {
                CodeFormatter::get_token_range(return_node.token().token())
            }
            #[cfg(feature = "luau")]
            LastStmt::Continue(token_ref) => CodeFormatter::get_token_range(token_ref.token()),
        }
    }

    pub fn last_stmt_add_trivia<'ast>(
        &self,
        last_stmt: LastStmt<'ast>,
        additional_indent_level: Option<usize>,
    ) -> LastStmt<'ast> {
        match last_stmt {
            LastStmt::Break(break_node) => {
                LastStmt::Break(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    break_node.into_owned(),
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ]),
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                )))
            }
            LastStmt::Return(return_node) => {
                let mut token = return_node.token().to_owned();
                let mut returns = return_node.returns().to_owned();

                if return_node.returns().is_empty() {
                    token = trivia_formatter::token_reference_add_trivia(
                        token,
                        FormatTriviaType::Append(vec![
                            self.create_indent_trivia(additional_indent_level)
                        ]),
                        FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                    );
                } else {
                    token = trivia_formatter::token_reference_add_trivia(
                        token,
                        FormatTriviaType::Append(vec![
                            self.create_indent_trivia(additional_indent_level)
                        ]),
                        FormatTriviaType::NoChange,
                    );

                    // TODO: This is copied from the Assignment/LocalAssignment formatters
                    // Retrieve last item and add new line to it
                    if let Some(last_pair) = returns.pop() {
                        match last_pair {
                            Pair::End(value) => {
                                let expression = trivia_formatter::expression_add_trailing_trivia(
                                    value,
                                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
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
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ]),
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                )))
            }
        }
    }

    pub fn format_block<'ast>(&mut self, block: Block<'ast>) -> Block<'ast> {
        let formatted_statements: Vec<(Stmt<'ast>, Option<Cow<'ast, TokenReference<'ast>>>)> =
            block
                .iter_stmts()
                .map(|stmt| {
                    let range_in_stmt = CodeFormatter::get_range_in_stmt(stmt.to_owned());
                    let additional_indent_level = self.get_range_indent_increase(range_in_stmt);
                    let stmt = self.format_stmt(stmt.to_owned());
                    (
                        self.stmt_add_trivia(stmt, additional_indent_level),
                        None, // The second parameter in the tuple is for semicolons - we do not want any semi-colons
                    )
                })
                .collect();

        let formatted_last_stmt = match block.last_stmt() {
            Some(last_stmt) => {
                let range_in_last_stmt = CodeFormatter::get_range_in_last_stmt(last_stmt);
                let additional_indent_level = self.get_range_indent_increase(range_in_last_stmt);
                let last_stmt = self.format_last_stmt(last_stmt.to_owned());
                Some((
                    self.last_stmt_add_trivia(last_stmt, additional_indent_level),
                    None,
                ))
            }
            None => None,
        };

        block
            .with_stmts(formatted_statements)
            .with_last_stmt(formatted_last_stmt)
    }
}
