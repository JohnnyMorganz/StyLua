use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Expression, LocalAssignment, Value, Var,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;
use std::boxed::Box;

#[derive(Default)]
pub struct AssignmentFormatter;

/// Formats an expression, adding any wanted trailing trivia
fn format_expression<'a>(
    expression: Expression<'a>,
    wanted_trailing_trivia: Vec<Token<'a>>,
) -> Expression<'a> {
    match expression {
        Expression::Value { value, binop } => Expression::Value {
            value: Box::new(match *value {
                Value::String(token_reference) => Value::String(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    token_reference.token().to_owned(),
                    wanted_trailing_trivia,
                ))),
                Value::Number(token_reference) => Value::Number(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    token_reference.token().to_owned(),
                    wanted_trailing_trivia,
                ))),
                Value::Symbol(token_reference) => Value::Symbol(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    token_reference.token().to_owned(),
                    wanted_trailing_trivia,
                ))),
                // TODO: Handle the remainder of these
                _ => *value,
                // Value::Function(token_reference) => *value, // Value::Function(Cow::Owned(TokenReference::new(Vec::new(), token_reference.token().clone(), Vec::new()))),
                // Value::FunctionCall(function_call) => *value, //Value::FunctionCall(Fun),
                // Value::TableConstructor(token_reference) => *value, // Value::TableConstructor(Cow::Owned(TokenReference::new(Vec::new(), token_reference.token().clone(), Vec::new()))),
                // Value::ParseExpression(token_reference) => *value, // Value::ParseExpression(Cow::Owned(TokenReference::new(Vec::new(), token_reference.token().clone(), Vec::new()))),
                // Value::Var(token_reference) => *value, // Value::Var(Cow::Owned(TokenReference::new(Vec::new(), token_reference.token().clone(), Vec::new()))),
            }),
            binop,
        },
        // TODO: Handle the remainder of these
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained,
            expression,
        },
        Expression::UnaryOperator { unop, expression } => {
            Expression::UnaryOperator { unop, expression }
        }
    }
}

fn format_token_reference<'a>(
    token_reference: Cow<'a, TokenReference<'a>>,
    wanted_trailing_trivia: Vec<Token<'a>>,
) -> Cow<'a, TokenReference<'a>> {
    Cow::Owned(TokenReference::new(
        Vec::new(),
        token_reference.token().to_owned(),
        wanted_trailing_trivia,
    ))
}

fn format_var<'a>(var: Var<'a>, wanted_trailing_trivia: Vec<Token<'a>>) -> Var<'a> {
    match var {
        Var::Name(token_ref) => Var::Name(Cow::Owned(TokenReference::new(
            Vec::new(),
            token_ref.token().to_owned(),
            wanted_trailing_trivia,
        ))),
        _ => panic!("problem!"),
    }
}

fn format_punctuated<'a, T>(
    old: Punctuated<'a, T>,
    value_formatter: &dyn Fn(T, Vec<Token<'a>>) -> T,
    wanted_trailing_trivia: Vec<Token<'a>>,
) -> Punctuated<'a, T> {
    let mut formatted: Punctuated<T> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = Cow::Owned(TokenReference::new(
                    Vec::new(),
                    punctuation.token().to_owned(),
                    vec![Token::new(TokenType::spaces(1))], // Single space whitespace
                ));

                // Format Value
                let formatted_value = value_formatter(value, vec![]);
                formatted.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = value_formatter(value, wanted_trailing_trivia.to_owned());
                formatted.push(Pair::new(formatted_value, None));
            }
        }
    }

    formatted
}

impl AssignmentFormatter {
    fn format_assignment<'ast>(&mut self, assignment: Assignment<'ast>) -> Assignment<'ast> {
        let var_list = format_punctuated(assignment.var_list().to_owned(), &format_var, vec![]);
        let expr_list = format_punctuated(
            assignment.expr_list().to_owned(),
            &format_expression,
            vec![Token::new(TokenType::Whitespace {
                characters: Cow::Owned(String::from("\n")),
            })],
        );

        assignment
            .with_var_list(var_list)
            .with_equal_token(Cow::Owned(TokenReference::symbol(" = ").unwrap()))
            .with_expr_list(expr_list)
    }

    fn format_local_assignment<'ast>(
        &mut self,
        assignment: LocalAssignment<'ast>,
    ) -> LocalAssignment<'ast> {
        if assignment.expr_list().is_empty() {
            let name_list = format_punctuated(
                assignment.name_list().to_owned(),
                &format_token_reference,
                vec![Token::new(TokenType::Whitespace {
                    characters: Cow::Owned(String::from("\n")),
                })],
            );
            assignment
                .with_local_token(Cow::Owned(TokenReference::symbol("local ").unwrap()))
                .with_name_list(name_list)
                .with_equal_token(None)
                .with_expr_list(Punctuated::new())
        } else {
            let name_list = format_punctuated(
                assignment.name_list().to_owned(),
                &format_token_reference,
                vec![],
            );
            let expr_list = format_punctuated(
                assignment.expr_list().to_owned(),
                &format_expression,
                vec![Token::new(TokenType::Whitespace {
                    characters: Cow::Owned(String::from("\n")),
                })],
            );

            assignment
                .with_local_token(Cow::Owned(TokenReference::symbol("local ").unwrap()))
                .with_name_list(name_list)
                .with_equal_token(Some(Cow::Owned(TokenReference::symbol(" = ").unwrap())))
                .with_expr_list(expr_list)
            // LocalAssignment {
            //     local_token: Cow::Owned(TokenReference::symbol("local ").unwrap()),
            //     name_list,
            //     equal_token: Some(Cow::Owned(TokenReference::symbol("= ").unwrap())),
            //     expr_list: Punctuated::new(),
            // }
        }
    }
}

impl<'ast> VisitorMut<'ast> for AssignmentFormatter {
    fn visit_assignment(&mut self, node: Assignment<'ast>) -> Assignment<'ast> {
        self.format_assignment(node)
    }

    fn visit_local_assignment(&mut self, node: LocalAssignment<'ast>) -> LocalAssignment<'ast> {
        self.format_local_assignment(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::formatters::assignment_formatter::AssignmentFormatter;
    use full_moon::visitors::VisitorMut;
    use full_moon::{parse, print};
    #[test]
    fn test_assignment_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("    x =   1").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "x = 1\n");
    }

    #[test]
    fn test_multiple_var_assignment_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("x    ,   y =   1,    'foo'").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "x, y = 1, 'foo'\n");
    }

    #[test]
    fn test_local_assignment_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x       =     'test'  ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x = 'test'\n");
    }

    #[test]
    fn test_local_assignment_no_expr_list_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x       ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x\n");
    }

    #[test]
    fn test_local_assignment_multiple_vars_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x,      y       =     'test'  ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x, y = 'test'\n");
    }
}
