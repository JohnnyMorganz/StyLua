use crate::{
    formatters::{trivia_util, CodeFormatter},
    IndentType,
};
#[cfg(feature = "luau")]
use full_moon::ast::types::{
    AsAssertion, CompoundAssignment, ExportedTypeDeclaration, IndexedTypeInfo, TypeDeclaration,
    TypeInfo, TypeSpecifier,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    BinOp, BinOpRhs, Call, Do, ElseIf, Expression, Field, FunctionArgs, FunctionBody, FunctionCall,
    FunctionDeclaration, GenericFor, If, Index, LocalFunction, MethodCall, NumericFor, Parameter,
    Prefix, Repeat, Return, Suffix, TableConstructor, UnOp, Value, Var, VarExpression, While,
};
use full_moon::tokenizer::{Token, TokenKind, TokenReference, TokenType};
use std::borrow::Cow;

/// Enum to determine how trivia should be added when using trivia formatter functions
#[derive(Clone, Debug)]
pub enum FormatTriviaType<'ast> {
    /// Trivia will be added to the end of the current trivia
    Append(Vec<Token<'ast>>),
    /// The current trivia will be replaced with the new trivia
    Replace(Vec<Token<'ast>>),
    /// Trivia will not be changed
    NoChange,
}

macro_rules! move_binop_comments {
    ($binop:expr, $trivia:expr, { $($operator:ident,)+ }) => {
        match $binop.bin_op() {
            $(
                BinOp::$operator(token) => {
                    let mut trailing_comments = token
                        .trailing_trivia()
                        .filter(|token| {
                            token.token_kind() == TokenKind::SingleLineComment
                                || token.token_kind()
                                    == TokenKind::MultiLineComment
                        })
                        .map(|x| {
                            // Prepend a single space beforehand
                            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                        })
                        .flatten()
                        .collect();

                    // Move the comments over
                    $trivia.append(&mut trailing_comments);

                    // Recreate BinOp with no comments
                    BinOp::$operator(Cow::Owned(token_reference_add_trivia(
                        token.to_owned().into_owned(),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Replace(vec![Token::new(
                            TokenType::spaces(1),
                        )]),
                    )))
                }
            )+
            _ => panic!("unknown binop found"),
        }
    };
}

fn no_comments<'ast>(token: &TokenReference<'ast>) -> String {
    token.token().to_string()
}

impl CodeFormatter {
    /// Creates indent trivia without including `self.indent_level`.
    /// You should pass the exact amount of indent you require to this function
    fn create_plain_indent_trivia<'ast>(&self, indent_level: usize) -> Token<'ast> {
        match self.config.indent_type {
            IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
            IndentType::Spaces => {
                Token::new(TokenType::spaces(indent_level * self.config.indent_width))
            }
        }
    }

