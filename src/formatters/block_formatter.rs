use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    trivia_util, CodeFormatter, Range,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Block, Expression, LastStmt, Prefix, Return, Stmt, UnOp, Value, Var,
};
use full_moon::node::Node;
use full_moon::tokenizer::TokenType;
use full_moon::tokenizer::{Token, TokenReference};
use std::borrow::Cow;

macro_rules! update_first_token {
    ($enum:ident, $var:ident, $token:expr, $update_method:ident) => {{
        let leading_trivia =
            CodeFormatter::trivia_remove_leading_newlines($token.leading_trivia().collect());
        let new_token = trivia_formatter::token_reference_add_trivia(
            $token.to_owned(),
            FormatTriviaType::Replace(leading_trivia),
            FormatTriviaType::NoChange,
        );
        Stmt::$enum($var.$update_method(Cow::Owned(new_token)))
    }};
}
impl CodeFormatter {
    pub fn get_token_range(token: &Token) -> Range {
        (token.start_position().bytes(), token.end_position().bytes())
    }

    pub fn get_range_in_expression(expression: &Expression) -> Range {
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

    pub fn format_return<'ast>(&mut self, return_node: &Return<'ast>) -> Return<'ast> {
        // Calculate trivia
        let additional_indent_level =
            self.get_range_indent_increase(CodeFormatter::get_token_range(return_node.token()));
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let mut trailing_trivia = vec![self.create_newline_trivia()];

        let (mut formatted_returns, mut comments_buf) =
            self.format_punctuated(return_node.returns(), &CodeFormatter::format_expression);

        let formatted_token = if formatted_returns.is_empty() {
            trivia_formatter::token_reference_add_trivia(
                crate::fmt_symbol!(self, return_node.token(), "return").into_owned(),
                FormatTriviaType::Append(leading_trivia),
                FormatTriviaType::Append(trailing_trivia),
            )
        } else {
            // Determine if we need to hang the condition
            let first_line_str =
                trivia_formatter::no_comments(return_node.token()) + &formatted_returns.to_string();
            let indent_spacing = (self.indent_level + additional_indent_level.unwrap_or(0))
                * self.config.indent_width;
            let require_multiline_expression = (indent_spacing
                + first_line_str
                    .trim()
                    .lines()
                    .next()
                    .expect("no lines")
                    .len())
                > self.config.column_width;

            if require_multiline_expression {
                // Add the expression list into the indent range, as it will be indented by one
                let expr_range = return_node.returns().range().expect("no range for returns");
                self.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

                // Hang each expression
                let mut new_list = Punctuated::new();
                for pair in return_node.returns().pairs() {
                    let expr = self.format_expression(pair.value());
                    let value = self.hang_expression_no_trailing_newline(
                        expr,
                        additional_indent_level,
                        None,
                    );
                    new_list.push(Pair::new(
                        value,
                        pair.punctuation()
                            .map(|x| crate::fmt_symbol!(self, x, ", ")),
                    ));
                }
                formatted_returns = new_list
            }

            // Append any trailing trivia (incl. comments buffer) to the end of the last return
            comments_buf.append(&mut trailing_trivia);
            if let Some(pair) = formatted_returns.pop() {
                let pair = pair.map(|expr| {
                    trivia_formatter::expression_add_trailing_trivia(
                        expr,
                        FormatTriviaType::Append(comments_buf),
                    )
                });
                formatted_returns.push(pair);
            }

            trivia_formatter::token_reference_add_trivia(
                crate::fmt_symbol!(self, return_node.token(), "return ").into_owned(),
                FormatTriviaType::Append(leading_trivia),
                FormatTriviaType::NoChange,
            )
        };

