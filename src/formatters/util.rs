use crate::context::Range;
use full_moon::ast::{Expression, Prefix, UnOp, Value, Var};
use full_moon::tokenizer::Token;

pub fn token_range(token: &Token) -> Range {
    (token.start_position().bytes(), token.end_position().bytes())
}

pub fn expression_range(expression: &Expression) -> Range {
    match expression {
        Expression::Parentheses { contained, .. } => token_range(contained.tokens().0),
        Expression::UnaryOperator { unop, .. } => match unop {
            UnOp::Minus(token_reference) => token_range(token_reference.token()),
            UnOp::Not(token_reference) => token_range(token_reference.token()),
            UnOp::Hash(token_reference) => token_range(token_reference.token()),
            other => panic!("unknown node {:?}", other),
        },
        Expression::BinaryOperator { lhs, .. } => expression_range(lhs),
        Expression::Value { value, .. } => {
            let value = &**value;
            match value {
                Value::Function((token_ref, _)) => token_range(token_ref.token()),
                Value::FunctionCall(function_call) => prefix_range(function_call.prefix()),
                Value::TableConstructor(table_constructor) => {
                    token_range(table_constructor.braces().tokens().0.token())
                }
                Value::Number(token_ref) => token_range(token_ref.token()),
                Value::ParenthesesExpression(expr) => expression_range(&expr),
                Value::String(token_ref) => token_range(token_ref.token()),
                Value::Symbol(token_ref) => token_range(token_ref.token()),
                Value::Var(var) => match var {
                    Var::Name(token_ref) => token_range(token_ref.token()),
                    Var::Expression(var_expr) => prefix_range(var_expr.prefix()),
                    other => panic!("unknown node {:?}", other),
                },
                other => panic!("unknown node {:?}", other),
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn prefix_range(prefix: &Prefix) -> Range {
    match prefix {
        Prefix::Name(token) => token_range(token.token()),
        Prefix::Expression(expression) => expression_range(expression),
        other => panic!("unknown node {:?}", other),
    }
}
