use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
    LocalFunction, MethodCall, Parameter, Value,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;
use std::boxed::Box;

use crate::formatters::{
    expression_formatter, format_contained_span, format_plain_token_reference, format_punctuated,
    format_symbol, format_token_reference,
};

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
            parentheses,
            arguments,
        } => {
            let formatted_arguments =
                format_punctuated(arguments, &expression_formatter::format_expression);

            FunctionArgs::Parentheses {
                parentheses: format_contained_span(parentheses),
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

/// Formats a FunctionBody node
pub fn format_function_body<'ast>(function_body: FunctionBody<'ast>) -> FunctionBody<'ast> {
    let parameters_parentheses =
        format_contained_span(function_body.parameters_parentheses().to_owned());
    let formatted_parameters = format_parameters(function_body.to_owned());
    let end_token = format_symbol(
        function_body.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    function_body
        .with_parameters_parentheses(parameters_parentheses)
        .with_parameters(formatted_parameters)
        .with_end_token(end_token)
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

/// Formats a FunctionName node
pub fn format_function_name<'ast>(function_name: FunctionName<'ast>) -> FunctionName<'ast> {
    // TODO: This is based off formatters::format_punctuated - can we merge them into one?
    let mut formatted_names = Punctuated::new();
    for pair in function_name.names().to_owned().into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = format_symbol(
                    punctuation.into_owned(),
                    TokenReference::symbol(".").unwrap(),
                );
                let formatted_value = format_token_reference(value);
                formatted_names.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = format_token_reference(value);
                formatted_names.push(Pair::new(formatted_value, None));
            }
        }
    }

    let mut formatted_method: Option<(
        Cow<'ast, TokenReference<'ast>>,
        Cow<'ast, TokenReference<'ast>>,
    )> = None;

    match function_name.method_colon() {
        Some(method_colon) => {
            match function_name.method_name() {
                Some(token_reference) => {
                    formatted_method = Some((
                        format_symbol(
                            method_colon.to_owned(),
                            TokenReference::symbol(":").unwrap(),
                        ),
                        Cow::Owned(format_plain_token_reference(token_reference.to_owned())),
                    ));
                }
                None => (),
            };
        }
        None => (),
    };

    function_name
        .with_names(formatted_names)
        .with_method(formatted_method)
}

/// Formats a FunctionDeclaration node
pub fn format_function_declaration<'ast>(
    function_declaration: FunctionDeclaration<'ast>,
) -> FunctionDeclaration<'ast> {
    let function_token = format_symbol(
        function_declaration.function_token().to_owned(),
        TokenReference::symbol("function ").unwrap(),
    );
    let formatted_function_name = format_function_name(function_declaration.name().to_owned());
    let formatted_function_body = format_function_body(function_declaration.body().to_owned());

    function_declaration
        .with_function_token(function_token)
        .with_name(formatted_function_name)
        .with_body(formatted_function_body)
}

/// Formats a LocalFunction node
pub fn format_local_function<'ast>(local_function: LocalFunction<'ast>) -> LocalFunction<'ast> {
    let local_token = format_symbol(
        local_function.local_token().to_owned(),
        TokenReference::symbol("local ").unwrap(),
    );
    let function_token = format_symbol(
        local_function.function_token().to_owned(),
        TokenReference::symbol("function ").unwrap(),
    );
    let formatted_name = Cow::Owned(format_plain_token_reference(
        local_function.name().to_owned(),
    ));
    let formatted_function_body = format_function_body(local_function.func_body().to_owned());

    local_function
        .with_local_token(local_token)
        .with_function_token(function_token)
        .with_name(formatted_name)
        .with_func_body(formatted_function_body)
}

/// Formats a MethodCall node
pub fn format_method_call<'ast>(method_call: MethodCall<'ast>) -> MethodCall<'ast> {
    let formatted_colon_token = format_plain_token_reference(method_call.colon_token().to_owned());
    let formatted_name = format_plain_token_reference(method_call.name().to_owned());
    let formatted_function_args = format_function_args(method_call.args().to_owned());
    method_call
        .with_colon_token(Cow::Owned(formatted_colon_token))
        .with_name(Cow::Owned(formatted_name))
        .with_args(formatted_function_args)
}

/// Formats a single Parameter node
pub fn format_parameter<'ast>(parameter: Parameter<'ast>) -> Parameter<'ast> {
    match parameter {
        Parameter::Ellipse(token) => Parameter::Ellipse(format_symbol(
            token.into_owned(),
            TokenReference::symbol("...").unwrap(),
        )),
        Parameter::Name(token_reference) => {
            Parameter::Name(format_token_reference(token_reference))
        }
    }
}

/// Utilises the FunctionBody iterator to format a list of Parameter nodes
fn format_parameters<'ast>(function_body: FunctionBody<'ast>) -> Punctuated<'ast, Parameter<'ast>> {
    let mut formatted_parameters = Punctuated::new();
    let mut parameters_iterator = function_body.iter_parameters().peekable();
    loop {
        match parameters_iterator.next() {
            Some(parameter) => {
                let formatted_parameter = format_parameter(parameter.to_owned());
                let mut punctuation = None;

                if let Some(_) = parameters_iterator.peek() {
                    punctuation = Some(Cow::Owned(TokenReference::symbol(", ").unwrap()));
                }

                formatted_parameters.push(Pair::new(formatted_parameter, punctuation))
            }
            None => break,
        }
    }
    formatted_parameters
}