    fn reindent_function_args<'ast>(
        &self,
        function_args: FunctionArgs<'ast>,
        indent_increase: usize,
    ) -> FunctionArgs<'ast> {
        let leading_trivia = vec![self.create_plain_indent_trivia(indent_increase)];
        match function_args {
            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            } => {
                let (start_parens, end_parens) = parentheses.tokens();
                if trivia_util::trivia_contains_newline(start_parens.trailing_trivia()) {
                    // Need to add indent to closing parentheses
                    let parentheses = ContainedSpan::new(
                        Cow::Owned(start_parens.to_owned()),
                        Cow::Owned(token_reference_add_trivia(
                            end_parens.to_owned(),
                            FormatTriviaType::Append(leading_trivia.to_owned()),
                            FormatTriviaType::NoChange,
                        )),
                    );
                    let mut new_arguments = Punctuated::new();

                    for argument in arguments.pairs() {
                        new_arguments.push(Pair::new(
                            expression_add_leading_trivia(
                                argument.value().to_owned(),
                                FormatTriviaType::Append(leading_trivia.to_owned()),
                            ),
                            argument.punctuation().map(|x| Cow::Owned(x.to_owned())),
                        ))
                    }

                    FunctionArgs::Parentheses {
                        parentheses,
                        arguments: new_arguments,
                    }
                } else {
                    FunctionArgs::Parentheses {
                        parentheses,
                        arguments,
                    }
                }
            }
            _ => panic!("got a formatted function args which isnt a parentheses"),
        }
    }

    // Reindents values which are normally expanded (TableConstructor, FunctionArgs, etc.) because the indentation has changed
    fn reindent_expanded_values<'ast>(
        &self,
        value: Value<'ast>,
        indent_increase: usize,
    ) -> Value<'ast> {
        match &value {
            Value::FunctionCall(function_call) => {
                let mut new_suffixes: Vec<Suffix<'ast>> = Vec::new();
                for suffix in function_call.iter_suffixes() {
                    let suffix = suffix.to_owned();
                    new_suffixes.push(match suffix {
                        Suffix::Call(call) => Suffix::Call(match call {
                            Call::AnonymousCall(function_args) => Call::AnonymousCall(
                                self.reindent_function_args(function_args, indent_increase),
                            ),
                            Call::MethodCall(method_call) => {
                                let new_args = self.reindent_function_args(
                                    method_call.args().to_owned(),
                                    indent_increase,
                                );
                                Call::MethodCall(method_call.with_args(new_args))
                            }
                        }),
                        _ => suffix,
                    })
                }

                Value::FunctionCall(function_call.to_owned().with_suffixes(new_suffixes))
            }
            Value::TableConstructor(table_constructor) => {
                let start_brace = table_constructor.braces().tokens().1;
                if trivia_util::trivia_contains_newline(start_brace.trailing_trivia()) {
                    let mut new_fields = Punctuated::new();
                    let leading_trivia = vec![self.create_plain_indent_trivia(indent_increase)];

                    // Is a multiline table
                    for field in table_constructor.fields().pairs() {
                        let new_field;
                        let leading_trivia = leading_trivia.to_owned();
                        match field.value().to_owned() {
                            Field::ExpressionKey {
                                brackets,
                                key,
                                equal,
                                value,
                            } => {
                                new_field = Field::ExpressionKey {
                                    brackets: contained_span_add_trivia(
                                        brackets,
                                        FormatTriviaType::Append(leading_trivia),
                                        FormatTriviaType::NoChange,
                                    ),
                                    key,
                                    equal,
                                    value,
                                }
                            }
                            Field::NameKey { key, equal, value } => {
                                new_field = Field::NameKey {
                                    key: Cow::Owned(token_reference_add_trivia(
                                        key.into_owned().to_owned(),
                                        FormatTriviaType::Append(leading_trivia),
                                        FormatTriviaType::NoChange,
                                    )),
                                    equal,
                                    value,
                                }
                            }
                            Field::NoKey(expression) => {
                                new_field = Field::NoKey(expression_add_leading_trivia(
                                    expression,
                                    FormatTriviaType::Append(leading_trivia),
                                ))
                            }
                        }

                        new_fields.push(Pair::new(
                            new_field,
                            field.punctuation().map(|x| Cow::Owned(x.to_owned())),
                        ))
                    }

                    Value::TableConstructor(table_constructor.to_owned().with_fields(new_fields))
                } else {
                    value
                }
            }

            // No other values are normally expanded, so its safe to ignore them
            _ => value,
        }
    }

    fn expression_split_binop<'ast>(
        &self,
        expression: Expression<'ast>,
        binop_leading_trivia: FormatTriviaType<'ast>,
        indent_increase: usize,
    ) -> Expression<'ast> {
        match expression {
            Expression::Parentheses {
                contained,
                expression,
            } => {
                // Examine the expression itself to see if needs to be split onto multiple lines
                let expression_str = expression.to_string();
                if expression_str.len()
                    + 2 // Account for the two parentheses
                    + (self.indent_level * self.config.indent_width) // Account for the current indent level
                    + (indent_increase * self.config.indent_width) // Account for any further indent increase
                    < 120
                {
                    // The expression inside the parentheses is small, we do not need to break it down further
                    return Expression::Parentheses {
                        contained,
                        expression,
                    };
                }

                // Increase the indent level of the trivia
                let mut current_indent_vec = match &binop_leading_trivia {
                    FormatTriviaType::Append(vec) | FormatTriviaType::Replace(vec) => vec.to_vec(),
                    FormatTriviaType::NoChange => {
                        panic!("we are hanging an expression with no indent trivia")
                    }
                };

                // Modify the parentheses to hang the expression
                let (start_token, end_token) = contained.tokens();

                let contained = ContainedSpan::new(
                    Cow::Owned(token_reference_add_trivia(
                        start_token.to_owned(),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append({
                            // Create a new line at the end of the start token, then indent enough for the first expression
                            let mut new_vec = current_indent_vec.to_vec();
                            new_vec.insert(0, self.create_newline_trivia());
                            new_vec.push(self.create_plain_indent_trivia(1));
                            new_vec
                        }),
                    )),
                    Cow::Owned(token_reference_add_trivia(
                        end_token.to_owned(),
                        FormatTriviaType::Append(current_indent_vec.to_vec()),
                        FormatTriviaType::NoChange,
                    )),
                );

                // Modify the binop leading trivia to increment by one
                current_indent_vec.push(self.create_plain_indent_trivia(1));
                let binop_leading_trivia = match binop_leading_trivia {
                    FormatTriviaType::Append(_) => FormatTriviaType::Append(current_indent_vec),
                    FormatTriviaType::Replace(_) => FormatTriviaType::Replace(current_indent_vec),
                    FormatTriviaType::NoChange => FormatTriviaType::NoChange,
                };

                Expression::Parentheses {
                    contained,
                    expression: Box::new(self.expression_split_binop(
                        *expression,
                        binop_leading_trivia,
                        indent_increase + 1, // Apply indent increase
                    )),
                }
            }
            Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
                unop,
                expression: Box::new(self.expression_split_binop(
                    *expression,
                    binop_leading_trivia,
                    indent_increase,
                )),
            },
            Expression::Value {
                value,
                binop,
                #[cfg(feature = "luau")]
                as_assertion,
            } => {
                // Need to check if there is a binop
                let mut trailing_trivia = trivia_util::get_value_trailing_comments(&value);

                let mut update_value = true;

                let binop = match binop {
                    Some(binop) => {
                        // Don't add the trivia if the binop is binding
                        let binop = match binop.bin_op() {
                            BinOp::GreaterThan(_)
                            | BinOp::GreaterThanEqual(_)
                            | BinOp::LessThan(_)
                            | BinOp::LessThanEqual(_)
                            | BinOp::TildeEqual(_)
                            | BinOp::TwoEqual(_) => {
                                // Remove the new value because we don't want that anymore
                                update_value = false;
                                // Return original binop
                                binop
                            }

                            _ => {
                                // Move any comments after the binop to trailing the value, otherwise we will create issues
                                let new_bin_op = move_binop_comments!(binop, trailing_trivia, {
                                    And,
                                    Caret,
                                    Minus,
                                    Or,
                                    Percent,
                                    Plus,
                                    Slash,
                                    Star,
                                    TwoDots,
                                });

                                let new_binop = binop_rhs_add_trivia(
                                    BinOpRhs::new(new_bin_op, Box::new(binop.rhs().to_owned())),
                                    binop_leading_trivia.to_owned(),
                                    FormatTriviaType::NoChange,
                                );
                                new_binop
                            }
                        };

                        let rhs = Box::new(self.expression_split_binop(
                            binop.rhs().to_owned(),
                            binop_leading_trivia.to_owned(),
                            indent_increase,
                        ));
                        Some(binop.with_rhs(rhs))
                    }
                    None => None,
                };

                trailing_trivia.push(self.create_newline_trivia());
                let new_value = match update_value {
                    true => value_add_trailing_trivia(
                        match *value {
                            // Handle any values which may have expressions inside of them
                            // which we may need to split onto multiple lines
                            Value::ParseExpression(expression) => {
                                Value::ParseExpression(self.expression_split_binop(
                                    expression.to_owned(),
                                    binop_leading_trivia.to_owned(),
                                    indent_increase,
                                ))
                            }
                            _ => *value,
                        },
                        FormatTriviaType::Replace(trailing_trivia),
                    ),
                    false => *value,
                };

                Expression::Value {
                    value: Box::new(self.reindent_expanded_values(new_value, indent_increase)),
                    binop,
                    #[cfg(feature = "luau")]
                    as_assertion,
                }
            }
        }
    }

    // Splits an expression at its binops, pushing each binop part onto a newline
    // Optionally, will also indent any further binops apart from the first one if an indent hang is wanted
    pub fn hang_expression<'ast>(
        &self,
        expression: Expression<'ast>,
        additional_indent_level: Option<usize>,
        hang_level: Option<usize>,
    ) -> Expression<'ast> {
        let additional_indent_level =
            additional_indent_level.unwrap_or(0) + hang_level.unwrap_or(0);
        let hang_level = self.indent_level + additional_indent_level;
        let indent_trivia = self.create_plain_indent_trivia(hang_level);

        self.expression_split_binop(
            expression,
            FormatTriviaType::Replace(vec![indent_trivia]),
            additional_indent_level + 1,
        )
    }

    fn else_if_block_add_trivia<'ast>(
        &self,
        else_if_block: ElseIf<'ast>,
        additional_indent_level: Option<usize>,
    ) -> ElseIf<'ast> {
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        // Need to take into account if we should make the conditions multiple lines
        let first_line_str = no_comments(else_if_block.else_if_token())
            + &else_if_block.condition().to_string()
            + &no_comments(else_if_block.then_token());
        let indent_characters = self.indent_level * self.config.indent_width;
        let require_multiline_condition = (indent_characters + first_line_str.len()) > 120
            || trivia_util::expression_contains_comments(else_if_block.condition());

        let mut else_if_token = token_reference_add_trivia(
            else_if_block.else_if_token().to_owned(),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::NoChange,
        );
        let mut then_token = token_reference_add_trivia(
            else_if_block.then_token().to_owned(),
            FormatTriviaType::NoChange,
            FormatTriviaType::Append(trailing_trivia),
        );

        let condition = match require_multiline_condition {
            true => {
                // Trim the trailing whitespace in if_token, add a new line and indent
                else_if_token = TokenReference::new(
                    else_if_token
                        .leading_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::ElseIf,
                    }),
                    vec![self.create_newline_trivia()],
                );
                // Trim the leading whitespace in then_token
                then_token = TokenReference::new(
                    vec![self.create_indent_trivia(additional_indent_level)],
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::Then,
                    }),
                    then_token.trailing_trivia().map(|x| x.to_owned()).collect(),
                );

                let condition = else_if_block.condition().to_owned();
                expression_add_leading_trivia(
                    self.hang_expression(condition, additional_indent_level, None),
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(Some(additional_indent_level.unwrap_or(0) + 1))
                    ]),
                )
            }
            false => else_if_block.condition().to_owned(),
        };

        else_if_block
            .with_else_if_token(Cow::Owned(else_if_token))
            .with_condition(condition)
            .with_then_token(Cow::Owned(then_token))
    }

    pub fn if_block_add_trivia<'ast>(
        &self,
        if_block: If<'ast>,
        additional_indent_level: Option<usize>,
    ) -> If<'ast> {
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        // Need to take into account if we should make the conditions multiple lines
        let first_line_str = no_comments(if_block.if_token())
            + &if_block.condition().to_string()
            + &no_comments(if_block.then_token());
        let indent_characters = self.indent_level * self.config.indent_width;
        let require_multiline_condition = (indent_characters + first_line_str.len()) > 120
            || trivia_util::expression_contains_comments(if_block.condition());

        let mut if_token = token_reference_add_trivia(
            if_block.if_token().to_owned(),
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::NoChange,
        );

        let mut then_token = token_reference_add_trivia(
            if_block.then_token().to_owned(),
            FormatTriviaType::NoChange,
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );

        let condition = match require_multiline_condition {
            true => {
                // Trim the trailing whitespace in if_token, add a new line and indent
                if_token = TokenReference::new(
                    if_token.leading_trivia().map(|x| x.to_owned()).collect(),
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::If,
                    }),
                    vec![self.create_newline_trivia()],
                );
                // Trim the leading whitespace in then_token
                then_token = TokenReference::new(
                    vec![self.create_indent_trivia(additional_indent_level)],
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::Then,
                    }),
                    then_token.trailing_trivia().map(|x| x.to_owned()).collect(),
                );

                let condition = if_block.condition().to_owned();
                expression_add_leading_trivia(
                    self.hang_expression(condition, additional_indent_level, None),
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(Some(additional_indent_level.unwrap_or(0) + 1))
                    ]),
                )
            }
            false => if_block.condition().to_owned(),
        };

        let end_token = token_reference_add_trivia(
            if_block.end_token().to_owned(),
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );

        let else_if_block = match if_block.else_if() {
            Some(else_if) => Some(
                else_if
                    .iter()
                    .map(|else_if| {
                        self.else_if_block_add_trivia(else_if.to_owned(), additional_indent_level)
                    })
                    .collect(),
            ),
            None => None,
        };

        let else_token = match if_block.else_token() {
            Some(else_token) => Some(Cow::Owned(token_reference_add_trivia(
                else_token.to_owned(),
                FormatTriviaType::Append(leading_trivia.to_owned()),
                FormatTriviaType::Append(trailing_trivia.to_owned()),
            ))),
            None => None,
        };

        if_block
            .with_if_token(Cow::Owned(if_token))
            .with_condition(condition)
            .with_then_token(Cow::Owned(then_token))
            .with_else_if(else_if_block)
            .with_else_token(else_token)
            .with_end_token(Cow::Owned(end_token))
    }

    pub fn repeat_block_add_trivia<'ast>(
        &self,
        repeat_block: Repeat<'ast>,
        additional_indent_level: Option<usize>,
    ) -> Repeat<'ast> {
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        // Need to take into account if we should make the conditions multiple lines
        let last_line_str =
            no_comments(repeat_block.until_token()) + &repeat_block.until().to_string();
        let indent_characters = self.indent_level * self.config.indent_width;
        let require_multiline_condition = (indent_characters + last_line_str.len()) > 120
            || trivia_util::expression_contains_inline_comments(repeat_block.until());

        let repeat_token = token_reference_add_trivia(
            repeat_block.repeat_token().to_owned(),
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );
        let until_token = token_reference_add_trivia(
            repeat_block.until_token().to_owned(),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::NoChange,
        );
        let until_expression = match require_multiline_condition {
            true => self.hang_expression(
                repeat_block.until().to_owned(),
                additional_indent_level,
                None,
            ),
            false => expression_add_trailing_trivia(
                repeat_block.until().to_owned(),
                FormatTriviaType::Append(trailing_trivia),
            ),
        };

        repeat_block
            .with_repeat_token(Cow::Owned(repeat_token))
            .with_until_token(Cow::Owned(until_token))
            .with_until(until_expression)
    }

    pub fn while_block_add_trivia<'ast>(
        &self,
        while_block: While<'ast>,
        additional_indent_level: Option<usize>,
    ) -> While<'ast> {
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        // Need to take into account if we should make the conditions multiple lines
        let first_line_str = no_comments(while_block.while_token())
            + &while_block.condition().to_string()
            + &no_comments(while_block.do_token());
        let indent_characters = self.indent_level * self.config.indent_width;
        let require_multiline_condition = (indent_characters + first_line_str.len()) > 120
            || trivia_util::expression_contains_comments(while_block.condition());

        let mut while_token = token_reference_add_trivia(
            while_block.while_token().to_owned(),
            FormatTriviaType::Append(leading_trivia.to_owned()),
            FormatTriviaType::NoChange,
        );
        let mut do_token = token_reference_add_trivia(
            while_block.do_token().to_owned(),
            FormatTriviaType::NoChange,
            FormatTriviaType::Append(trailing_trivia.to_owned()),
        );

        let condition = match require_multiline_condition {
            true => {
                // Trim the trailing whitespace in if_token, add a new line and indent
                while_token = TokenReference::new(
                    while_token.leading_trivia().map(|x| x.to_owned()).collect(),
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::While,
                    }),
                    vec![self.create_newline_trivia()],
                );
                // Trim the leading whitespace in then_token
                do_token = TokenReference::new(
                    vec![self.create_indent_trivia(additional_indent_level)],
                    Token::new(full_moon::tokenizer::TokenType::Symbol {
                        symbol: full_moon::tokenizer::Symbol::Do,
                    }),
                    do_token.trailing_trivia().map(|x| x.to_owned()).collect(),
                );

                let condition = while_block.condition().to_owned();
                expression_add_leading_trivia(
                    self.hang_expression(condition, additional_indent_level, None),
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(Some(additional_indent_level.unwrap_or(0) + 1))
                    ]),
                )
            }
            false => while_block.condition().to_owned(),
        };

        let end_token = token_reference_add_trivia(
            while_block.end_token().to_owned(),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::Append(trailing_trivia),
        );

        while_block
            .with_while_token(Cow::Owned(while_token))
            .with_condition(condition)
            .with_do_token(Cow::Owned(do_token))
            .with_end_token(Cow::Owned(end_token))
    }

    pub fn return_add_trivia<'ast>(
        &self,
        return_node: Return<'ast>,
        additional_indent_level: Option<usize>,
    ) -> Return<'ast> {
        {
            let mut token = return_node.token().to_owned();
            let mut returns = return_node.returns().to_owned();

            if return_node.returns().is_empty() {
                token = token_reference_add_trivia(
                    token,
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ]),
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                );
            } else {
                token = token_reference_add_trivia(
                    token,
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ]),
                    FormatTriviaType::NoChange,
                );

                // If the expression is too long, we should hang it
                // otherwise, retrieve the last item and add a new line to it
                let first_line_str =
                    no_comments(return_node.token()) + &return_node.returns().to_string();
                let indent_characters = self.indent_level * self.config.indent_width;
                let require_multiline_return = (indent_characters
                    + first_line_str
                        .trim()
                        .lines()
                        .next()
                        .expect("no lines")
                        .len())
                    > 120;

                match require_multiline_return {
                    true => {
                        // Hang each expression
                        let mut new_list = Punctuated::new();
                        for pair in returns.pairs() {
                            let value = self.hang_expression(
                                pair.value().to_owned(),
                                additional_indent_level,
                                None,
                            );
                            new_list.push(Pair::new(
                                value,
                                pair.punctuation().map(|x| Cow::Owned(x.to_owned())),
                            ))
                        }
                        returns = new_list
                    }
                    false => {
                        // Retrieve last item and add new line to it
                        if let Some(last_pair) = returns.pop() {
                            match last_pair {
                                Pair::End(value) => {
                                    let expression = expression_add_trailing_trivia(
                                        value,
                                        FormatTriviaType::Append(
                                            vec![self.create_newline_trivia()],
                                        ),
                                    );
                                    returns.push(Pair::End(expression));
                                }
                                Pair::Punctuated(_, _) => {
                                    panic!("we got a punctuated as the last sequence in expression")
                                }
                            }
                        }
                    }
                }
            }

            return_node
                .with_token(Cow::Owned(token))
                .with_returns(returns)
        }
    }
}

