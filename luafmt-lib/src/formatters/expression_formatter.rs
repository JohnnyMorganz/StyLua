use full_moon::ast::{
    BinOp, BinOpRhs, Expression, Index, Prefix, Suffix, UnOp, Value, Var, VarExpression,
};
use full_moon::tokenizer::TokenReference;
use std::boxed::Box;

use crate::formatters::{functions_formatter, table_formatter, CodeFormatter};

pub fn format_binop<'ast>(code_formatter: &CodeFormatter, binop: BinOp<'ast>) -> BinOp<'ast> {
    match binop {
        BinOp::And(token) => BinOp::And(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" and ").unwrap()),
        ),
        BinOp::Caret(token) => BinOp::Caret(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" ^ ").unwrap()),
        ),
        BinOp::GreaterThan(token) => BinOp::GreaterThan(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" > ").unwrap()),
        ),
        BinOp::GreaterThanEqual(token) => BinOp::GreaterThanEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" >= ").unwrap()),
        ),
        BinOp::LessThan(token) => BinOp::LessThan(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" < ").unwrap()),
        ),
        BinOp::LessThanEqual(token) => BinOp::LessThanEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" <= ").unwrap()),
        ),
        BinOp::Minus(token) => BinOp::Minus(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" - ").unwrap()),
        ),
        BinOp::Or(token) => BinOp::Or(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" or ").unwrap()),
        ),
        BinOp::Percent(token) => BinOp::Percent(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" % ").unwrap()),
        ),
        BinOp::Plus(token) => BinOp::Plus(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" + ").unwrap()),
        ),
        BinOp::Slash(token) => BinOp::Slash(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" / ").unwrap()),
        ),
        BinOp::Star(token) => BinOp::Star(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" * ").unwrap()),
        ),
        BinOp::TildeEqual(token) => BinOp::TildeEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" ~= ").unwrap()),
        ),
        BinOp::TwoDots(token) => BinOp::TwoDots(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" .. ").unwrap()),
        ),
        BinOp::TwoEqual(token) => BinOp::TwoEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" == ").unwrap()),
        ),
    }
}

pub fn format_bin_op_rhs<'ast>(
    code_formatter: &CodeFormatter,
    bin_op_rhs: BinOpRhs<'ast>,
) -> BinOpRhs<'ast> {
    BinOpRhs::new(
        format_binop(code_formatter, bin_op_rhs.bin_op().to_owned()),
        Box::new(format_expression(
            code_formatter,
            bin_op_rhs.rhs().to_owned(),
        )),
    )
}

/// Formats an Expression node
pub fn format_expression<'ast>(
    code_formatter: &CodeFormatter,
    expression: Expression<'ast>,
) -> Expression<'ast> {
    match expression {
        Expression::Value { value, binop } => Expression::Value {
            value: Box::new(format_value(code_formatter, *value)),
            binop: match binop {
                Some(value) => Some(format_bin_op_rhs(code_formatter, value)),
                None => None,
            },
        },
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: code_formatter.format_contained_span(contained),
            expression: Box::new(format_expression(code_formatter, *expression)),
        },
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop: format_unop(code_formatter, unop),
            expression: Box::new(format_expression(code_formatter, *expression)),
        },
    }
}

/// Formats an Index Node
pub fn format_index<'ast>(code_formatter: &CodeFormatter, index: Index<'ast>) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: code_formatter.format_contained_span(brackets),
            expression: format_expression(code_formatter, expression),
        },

        Index::Dot { dot, name } => Index::Dot {
            dot: code_formatter.format_token_reference(dot),
            name: code_formatter.format_token_reference(name),
        },
    }
}

/// Formats a Prefix Node
pub fn format_prefix<'ast>(code_formatter: &CodeFormatter, prefix: Prefix<'ast>) -> Prefix<'ast> {
    match prefix {
        Prefix::Expression(expression) => {
            Prefix::Expression(format_expression(code_formatter, expression))
        }
        Prefix::Name(token_reference) => {
            Prefix::Name(code_formatter.format_token_reference(token_reference))
        }
    }
}

/// Formats a Suffix Node
pub fn format_suffix<'ast>(code_formatter: &CodeFormatter, suffix: Suffix<'ast>) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => Suffix::Call(functions_formatter::format_call(code_formatter, call)),
        Suffix::Index(index) => Suffix::Index(format_index(code_formatter, index)),
    }
}

/// Formats a Value Node
pub fn format_value<'ast>(code_formatter: &CodeFormatter, value: Value<'ast>) -> Value<'ast> {
    match value {
        Value::Function((token_reference, function_body)) => Value::Function((
            code_formatter.format_symbol(
                token_reference.into_owned(),
                TokenReference::symbol("function").unwrap(),
            ),
            functions_formatter::format_function_body(code_formatter, function_body),
        )),
        Value::FunctionCall(function_call) => Value::FunctionCall(
            functions_formatter::format_function_call(code_formatter, function_call),
        ),
        Value::Number(token_reference) => {
            Value::Number(code_formatter.format_token_reference(token_reference))
        }
        Value::ParseExpression(expression) => {
            Value::ParseExpression(format_expression(code_formatter, expression))
        }
        Value::String(token_reference) => {
            Value::String(code_formatter.format_token_reference(token_reference))
        }
        Value::Symbol(token_reference) => {
            Value::Symbol(code_formatter.format_token_reference(token_reference))
        }
        Value::TableConstructor(table_constructor) => Value::TableConstructor(
            table_formatter::format_table_constructor(code_formatter, table_constructor),
        ),
        Value::Var(var) => Value::Var(format_var(code_formatter, var)),
    }
}

/// Formats a Var Node
pub fn format_var<'ast>(code_formatter: &CodeFormatter, var: Var<'ast>) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => {
            Var::Name(code_formatter.format_token_reference(token_reference))
        }
        Var::Expression(var_expression) => {
            Var::Expression(format_var_expression(code_formatter, var_expression))
        }
    }
}

pub fn format_var_expression<'ast>(
    code_formatter: &CodeFormatter,
    var_expression: VarExpression<'ast>,
) -> VarExpression<'ast> {
    let formatted_prefix = format_prefix(code_formatter, var_expression.prefix().to_owned());
    let formatted_suffixes = var_expression
        .iter_suffixes()
        .map(|x| format_suffix(code_formatter, x.to_owned()))
        .collect();
    var_expression
        .with_prefix(formatted_prefix)
        .with_suffixes(formatted_suffixes)
}

/// Formats an UnOp Node
pub fn format_unop<'ast>(code_formatter: &CodeFormatter, unop: UnOp<'ast>) -> UnOp<'ast> {
    match unop {
        UnOp::Minus(token) => UnOp::Minus(
            code_formatter.format_symbol(token.into_owned(), TokenReference::symbol("-").unwrap()),
        ),
        UnOp::Not(token) => UnOp::Not(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol("not ").unwrap()),
        ),
        UnOp::Hash(token) => UnOp::Hash(
            code_formatter.format_symbol(token.into_owned(), TokenReference::symbol("#").unwrap()),
        ),
    }
}
