#[cfg(feature = "luau")]
use crate::formatters::general::format_symbol;
use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context, FormatNode},
    fmt_symbol,
    formatters::{
        assignment::hang_punctuated_list,
        expression::{format_expression, hang_expression},
        general::{format_punctuated, format_punctuated_multiline},
        stmt::format_stmt,
        trivia::{
            strip_trailing_trivia, strip_trivia, FormatTriviaType, UpdateLeadingTrivia,
            UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util,
    },
    shape::Shape,
};
use full_moon::ast::{
    punctuated::Punctuated, Block, Expression, LastStmt, Prefix, Return, Stmt, Value, Var,
};
use full_moon::tokenizer::TokenType;
use full_moon::tokenizer::{Token, TokenReference};

macro_rules! update_first_token {
    ($enum:ident, $var:ident, $token:expr, $update_method:ident) => {{
        let leading_trivia = trivia_remove_leading_newlines($token.leading_trivia().collect());
        let new_token = $token.update_leading_trivia(FormatTriviaType::Replace(leading_trivia));
        Stmt::$enum($var.$update_method(new_token))
    }};
}

fn is_function_or_table_constructor(expression: &Expression) -> bool {
    match expression {
        Expression::Value { value, .. } => {
            matches!(&**value, Value::TableConstructor(_) | Value::Function(_))
        }
        _ => false,
    }
}

pub fn format_return(ctx: &Context, return_node: &Return, shape: Shape) -> Return {
    if return_node.returns().is_empty() {
        Return::new().with_token(fmt_symbol!(ctx, return_node.token(), "return", shape))
    } else {
        const RETURN_LEN: usize = "return ".len();

        let token = fmt_symbol!(ctx, return_node.token(), "return ", shape);
        let shape = shape + RETURN_LEN;

        let returns = return_node.returns();

        let contains_comments = trivia_util::contains_comments(
            returns.update_trailing_trivia(FormatTriviaType::Replace(Vec::new())), // We can ignore trailing trivia, as that won't affect anything
        );

        // See if we need to format multiline
        // If we contain comments, we immediately force multiline, and return an empty Punctuated sequence as a placeholder (it will never be used)
        // If not, format the sequence on a single line, and test the shape. We return the singleline output in case we want to use it.
        // We do it this way so that the singleline return is evaluated lazily - we don't want to create it if we never use it, but if we
        // create it, we need to keep it in case we want to use it.
        let (should_format_multiline, singleline_returns) = if contains_comments {
            (true, Punctuated::new())
        } else {
            // Special case:
            // The singleline returns is full of multiline tables or anonymous functions
            // If so, we should just format inline, normally.
            if returns.iter().all(is_function_or_table_constructor) {
                (
                    false,
                    format_punctuated(ctx, returns, shape, format_expression),
                )
            } else {
                // Firstly attempt to format the returns onto a single line, using an infinite column width shape
                let singleline_returns =
                    format_punctuated(ctx, returns, shape.with_infinite_width(), format_expression);

                // Test the return to see if its over width
                let singleline_shape =
                    shape + strip_trailing_trivia(&singleline_returns).to_string().len();
                (singleline_shape.over_budget(), singleline_returns)
            }
        };

        // TODO: this is similar to assignment tactics - can we abstract them into a common function?
        let formatted_returns = if should_format_multiline {
            if returns.len() > 1 {
                // Format the punctuated onto multiple lines
                let hang_level = Some(1);
                let multiline_returns =
                    format_punctuated_multiline(ctx, returns, shape, format_expression, hang_level);

                let mut output_returns = Punctuated::new();

                // Look through each punctuated sequence to see if we need to hang the item further
                for (idx, (formatted, original)) in
                    multiline_returns.into_pairs().zip(returns).enumerate()
                {
                    // Recreate the shape
                    let shape = if idx == 0 {
                        shape
                    } else {
                        shape
                            .reset()
                            .with_indent(shape.indent().add_indent_level(hang_level.unwrap()))
                    };

                    if trivia_util::contains_comments(&formatted)
                        || shape.take_first_line(&formatted).over_budget()
                    {
                        // Hang the pair, using the original expression for formatting
                        output_returns
                            .push(formatted.map(|_| hang_expression(ctx, original, shape, Some(1))))
                    } else {
                        // Add the pair as it is
                        output_returns.push(formatted);
                    }
                }

                output_returns
            } else {
                // Create an example hanging the expression - we need to create a new context so that we don't overwrite it
                let hanging_returns = hang_punctuated_list(ctx, returns, shape);
                let hanging_shape = shape.take_first_line(&strip_trivia(&hanging_returns));

                // Create an example formatting the expression normally
                let formatted_returns = format_punctuated(ctx, returns, shape, format_expression);
                let formatting_shape =
                    shape.take_first_line(&strip_trailing_trivia(&formatted_returns));

                // Find the better format out of the hanging shape or the normal formatting
                if hanging_shape.used_width() < formatting_shape.used_width() {
                    // Hanging version is better
                    hanging_returns
                } else {
                    formatted_returns
                }
            }
        } else {
            singleline_returns
        };

        Return::new()
            .with_token(token)
            .with_returns(formatted_returns)
    }
}