pub fn function_call_add_trivia<'ast>(
    function_call: FunctionCall<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> FunctionCall<'ast> {
    function_call_add_trailing_trivia(
        function_call_add_leading_trivia(function_call, leading_trivia),
        trailing_trivia,
    )
}

pub fn generic_for_add_trivia<'ast>(
    generic_for: GenericFor<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> GenericFor<'ast> {
    let for_token = token_reference_add_trivia(
        generic_for.for_token().to_owned(),
        leading_trivia.to_owned(),
        FormatTriviaType::NoChange,
    );
    let do_token = token_reference_add_trivia(
        generic_for.do_token().to_owned(),
        FormatTriviaType::NoChange,
        trailing_trivia.to_owned(),
    );
    let end_token = token_reference_add_trivia(
        generic_for.end_token().to_owned(),
        leading_trivia,
        trailing_trivia,
    );
    generic_for
        .with_for_token(Cow::Owned(for_token))
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

pub fn function_declaration_add_trivia<'ast>(
    function_declaration: FunctionDeclaration<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> FunctionDeclaration<'ast> {
    let function_token = token_reference_add_trivia(
        function_declaration.function_token().to_owned(),
        leading_trivia.to_owned(),
        FormatTriviaType::NoChange,
    );

    let mut function_body = function_declaration.body().to_owned();

    #[cfg(feature = "luau")]
    {
        let (parameters_parentheses, return_type) = match function_body.return_type() {
            Some(return_type) => (
                function_body.parameters_parentheses().to_owned(),
                Some(type_specifier_add_trailing_trivia(
                    return_type.to_owned(),
                    trailing_trivia.to_owned(),
                )),
            ),
            None => {
                // No return type, so add trivia to the parentheses instead
                let parameters_parentheses = contained_span_add_trivia(
                    function_body.parameters_parentheses().to_owned(),
                    FormatTriviaType::NoChange,
                    trailing_trivia.to_owned(),
                );
                (parameters_parentheses, None)
            }
        };

        function_body = function_body
            .with_parameters_parentheses(parameters_parentheses)
            .with_return_type(return_type);
    }

    #[cfg(not(feature = "luau"))]
    {
        let parameters_parentheses = contained_span_add_trivia(
            function_body.parameters_parentheses().to_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia.to_owned(),
        );
        function_body = function_body.with_parameters_parentheses(parameters_parentheses);
    };

    let end_token = token_reference_add_trivia(
        function_body.end_token().to_owned(),
        leading_trivia,
        trailing_trivia,
    );

    function_declaration
        .with_function_token(Cow::Owned(function_token))
        .with_body(function_body.with_end_token(Cow::Owned(end_token)))
}

