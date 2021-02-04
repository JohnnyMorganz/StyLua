use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    trivia_util, CodeFormatter, Range,
};
use full_moon::ast::{Block, Expression, LastStmt, Prefix, Return, Stmt, UnOp, Value, Var};
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
    fn get_range_in_stmt(stmt: &Stmt) -> Range {
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

    pub fn format_return<'ast>(&mut self, return_node: &Return<'ast>) -> Return<'ast> {
        let (mut formatted_returns, comments_buf) =
            self.format_punctuated(return_node.returns(), &CodeFormatter::format_expression);

        let wanted_token: TokenReference<'ast> = if formatted_returns.is_empty() {
            TokenReference::symbol("return").unwrap()
        } else {
            // Append the comments buffer to the last return
            if let Some(pair) = formatted_returns.pop() {
                let pair = pair.map(|expr| {
                    trivia_formatter::expression_add_trailing_trivia(
                        expr,
                        FormatTriviaType::Append(comments_buf),
                    )
                });
                formatted_returns.push(pair);
            }

            TokenReference::symbol("return ").unwrap()
        };
        let formatted_token = self.format_symbol(return_node.token(), &wanted_token);

        Return::new()
            .with_token(formatted_token)
            .with_returns(formatted_returns)
    }

    pub fn format_last_stmt<'ast>(&mut self, last_stmt: LastStmt<'ast>) -> LastStmt<'ast> {
        match last_stmt {
            LastStmt::Break(token) => LastStmt::Break(crate::fmt_symbol!(self, &token, "break")),
            LastStmt::Return(return_node) => LastStmt::Return(self.format_return(&return_node)),
            #[cfg(feature = "luau")]
            LastStmt::Continue(token) => LastStmt::Continue(self.format_symbol(
                &token,
                &TokenReference::new(
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
                LastStmt::Return(self.return_add_trivia(return_node, additional_indent_level))
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
        let mut formatted_statements: Vec<(Stmt<'ast>, Option<Cow<'ast, TokenReference<'ast>>>)> =
            Vec::new();

        let mut stmt_iterator = block.iter_stmts().peekable();
        while let Some(stmt) = stmt_iterator.next() {
            let range_in_stmt = CodeFormatter::get_range_in_stmt(stmt);
            let additional_indent_level = self.get_range_indent_increase(range_in_stmt);
            let stmt = self.format_stmt(stmt);

            // Need to check next statement if it is a function call, with a parameters expression as the prefix
            // If so, removing a semicolon may lead to ambiguous syntax
            // Ambiguous syntax can only occur if the current statement is a (Local)Assignment, FunctionCall or a Repeat block
            let require_semicolon = match stmt {
                Stmt::Assignment(_)
                | Stmt::LocalAssignment(_)
                | Stmt::FunctionCall(_)
                | Stmt::Repeat(_) => {
                    let next_stmt = stmt_iterator.peek();
                    match next_stmt {
                        Some(next_stmt) => match next_stmt {
                            Stmt::FunctionCall(function_call) => match function_call.prefix() {
                                Prefix::Expression(expr) => match expr {
                                    Expression::Parentheses { .. } => true,
                                    _ => false,
                                },
                                _ => false,
                            },
                            _ => false,
                        },
                        None => false,
                    }
                }
                _ => false,
            };

            let mut trivia_stmt = self.stmt_add_trivia(stmt, additional_indent_level);

            // If we have a semicolon, we need to push all the trailing trivia from the statement
            // and move it to the end of the semicolon
            let semicolon = match require_semicolon {
                true => {
                    let (updated_stmt, trivia) = trivia_util::get_stmt_trailing_trivia(trivia_stmt);
                    trivia_stmt = updated_stmt;
                    Some(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                        TokenReference::symbol(";").expect("could not make semicolon"),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(trivia),
                    )))
                }
                false => None,
            };

            formatted_statements.push((trivia_stmt, semicolon))
        }

        // Drop the stmt_iterator as we do not need it anymore and we still need to use `block`
        drop(stmt_iterator);

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
