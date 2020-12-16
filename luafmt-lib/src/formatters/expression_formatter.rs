use full_moon::ast::{
    span::ContainedSpan, BinOp, BinOpRhs, Expression, Index, Prefix, Suffix, UnOp, Value, Var,
    VarExpression,
};
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType};
use std::{borrow::Cow, boxed::Box};

use crate::formatters::{format_token_reference, functions_formatter, table_formatter};

pub fn format_binop<'ast>(binop: BinOp<'ast>) -> BinOp<'ast> {
    match binop {
        BinOp::And(_) => BinOp::And(Cow::Owned(TokenReference::symbol(" and ").unwrap())),
        BinOp::Caret(_) => BinOp::Caret(Cow::Owned(TokenReference::symbol(" ^ ").unwrap())),
        BinOp::GreaterThan(_) => {
            BinOp::GreaterThan(Cow::Owned(TokenReference::symbol(" > ").unwrap()))
        }
        BinOp::GreaterThanEqual(_) => {
            BinOp::GreaterThanEqual(Cow::Owned(TokenReference::symbol(" >= ").unwrap()))
        }
        BinOp::LessThan(_) => BinOp::LessThan(Cow::Owned(TokenReference::symbol(" < ").unwrap())),
        BinOp::LessThanEqual(_) => {
            BinOp::LessThanEqual(Cow::Owned(TokenReference::symbol(" <= ").unwrap()))
        }
        BinOp::Minus(_) => BinOp::Minus(Cow::Owned(TokenReference::symbol(" - ").unwrap())),
        BinOp::Or(_) => BinOp::Or(Cow::Owned(TokenReference::symbol(" or ").unwrap())),
        BinOp::Percent(_) => BinOp::Percent(Cow::Owned(TokenReference::symbol(" % ").unwrap())),
        BinOp::Plus(_) => BinOp::Plus(Cow::Owned(TokenReference::symbol(" + ").unwrap())),
        BinOp::Slash(_) => BinOp::Slash(Cow::Owned(TokenReference::symbol(" / ").unwrap())),
        BinOp::Star(_) => BinOp::Star(Cow::Owned(TokenReference::symbol(" * ").unwrap())),
        BinOp::TildeEqual(_) => {
            BinOp::TildeEqual(Cow::Owned(TokenReference::symbol(" ~= ").unwrap()))
        }
        BinOp::TwoDots(_) => BinOp::TwoDots(Cow::Owned(TokenReference::symbol(" .. ").unwrap())),
        BinOp::TwoEqual(_) => BinOp::TwoEqual(Cow::Owned(TokenReference::symbol(" == ").unwrap())),
    }
}

pub fn format_bin_op_rhs<'ast>(bin_op_rhs: BinOpRhs<'ast>) -> BinOpRhs<'ast> {
    BinOpRhs::new(
        format_binop(bin_op_rhs.bin_op().to_owned()),
        Box::new(format_expression(bin_op_rhs.rhs().to_owned())),
    )
}

/// Formats an Expression node
pub fn format_expression<'ast>(expression: Expression<'ast>) -> Expression<'ast> {
    match expression {
        Expression::Value { value, binop } => Expression::Value {
            value: Box::new(format_value(*value)),
            binop: match binop {
                Some(value) => Some(format_bin_op_rhs(value)),
                None => None,
            },
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
            unop: format_unop(unop),
            expression: Box::new(format_expression(*expression)),
        },
    }
}

/// Formats an Index Node
pub fn format_index<'ast>(index: Index<'ast>) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets: _,
            expression,
        } => Index::Brackets {
            brackets: ContainedSpan::new(
                Cow::Owned(TokenReference::symbol("[").unwrap()),
                Cow::Owned(TokenReference::symbol("]").unwrap()),
            ),
            expression: format_expression(expression),
        },

        Index::Dot { dot, name } => Index::Dot {
            dot: format_token_reference(dot),
            name: format_token_reference(name),
        },
    }
}

/// Formats a Prefix Node
pub fn format_prefix<'ast>(prefix: Prefix<'ast>) -> Prefix<'ast> {
    match prefix {
        Prefix::Expression(expression) => Prefix::Expression(format_expression(expression)),
        Prefix::Name(token_reference) => Prefix::Name(format_token_reference(token_reference)),
    }
}

/// Formats a Suffix Node
pub fn format_suffix<'ast>(suffix: Suffix<'ast>) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => Suffix::Call(functions_formatter::format_call(call)),
        Suffix::Index(index) => Suffix::Index(format_index(index)),
    }
}

/// Formats a Value Node
pub fn format_value<'ast>(value: Value<'ast>) -> Value<'ast> {
    match value {
        Value::Function((function_token, function_body)) => {
            Value::Function((function_token, function_body))
        } // TODO
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(functions_formatter::format_function_call(function_call))
        }
        Value::Number(token_reference) => Value::Number(format_token_reference(token_reference)),
        Value::ParseExpression(expression) => Value::ParseExpression(format_expression(expression)),
        Value::String(token_reference) => {
            // TODO: Should this be handled through format_token_reference instead?
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
        Value::Symbol(token_reference) => Value::Symbol(format_token_reference(token_reference)),
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(table_formatter::format_table_constructor(table_constructor))
        }
        Value::Var(var) => Value::Var(format_var(var)),
    }
}

/// Formats a Var Node
pub fn format_var<'ast>(var: Var<'ast>) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(format_token_reference(token_reference)),
        Var::Expression(var_expression) => Var::Expression(format_var_expression(var_expression)),
    }
}

pub fn format_var_expression<'ast>(var_expression: VarExpression<'ast>) -> VarExpression<'ast> {
    let formatted_prefix = format_prefix(var_expression.prefix().to_owned());
    let formatted_suffixes = var_expression
        .iter_suffixes()
        .map(|x| format_suffix(x.to_owned()))
        .collect();
    var_expression
        .with_prefix(formatted_prefix)
        .with_suffixes(formatted_suffixes)
}

/// Formats an UnOp Node
pub fn format_unop<'ast>(unop: UnOp<'ast>) -> UnOp<'ast> {
    match unop {
        UnOp::Minus(_) => UnOp::Minus(Cow::Owned(TokenReference::symbol("-").unwrap())),
        UnOp::Not(_) => UnOp::Not(Cow::Owned(TokenReference::symbol("not ").unwrap())),
        UnOp::Hash(_) => UnOp::Hash(Cow::Owned(TokenReference::symbol("#").unwrap())),
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