pub fn local_function_add_trivia<'ast>(
    local_function: LocalFunction<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> LocalFunction<'ast> {
    let local_token = token_reference_add_trivia(
        local_function.local_token().to_owned(),
        leading_trivia.to_owned(),
        FormatTriviaType::NoChange,
    );

    let mut function_body = local_function.func_body().to_owned();

    #[cfg(feature = "luau")]
    {
        let (parameters_parentheses, return_type) = match function_body.return_type() {
            Some(return_type) => (
                function_body.parameters_parentheses().to_owned(),
                Some(type_specifier_add_trailing_trivia(
                    return_type.to_owned(),
                    trailing_trivia.to_owned(),
                )),
            ),
            None => {
                // No return type, so add trivia to the parentheses instead
                let parameters_parentheses = contained_span_add_trivia(
                    function_body.parameters_parentheses().to_owned(),
                    FormatTriviaType::NoChange,
                    trailing_trivia.to_owned(),
                );
                (parameters_parentheses, None)
            }
        };

        function_body = function_body
            .with_parameters_parentheses(parameters_parentheses)
            .with_return_type(return_type);
    }

    #[cfg(not(feature = "luau"))]
    {
        let parameters_parentheses = contained_span_add_trivia(
            function_body.parameters_parentheses().to_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia.to_owned(),
        );
        function_body = function_body.with_parameters_parentheses(parameters_parentheses);
    };

    let end_token = token_reference_add_trivia(
        function_body.end_token().to_owned(),
        leading_trivia,
        trailing_trivia,
    );

    local_function
        .with_local_token(Cow::Owned(local_token))
        .with_func_body(function_body.with_end_token(Cow::Owned(end_token)))
}

