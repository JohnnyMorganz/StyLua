use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
    LocalFunction, MethodCall, Parameter, Value,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::borrow::Cow;
use std::boxed::Box;

#[cfg(feature = "luau")]
use crate::formatters::luau_formatter;
use crate::formatters::{
    block_formatter, expression_formatter, get_line_ending_character, trivia_formatter,
    CodeFormatter,
};

/// Formats an Anonymous Function
/// This doesn't have its own struct, but it is part of Value::Function
pub fn format_anonymous_function<'ast>(
    code_formatter: &mut CodeFormatter,
    function_token: Cow<'ast, TokenReference<'ast>>,
    function_body: FunctionBody<'ast>,
) -> (Cow<'ast, TokenReference<'ast>>, FunctionBody<'ast>) {
    let function_token_range = block_formatter::get_token_range(function_token.token());
    let additional_indent_level = code_formatter.get_range_indent_increase(function_token_range); //code_formatter.get_token_indent_increase(function_token.token());

    let function_token = code_formatter.format_symbol(
        function_token.into_owned(),
        TokenReference::symbol("function").unwrap(),
    );

    let function_body = format_function_body(code_formatter, function_body);

    // Need to insert any additional trivia, as it isn't being inserted elsewhere
    let parameters_parentheses = trivia_formatter::contained_span_add_trivia(
        function_body.parameters_parentheses().to_owned(),
        None,
        Some(vec![code_formatter.create_newline_trivia()]),
    );

    let end_token = Cow::Owned(trivia_formatter::token_reference_add_trivia(
        function_body.end_token().to_owned(),
        Some(vec![
            code_formatter.create_indent_trivia(additional_indent_level)
        ]),
        None,
    ));

    (
        function_token,
        function_body
            .with_parameters_parentheses(parameters_parentheses)
            .with_end_token(end_token),
    )
}

/// Formats a Call node
pub fn format_call<'ast>(code_formatter: &mut CodeFormatter, call: Call<'ast>) -> Call<'ast> {
    match call {
        Call::AnonymousCall(function_args) => {
            Call::AnonymousCall(format_function_args(code_formatter, function_args))
        }
        Call::MethodCall(method_call) => {
            Call::MethodCall(format_method_call(code_formatter, method_call))
        }
    }
}