// Only formats a block within the last stmt
fn format_last_stmt_block(ctx: &Context, last_stmt: &LastStmt, shape: Shape) -> LastStmt {
    match last_stmt {
        LastStmt::Return(return_node) => {
            let returns = return_node
                .returns()
                .pairs()
                .map(|pair| {
                    pair.to_owned().map(|expression| {
                        let shape = shape.reset().increment_block_indent();
                        super::stmt::stmt_block::format_expression_block(ctx, &expression, shape)
                    })
                })
                .collect();

            LastStmt::Return(return_node.to_owned().with_returns(returns))
        }
        other => other.to_owned(),
    }
}

pub fn format_last_stmt_no_trivia(ctx: &Context, last_stmt: &LastStmt, shape: Shape) -> LastStmt {
    match last_stmt {
        LastStmt::Break(token) => LastStmt::Break(fmt_symbol!(ctx, token, "break", shape)),

        LastStmt::Return(return_node) => LastStmt::Return(format_return(ctx, return_node, shape)),
        #[cfg(feature = "luau")]
        LastStmt::Continue(token) => LastStmt::Continue(format_symbol(
            ctx,
            token,
            &TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: "continue".into(),
                }),
                vec![],
            ),
            shape,
        )),

        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_last_stmt(ctx: &Context, last_stmt: &LastStmt, shape: Shape) -> LastStmt {
    let should_format = ctx.should_format_node(last_stmt);
    if let FormatNode::Skip = should_format {
        return last_stmt.to_owned();
    } else if let FormatNode::NotInRange = should_format {
        return format_last_stmt_block(ctx, last_stmt, shape);
    }

    // Calculate trivia
    let leading_trivia = FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]);
    let trailing_trivia = FormatTriviaType::Append(vec![create_newline_trivia(ctx)]);

    format_last_stmt_no_trivia(ctx, last_stmt, shape).update_trivia(leading_trivia, trailing_trivia)
}

fn trivia_remove_leading_newlines(trivia: Vec<&Token>) -> Vec<Token> {
    trivia
        .iter()
        .skip_while(|x| match x.token_type() {
            TokenType::Whitespace { ref characters } => characters.contains('\n'),
            _ => false,
        })
        .map(|x| x.to_owned().to_owned())
        .collect()
}