pub fn numeric_for_add_trivia<'ast>(
    numeric_for: NumericFor<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> NumericFor<'ast> {
    // TODO: This is a copy of generic_for, can we reduce this?
    let for_token = token_reference_add_trivia(
        numeric_for.for_token().to_owned(),
        leading_trivia.to_owned(),
        FormatTriviaType::NoChange,
    );
    let do_token = token_reference_add_trivia(
        numeric_for.do_token().to_owned(),
        FormatTriviaType::NoChange,
        trailing_trivia.to_owned(),
    );
    let end_token = token_reference_add_trivia(
        numeric_for.end_token().to_owned(),
        leading_trivia,
        trailing_trivia,
    );
    numeric_for
        .with_for_token(Cow::Owned(for_token))
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

// Remainder of Nodes
macro_rules! binop_leading_trivia {
    ($enum:ident, $value:ident, $leading_trivia:ident, { $($operator:ident,)+ }) => {
        match $value {
            $(
                $enum::$operator(token) => $enum::$operator(Cow::Owned(token_reference_add_trivia(token.into_owned(), $leading_trivia, FormatTriviaType::NoChange))),
            )+
        }
    };
}

pub fn binop_rhs_add_trivia<'ast>(
    binop_rhs: BinOpRhs<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> BinOpRhs<'ast> {
    let binop = if let FormatTriviaType::NoChange = leading_trivia {
        binop_rhs.bin_op().to_owned()
    } else {
        let op = binop_rhs.bin_op().to_owned();
        binop_leading_trivia!(BinOp, op, leading_trivia, {
            And,
            Caret,
            GreaterThan,
            GreaterThanEqual,
            LessThan,
            LessThanEqual,
            Minus,
            Or,
            Percent,
            Plus,
            Slash,
            Star,
            TildeEqual,
            TwoDots,
            TwoEqual,
        })
    };

    let rhs = std::boxed::Box::new(expression_add_trailing_trivia(
        binop_rhs.rhs().to_owned(),
        trailing_trivia,
    ));
    binop_rhs.with_bin_op(binop).with_rhs(rhs)
}

pub fn binop_rhs_add_trailing_trivia<'ast>(
    binop_rhs: BinOpRhs<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> BinOpRhs<'ast> {
    let rhs = std::boxed::Box::new(expression_add_trailing_trivia(
        binop_rhs.rhs().to_owned(),
        trailing_trivia,
    ));
    binop_rhs.with_rhs(rhs)
}

/// Adds trailing trivia at the end of a ContainedSpan node
pub fn contained_span_add_trivia<'ast>(
    contained_span: ContainedSpan<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> ContainedSpan<'ast> {
    let (start_token, end_token) = contained_span.tokens();
    ContainedSpan::new(
        Cow::Owned(token_reference_add_trivia(
            start_token.to_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        )),
        Cow::Owned(token_reference_add_trivia(
            end_token.to_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia,
        )),
    )
}