        Return::new()
            .with_token(Cow::Owned(formatted_token))
            .with_returns(formatted_returns)
    }

    pub fn format_last_stmt<'ast>(&mut self, last_stmt: &LastStmt<'ast>) -> LastStmt<'ast> {
        crate::check_should_format!(self, last_stmt);

        match last_stmt {
            LastStmt::Break(token) => {
                LastStmt::Break(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    crate::fmt_symbol!(self, token, "break").into_owned(),
                    FormatTriviaType::Append(vec![self.create_indent_trivia(
                        self.get_range_indent_increase(CodeFormatter::get_token_range(token)),
                    )]),
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                )))
            }

            LastStmt::Return(return_node) => LastStmt::Return(self.format_return(return_node)),
            #[cfg(feature = "luau")]
            LastStmt::Continue(token) => {
                LastStmt::Continue(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    self.format_symbol(
                        token,
                        &TokenReference::new(
                            vec![],
                            Token::new(TokenType::Identifier {
                                identifier: Cow::Owned(String::from("continue")),
                            }),
                            vec![],
                        ),
                    )
                    .into_owned(),
                    FormatTriviaType::Append(vec![self.create_indent_trivia(
                        self.get_range_indent_increase(CodeFormatter::get_token_range(token)),
                    )]),
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                )))
            }
        }
    }

    fn trivia_remove_leading_newlines<'ast>(trivia: Vec<&Token<'ast>>) -> Vec<Token<'ast>> {
        trivia
            .iter()
            .skip_while(|x| match x.token_type() {
                TokenType::Whitespace { ref characters } => characters.contains('\n'),
                _ => false,
            })
            .map(|x| x.to_owned().to_owned())
            .collect()
    }

    fn prefix_remove_leading_newlines<'ast>(prefix: &Prefix<'ast>) -> Prefix<'ast> {
        match prefix {
            Prefix::Name(token) => {
                let leading_trivia =
                    CodeFormatter::trivia_remove_leading_newlines(token.leading_trivia().collect());
                let new_token = trivia_formatter::token_reference_add_trivia(
                    token.to_owned().into_owned(),
                    FormatTriviaType::Replace(leading_trivia),
                    FormatTriviaType::NoChange,
                );
                Prefix::Name(Cow::Owned(new_token))
            }
            Prefix::Expression(expr) => Prefix::Expression(match expr {
                Expression::Parentheses {
                    contained,
                    expression,
                } => {
                    let (start_parens, end_parens) = contained.tokens();
                    let leading_trivia = CodeFormatter::trivia_remove_leading_newlines(
                        start_parens.leading_trivia().collect(),
                    );
                    let new_token = trivia_formatter::token_reference_add_trivia(
                        start_parens.to_owned(),
                        FormatTriviaType::Replace(leading_trivia),
                        FormatTriviaType::NoChange,
                    );
                    Expression::Parentheses {
                        contained: full_moon::ast::span::ContainedSpan::new(
                            Cow::Owned(new_token),
                            Cow::Owned(end_parens.to_owned()),
                        ),
                        expression: Box::new(*expression.to_owned()),
                    }
                }
                other => {
                    unreachable!("got non-parentheses expression as prefix {:?}", other)
                }
            }),
        }
    }

    fn var_remove_leading_newline(var: Var) -> Var {
        match var {
            Var::Name(token) => {
                let leading_trivia =
                    CodeFormatter::trivia_remove_leading_newlines(token.leading_trivia().collect());
                let new_token = trivia_formatter::token_reference_add_trivia(
                    token.into_owned(),
                    FormatTriviaType::Replace(leading_trivia),
                    FormatTriviaType::NoChange,
                );
                Var::Name(Cow::Owned(new_token))
            }
            Var::Expression(var_expr) => {
                let prefix = CodeFormatter::prefix_remove_leading_newlines(var_expr.prefix());
                Var::Expression(var_expr.with_prefix(prefix))
            }
        }
    }

    fn stmt_remove_leading_newlines(stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Assignment(assignment) => {
                let mut var_list = Punctuated::new();

                for (idx, pair) in assignment.var_list().pairs().enumerate() {
                    if idx == 0 {
                        let pair = pair
                            .to_owned()
                            .map(CodeFormatter::var_remove_leading_newline);
                        var_list.push(pair);
                    } else {
                        var_list.push(pair.to_owned());
                    }
                }

                Stmt::Assignment(assignment.with_var_list(var_list))
            }
            Stmt::Do(do_block) => {
                update_first_token!(Do, do_block, do_block.do_token(), with_do_token)
            }
            Stmt::FunctionCall(function_call) => {
                let prefix = CodeFormatter::prefix_remove_leading_newlines(function_call.prefix());
                Stmt::FunctionCall(function_call.with_prefix(prefix))
            }
            Stmt::FunctionDeclaration(function_declaration) => {
                update_first_token!(
                    FunctionDeclaration,
                    function_declaration,
                    function_declaration.function_token(),
                    with_function_token
                )
            }
            Stmt::GenericFor(generic_for) => update_first_token!(
                GenericFor,
                generic_for,
                generic_for.for_token(),
                with_for_token
            ),
            Stmt::If(if_block) => {
                update_first_token!(If, if_block, if_block.if_token(), with_if_token)
            }
            Stmt::LocalAssignment(local_assignment) => update_first_token!(
                LocalAssignment,
                local_assignment,
                local_assignment.local_token(),
                with_local_token
            ),
            Stmt::LocalFunction(local_function) => update_first_token!(
                LocalFunction,
                local_function,
                local_function.local_token(),
                with_local_token
            ),
            Stmt::NumericFor(numeric_for) => update_first_token!(
                NumericFor,
                numeric_for,
                numeric_for.for_token(),
                with_for_token
            ),
            Stmt::Repeat(repeat_block) => {
                update_first_token!(
                    Repeat,
                    repeat_block,
                    repeat_block.repeat_token(),
                    with_repeat_token
                )
            }
            Stmt::While(while_block) => {
                update_first_token!(
                    While,
                    while_block,
                    while_block.while_token(),
                    with_while_token
                )
            }
            #[cfg(feature = "luau")]
            Stmt::CompoundAssignment(compound_assignment) => {
                let lhs =
                    CodeFormatter::var_remove_leading_newline(compound_assignment.lhs().to_owned());
                Stmt::CompoundAssignment(compound_assignment.with_lhs(lhs))
            }

            #[cfg(feature = "luau")]
            Stmt::ExportedTypeDeclaration(exported_type_declaration) => update_first_token!(
                ExportedTypeDeclaration,
                exported_type_declaration,
                exported_type_declaration.export_token(),
                with_export_token
            ),
            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(type_declaration) => update_first_token!(
                TypeDeclaration,
                type_declaration,
                type_declaration.type_token(),
                with_type_token
            ),
        }
    }

    fn last_stmt_remove_leading_newlines(last_stmt: LastStmt) -> LastStmt {
        match last_stmt {
            LastStmt::Break(token) => {
                let leading_trivia =
                    CodeFormatter::trivia_remove_leading_newlines(token.leading_trivia().collect());
                let new_token = trivia_formatter::token_reference_add_trivia(
                    token.into_owned(),
                    FormatTriviaType::Replace(leading_trivia),
                    FormatTriviaType::NoChange,
                );
                LastStmt::Break(Cow::Owned(new_token))
            }
            LastStmt::Return(return_node) => {
                let old_token = return_node.token();
                let token = Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    old_token.to_owned(),
                    FormatTriviaType::Replace(CodeFormatter::trivia_remove_leading_newlines(
                        old_token.leading_trivia().collect(),
                    )),
                    FormatTriviaType::NoChange,
                ));

                LastStmt::Return(return_node.with_token(token))
            }
            #[cfg(feature = "luau")]
            LastStmt::Continue(token) => {
                let leading_trivia =
                    CodeFormatter::trivia_remove_leading_newlines(token.leading_trivia().collect());
                let new_token = trivia_formatter::token_reference_add_trivia(
                    token.into_owned(),
                    FormatTriviaType::Replace(leading_trivia),
                    FormatTriviaType::NoChange,
                );
                LastStmt::Continue(Cow::Owned(new_token))
            }
        }
    }

    pub fn format_block<'ast>(&mut self, block: Block<'ast>) -> Block<'ast> {
        let mut formatted_statements: Vec<(Stmt<'ast>, Option<Cow<'ast, TokenReference<'ast>>>)> =
            Vec::new();
        let mut found_first_stmt = false;
        let mut stmt_iterator = block.iter_stmts().peekable();
        while let Some(stmt) = stmt_iterator.next() {
            let mut stmt = self.format_stmt(stmt);

            // If this is the first stmt, then remove any leading newlines
            if !found_first_stmt {
                if self.should_format_node(&stmt) {
                    stmt = CodeFormatter::stmt_remove_leading_newlines(stmt);
                }
                found_first_stmt = true;
            }

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
                        Some(Stmt::FunctionCall(function_call)) => matches!(
                            function_call.prefix(),
                            Prefix::Expression(Expression::Parentheses { .. })
                        ),
                        _ => false,
                    }
                }
                _ => false,
            };

            // If we have a semicolon, we need to push all the trailing trivia from the statement
            // and move it to the end of the semicolon
            let semicolon = match require_semicolon {
                true => {
                    let (updated_stmt, trivia) = trivia_util::get_stmt_trailing_trivia(stmt);
                    stmt = updated_stmt;
                    Some(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                        TokenReference::symbol(";").expect("could not make semicolon"),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(trivia),
                    )))
                }
                false => None,
            };

            formatted_statements.push((stmt, semicolon))
        }

        // Drop the stmt_iterator as we do not need it anymore and we still need to use `block`
        drop(stmt_iterator);

        let formatted_last_stmt = match block.last_stmt() {
            Some(last_stmt) => {
                let mut last_stmt = self.format_last_stmt(last_stmt);
                // If this is the first stmt, then remove any leading newlines
                if !found_first_stmt && self.should_format_node(&last_stmt) {
                    last_stmt = CodeFormatter::last_stmt_remove_leading_newlines(last_stmt);
                }
                Some((last_stmt, None))
            }
            None => None,
        };

        block
            .with_stmts(formatted_statements)
            .with_last_stmt(formatted_last_stmt)
    }
}
