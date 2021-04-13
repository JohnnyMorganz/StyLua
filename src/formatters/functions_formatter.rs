use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
    LocalFunction, MethodCall, Parameter, Suffix, Value,
};
use full_moon::node::Node;
use full_moon::tokenizer::{Symbol, Token, TokenKind, TokenReference, TokenType};
use std::boxed::Box;

use crate::formatters::{
    trivia_formatter::{
        strip_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
    },
    trivia_util, CodeFormatter, EndTokenType,
};

impl CodeFormatter {
    /// Formats an Anonymous Function
    /// This doesn't have its own struct, but it is part of Value::Function
    pub fn format_anonymous_function<'ast>(
        &mut self,
        function_token: &TokenReference<'ast>,
        function_body: &FunctionBody<'ast>,
    ) -> (TokenReference<'ast>, FunctionBody<'ast>) {
        let function_token_range = CodeFormatter::get_token_range(function_token.token());
        let additional_indent_level = self.get_range_indent_increase(function_token_range); //code_formatter.get_token_indent_increase(function_token.token());

        let function_token = crate::fmt_symbol!(self, function_token, "function");
        let mut function_body = self.format_function_body(function_body, false);

        // Need to insert any additional trivia, as it isn't being inserted elsewhere
        #[cfg(feature = "luau")]
        {
            let (parameters_parentheses, return_type) = match function_body.return_type() {
                Some(return_type) => (
                    function_body.parameters_parentheses().to_owned(),
                    Some(
                        return_type.update_trailing_trivia(FormatTriviaType::Append(vec![
                            self.create_newline_trivia()
                        ])),
                    ),
                ),
                None => (
                    // No return type, so add trivia to the parentheses instead
                    function_body
                        .parameters_parentheses()
                        .update_trailing_trivia(FormatTriviaType::Append(vec![
                            self.create_newline_trivia()
                        ])),
                    None,
                ),
            };

            function_body = function_body
                .with_parameters_parentheses(parameters_parentheses)
                .with_return_type(return_type);
        }

        #[cfg(not(feature = "luau"))]
        {
            let parameters_parentheses = function_body
                .parameters_parentheses()
                .update_trailing_trivia(FormatTriviaType::Append(vec![
                    self.create_newline_trivia()
                ]));
            function_body = function_body.with_parameters_parentheses(parameters_parentheses);
        };

        let end_token = function_body
            .end_token()
            .update_leading_trivia(FormatTriviaType::Append(vec![
                self.create_indent_trivia(additional_indent_level)
            ]));

        (function_token, function_body.with_end_token(end_token))
    }

    /// Formats a Call node
    pub fn format_call<'ast>(&mut self, call: &Call<'ast>) -> Call<'ast> {
        match call {
            Call::AnonymousCall(function_args) => {
                Call::AnonymousCall(self.format_function_args(function_args))
            }
            Call::MethodCall(method_call) => Call::MethodCall(self.format_method_call(method_call)),
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a FunctionArgs node
    pub fn format_function_args<'ast>(
        &mut self,
        function_args: &FunctionArgs<'ast>,
    ) -> FunctionArgs<'ast> {
        match function_args {
            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            } => {
                let (start_parens, end_parens) = parentheses.tokens();
                // Find the range of the function arguments
                let function_call_range = (
                    Token::end_position(&start_parens).bytes(),
                    Token::start_position(&end_parens).bytes(),
                );
                let current_indent_width = self.get_indent_width()
                    + (self
                        .get_range_indent_increase(CodeFormatter::get_token_range(start_parens))
                        .unwrap_or(0)
                        * self.config.indent_width);

                // Format all the arguments, so that we can prepare them and check to see whether they need expanding
                // We will ignore punctuation for now
                let mut first_iter_formatted_arguments = Vec::new();
                for argument in arguments.iter() {
                    first_iter_formatted_arguments.push(self.format_expression(argument))
                }

                // Apply some heuristics to determine whether we should expand the function call
                // TODO: These are subject to change

                // If there is a comment present anywhere in between the start parentheses and end parentheses, we should keep it multiline
                let force_mutliline: bool =
                    if trivia_util::token_trivia_contains_comments(start_parens.trailing_trivia())
                        || trivia_util::token_trivia_contains_comments(end_parens.leading_trivia())
                    {
                        true
                    } else {
                        let mut contains_comments = false;
                        for argument in arguments.pairs() {
                            // Only check the leading and trailing trivia of the expression
                            // If the expression has inline comments, it should be handled elsewhere
                            // Allow this, as this is what rustfmt creates
                            #[allow(clippy::blocks_in_if_conditions)]
                            if trivia_util::get_expression_leading_trivia(argument.value())
                                .iter()
                                .chain(
                                    trivia_util::get_expression_trailing_trivia(argument.value())
                                        .iter(),
                                )
                                .any(|x| {
                                    x.token_kind() == TokenKind::SingleLineComment
                                        || x.token_kind() == TokenKind::MultiLineComment
                                })
                            {
                                contains_comments = true;
                            } else if let Some(punctuation) = argument.punctuation() {
                                if trivia_util::token_contains_comments(punctuation) {
                                    contains_comments = true;
                                }
                            };

                            if contains_comments {
                                break;
                            }
                        }

                        contains_comments
                    };

                let mut is_multiline = force_mutliline;

                if !force_mutliline {
                    // If we only have one argument then we will not make it multi line (expanding it would have little value)
                    // Unless, the argument is a hangable expression
                    if first_iter_formatted_arguments.len() == 1
                        && !trivia_util::can_hang_expression(
                            first_iter_formatted_arguments.first().unwrap(),
                        )
                    {
                        is_multiline = false;
                    } else {
                        // Find how far we are currently indented, we can use this to determine when to expand
                        // We will expand on two occasions:
                        // 1) If a group of arguments fall on a single line, and they surpass the column width setting
                        // 2) If we have a mixture of multiline (tables/anonymous functions) and other values. For
                        //    example, call({ ... }, foo, { ... }), should be expanded, but
                        //    call(foo, { ... }) or call(foo, { ... }, foo) can stay on one line, provided the
                        //    single line arguments dont surpass the column width setting

                        // TODO: We need to add more to this - there may be quite a lot before this function call
                        let mut width_passed = current_indent_width;

                        // Use state values to determine the type of arguments we have seen so far
                        let mut seen_multiline_arg = false; // Whether we have seen a multiline table/function already
                        let mut seen_other_arg_after_multiline = false; // Whether we have seen a non multiline table/function after a multiline one. In this case, we should expand

                        for argument in first_iter_formatted_arguments.iter() {
                            match argument {
                                Expression::Value { value, .. } => {
                                    match &**value {
                                        // Check to see if we have a table constructor, or anonymous function
                                        Value::Function(_) => {
                                            // If we have a mixture of multiline args, and other arguments
                                            // Then the function args should be expanded
                                            if seen_multiline_arg && seen_other_arg_after_multiline
                                            {
                                                is_multiline = true;
                                                break;
                                            }

                                            seen_multiline_arg = true;

                                            // Reset the width count back
                                            width_passed = current_indent_width;
                                        }
                                        Value::TableConstructor(table) => {
                                            // Check to see whether it has been expanded
                                            let start_brace = table.braces().tokens().0;
                                            let is_expanded = trivia_util::trivia_contains_newline(
                                                start_brace.trailing_trivia(),
                                            );
                                            if is_expanded {
                                                // If we have a mixture of multiline args, and other arguments
                                                // Then the function args should be expanded
                                                if seen_multiline_arg
                                                    && seen_other_arg_after_multiline
                                                {
                                                    is_multiline = true;
                                                    break;
                                                }

                                                seen_multiline_arg = true;
                                                // Reset the width count back
                                                width_passed = current_indent_width;
                                            } else {
                                                // We have a collapsed table constructor - add the width, and if it fails,
                                                // we need to expand
                                                width_passed += argument.to_string().len();
                                                if width_passed > self.config.column_width - 20 {
                                                    is_multiline = true;
                                                    break;
                                                }
                                            }
                                        }
                                        _ => {
                                            // If we previously had a table/anonymous function, and we have something else
                                            // in the mix, update the state to respond to this
                                            if seen_multiline_arg {
                                                seen_other_arg_after_multiline = true;
                                            }
                                            width_passed += argument.to_string().len();
                                            if width_passed > self.config.column_width - 20 {
                                                // We have passed 80 characters without a table or anonymous function
                                                // There is nothing else stopping us from expanding - so we will
                                                is_multiline = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                // TODO: Parentheses/UnOp, do we need to do more checking?
                                // We will continue counting on the width_passed
                                _ => {
                                    // If we previously had a table/anonymous function, and we have something else
                                    // in the mix, update the state to respond to this
                                    if seen_multiline_arg {
                                        seen_other_arg_after_multiline = true;
                                    }

                                    width_passed += argument.to_string().len();
                                    if width_passed > self.config.column_width - 20 {
                                        // We have passed 80 characters without a table or anonymous function
                                        // There is nothing else stopping us from expanding - so we will
                                        is_multiline = true;
                                        break;
                                    }
                                }
                            }

                            // Add width which would be taken up by comment and space
                            width_passed += 2;
                        }
                    }
                }

                if is_multiline {
                    // TODO: This is similar to multiline in TableConstructor, can we resolve?
                    // Format start and end brace properly with correct trivia

                    // Calculate to see if the end parentheses requires any additional indentation
                    let end_parens_additional_indent_level = self.get_range_indent_increase((
                        Token::start_position(&end_parens).bytes(),
                        Token::end_position(&end_parens).bytes(),
                    ));
                    let end_parens_leading_trivia =
                        vec![self.create_indent_trivia(end_parens_additional_indent_level)];

                    // Add new_line trivia to start_parens
                    let start_parens_token = crate::fmt_symbol!(self, start_parens, "(")
                        .update_trailing_trivia(FormatTriviaType::Append(vec![
                            self.create_newline_trivia()
                        ]));

                    let end_parens_token = TokenReference::new(
                        end_parens_leading_trivia,
                        Token::new(TokenType::Symbol {
                            symbol: Symbol::RightParen,
                        }),
                        vec![],
                    );

                    let parentheses = ContainedSpan::new(
                        start_parens_token,
                        self.format_symbol(end_parens, &end_parens_token),
                    );

                    let mut formatted_arguments = Punctuated::new();
                    // Iterate through the original formatted_arguments, so we can see how the formatted version would look like
                    let mut formatted_versions = first_iter_formatted_arguments.iter();

                    self.add_indent_range(function_call_range);

                    for argument in arguments.pairs() {
                        // See what the formatted version of the argument would look like
                        let formatted_version = formatted_versions
                            .next()
                            .expect("less arguments than expected");

                        let argument_range =
                            CodeFormatter::get_range_in_expression(argument.value());
                        let additional_indent_level =
                            self.get_range_indent_increase(argument_range);

                        let indent_spacing = (self.indent_level
                            + additional_indent_level.unwrap_or(0))
                            * self.config.indent_width;
                        let require_multiline_expression =
                            trivia_util::can_hang_expression(argument.value())
                                && indent_spacing
                                    + formatted_version
                                        .to_string()
                                        .lines()
                                        .next()
                                        .expect("no lines")
                                        .len()
                                    > self.config.column_width;

                        // Unfortunately, we need to format again, taking into account in indent increase
                        // TODO: Can we fix this? We don't want to have to format twice
                        let mut formatted_argument = self.format_expression(argument.value());

                        // Hang the expression if necessary
                        if require_multiline_expression {
                            formatted_argument = self.hang_expression_no_trailing_newline(
                                formatted_argument,
                                additional_indent_level,
                                None,
                            );
                        }

                        // Add the leading indent for the argument
                        formatted_argument =
                            formatted_argument.update_leading_trivia(FormatTriviaType::Append(
                                vec![self.create_indent_trivia(additional_indent_level)],
                            ));

                        let punctuation = match argument.punctuation() {
                            Some(punctuation) => {
                                // Continue adding a comma and a new line for multiline function args
                                let symbol = crate::fmt_symbol!(self, punctuation, ",");
                                let symbol =
                                    symbol.update_trailing_trivia(FormatTriviaType::Append(vec![
                                        self.create_newline_trivia(),
                                    ]));

                                Some(symbol)
                            }
                            None => Some(TokenReference::new(
                                vec![],
                                self.create_newline_trivia(),
                                vec![],
                            )),
                        };

                        formatted_arguments.push(Pair::new(formatted_argument, punctuation))
                    }

                    FunctionArgs::Parentheses {
                        parentheses,
                        arguments: formatted_arguments,
                    }
                } else {
                    let parentheses = self.format_contained_span(&parentheses);

                    // If theres comments connected to the opening parentheses, we need to move them
                    let (start_parens, end_parens) = parentheses.tokens();
                    let mut parens_comments: Vec<Token<'ast>> = start_parens
                        .trailing_trivia()
                        .filter(|token| {
                            token.token_kind() == TokenKind::SingleLineComment
                                || token.token_kind() == TokenKind::MultiLineComment
                        })
                        .map(|x| {
                            // Prepend a single space beforehand
                            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                        })
                        .flatten()
                        .collect();

                    // Format the arguments, and move any comments within them
                    let (formatted_arguments, mut comments_buffer) =
                        self.format_punctuated(arguments, &CodeFormatter::format_expression);

                    parens_comments.append(&mut comments_buffer);

                    // Recreate parentheses with the comments removed from the opening parens
                    // and all the comments placed at the end of the closing parens
                    let parentheses = ContainedSpan::new(
                        start_parens.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
                        end_parens
                            .update_trailing_trivia(FormatTriviaType::Append(parens_comments)),
                    );

                    FunctionArgs::Parentheses {
                        parentheses,
                        arguments: formatted_arguments,
                    }
                }
            }

            FunctionArgs::String(token_reference) => {
                let mut arguments = Punctuated::new();
                let new_expression = self.format_expression(&Expression::Value {
                    value: Box::new(Value::String(token_reference.to_owned())),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                });

                // Remove any trailing comments from the expression, and move them into a buffer
                let (new_expression, comments_buffer) =
                    trivia_util::get_expression_trailing_comments(&new_expression);

                // Create parentheses, and add the trailing comments to the end of the parentheses
                let parentheses = ContainedSpan::new(
                    TokenReference::symbol("(").unwrap(),
                    TokenReference::symbol(")").unwrap(),
                )
                .update_trailing_trivia(FormatTriviaType::Append(comments_buffer));

                arguments.push(Pair::new(new_expression, None)); // Only single argument, so no trailing comma

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments,
                }
            }

            FunctionArgs::TableConstructor(table_constructor) => {
                let mut arguments = Punctuated::new();
                let new_expression = self.format_expression(&Expression::Value {
                    value: Box::new(Value::TableConstructor(table_constructor.to_owned())),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                });

                // Remove any trailing comments from the expression, and move them into a buffer
                let (new_expression, comments_buffer) =
                    trivia_util::get_expression_trailing_comments(&new_expression);

                // Create parentheses, and add the trailing comments to the end of the parentheses
                let parentheses = ContainedSpan::new(
                    TokenReference::symbol("(").unwrap(),
                    TokenReference::symbol(")").unwrap(),
                )
                .update_trailing_trivia(FormatTriviaType::Append(comments_buffer));

                arguments.push(Pair::new(new_expression, None)); // Only single argument, so no trailing comma

                FunctionArgs::Parentheses {
                    parentheses,
                    arguments,
                }
            }
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats a FunctionBody node
    pub fn format_function_body<'ast>(
        &mut self,
        function_body: &FunctionBody<'ast>,
        add_trivia: bool,
    ) -> FunctionBody<'ast> {
        // Calculate trivia
        let additional_indent_level = self
            .get_range_indent_increase(CodeFormatter::get_token_range(function_body.end_token()));
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        // Check if the parameters should be placed across multiple lines
        let multiline_params = {
            #[cfg(feature = "luau")]
            let mut type_specifiers = function_body.type_specifiers();

            // Check whether they contain comments
            let contains_comments = function_body.parameters().pairs().any(|pair| {
                let contains_comments = pair
                    .punctuation()
                    .map_or(false, |punc| trivia_util::token_contains_comments(punc))
                    || trivia_util::contains_comments(pair.value());
                #[cfg(feature = "luau")]
                let type_specifier_comments = type_specifiers
                    .next()
                    .flatten()
                    .map_or(false, |type_specifier| {
                        trivia_util::contains_comments(type_specifier)
                    });
                #[cfg(not(feature = "luau"))]
                let type_specifier_comments = false;
                contains_comments || type_specifier_comments
            });

            contains_comments
                || {
                    // Check the length of the parameters. We need to format them first onto a single line to check if required
                    let types_length: usize;

                    #[cfg(feature = "luau")]
                    {
                        types_length = function_body
                            .type_specifiers()
                            .chain(std::iter::once(function_body.return_type())) // Include optional return type
                            .map(|x| {
                                x.map_or(0, |specifier| {
                                    self.format_type_specifier(specifier).to_string().len()
                                })
                            })
                            .sum::<usize>()
                    }
                    #[cfg(not(feature = "luau"))]
                    {
                        types_length = 0
                    }

                    let line_length = self
                    .format_singleline_parameters(function_body)
                    .to_string()
                    .len()
                        + 2 // Account for the parentheses around the parameters
                        + types_length // Account for type specifiers and return type
                        + self.indent_level * self.config.indent_width
                        + self.get_range_indent_increase(CodeFormatter::get_token_range(function_body.parameters_parentheses().tokens().0)).unwrap_or(0) * self.config.indent_width;

                    line_length > self.config.column_width
                }
        };

        let (formatted_parameters, mut parameters_parentheses) = match multiline_params {
            true => {
                // TODO: This is similar to multiline in FunctionArgs, can we resolve?
                // Format start and end brace properly with correct trivia
                let (start_parens, end_parens) = function_body.parameters_parentheses().tokens();

                // Calculate to see if the end parentheses requires any additional indentation
                let end_parens_additional_indent_level = self.get_range_indent_increase((
                    Token::start_position(&end_parens).bytes(),
                    Token::end_position(&end_parens).bytes(),
                ));
                let end_parens_leading_trivia = vec![
                    self.create_newline_trivia(),
                    self.create_indent_trivia(end_parens_additional_indent_level),
                ];

                // Add new_line trivia to start_parens
                let start_parens_token = crate::fmt_symbol!(self, start_parens, "(")
                    .update_trailing_trivia(FormatTriviaType::Append(vec![
                        self.create_newline_trivia()
                    ]));

                let end_parens_token = TokenReference::new(
                    end_parens_leading_trivia,
                    Token::new(TokenType::Symbol {
                        symbol: Symbol::RightParen,
                    }),
                    vec![],
                );

                (
                    self.format_multiline_parameters(function_body),
                    ContainedSpan::new(
                        start_parens_token,
                        self.format_symbol(end_parens, &end_parens_token),
                    ),
                )
            }
            false => (
                self.format_singleline_parameters(function_body),
                self.format_contained_span(function_body.parameters_parentheses()),
            ),
        };

        #[cfg(feature = "luau")]
        let type_specifiers;
        #[cfg(feature = "luau")]
        let return_type;
        #[allow(unused_mut)]
        let mut added_trailing_trivia = false;

        #[cfg(feature = "luau")]
        {
            type_specifiers = function_body
                .type_specifiers()
                .map(|x| x.map(|specifier| self.format_type_specifier(specifier)))
                .collect();

            return_type = match function_body.return_type() {
                Some(return_type) => Some({
                    let formatted = self.format_type_specifier(return_type);
                    if add_trivia {
                        added_trailing_trivia = true;
                        formatted.update_trailing_trivia(FormatTriviaType::Append(
                            trailing_trivia.to_owned(),
                        ))
                    } else {
                        formatted
                    }
                }),
                None => None,
            };
        }

        if !added_trailing_trivia && add_trivia {
            parameters_parentheses = parameters_parentheses
                .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia.to_owned()))
        }

        let end_token = if add_trivia {
            self.format_end_token(function_body.end_token(), EndTokenType::BlockEnd)
                .update_trivia(
                    FormatTriviaType::Append(leading_trivia),
                    FormatTriviaType::Append(trailing_trivia),
                )
        } else {
            self.format_end_token(function_body.end_token(), EndTokenType::BlockEnd)
        };

        let function_body = function_body
            .to_owned()
            .with_parameters_parentheses(parameters_parentheses)
            .with_parameters(formatted_parameters)
            .with_end_token(end_token);

        #[cfg(feature = "luau")]
        let function_body = function_body
            .with_type_specifiers(type_specifiers)
            .with_return_type(return_type);

        function_body
    }

    /// Formats a FunctionCall node
    pub fn format_function_call<'ast>(
        &mut self,
        function_call: &FunctionCall<'ast>,
    ) -> FunctionCall<'ast> {
        let formatted_prefix = self.format_prefix(function_call.prefix());

        let num_suffixes = function_call.suffixes().count();
        let should_hang = {
            // Hang if there is atleast more than one method call suffix
            if function_call
                .suffixes()
                .filter(|x| matches!(x, Suffix::Call(Call::MethodCall(_))))
                .count()
                > 1
            {
                // Check if either a), we are surpassing the column width
                // Or b), one of the INTERNAL (not the last call) method call's arguments is multiline [function/table]

                // Create a temporary formatted version of suffixes to use for this check
                let formatted_suffixes = function_call
                    .suffixes()
                    .map(|x| self.format_suffix(x))
                    .collect();
                let preliminary_function_call = FunctionCall::new(formatted_prefix.to_owned())
                    .with_suffixes(formatted_suffixes);

                let outcome = if strip_trivia(&preliminary_function_call)
                    .to_string()
                    .lines()
                    .next()
                    .expect("no lines")
                    .len()
                    > self.config.column_width
                {
                    true
                } else {
                    let suffixes = preliminary_function_call.suffixes().enumerate();
                    let mut contains_newline = false;
                    for (idx, suffix) in suffixes {
                        // Check to see whether this suffix is an "internal" method call suffix
                        // i.e. we are not at the last MethodCall suffix
                        let mut remaining_suffixes =
                            preliminary_function_call.suffixes().skip(idx + 1);
                        if remaining_suffixes
                            .any(|x| matches!(x, Suffix::Call(Call::MethodCall(_))))
                        {
                            if matches!(suffix, Suffix::Call(Call::MethodCall(_)))
                                && strip_trivia(suffix).to_string().contains('\n')
                            {
                                contains_newline = true;
                                break;
                            }
                        }
                    }

                    contains_newline
                };

                outcome
            } else {
                false
            }
        };

        let mut formatted_suffixes = Vec::with_capacity(num_suffixes);
        for suffix in function_call.suffixes() {
            // Calculate the range before formatting, otherwise it will reset to (0,0)
            let range = (
                suffix.start_position().expect("no suffix position").bytes(),
                suffix.end_position().expect("no suffix position").bytes(),
            );

            let indent_level = if should_hang && matches!(suffix, Suffix::Call(Call::MethodCall(_)))
            {
                // Calculate the indentation level for the suffix, before we add the range into the indent ranges
                let indent_level = self.get_range_indent_increase(range).unwrap_or(0);
                self.add_indent_range(range);
                Some(indent_level + 1)
            } else {
                None
            };

            let mut suffix = self.format_suffix(suffix);

            if indent_level.is_some() {
                suffix = suffix.update_leading_trivia(FormatTriviaType::Append(vec![
                    self.create_newline_trivia(),
                    self.create_indent_trivia(indent_level),
                ]));
            }

            formatted_suffixes.push(suffix);
        }

        FunctionCall::new(formatted_prefix).with_suffixes(formatted_suffixes)
    }

    /// Formats a FunctionName node
    pub fn format_function_name<'ast>(
        &mut self,
        function_name: &FunctionName<'ast>,
    ) -> FunctionName<'ast> {
        // TODO: This is based off formatters::format_punctuated - can we merge them into one?
        let mut formatted_names = Punctuated::new();
        for pair in function_name.names().to_owned().into_pairs() {
            // Format Punctuation
            match pair {
                Pair::Punctuated(value, punctuation) => {
                    let formatted_punctuation = crate::fmt_symbol!(self, &punctuation, ".");
                    let formatted_value = self.format_token_reference(&value);
                    formatted_names.push(Pair::new(formatted_value, Some(formatted_punctuation)));
                }
                Pair::End(value) => {
                    let formatted_value = self.format_token_reference(&value);
                    formatted_names.push(Pair::new(formatted_value, None));
                }
            }
        }

        let mut formatted_method: Option<(TokenReference<'ast>, TokenReference<'ast>)> = None;

        if let Some(method_colon) = function_name.method_colon() {
            if let Some(token_reference) = function_name.method_name() {
                formatted_method = Some((
                    crate::fmt_symbol!(self, method_colon, ":"),
                    self.format_token_reference(token_reference),
                ));
            }
        };

        FunctionName::new(formatted_names).with_method(formatted_method)
    }

    /// Formats a FunctionDeclaration node
    pub fn format_function_declaration<'ast>(
        &mut self,
        function_declaration: &FunctionDeclaration<'ast>,
    ) -> FunctionDeclaration<'ast> {
        // Calculate trivia
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(function_declaration.function_token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];

        let function_token =
            crate::fmt_symbol!(self, function_declaration.function_token(), "function ")
                .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
        let formatted_function_name = self.format_function_name(function_declaration.name());
        let formatted_function_body = self.format_function_body(function_declaration.body(), true);

        FunctionDeclaration::new(formatted_function_name)
            .with_function_token(function_token)
            .with_body(formatted_function_body)
    }

    /// Formats a LocalFunction node
    pub fn format_local_function<'ast>(
        &mut self,
        local_function: &LocalFunction<'ast>,
    ) -> LocalFunction<'ast> {
        // Calculate trivia
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(local_function.local_token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];

        let local_token = crate::fmt_symbol!(self, local_function.local_token(), "local ")
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        let function_token = crate::fmt_symbol!(self, local_function.function_token(), "function ");
        let formatted_name = self.format_token_reference(local_function.name());
        let formatted_function_body = self.format_function_body(local_function.body(), true);

        LocalFunction::new(formatted_name)
            .with_local_token(local_token)
            .with_function_token(function_token)
            .with_func_body(formatted_function_body)
    }

    /// Formats a MethodCall node
    pub fn format_method_call<'ast>(&mut self, method_call: &MethodCall<'ast>) -> MethodCall<'ast> {
        let formatted_colon_token = self.format_token_reference(method_call.colon_token());
        let formatted_name = self.format_token_reference(method_call.name());
        let formatted_function_args = self.format_function_args(method_call.args());

        MethodCall::new(formatted_name, formatted_function_args)
            .with_colon_token(formatted_colon_token)
    }

    /// Formats a single Parameter node
    pub fn format_parameter<'ast>(&mut self, parameter: &Parameter<'ast>) -> Parameter<'ast> {
        match parameter {
            Parameter::Ellipse(token) => Parameter::Ellipse(crate::fmt_symbol!(self, token, "...")),
            Parameter::Name(token_reference) => {
                Parameter::Name(self.format_token_reference(token_reference))
            }
            other => panic!("unknown node {:?}", other),
        }
    }

    /// Formats the [`Parameters`] in the provided [`FunctionBody`] onto a single line.
    fn format_singleline_parameters<'ast>(
        &mut self,
        function_body: &FunctionBody<'ast>,
    ) -> Punctuated<'ast, Parameter<'ast>> {
        let mut formatted_parameters = Punctuated::new();

        for pair in function_body.parameters().pairs() {
            let parameter = self.format_parameter(pair.value());
            let punctuation = match pair.punctuation() {
                Some(punctuation) => Some(crate::fmt_symbol!(self, punctuation, ", ")),
                None => None,
            };

            formatted_parameters.push(Pair::new(parameter, punctuation));
        }

        formatted_parameters
    }

    /// Formats the [`Parameters`] in the provided [`FunctionBody`], split across multiple lines.
    fn format_multiline_parameters<'ast>(
        &mut self,
        function_body: &FunctionBody<'ast>,
    ) -> Punctuated<'ast, Parameter<'ast>> {
        let mut formatted_parameters = Punctuated::new();

        for pair in function_body.parameters().pairs() {
            // Calculate indent increase
            let additional_indent_level = self.get_range_indent_increase(match pair.value() {
                Parameter::Name(token) | Parameter::Ellipse(token) => {
                    CodeFormatter::get_token_range(token)
                }
                other => panic!("unknown node {:?}", other),
            });

            let parameter = self.format_parameter(pair.value()).update_leading_trivia(
                FormatTriviaType::Append(vec![
                    self.create_indent_trivia(Some(additional_indent_level.unwrap_or(0) + 1))
                ]),
            );

            let punctuation = match pair.punctuation() {
                Some(punctuation) => Some(
                    crate::fmt_symbol!(self, punctuation, ",").update_trailing_trivia(
                        FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                    ),
                ),
                None => None,
            };

            formatted_parameters.push(Pair::new(parameter, punctuation))
        }

        formatted_parameters
    }
}