/// Adds trailing trivia at the end of a Call node
pub fn call_add_trailing_trivia<'ast>(
    call: Call<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Call<'ast> {
    match call {
        Call::AnonymousCall(function_args) => Call::AnonymousCall(
            function_args_add_trailing_trivia(function_args, trailing_trivia),
        ),
        Call::MethodCall(method_call) => Call::MethodCall(method_call_add_trailing_trivia(
            method_call,
            trailing_trivia,
        )),
    }
}

/// Adds leading trivia to the start of an Expression node
pub fn expression_add_leading_trivia<'ast>(
    expression: Expression<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> Expression<'ast> {
    match expression {
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: contained_span_add_trivia(
                contained,
                leading_trivia,
                FormatTriviaType::NoChange,
            ),
            expression,
        },
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop: unop_add_leading_trivia(unop, leading_trivia),
            expression,
        },
        Expression::Value {
            value,
            binop,
            #[cfg(feature = "luau")]
            as_assertion,
        } => Expression::Value {
            value: Box::new(value_add_leading_trivia(*value, leading_trivia)),
            binop,
            #[cfg(feature = "luau")]
            as_assertion,
        },
    }
}

/// Adds traviling trivia at the end of an Expression node
pub fn expression_add_trailing_trivia<'ast>(
    expression: Expression<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Expression<'ast> {
    match expression {
        Expression::Value {
            value,
            binop,
            #[cfg(feature = "luau")]
            as_assertion,
        } => {
            #[cfg(feature = "luau")]
            if let Some(as_assertion) = as_assertion {
                return Expression::Value {
                    value,
                    binop,
                    as_assertion: Some(as_assertion_add_trailing_trivia(
                        as_assertion,
                        trailing_trivia,
                    )),
                };
            }

            if let Some(binop) = binop {
                Expression::Value {
                    value,
                    binop: Some(binop_rhs_add_trailing_trivia(binop, trailing_trivia)),
                    #[cfg(feature = "luau")]
                    as_assertion,
                }
            } else {
                Expression::Value {
                    value: Box::new(value_add_trailing_trivia(*value, trailing_trivia)),
                    binop,
                    #[cfg(feature = "luau")]
                    as_assertion,
                }
            }
        }

        // Add trailing trivia to the end of parentheses
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: contained_span_add_trivia(
                contained,
                FormatTriviaType::NoChange,
                trailing_trivia,
            ),
            expression,
        },

        // Keep recursing down until we find an Expression::Value
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop,
            expression: Box::new(expression_add_trailing_trivia(*expression, trailing_trivia)),
        },
    }
}

/// Adds trailing trivia at the end of a FunctinoArgs node
pub fn function_args_add_trailing_trivia<'ast>(
    function_args: FunctionArgs<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> FunctionArgs<'ast> {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => FunctionArgs::Parentheses {
            parentheses: contained_span_add_trivia(
                parentheses,
                FormatTriviaType::NoChange,
                trailing_trivia,
            ),
            arguments,
        },

        // Add for completeness
        FunctionArgs::String(token_reference) => {
            FunctionArgs::String(Cow::Owned(token_reference_add_trivia(
                token_reference.into_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            )))
        }
        FunctionArgs::TableConstructor(table_constructor) => {
            FunctionArgs::TableConstructor(table_constructor_add_trivia(
                table_constructor,
                FormatTriviaType::NoChange,
                trailing_trivia,
            ))
        }
    }
}

/// Adds trailing trivia at the end of a FunctionBody node
pub fn function_body_add_trailing_trivia<'ast>(
    function_body: FunctionBody<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> FunctionBody<'ast> {
    let function_body_token = function_body.end_token().to_owned();
    function_body.with_end_token(Cow::Owned(token_reference_add_trivia(
        function_body_token,
        FormatTriviaType::NoChange,
        trailing_trivia,
    )))
}

/// Adds leading trivia to the start of a FunctionCall node
pub fn function_call_add_leading_trivia<'ast>(
    function_call: FunctionCall<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> FunctionCall<'ast> {
    let prefix = prefix_add_leading_trivia(function_call.prefix().to_owned(), leading_trivia);
    function_call.with_prefix(prefix)
}

/// Adds trailing trivia at the end of a FunctionCall node
pub fn function_call_add_trailing_trivia<'ast>(
    function_call: FunctionCall<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> FunctionCall<'ast> {
    let mut new_suffixes: Vec<Suffix<'ast>> = function_call
        .iter_suffixes()
        .map(|x| x.to_owned())
        .collect();
    if let Some(last_suffix) = new_suffixes.pop() {
        new_suffixes.push(suffix_add_trailing_trivia(
            last_suffix.to_owned(),
            trailing_trivia,
        ))
    }

    function_call.with_suffixes(new_suffixes)
}

/// Adds trailing trivia at the end of an Index node
pub fn index_add_trailing_trivia<'ast>(
    index: Index<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: contained_span_add_trivia(
                brackets,
                FormatTriviaType::NoChange,
                trailing_trivia,
            ),
            expression,
        },
        Index::Dot { dot, name } => Index::Dot {
            dot,
            name: Cow::Owned(token_reference_add_trivia(
                name.into_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            )),
        },
    }
}

/// Adds trailing trivia at the end of a MethodCall node
pub fn method_call_add_trailing_trivia<'ast>(
    method_call: MethodCall<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> MethodCall<'ast> {
    let method_call_args = method_call.args().to_owned();
    method_call.with_args(function_args_add_trailing_trivia(
        method_call_args,
        trailing_trivia,
    ))
}

/// Adds trivia to a Parameter node
pub fn parameter_add_trivia<'ast>(
    parameter: Parameter<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Parameter<'ast> {
    match parameter {
        Parameter::Ellipse(token) => Parameter::Ellipse(Cow::Owned(token_reference_add_trivia(
            token.into_owned(),
            leading_trivia,
            trailing_trivia,
        ))),
        Parameter::Name(token) => Parameter::Name(Cow::Owned(token_reference_add_trivia(
            token.into_owned(),
            leading_trivia,
            trailing_trivia,
        ))),
    }
}

