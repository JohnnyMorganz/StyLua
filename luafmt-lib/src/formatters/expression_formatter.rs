use full_moon::ast::{span::ContainedSpan, Expression, UnOp, Value, Var};
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType};
use std::borrow::Cow;

use crate::formatters::format_token_reference;

/// Formats a Var Node
pub fn format_var<'ast>(var: Var<'ast>) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(format_token_reference(token_reference)),
        Var::Expression(var_expression) => Var::Expression(var_expression), // TODO
    }
}

/// Formats a Value Node
pub fn format_value<'ast>(value: Value<'ast>) -> Value<'ast> {
    match value {
        Value::String(token_reference) => {
            let old_token_info = &*token_reference.token_type();

            match old_token_info {
                TokenType::StringLiteral {
                    literal,
                    multi_line,
                    ..
                } => {
                    let string_token = Token::new(TokenType::StringLiteral {
                        literal: literal.clone(),
                        multi_line: match multi_line {
                            Some(size) => Some(*size),
                            None => None,
                        },
                        quote_type: StringLiteralQuoteType::Double,
                    });
                    Value::String(Cow::Owned(TokenReference::new(
                        Vec::new(),
                        string_token,
                        Vec::new(),
                    )))
                }
                _ => panic!("have string with a non string-literal token kind"),
            }
        }

        Value::Number(token_reference) => Value::Number(format_token_reference(token_reference)),
        Value::Symbol(token_reference) => Value::Symbol(format_token_reference(token_reference)),
        Value::Var(var) => Value::Var(format_var(var)),
        // TODO: Handle remainder
        _ => value,
    }
}

/// Formats an Expression node
pub fn format_expression<'a>(expression: Expression<'a>) -> Expression<'a> {
    match expression {
        Expression::Value { value, binop } => Expression::Value {
            value: Box::new(format_value(*value)),
            binop,
        },
        Expression::Parentheses {
            contained: _,
            expression,
        } => Expression::Parentheses {
            contained: ContainedSpan::new(
                Cow::Owned(TokenReference::symbol("(").unwrap()),
                Cow::Owned(TokenReference::symbol(")").unwrap()),
            ),
            expression: Box::new(format_expression(*expression)),
        },
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop: match unop {
                UnOp::Minus(token_reference) => {
                    UnOp::Minus(format_token_reference(token_reference))
                }
                UnOp::Not(token_reference) => UnOp::Not(format_token_reference(token_reference)),
                UnOp::Hash(token_reference) => UnOp::Hash(format_token_reference(token_reference)),
            },
            expression: Box::new(format_expression(*expression)),
        },
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::formatters::value_formatter::ValueFormatter;
//     use full_moon::visitors::VisitorMut;
//     use full_moon::{parse, print};
//     #[test]
//     fn test_string_value_formatter() {
//         let mut visitor = ValueFormatter::default();
//         let ast = parse("local x = 'test'      ").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local x = \"test\"");
//     }
// }