/// Formats a FunctionArgs node
pub fn format_function_args<'ast>(
    code_formatter: &mut CodeFormatter,
    function_args: FunctionArgs<'ast>,
) -> FunctionArgs<'ast> {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => {
            let (start_parens, end_parens) = parentheses.tokens();
            // Find the range of the function arguments
            let function_call_range = (
                start_parens.end_position().bytes(),
                end_parens.start_position().bytes(),
            );
            let is_multiline = (function_call_range.1 - function_call_range.0) > 80; // TODO: Properly determine this arbitrary number, and see if other factors should come into play

            if is_multiline {
                // TODO: This is similar to multiline in TableConstructor, can we resolve?
                // Format start and end brace properly with correct trivia

                // Calculate to see if the end parentheses requires any additional indentation
                let end_parens_additional_indent_level =
                    code_formatter.get_range_indent_increase((
                        end_parens.start_position().bytes(),
                        end_parens.end_position().bytes(),
                    ));
                let end_parens_leading_trivia =
                    vec![code_formatter.create_indent_trivia(end_parens_additional_indent_level)];

                // Add new_line trivia to start_brace
                let start_parens_token = TokenReference::symbol(
                    &(String::from("(")
                        + &get_line_ending_character(&code_formatter.config.line_endings)),
                )
                .unwrap();
                let end_parens_token = TokenReference::new(
                    end_parens_leading_trivia,
                    Token::new(TokenType::Symbol {
                        symbol: Symbol::RightParen,
                    }),
                    vec![],
                );
                let parentheses = ContainedSpan::new(
                    code_formatter.format_symbol(start_parens.to_owned(), start_parens_token),
                    code_formatter.format_symbol(end_parens.to_owned(), end_parens_token),
                );

                let mut formatted_arguments = Punctuated::new();
                let mut current_arguments = arguments.iter().peekable();

                code_formatter.add_indent_range(function_call_range);

                loop {
                    match current_arguments.next() {
                        Some(argument) => {
                            let argument_range = block_formatter::get_range_in_expression(argument);
                            let additional_indent_level =
                                code_formatter.get_range_indent_increase(argument_range);

                            let formatted_argument =
                                trivia_formatter::expression_add_leading_trivia(
                                    expression_formatter::format_expression(
                                        code_formatter,
                                        argument.to_owned(),
                                    ),
                                    vec![code_formatter
                                        .create_indent_trivia(additional_indent_level)],
                                );

                            let punctuation = match current_arguments.peek() {
                                Some(_) => {
                                    let symbol = String::from(",")
                                        + &get_line_ending_character(
                                            &code_formatter.config.line_endings,
                                        );
                                    Some(Cow::Owned(TokenReference::symbol(&symbol).unwrap()))
                                }
                                None => Some(Cow::Owned(TokenReference::new(
                                    vec![],
                                    Token::new(TokenType::Whitespace {
                                        characters: Cow::Owned(get_line_ending_character(
                                            &code_formatter.config.line_endings,
                                        )),
                                    }),
                                    vec![],
                                ))),
                            };

                            formatted_arguments.push(Pair::new(formatted_argument, punctuation))
                        }
                        None => break,
                    }
                }

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments: formatted_arguments,
                }
            } else {
                let formatted_arguments = code_formatter
                    .format_punctuated(arguments, &expression_formatter::format_expression);

                FunctionArgs::Parentheses {
                    parentheses: code_formatter.format_contained_span(parentheses),
                    arguments: formatted_arguments,
                }
            }
        }

        FunctionArgs::String(token_reference) => {
            let mut arguments = Punctuated::new();
            arguments.push(Pair::new(
                expression_formatter::format_expression(
                    code_formatter,
                    Expression::Value {
                        value: Box::new(Value::String(token_reference)),
                        binop: None,
                        #[cfg(feature = "luau")]
                        as_assertion: None,
                    },
                ),
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
                expression_formatter::format_expression(
                    code_formatter,
                    Expression::Value {
                        value: Box::new(Value::TableConstructor(table_constructor)),
                        binop: None,
                        #[cfg(feature = "luau")]
                        as_assertion: None,
                    },
                ),
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
pub fn format_function_body<'ast>(
    code_formatter: &mut CodeFormatter,
    function_body: FunctionBody<'ast>,
) -> FunctionBody<'ast> {
    let parameters_parentheses =
        code_formatter.format_contained_span(function_body.parameters_parentheses().to_owned());
    let formatted_parameters = format_parameters(code_formatter, function_body.to_owned());

    #[cfg(feature = "luau")]
    let mut type_specifiers;
    #[cfg(feature = "luau")]
    let return_type;

    #[cfg(feature = "luau")]
    {
        type_specifiers = Vec::new();
        for specifier in function_body.type_specifiers() {
            let formatted_specifier = match specifier {
                Some(specifier) => Some(luau_formatter::format_type_specifier(
                    code_formatter,
                    specifier.to_owned(),
                )),
                None => None,
            };
            type_specifiers.push(formatted_specifier);
        }

        return_type = match function_body.return_type() {
            Some(return_type) => Some(luau_formatter::format_type_specifier(
                code_formatter,
                return_type.to_owned(),
            )),
            None => None,
        };
    }

    let end_token = code_formatter.format_symbol(
        function_body.end_token().to_owned(),
        TokenReference::symbol("end").unwrap(),
    );

    #[cfg(feature = "luau")]
    let function_body = function_body
        .with_type_specifiers(type_specifiers)
        .with_return_type(return_type);

    function_body
        .with_parameters_parentheses(parameters_parentheses)
        .with_parameters(formatted_parameters)
        .with_end_token(end_token)
}

/// Formats a FunctionCall node
pub fn format_function_call<'ast>(
    code_formatter: &mut CodeFormatter,
    function_call: FunctionCall<'ast>,
) -> FunctionCall<'ast> {
    let formatted_prefix =
        expression_formatter::format_prefix(code_formatter, function_call.prefix().to_owned());
    let formatted_suffixes = function_call
        .iter_suffixes()
        .map(|x| expression_formatter::format_suffix(code_formatter, x.to_owned()))
        .collect();
    function_call
        .with_prefix(formatted_prefix)
        .with_suffixes(formatted_suffixes)
}

