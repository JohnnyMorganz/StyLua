use full_moon::ast::{BinOp, Expression, Index, Prefix, Suffix, UnOp, Value, Var, VarExpression};
use full_moon::tokenizer::{Symbol, TokenReference, TokenType};
use std::boxed::Box;

use crate::formatters::CodeFormatter;

#[macro_export]
macro_rules! fmt_op {
    ($fmter:expr, $enum:ident, $value:ident, { $($operator:ident = $output:expr,)+ }) => {
        match $value {
            $(
                $enum::$operator(token) => $enum::$operator(crate::fmt_symbol!($fmter, token, $output)),
            )+
            other => panic!("unknown node {:?}", other),
        }
    };
}

impl CodeFormatter {
    pub fn format_binop<'ast>(&self, binop: &BinOp<'ast>) -> BinOp<'ast> {
        fmt_op!(self, BinOp, binop, {
            And = " and ",
            Caret = " ^ ",
            GreaterThan = " > ",
            GreaterThanEqual = " >= ",
            LessThan = " < ",
            LessThanEqual = " <= ",
            Minus = " - ",
            Or = " or ",
            Percent = " % ",
            Plus = " + ",
            Slash = " / ",
            Star = " * ",
            TildeEqual = " ~= ",
            TwoDots = " .. ",
            TwoEqual = " == ",
        })
    }

    /// Check to determine whether expression parentheses are required, depending on the provided
    /// internal expression contained within the parentheses
    fn check_excess_parentheses(internal_expression: &Expression) -> bool {
        match internal_expression {
            // Parentheses inside parentheses, not necessary
            Expression::Parentheses { .. } => true,
            // Check whether the expression relating to the UnOp is safe
            Expression::UnaryOperator { expression, .. } => {
                CodeFormatter::check_excess_parentheses(expression)
            }
            // Don't bother removing them if there is a binop, as they may be needed. TODO: can we be more intelligent here?
            Expression::BinaryOperator { .. } => false,
            Expression::Value { value, .. } => {
                match &**value {
                    // Internal expression is a function call
                    // We could potentially be culling values, so we should not remove parentheses
                    Value::FunctionCall(_) => false,
                    // String literal inside of parentheses
                    // This could be a part of a function call e.g. ("hello"):sub(), so we must leave the parentheses
                    Value::String(_) => false,
                    Value::Symbol(token_ref) => {
                        match token_ref.token_type() {
                            TokenType::Symbol { symbol } => match symbol {
                                // If we have an ellipse inside of parentheses, we may also be culling values
                                // Therefore, we don't remove parentheses
                                Symbol::Ellipse => false,
                                _ => true,
                            },
                            _ => true,
                        }
                    }
                    _ => true,
                }
            }
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats an Expression node
    pub fn format_expression<'ast>(&mut self, expression: &Expression<'ast>) -> Expression<'ast> {
        match expression {
            Expression::Value {
                value,
                #[cfg(feature = "luau")]
                type_assertion,
            } => Expression::Value {
                value: Box::new(self.format_value(value)),
                #[cfg(feature = "luau")]
                type_assertion: match type_assertion {
                    Some(assertion) => Some(self.format_type_assertion(assertion)),
                    None => None,
                },
            },
            Expression::Parentheses {
                contained,
                expression,
            } => {
                // Examine whether the internal expression requires parentheses
                // If it doesn't, `use_internal_expression` will return a Some(), containing the external expression
                // We should then return that external expression
                // Otherwise, it will return None, and therefore we should use the original expression
                let use_internal_expression = CodeFormatter::check_excess_parentheses(expression);

                if use_internal_expression {
                    self.format_expression(expression)
                } else {
                    Expression::Parentheses {
                        contained: self.format_contained_span(&contained),
                        expression: Box::new(self.format_expression(expression)),
                    }
                }
            }
            Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
                unop: self.format_unop(unop),
                expression: Box::new(self.format_expression(expression)),
            },
            Expression::BinaryOperator { lhs, binop, rhs } => Expression::BinaryOperator {
                lhs: Box::new(self.format_expression(lhs)),
                binop: self.format_binop(binop),
                rhs: Box::new(self.format_expression(rhs)),
            },
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats an Index Node
    pub fn format_index<'ast>(&mut self, index: &Index<'ast>) -> Index<'ast> {
        match index {
            Index::Brackets {
                brackets,
                expression,
            } => Index::Brackets {
                brackets: self.format_contained_span(&brackets),
                expression: self.format_expression(expression),
            },

            Index::Dot { dot, name } => Index::Dot {
                dot: self.format_token_reference(dot),
                name: self.format_token_reference(name),
            },
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a Prefix Node
    pub fn format_prefix<'ast>(&mut self, prefix: &Prefix<'ast>) -> Prefix<'ast> {
        match prefix {
            Prefix::Expression(expression) => {
                Prefix::Expression(self.format_expression(expression))
            }
            Prefix::Name(token_reference) => {
                Prefix::Name(self.format_token_reference(token_reference))
            }
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a Suffix Node
    pub fn format_suffix<'ast>(&mut self, suffix: &Suffix<'ast>) -> Suffix<'ast> {
        match suffix {
            Suffix::Call(call) => Suffix::Call(self.format_call(call)),
            Suffix::Index(index) => Suffix::Index(self.format_index(index)),
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a Value Node
    pub fn format_value<'ast>(&mut self, value: &Value<'ast>) -> Value<'ast> {
        match value {
            Value::Function((token_reference, function_body)) => {
                Value::Function(self.format_anonymous_function(token_reference, function_body))
            }
            Value::FunctionCall(function_call) => {
                Value::FunctionCall(self.format_function_call(function_call))
            }
            Value::Number(token_reference) => {
                Value::Number(self.format_token_reference(token_reference))
            }
            Value::ParenthesesExpression(expression) => {
                Value::ParenthesesExpression(self.format_expression(expression))
            }
            Value::String(token_reference) => {
                Value::String(self.format_token_reference(token_reference))
            }
            Value::Symbol(token_reference) => {
                Value::Symbol(self.format_token_reference(token_reference))
            }
            Value::TableConstructor(table_constructor) => {
                Value::TableConstructor(self.format_table_constructor(table_constructor))
            }
            Value::Var(var) => Value::Var(self.format_var(var)),
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a Var Node
    pub fn format_var<'ast>(&mut self, var: &Var<'ast>) -> Var<'ast> {
        match var {
            Var::Name(token_reference) => Var::Name(self.format_token_reference(token_reference)),
            Var::Expression(var_expression) => {
                Var::Expression(self.format_var_expression(var_expression))
            }
            other => panic!("unknown node {:?}", other),
        }
    }

    pub fn format_var_expression<'ast>(
        &mut self,
        var_expression: &VarExpression<'ast>,
    ) -> VarExpression<'ast> {
        let formatted_prefix = self.format_prefix(var_expression.prefix());
        let formatted_suffixes = var_expression
            .suffixes()
            .map(|x| self.format_suffix(x))
            .collect();

        VarExpression::new(formatted_prefix).with_suffixes(formatted_suffixes)
    }

    /// Formats an UnOp Node
    pub fn format_unop<'ast>(&self, unop: &UnOp<'ast>) -> UnOp<'ast> {
        fmt_op!(self, UnOp, unop, {
            Minus = "-",
            Not = "not ",
            Hash = "#",
        })
    }
}