fn prefix_remove_leading_newlines(prefix: &Prefix) -> Prefix {
    match prefix {
        Prefix::Name(token) => {
            let leading_trivia = trivia_remove_leading_newlines(token.leading_trivia().collect());
            Prefix::Name(token.update_leading_trivia(FormatTriviaType::Replace(leading_trivia)))
        }
        Prefix::Expression(expr) => Prefix::Expression(match expr {
            Expression::Parentheses {
                contained,
                expression,
            } => {
                let (start_parens, end_parens) = contained.tokens();
                let leading_trivia =
                    trivia_remove_leading_newlines(start_parens.leading_trivia().collect());
                Expression::Parentheses {
                    contained: full_moon::ast::span::ContainedSpan::new(
                        start_parens
                            .update_leading_trivia(FormatTriviaType::Replace(leading_trivia)),
                        end_parens.to_owned(),
                    ),
                    expression: Box::new(*expression.to_owned()),
                }
            }
            other => {
                unreachable!("got non-parentheses expression as prefix {:?}", other)
            }
        }),

        other => panic!("unknown node {:?}", other),
    }
}

fn var_remove_leading_newline(var: Var) -> Var {
    match var {
        Var::Name(token) => {
            let leading_trivia = trivia_remove_leading_newlines(token.leading_trivia().collect());
            Var::Name(token.update_leading_trivia(FormatTriviaType::Replace(leading_trivia)))
        }
        Var::Expression(var_expr) => {
            let prefix = prefix_remove_leading_newlines(var_expr.prefix());
            Var::Expression(var_expr.with_prefix(prefix))
        }
        other => panic!("unknown node {:?}", other),
    }
}

