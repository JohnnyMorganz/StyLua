use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionCall, MethodCall, Value,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;
use std::boxed::Box;

use crate::formatters::{expression_formatter, format_plain_token_reference, format_punctuated};

/// Formats a Call node
pub fn format_call<'ast>(call: Call<'ast>) -> Call<'ast> {
    match call {
        Call::AnonymousCall(function_args) => {
            Call::AnonymousCall(format_function_args(function_args))
        }
        Call::MethodCall(method_call) => Call::MethodCall(format_method_call(method_call)),
    }
}

/// Formats a FunctionArgs node
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

/// Formats a FunctionCall node
pub fn format_function_call<'ast>(function_call: FunctionCall<'ast>) -> FunctionCall<'ast> {
    let formatted_prefix = expression_formatter::format_prefix(function_call.prefix().to_owned());
    let formatted_suffixes = function_call
        .iter_suffixes()
        .map(|x| expression_formatter::format_suffix(x.to_owned()))
        .collect();
    function_call
        .with_prefix(formatted_prefix)
        .with_suffixes(formatted_suffixes)
}

/// Formats a MethodCall node
pub fn format_method_call<'ast>(method_call: MethodCall<'ast>) -> MethodCall<'ast> {
    let formatted_colon_token = format_plain_token_reference(method_call.colon_token().to_owned());
    let formatted_name = format_plain_token_reference(method_call.colon_token().to_owned());
    let formatted_function_args = format_function_args(method_call.args().to_owned());
    method_call
        .with_colon_token(Cow::Owned(formatted_colon_token))
        .with_name(Cow::Owned(formatted_name))
        .with_args(formatted_function_args)
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