/// Adds leading trivia to the start of a Prefix node
pub fn prefix_add_leading_trivia<'ast>(
    prefix: Prefix<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> Prefix<'ast> {
    match prefix {
        Prefix::Name(token_reference) => Prefix::Name(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        Prefix::Expression(expression) => {
            Prefix::Expression(expression_add_leading_trivia(expression, leading_trivia))
        }
    }
}

/// Adds trailing trivia at the end of a Suffix node
pub fn suffix_add_trailing_trivia<'ast>(
    suffix: Suffix<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => Suffix::Call(call_add_trailing_trivia(call, trailing_trivia)),
        Suffix::Index(index) => Suffix::Index(index_add_trailing_trivia(index, trailing_trivia)),
    }
}

/// Adds trivia to a TableConstructor node
pub fn table_constructor_add_trivia<'ast>(
    table_constructor: TableConstructor<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> TableConstructor<'ast> {
    let table_constructor_braces = contained_span_add_trivia(
        table_constructor.braces().to_owned(),
        leading_trivia,
        trailing_trivia,
    );
    table_constructor.with_braces(table_constructor_braces)
}

/// Adds trivia to a TokenReferenece
pub fn token_reference_add_trivia<'ast>(
    token_reference: TokenReference<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> TokenReference<'ast> {
    let added_leading_trivia = match leading_trivia {
        FormatTriviaType::Append(trivia) => {
            let mut current: Vec<Token<'ast>> = token_reference
                .leading_trivia()
                .map(|x| x.to_owned())
                .collect();
            current.extend(trivia);
            current
        }
        FormatTriviaType::Replace(trivia) => trivia,
        FormatTriviaType::NoChange => token_reference
            .leading_trivia()
            .map(|x| x.to_owned())
            .collect(),
    };

    let added_trailing_trivia = match trailing_trivia {
        FormatTriviaType::Append(trivia) => {
            let mut current: Vec<Token<'ast>> = token_reference
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect();
            current.extend(trivia);
            current
        }
        FormatTriviaType::Replace(trivia) => trivia,
        FormatTriviaType::NoChange => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
    };

    TokenReference::new(
        added_leading_trivia,
        token_reference.token().to_owned(),
        added_trailing_trivia,
    )
}

/// Adds leading trivia to the start of an UnOp node
pub fn unop_add_leading_trivia<'ast>(
    unop: UnOp<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> UnOp<'ast> {
    match unop {
        UnOp::Hash(token_reference) => UnOp::Hash(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        UnOp::Minus(token_reference) => UnOp::Minus(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        UnOp::Not(token_reference) => UnOp::Not(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
    }
}

pub fn value_add_leading_trivia<'ast>(
    value: Value<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> Value<'ast> {
    match value {
        Value::Function((token, function_body)) => Value::Function((
            Cow::Owned(token_reference_add_trivia(
                token.into_owned(),
                leading_trivia,
                FormatTriviaType::NoChange,
            )),
            function_body,
        )),
        Value::FunctionCall(function_call) => Value::FunctionCall(
            function_call_add_leading_trivia(function_call, leading_trivia),
        ),
        Value::Number(token_reference) => Value::Number(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        Value::ParseExpression(expression) => {
            Value::ParseExpression(expression_add_leading_trivia(expression, leading_trivia))
        }
        Value::String(token_reference) => Value::String(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        Value::Symbol(token_reference) => Value::Symbol(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(table_constructor_add_trivia(
                table_constructor,
                leading_trivia,
                FormatTriviaType::NoChange,
            ))
        }
        Value::Var(var) => Value::Var(var_add_leading_trivia(var, leading_trivia)),
    }
}

/// Adds trailing trivia at the end of a Value node
pub fn value_add_trailing_trivia<'ast>(
    value: Value<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Value<'ast> {
    match value {
        Value::Function((token, function_body)) => Value::Function((
            token,
            function_body_add_trailing_trivia(function_body, trailing_trivia),
        )),
        Value::FunctionCall(function_call) => Value::FunctionCall(
            function_call_add_trailing_trivia(function_call, trailing_trivia),
        ),
        Value::Number(token_reference) => Value::Number(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia,
        ))),
        Value::ParseExpression(expression) => {
            Value::ParseExpression(expression_add_trailing_trivia(expression, trailing_trivia))
        }
        Value::String(token_reference) => Value::String(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia,
        ))),
        Value::Symbol(token_reference) => Value::Symbol(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia,
        ))),
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(table_constructor_add_trivia(
                table_constructor,
                FormatTriviaType::NoChange,
                trailing_trivia,
            ))
        }
        Value::Var(var) => Value::Var(var_add_trailing_trivia(var, trailing_trivia)),
    }
}

/// Adds leading trivia to the start of a Var node
pub fn var_add_leading_trivia<'ast>(
    var: Var<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))),
        Var::Expression(var_expresion) => Var::Expression(var_expression_add_leading_trivia(
            var_expresion,
            leading_trivia,
        )),
    }
}

/// Adds trailing trivia at the end of a Var node
pub fn var_add_trailing_trivia<'ast>(
    var: Var<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            FormatTriviaType::NoChange,
            trailing_trivia,
        ))),
        Var::Expression(var_expression) => Var::Expression(var_expression_add_trailing_trivia(
            var_expression,
            trailing_trivia,
        )),
    }
}

/// Adds leading trivia to the start of a VarExpression node
pub fn var_expression_add_leading_trivia<'ast>(
    var_expresion: VarExpression<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> VarExpression<'ast> {
    let prefix = prefix_add_leading_trivia(var_expresion.prefix().to_owned(), leading_trivia);
    var_expresion.with_prefix(prefix)
}

/// Adds trailing trivia at the end of a VarExpression node
pub fn var_expression_add_trailing_trivia<'ast>(
    var_expression: VarExpression<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> VarExpression<'ast> {
    // TODO: This is copied from FunctionCall, can we combine them?
    let mut new_suffixes: Vec<Suffix<'ast>> = var_expression
        .iter_suffixes()
        .map(|x| x.to_owned())
        .collect();
    if let Some(last_suffix) = new_suffixes.pop() {
        new_suffixes.push(suffix_add_trailing_trivia(
            last_suffix.to_owned(),
            trailing_trivia,
        ))
    }

    var_expression.with_suffixes(new_suffixes)
}