fn stmt_remove_leading_newlines(stmt: Stmt) -> Stmt {
    match stmt {
        Stmt::Assignment(assignment) => {
            let mut var_list = Punctuated::new();

            for (idx, pair) in assignment.variables().pairs().enumerate() {
                if idx == 0 {
                    let pair = pair.to_owned().map(var_remove_leading_newline);
                    var_list.push(pair);
                } else {
                    var_list.push(pair.to_owned());
                }
            }

            Stmt::Assignment(assignment.with_variables(var_list))
        }
        Stmt::Do(do_block) => {
            update_first_token!(Do, do_block, do_block.do_token(), with_do_token)
        }
        Stmt::FunctionCall(function_call) => {
            let prefix = prefix_remove_leading_newlines(function_call.prefix());
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
            let lhs = var_remove_leading_newline(compound_assignment.lhs().to_owned());
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
        #[cfg(feature = "lua52")]
        Stmt::Goto(goto) => update_first_token!(Goto, goto, goto.goto_token(), with_goto_token),
        #[cfg(feature = "lua52")]
        Stmt::Label(label) => {
            update_first_token!(Label, label, label.left_colons(), with_left_colons)
        }
        other => panic!("unknown node {:?}", other),
    }
}

fn last_stmt_remove_leading_newlines(last_stmt: LastStmt) -> LastStmt {
    match last_stmt {
        LastStmt::Break(token) => {
            let leading_trivia = trivia_remove_leading_newlines(token.leading_trivia().collect());
            LastStmt::Break(token.update_leading_trivia(FormatTriviaType::Replace(leading_trivia)))
        }
        LastStmt::Return(return_node) => {
            let token = return_node
                .token()
                .update_leading_trivia(FormatTriviaType::Replace(trivia_remove_leading_newlines(
                    return_node.token().leading_trivia().collect(),
                )));

            LastStmt::Return(return_node.with_token(token))
        }
        #[cfg(feature = "luau")]
        LastStmt::Continue(token) => {
            let leading_trivia = trivia_remove_leading_newlines(token.leading_trivia().collect());
            LastStmt::Continue(
                token.update_leading_trivia(FormatTriviaType::Replace(leading_trivia)),
            )
        }
        other => panic!("unknown node {:?}", other),
    }
}

/// Formats a block node. Note: the given shape to the block formatter should already be at the correct indentation level
pub fn format_block(ctx: &Context, block: &Block, shape: Shape) -> Block {
    let mut ctx = *ctx;
    let mut formatted_statements: Vec<(Stmt, Option<TokenReference>)> = Vec::new();
    let mut found_first_stmt = false;
    let mut stmt_iterator = block.stmts_with_semicolon().peekable();

    while let Some((stmt, semi)) = stmt_iterator.next() {
        ctx = ctx.check_toggle_formatting(stmt);

        let shape = shape.reset();
        let mut stmt = format_stmt(&ctx, stmt, shape);

        // If this is the first stmt, then remove any leading newlines
        if !found_first_stmt {
            if let FormatNode::Normal = ctx.should_format_node(&stmt) {
                stmt = stmt_remove_leading_newlines(stmt);
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
                    Some((Stmt::FunctionCall(function_call), _)) => matches!(
                        function_call.prefix(),
                        Prefix::Expression(Expression::Parentheses { .. })
                    ),
                    Some((Stmt::Assignment(assignment), _)) => {
                        match assignment.variables().iter().next() {
                            Some(Var::Expression(var_expression)) => {
                                matches!(
                                    var_expression.prefix(),
                                    Prefix::Expression(Expression::Parentheses { .. })
                                )
                            }
                            _ => false,
                        }
                    }
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
                Some(
                    match semi {
                        Some(semi) => crate::fmt_symbol!(&ctx, semi, ";", shape),
                        None => TokenReference::symbol(";").expect("could not make semicolon"),
                    }
                    .update_trailing_trivia(FormatTriviaType::Append(trivia)),
                )
            }
            false => match semi {
                Some(semi) => {
                    // We used to have a semicolon, but now we are removing it
                    // We want to keep any old comments on the semicolon token, otherwise we will lose it
                    // Move the comments to the end of the stmt, but before the newline token
                    // TODO: this is a bit of a hack - we should probably move newline appending to this function
                    let trivia = trivia_util::get_stmt_trailing_trivia(stmt.to_owned())
                        .1
                        .iter()
                        .rev()
                        .skip(1) // Remove the newline at the end
                        .rev()
                        .cloned()
                        .chain(
                            semi.leading_trivia()
                                .chain(semi.trailing_trivia())
                                .filter(|token| trivia_util::trivia_is_comment(token))
                                .flat_map(|x| {
                                    // Prepend a single space beforehand
                                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                                }),
                        )
                        .chain(std::iter::once(create_newline_trivia(&ctx)))
                        .collect();

                    stmt = stmt.update_trailing_trivia(FormatTriviaType::Replace(trivia));

                    None
                }
                None => None,
            },
        };

        formatted_statements.push((stmt, semicolon))
    }

    // Drop the stmt_iterator as we do not need it anymore and we still need to use `block`
    drop(stmt_iterator);

    let formatted_last_stmt = match block.last_stmt_with_semicolon() {
        Some((last_stmt, semi)) => {
            ctx = ctx.check_toggle_formatting(last_stmt);

            let shape = shape.reset();
            let mut last_stmt = format_last_stmt(&ctx, last_stmt, shape);
            // If this is the first stmt, then remove any leading newlines
            if !found_first_stmt && matches!(ctx.should_format_node(&last_stmt), FormatNode::Normal)
            {
                last_stmt = last_stmt_remove_leading_newlines(last_stmt);
            }

            // LastStmt will never need a semicolon
            // We need to check if we previously had a semicolon, and keep the comments if so
            let semicolon = match semi {
                Some(semi) => {
                    // Append semicolon trailing trivia to the end, but before the newline
                    // TODO: this is a bit of a hack - we should probably move newline appending to this function
                    let trivia = trivia_util::last_stmt_trailing_trivia(&last_stmt)
                        .iter()
                        .rev()
                        .skip(1) // Remove the newline at the end
                        .rev()
                        .cloned()
                        .chain(
                            semi.leading_trivia()
                                .chain(semi.trailing_trivia())
                                .filter(|token| trivia_util::trivia_is_comment(token))
                                .flat_map(|x| {
                                    // Prepend a single space beforehand
                                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                                }),
                        )
                        .chain(std::iter::once(create_newline_trivia(&ctx)))
                        .collect();

                    last_stmt = last_stmt.update_trailing_trivia(FormatTriviaType::Replace(trivia));

                    None
                }
                None => None,
            };
            Some((last_stmt, semicolon))
        }
        None => None,
    };

    block
        .to_owned()
        .with_stmts(formatted_statements)
        .with_last_stmt(formatted_last_stmt)
}
