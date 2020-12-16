use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Expression, FunctionArgs, Value,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;
use std::boxed::Box;

use crate::formatters::{expression_formatter, format_punctuated};

pub fn format_function_args<'ast>(function_args: FunctionArgs<'ast>) -> FunctionArgs<'ast> {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses: _,
            arguments,
        } => {
            let formatted_arguments =
                format_punctuated(arguments, &expression_formatter::format_expression);

            FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    Cow::Owned(TokenReference::symbol("(").unwrap()),
                    Cow::Owned(TokenReference::symbol(")").unwrap()),
                ),
                arguments: formatted_arguments,
            }
        }

        FunctionArgs::String(token_reference) => {
            let mut arguments = Punctuated::new();
            arguments.push(Pair::new(
                expression_formatter::format_expression(Expression::Value {
                    value: Box::new(Value::String(token_reference)),
                    binop: None,
                }),
                None, // Only single argument, so no trailing comma
            ));

            FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    Cow::Owned(TokenReference::symbol("(").unwrap()),
                    Cow::Owned(TokenReference::symbol(")").unwrap()),
                ),
                arguments,
            }
        }

        FunctionArgs::TableConstructor(table_constructor) => {
            let mut arguments = Punctuated::new();
            arguments.push(Pair::new(
                expression_formatter::format_expression(Expression::Value {
                    value: Box::new(Value::TableConstructor(table_constructor)),
                    binop: None,
                }),
                None,
            ));

            FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    Cow::Owned(TokenReference::symbol("(").unwrap()),
                    Cow::Owned(TokenReference::symbol(")").unwrap()),
                ),
                arguments,
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::formatters::functions_formatter::FunctionsFormatter;
//     use full_moon::visitors::VisitorMut;
//     use full_moon::{parse, print};
//     #[test]
//     fn test_function_args_string() {
//         let mut visitor = FunctionsFormatter::default();
//         let ast = parse("foo'bar'").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "foo('bar')");
//     }

//     #[test]
//     fn test_function_args_table_constructor() {
//         let mut visitor = FunctionsFormatter::default();
//         let ast = parse("foo{bar = 'baz'}").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "foo({bar = 'baz'})");
//     }

//     #[test]
//     fn test_function_args_single_argument() {
//         let mut visitor = FunctionsFormatter::default();
//         let ast = parse("foo(   bar )").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "foo(bar)");
//     }

//     #[test]
//     fn test_function_args_multiple_arguments() {
//         let mut visitor = FunctionsFormatter::default();
//         let ast = parse("foo(bar,baz     ,    foo)").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "foo(bar, baz, foo)");
//     }
// }
