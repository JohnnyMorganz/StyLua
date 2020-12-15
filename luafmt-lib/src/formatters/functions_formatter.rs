use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Expression, FunctionArgs, Value,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;
use std::boxed::Box;

#[derive(Default)]
pub struct FunctionsFormatter;

impl FunctionsFormatter {
    /// Formats Function Arguments to ensure they are encompassed by parentheses
    fn format_function_args<'ast>(
        &mut self,
        function_args: FunctionArgs<'ast>,
    ) -> FunctionArgs<'ast> {
        match function_args {
            FunctionArgs::String(token_ref) => {
                let mut arguments = Punctuated::new();
                arguments.push(Pair::new(
                    Expression::Value {
                        value: Box::new(Value::String(token_ref)),
                        binop: None,
                    },
                    None,
                ));

                FunctionArgs::Parentheses {
                    parentheses: ContainedSpan::new(
                        Cow::Owned(TokenReference::symbol("(").unwrap()),
                        Cow::Owned(TokenReference::symbol(")").unwrap()),
                    ),
                    arguments: arguments,
                }
            },
            FunctionArgs::TableConstructor(table_constructor) => {
                let mut arguments = Punctuated::new();
                arguments.push(Pair::new(
                    Expression::Value {
                        value: Box::new(Value::TableConstructor(table_constructor)),
                        binop: None,
                    },
                    None,
                ));

                FunctionArgs::Parentheses {
                    parentheses: ContainedSpan::new(
                        Cow::Owned(TokenReference::symbol("(").unwrap()),
                        Cow::Owned(TokenReference::symbol(")").unwrap()),
                    ),
                    arguments: arguments,
                }
            }
            _ => function_args
        }
    }
}

impl<'ast> VisitorMut<'ast> for FunctionsFormatter {
    fn visit_function_args(&mut self, node: FunctionArgs<'ast>) -> FunctionArgs<'ast> {
        self.format_function_args(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::formatters::functions_formatter::FunctionsFormatter;
    use full_moon::visitors::VisitorMut;
    use full_moon::{parse, print};
    #[test]
    fn test_function_args_string() {
        let mut visitor = FunctionsFormatter::default();
        let ast = parse("foo'bar'").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "foo('bar')");
    }

    #[test]
    fn test_function_args_table_constructor() {
        let mut visitor = FunctionsFormatter::default();
        let ast = parse("foo{bar = 'baz'}").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "foo({bar = 'baz'})");
    }
}