/// Formats a FunctionName node
pub fn format_function_name<'ast>(
    code_formatter: &mut CodeFormatter,
    function_name: FunctionName<'ast>,
) -> FunctionName<'ast> {
    // TODO: This is based off formatters::format_punctuated - can we merge them into one?
    let mut formatted_names = Punctuated::new();
    for pair in function_name.names().to_owned().into_pairs() {
        // Format Punctuation
        match pair {
            Pair::Punctuated(value, punctuation) => {
                let formatted_punctuation = code_formatter.format_symbol(
                    punctuation.into_owned(),
                    TokenReference::symbol(".").unwrap(),
                );
                let formatted_value = code_formatter.format_token_reference(value);
                formatted_names.push(Pair::new(formatted_value, Some(formatted_punctuation)));
            }
            Pair::End(value) => {
                let formatted_value = code_formatter.format_token_reference(value);
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
                        code_formatter.format_symbol(
                            method_colon.to_owned(),
                            TokenReference::symbol(":").unwrap(),
                        ),
                        Cow::Owned(
                            code_formatter.format_plain_token_reference(token_reference.to_owned()),
                        ),
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
    code_formatter: &mut CodeFormatter,
    function_declaration: FunctionDeclaration<'ast>,
) -> FunctionDeclaration<'ast> {
    let function_token = code_formatter.format_symbol(
        function_declaration.function_token().to_owned(),
        TokenReference::symbol("function ").unwrap(),
    );
    let formatted_function_name =
        format_function_name(code_formatter, function_declaration.name().to_owned());
    let formatted_function_body =
        format_function_body(code_formatter, function_declaration.body().to_owned());

    function_declaration
        .with_function_token(function_token)
        .with_name(formatted_function_name)
        .with_body(formatted_function_body)
}

/// Formats a LocalFunction node
pub fn format_local_function<'ast>(
    code_formatter: &mut CodeFormatter,
    local_function: LocalFunction<'ast>,
) -> LocalFunction<'ast> {
    let local_token = code_formatter.format_symbol(
        local_function.local_token().to_owned(),
        TokenReference::symbol("local ").unwrap(),
    );
    let function_token = code_formatter.format_symbol(
        local_function.function_token().to_owned(),
        TokenReference::symbol("function ").unwrap(),
    );
    let formatted_name =
        Cow::Owned(code_formatter.format_plain_token_reference(local_function.name().to_owned()));
    let formatted_function_body =
        format_function_body(code_formatter, local_function.func_body().to_owned());

    local_function
        .with_local_token(local_token)
        .with_function_token(function_token)
        .with_name(formatted_name)
        .with_func_body(formatted_function_body)
}

/// Formats a MethodCall node
pub fn format_method_call<'ast>(
    code_formatter: &mut CodeFormatter,
    method_call: MethodCall<'ast>,
) -> MethodCall<'ast> {
    let formatted_colon_token =
        code_formatter.format_plain_token_reference(method_call.colon_token().to_owned());
    let formatted_name = code_formatter.format_plain_token_reference(method_call.name().to_owned());
    let formatted_function_args =
        format_function_args(code_formatter, method_call.args().to_owned());
    method_call
        .with_colon_token(Cow::Owned(formatted_colon_token))
        .with_name(Cow::Owned(formatted_name))
        .with_args(formatted_function_args)
}

/// Formats a single Parameter node
pub fn format_parameter<'ast>(
    code_formatter: &mut CodeFormatter,
    parameter: Parameter<'ast>,
) -> Parameter<'ast> {
    match parameter {
        Parameter::Ellipse(token) => Parameter::Ellipse(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol("...").unwrap()),
        ),
        Parameter::Name(token_reference) => {
            Parameter::Name(code_formatter.format_token_reference(token_reference))
        }
    }
}

/// Utilises the FunctionBody iterator to format a list of Parameter nodes
fn format_parameters<'ast>(
    code_formatter: &mut CodeFormatter,
    function_body: FunctionBody<'ast>,
) -> Punctuated<'ast, Parameter<'ast>> {
    let mut formatted_parameters = Punctuated::new();
    let mut parameters_iterator = function_body.parameters().iter().peekable();
    loop {
        match parameters_iterator.next() {
            Some(parameter) => {
                let formatted_parameter = format_parameter(code_formatter, parameter.to_owned());
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
