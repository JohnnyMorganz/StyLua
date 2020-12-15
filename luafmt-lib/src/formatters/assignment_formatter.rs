use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Expression, LocalAssignment, Value, Var
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;
use std::boxed::Box;

#[derive(Default)]
pub struct AssignmentFormatter;

// TODO: Can we simplify these three functions into one, general function?

/// Formats a punctuated list of Expressions
fn format_punctuated_expression<'a>(
    old: Punctuated<'a, Expression<'a>>,
) -> Punctuated<'a, Expression<'a>> {
    let mut formatted: Punctuated<Expression<'a>> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        let punc = match pair.punctuation() {
            Some(value) => {
                let whitespace = vec![Token::new(TokenType::spaces(1))];

                Some(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    value.token().clone(),
                    whitespace,
                )))
            }
            None => None,
        };

        // Format Value
        let formatted_value = match pair.into_value() {
            Expression::Value { value, binop } => Expression::Value {
                value: Box::new(match *value {
                    Value::String(token_reference) => {
                        Value::String(Cow::Owned(TokenReference::new(
                            Vec::new(),
                            token_reference.token().clone(),
                            Vec::new(),
                        )))
                    }
                    Value::Number(token_reference) => {
                        Value::Number(Cow::Owned(TokenReference::new(
                            Vec::new(),
                            token_reference.token().clone(),
                            Vec::new(),
                        )))
                    }
                    Value::Symbol(token_reference) => {
                        Value::Symbol(Cow::Owned(TokenReference::new(
                            Vec::new(),
                            token_reference.token().clone(),
                            Vec::new(),
                        )))
                    }
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
        };
        formatted.push(Pair::new(formatted_value, punc));
    }

    formatted
}

/// Formats a punctuated list of token references
fn format_punctuated_tokens<'a>(
    old: Punctuated<'a, Cow<'a, TokenReference<'a>>>,
) -> Punctuated<'a, Cow<'a, TokenReference<'a>>> {
    let mut formatted: Punctuated<Cow<'a, TokenReference<'a>>> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        let punc = match pair.punctuation() {
            Some(value) => {
                let whitespace = vec![Token::new(TokenType::spaces(1))];

                Some(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    value.token().clone(),
                    whitespace,
                )))
            }
            None => None,
        };

        // Format Value
        // let formatted_value = match pair.into_value() {
        //     TokenReference { leading_trivia, token, trailing_trivia } => {
        //         Cow::Owned(TokenReference::new(Vec::new(), token.clone(), Vec::new()))
        //     },
        //     // Expression::Value(value) => {
        //     //     match value {
        //     //         Expression::Value::String(value) => Cow::Owned(TokenReference::new(Vec::new(), value.token().clone(), Vec::new()))
        //     //     }
        //     // }
        // };

        // // Clear any trailing_trivia in value
        // //println!("{:?}", value);
        // let value = Cow::Owned(TokenReference::new(Vec::new(), pair.into_value(), Vec::new()));
        let value = pair.into_value();
        let formatted_value = Cow::Owned(TokenReference::new(
            Vec::new(),
            value.into_owned().token().clone(),
            Vec::new(),
        ));
        formatted.push(Pair::new(formatted_value, punc));
    }

    formatted
}

/// Formats a punctuated list of Vars
fn format_punctuated<'a>(old: Punctuated<'a, Var<'a>>) -> Punctuated<'a, Var<'a>> {
    let mut formatted: Punctuated<Var<'a>> = Punctuated::new();
    for pair in old.into_pairs() {
        // Format Punctuation
        let punc = match pair.punctuation() {
            Some(value) => {
                println!("{}", value);
                let whitespace = vec![Token::new(TokenType::spaces(1))];

                Some(Cow::Owned(TokenReference::new(
                    Vec::new(),
                    value.token().clone(),
                    whitespace,
                )))
            }
            None => None,
        };

        // Format Value
        let formatted_value = match pair.into_value() {
            Var::Name(token_ref) => {
                Var::Name(Cow::Owned(TokenReference::new(Vec::new(), token_ref.token().clone(), Vec::new())))
            },
            _ => panic!("problem!")
        };

        formatted.push(Pair::new(formatted_value, punc));
    }

    formatted
}

impl AssignmentFormatter {
    fn format_assignment<'ast>(&mut self, assignment: Assignment<'ast>) -> Assignment<'ast> {
        let var_list = format_punctuated(assignment.var_list().clone());
        // TODO: Do we need these clones?
        let expr_list = format_punctuated_expression(assignment.expr_list().clone());

        assignment
            .with_var_list(var_list)
            .with_equal_token(Cow::Owned(TokenReference::symbol(" = ").unwrap()))
            .with_expr_list(expr_list)
    }

    fn format_local_assignment<'ast>(
        &mut self,
        assignment: LocalAssignment<'ast>,
    ) -> LocalAssignment<'ast> {
        let name_list = format_punctuated_tokens(assignment.name_list().clone());

        if assignment.expr_list().is_empty() {
            assignment
                .with_local_token(Cow::Owned(TokenReference::symbol("local ").unwrap()))
                .with_name_list(name_list)
                .with_equal_token(None)
                .with_expr_list(Punctuated::new())
        } else {
            let expr_list = format_punctuated_expression(assignment.expr_list().clone());

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
        assert_eq!(print(&visitor.visit_ast(ast)), "x = 1");
    }

    #[test]
    fn test_multiple_var_assignment_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("x    ,   y =   1,    'foo'").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "x, y = 1, 'foo'");
    }

    #[test]
    fn test_local_assignment_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x       =     'test'  ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x = 'test'");
    }

    #[test]
    fn test_local_assignment_no_expr_list_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x       ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x");
    }

    #[test]
    fn test_local_assignment_multiple_vars_formatter() {
        let mut visitor = AssignmentFormatter::default();
        let ast = parse("local      x,      y       =     'test'  ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x, y = 'test'");
    }
}
