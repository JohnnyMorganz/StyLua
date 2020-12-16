use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Block, FunctionArgs, LocalAssignment,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;

pub mod assignment_formatter;
pub mod eof_formatter;
pub mod expression_formatter;
pub mod functions_formatter;
pub mod table_formatter;
pub mod trivia_formatter;

#[derive(Default)]
pub struct FileFormatter {
    indent_level: usize,
}

pub fn create_indent_trivia<'ast>(indent_level: &usize) -> Token<'ast> {
    Token::new(TokenType::tabs(*indent_level))
}

pub fn create_newline_trivia<'ast>() -> Token<'ast> {
    Token::new(TokenType::Whitespace {
        characters: Cow::Owned(String::from("\n")), // TODO: Support CRLF line endings
    })
}

pub fn format_plain_token_reference<'a>(token_reference: TokenReference<'a>) -> TokenReference<'a> {
    TokenReference::new(Vec::new(), token_reference.token().to_owned(), Vec::new())
}

pub fn format_token_reference<'a>(
    token_reference: Cow<'a, TokenReference<'a>>,
) -> Cow<'a, TokenReference<'a>> {
    Cow::Owned(format_plain_token_reference(token_reference.into_owned()))
}

pub fn format_punctuation<'ast>(
    punctuation: Cow<'ast, TokenReference<'ast>>,
) -> Cow<'ast, TokenReference<'ast>> {
    Cow::Owned(TokenReference::new(
        Vec::new(),
        punctuation.token().to_owned(),
        vec![Token::new(TokenType::spaces(1))], // Single space whitespace
    ))
}

pub fn format_punctuated<'a, T>(
    old: Punctuated<'a, T>,
    value_formatter: &dyn Fn(T) -> T,
    // wanted_trailing_trivia: Vec<Token<'a>>,
) -> Punctuated<'a, T> {
    let mut formatted: Punctuated<T> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = format_punctuation(punctuation);
                let formatted_value = value_formatter(value);
                formatted.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = value_formatter(value);
                formatted.push(Pair::new(formatted_value, None));
            }
        }
    }

    formatted
}

// Indents will increase at the start of a new block, or within tables, and decrease at the end of them
// New lines will occur at the end of assignments

// The following visitors are unnecessary to use, as they should be handled by other visitors:
// visit_expression -> Never present on its own, part of larger syntax
// visit_value -> Part of expression
// visit_bin_op -> Part of Expression::Value
// visit_table_constructor -> Always presented within an expression, handled by Value
// visit_var -> Always handled by assignments or values
// visit_var_expression -> Part of Var
// visit_suffix -> Handled by FunctionCall/VarExpression
// visit_call -> Handled within FunctionCall
// visit_anonymous_call -> Part of Call
// visit_method_call -> Part of Call
// visit_function_args -> Part of MethodCall

impl<'ast> VisitorMut<'ast> for FileFormatter {
    fn visit_block(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level += 1;
        node
    }

    fn visit_block_end(&mut self, node: Block<'ast>) -> Block<'ast> {
        self.indent_level -= 1;
        node
    }

    fn visit_assignment(&mut self, node: Assignment<'ast>) -> Assignment<'ast> {
        assignment_formatter::format_assignment(node)
    }

    fn visit_assignment_end(&mut self, node: Assignment<'ast>) -> Assignment<'ast> {
        // Add newline at the end of Assignment expression list
        // Expression list should already be formatted
        let mut formatted_expression_list = node.expr_list().to_owned();

        // Retrieve last item and add new line to it
        if let Some(last_pair) = formatted_expression_list.pop() {
            match last_pair {
                Pair::End(value) => {
                    let expression = trivia_formatter::expression_add_trailing_trivia(
                        value,
                        vec![create_newline_trivia()],
                    );
                    formatted_expression_list.push(Pair::End(expression));
                }
                Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
            }
        }

        node.with_expr_list(formatted_expression_list)
    }

    fn visit_local_assignment(&mut self, node: LocalAssignment<'ast>) -> LocalAssignment<'ast> {
        assignment_formatter::format_local_assignment(node)
    }

    fn visit_local_assignment_end(&mut self, node: LocalAssignment<'ast>) -> LocalAssignment<'ast> {
        // Add newline at the end of LocalAssignment expression list
        // Expression list should already be formatted
        let mut formatted_expression_list = node.expr_list().to_owned();

        // Retrieve last item and add new line to it
        if let Some(last_pair) = formatted_expression_list.pop() {
            match last_pair {
                Pair::End(value) => {
                    let expression = trivia_formatter::expression_add_trailing_trivia(
                        value,
                        vec![create_newline_trivia()],
                    );
                    formatted_expression_list.push(Pair::End(expression));
                }
                Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
            }
        }

        node.with_expr_list(formatted_expression_list)
    }

    // TODO: Remove this, we will never have FunctionArgs by themselves. This should be handled in FunctionCall
    fn visit_function_args(&mut self, function_args: FunctionArgs<'ast>) -> FunctionArgs<'ast> {
        functions_formatter::format_function_args(function_args)
    }
}