#[cfg(feature = "luau")]
pub fn type_info_add_trailing_trivia<'ast>(
    type_info: TypeInfo<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> TypeInfo<'ast> {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let braces = contained_span_add_trivia(
                braces.to_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            );
            TypeInfo::Array { braces, type_info }
        }
        TypeInfo::Basic(token_reference) => {
            TypeInfo::Basic(Cow::Owned(token_reference_add_trivia(
                token_reference.to_owned().into_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            )))
        }
        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let return_type =
                Box::new(type_info_add_trailing_trivia(*return_type, trailing_trivia));

            TypeInfo::Callback {
                parentheses,
                arguments,
                arrow,
                return_type,
            }
        }
        TypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let arrows =
                contained_span_add_trivia(arrows, FormatTriviaType::NoChange, trailing_trivia);

            TypeInfo::Generic {
                base,
                arrows,
                generics,
            }
        }

        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => {
            let right = Box::new(type_info_add_trailing_trivia(*right, trailing_trivia));
            TypeInfo::Intersection {
                left,
                ampersand,
                right,
            }
        }

        TypeInfo::Module {
            module,
            punctuation,
            type_info,
        } => {
            let type_info = Box::new(indexed_type_info_add_trailing_trivia(
                *type_info,
                trailing_trivia,
            ));
            TypeInfo::Module {
                module,
                punctuation,
                type_info,
            }
        }

        TypeInfo::Optional {
            base,
            question_mark,
        } => {
            let question_mark = Cow::Owned(token_reference_add_trivia(
                question_mark.to_owned().into_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            ));
            TypeInfo::Optional {
                base,
                question_mark,
            }
        }

        TypeInfo::Table { braces, fields } => {
            let braces =
                contained_span_add_trivia(braces, FormatTriviaType::NoChange, trailing_trivia);
            TypeInfo::Table { braces, fields }
        }

        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => {
            let parentheses =
                contained_span_add_trivia(parentheses, FormatTriviaType::NoChange, trailing_trivia);
            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            }
        }

        TypeInfo::Tuple { parentheses, types } => {
            let parentheses =
                contained_span_add_trivia(parentheses, FormatTriviaType::NoChange, trailing_trivia);
            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let right = Box::new(type_info_add_trailing_trivia(*right, trailing_trivia));
            TypeInfo::Union { left, pipe, right }
        }
    }
}

#[cfg(feature = "luau")]
pub fn indexed_type_info_add_trailing_trivia<'ast>(
    indexed_type_info: IndexedTypeInfo<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> IndexedTypeInfo<'ast> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(token_reference) => {
            IndexedTypeInfo::Basic(Cow::Owned(token_reference_add_trivia(
                token_reference.to_owned().into_owned(),
                FormatTriviaType::NoChange,
                trailing_trivia,
            )))
        }
        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let arrows =
                contained_span_add_trivia(arrows, FormatTriviaType::NoChange, trailing_trivia);

            IndexedTypeInfo::Generic {
                base,
                arrows,
                generics,
            }
        }
    }
}

#[cfg(feature = "luau")]
pub fn as_assertion_add_trailing_trivia<'ast>(
    as_assertion: AsAssertion<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> AsAssertion<'ast> {
    let cast_to = type_info_add_trailing_trivia(as_assertion.cast_to().to_owned(), trailing_trivia);
    as_assertion.with_cast_to(cast_to)
}

#[cfg(feature = "luau")]
pub fn type_specifier_add_trailing_trivia<'ast>(
    type_specifier: TypeSpecifier<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> TypeSpecifier<'ast> {
    let type_info =
        type_info_add_trailing_trivia(type_specifier.type_info().to_owned(), trailing_trivia);
    type_specifier.with_type_info(type_info)
}

#[cfg(feature = "luau")]
pub fn type_declaration_add_trivia<'ast>(
    type_declaration: TypeDeclaration<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> TypeDeclaration<'ast> {
    let type_token;
    if let FormatTriviaType::NoChange = leading_trivia {
        // TODO: Cleanup - this is unnecessary
        type_token = Cow::Owned(type_declaration.type_token().to_owned())
    } else {
        type_token = Cow::Owned(token_reference_add_trivia(
            type_declaration.type_token().to_owned(),
            leading_trivia,
            FormatTriviaType::NoChange,
        ))
    };

    let type_definition;
    if let FormatTriviaType::NoChange = trailing_trivia {
        // TODO: Cleanup - this is unnecessary
        type_definition = type_declaration.type_definition().to_owned()
    } else {
        type_definition = type_info_add_trailing_trivia(
            type_declaration.type_definition().to_owned(),
            trailing_trivia,
        )
    }

    type_declaration
        .with_type_token(type_token)
        .with_type_definition(type_definition)
}

#[cfg(feature = "luau")]
pub fn exported_type_declaration_add_trivia<'ast>(
    exported_type_declaration: ExportedTypeDeclaration<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> ExportedTypeDeclaration<'ast> {
    let export_token = Cow::Owned(token_reference_add_trivia(
        exported_type_declaration.export_token().to_owned(),
        leading_trivia,
        FormatTriviaType::NoChange,
    ));
    let type_declaration = type_declaration_add_trivia(
        exported_type_declaration.type_declaration().to_owned(),
        FormatTriviaType::NoChange,
        trailing_trivia,
    );

    exported_type_declaration
        .with_export_token(export_token)
        .with_type_declaration(type_declaration)
}

#[cfg(feature = "luau")]
pub fn compound_assignment_add_trivia<'ast>(
    compound_assignment: CompoundAssignment<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    trailing_trivia: FormatTriviaType<'ast>,
) -> CompoundAssignment<'ast> {
    let lhs = var_add_leading_trivia(compound_assignment.lhs().to_owned(), leading_trivia);
    let rhs = expression_add_trailing_trivia(compound_assignment.rhs().to_owned(), trailing_trivia);

    compound_assignment.with_lhs(lhs).with_rhs(rhs)
}
