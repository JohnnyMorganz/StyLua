use full_moon::ast::{
    BinOp, BinOpRhs, Expression, Index, Prefix, Suffix, UnOp, Value, Var, VarExpression,
};
use full_moon::tokenizer::TokenReference;
use std::boxed::Box;

use crate::formatters::{
    format_contained_span, format_symbol, format_token_reference, functions_formatter,
    table_formatter,
};

pub fn format_binop<'ast>(binop: BinOp<'ast>) -> BinOp<'ast> {
    match binop {
        BinOp::And(token) => BinOp::And(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" and ").unwrap(),
        )),
        BinOp::Caret(token) => BinOp::Caret(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" ^ ").unwrap(),
        )),
        BinOp::GreaterThan(token) => BinOp::GreaterThan(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" > ").unwrap(),
        )),
        BinOp::GreaterThanEqual(token) => BinOp::GreaterThanEqual(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" >= ").unwrap(),
        )),
        BinOp::LessThan(token) => BinOp::LessThan(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" < ").unwrap(),
        )),
        BinOp::LessThanEqual(token) => BinOp::LessThanEqual(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" <= ").unwrap(),
        )),
        BinOp::Minus(token) => BinOp::Minus(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" - ").unwrap(),
        )),
        BinOp::Or(token) => BinOp::Or(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" or ").unwrap(),
        )),
        BinOp::Percent(token) => BinOp::Percent(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" % ").unwrap(),
        )),
        BinOp::Plus(token) => BinOp::Plus(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" + ").unwrap(),
        )),
        BinOp::Slash(token) => BinOp::Slash(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" / ").unwrap(),
        )),
        BinOp::Star(token) => BinOp::Star(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" * ").unwrap(),
        )),
        BinOp::TildeEqual(token) => BinOp::TildeEqual(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" ~= ").unwrap(),
        )),
        BinOp::TwoDots(token) => BinOp::TwoDots(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" .. ").unwrap(),
        )),
        BinOp::TwoEqual(token) => BinOp::TwoEqual(format_symbol(
            token.into_owned(),
            TokenReference::symbol(" == ").unwrap(),
        )),
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
            contained,
            expression,
        } => Expression::Parentheses {
            contained: format_contained_span(contained),
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
            brackets,
            expression,
        } => Index::Brackets {
            brackets: format_contained_span(brackets),
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
        Value::Function((token_reference, function_body)) => Value::Function((
            format_symbol(
                token_reference.into_owned(),
                TokenReference::symbol("function").unwrap(),
            ),
            functions_formatter::format_function_body(function_body),
        )),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(functions_formatter::format_function_call(function_call))
        }
        Value::Number(token_reference) => Value::Number(format_token_reference(token_reference)),
        Value::ParseExpression(expression) => Value::ParseExpression(format_expression(expression)),
        Value::String(token_reference) => Value::String(format_token_reference(token_reference)),
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
        UnOp::Minus(token) => UnOp::Minus(format_symbol(
            token.into_owned(),
            TokenReference::symbol("-").unwrap(),
        )),
        UnOp::Not(token) => UnOp::Not(format_symbol(
            token.into_owned(),
            TokenReference::symbol("not ").unwrap(),
        )),
        UnOp::Hash(token) => UnOp::Hash(format_symbol(
            token.into_owned(),
            TokenReference::symbol("#").unwrap(),
        )),
    }
}
