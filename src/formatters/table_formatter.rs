use crate::formatters::{
    get_line_ending_character,
    trivia_formatter::{self, FormatTriviaType},
    CodeFormatter,
};
#[cfg(feature = "luau")]
use full_moon::ast::types::{IndexedTypeInfo, TypeInfo};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Call, Expression, Field, FunctionArgs, Index, Suffix, TableConstructor, Value, Var,
};
use full_moon::tokenizer::{Symbol, Token, TokenReference, TokenType};
use std::borrow::Cow;

// TODO: Can we clean this up? A lot of this code is repeated in trivia_formatter
fn function_args_trailing_trivia<'ast>(function_args: &FunctionArgs<'ast>) -> Vec<Token<'ast>> {
    match function_args {
        FunctionArgs::Parentheses { parentheses, .. } => {
            let (_, end_brace) = parentheses.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
        FunctionArgs::String(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        FunctionArgs::TableConstructor(table_constructor) => {
            let (_, end_brace) = table_constructor.braces().tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
    }
}

fn suffix_trailing_trivia<'ast>(suffix: &Suffix<'ast>) -> Vec<Token<'ast>> {
    match suffix {
        Suffix::Index(index) => match index {
            Index::Brackets { brackets, .. } => {
                let (_, end_brace) = brackets.tokens();
                end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
            }
            Index::Dot { name, .. } => name.trailing_trivia().map(|x| x.to_owned()).collect(),
        },
        Suffix::Call(call) => match call {
            Call::AnonymousCall(function_args) => function_args_trailing_trivia(function_args),
            Call::MethodCall(method_call) => function_args_trailing_trivia(method_call.args()),
        },
    }
}

#[cfg(feature = "luau")]
fn indexed_type_info_trailing_trivia<'ast>(
    indexed_type_info: &IndexedTypeInfo<'ast>,
) -> Vec<Token<'ast>> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        IndexedTypeInfo::Generic { arrows, .. } => {
            let (_, end_brace) = arrows.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
    }
}

#[cfg(feature = "luau")]
fn type_info_trailing_trivia<'ast>(type_info: &TypeInfo<'ast>) -> Vec<Token<'ast>> {
    match type_info {
        TypeInfo::Array { braces, .. } => {
            let (_, end_brace) = braces.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
        TypeInfo::Basic(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        TypeInfo::Callback { return_type, .. } => type_info_trailing_trivia(return_type),
        TypeInfo::Generic { arrows, .. } => {
            let (_, end_brace) = arrows.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }

        TypeInfo::Intersection { right, .. } => type_info_trailing_trivia(right),

        TypeInfo::Module { type_info, .. } => indexed_type_info_trailing_trivia(type_info),

        TypeInfo::Optional { question_mark, .. } => question_mark
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),

        TypeInfo::Table { braces, .. } => {
            let (_, end_brace) = braces.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }

        TypeInfo::Typeof { parentheses, .. } => {
            let (_, end_brace) = parentheses.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }

        TypeInfo::Tuple { parentheses, .. } => {
            let (_, end_brace) = parentheses.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }

        TypeInfo::Union { right, .. } => type_info_trailing_trivia(right),
    }
}

fn get_expression_trailing_trivia<'ast>(expression: Expression<'ast>) -> Vec<Token<'ast>> {
    match expression {
        Expression::Parentheses { contained, .. } => {
            let (_, end_parentheses) = contained.tokens();
            end_parentheses
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect()
        }
        Expression::UnaryOperator { expression, .. } => get_expression_trailing_trivia(*expression),
        Expression::Value {
            value,
            binop,
            #[cfg(feature = "luau")]
            as_assertion,
        } => {
            #[cfg(feature = "luau")]
            if let Some(as_assertion) = as_assertion {
                return type_info_trailing_trivia(as_assertion.cast_to());
            }

            if let Some(binop) = binop {
                get_expression_trailing_trivia(binop.rhs().to_owned())
            } else {
                match *value {
                    Value::Function((_, function_body)) => function_body
                        .end_token()
                        .trailing_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Value::FunctionCall(function_call) => {
                        if let Some(last_suffix) = function_call.iter_suffixes().last() {
                            suffix_trailing_trivia(last_suffix)
                        } else {
                            // TODO: is it possible for this to happen?
                            vec![]
                        }
                    }
                    Value::String(token_reference) => token_reference
                        .trailing_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Value::TableConstructor(table_constructor) => {
                        let (_, end_brace) = table_constructor.braces().tokens();
                        end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
                    }
                    Value::Number(token_reference) => token_reference
                        .trailing_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Value::ParseExpression(expr) => get_expression_trailing_trivia(expr),
                    Value::Symbol(token_reference) => token_reference
                        .trailing_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Value::Var(var) => {
                        match var {
                            Var::Name(token_reference) => token_reference
                                .trailing_trivia()
                                .map(|x| x.to_owned())
                                .collect(),
                            Var::Expression(var_expr) => {
                                if let Some(last_suffix) = var_expr.iter_suffixes().last() {
                                    suffix_trailing_trivia(last_suffix)
                                } else {
                                    // TODO: is it possible for this to happen?
                                    vec![]
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Used to provide information about the table
pub enum TableType {
    /// The table will have multline fields
    MultiLine,
    /// The table will be on a single line
    SingleLine,
    /// The table has no fields
    Empty,
}

impl CodeFormatter {
    pub fn format_field<'ast>(
        &mut self,
        field: &Field<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> (Field<'ast>, Vec<Token<'ast>>) {
        let trailing_trivia;
        let field = match field {
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            } => {
                trailing_trivia = get_expression_trailing_trivia(value.to_owned());
                Field::ExpressionKey {
                    brackets: trivia_formatter::contained_span_add_trivia(
                        self.format_contained_span(brackets.to_owned()),
                        leading_trivia,
                        FormatTriviaType::NoChange,
                    ),
                    key: self.format_expression(key.to_owned()),
                    equal: crate::fmt_symbol!(self, equal.to_owned().into_owned(), " = "),
                    // We will remove all the trivia from this value, and place it after the comma
                    value: trivia_formatter::expression_add_trailing_trivia(
                        self.format_expression(value.to_owned()),
                        FormatTriviaType::Replace(vec![]),
                    ),
                }
            }
            Field::NameKey { key, equal, value } => {
                trailing_trivia = get_expression_trailing_trivia(value.to_owned());
                Field::NameKey {
                    key: Cow::Owned(trivia_formatter::token_reference_add_trivia(
                        self.format_token_reference(key.to_owned()).into_owned(),
                        leading_trivia,
                        FormatTriviaType::NoChange,
                    )),
                    equal: crate::fmt_symbol!(self, equal.to_owned().into_owned(), " = "),
                    value: trivia_formatter::expression_add_trailing_trivia(
                        self.format_expression(value.to_owned()),
                        FormatTriviaType::Replace(vec![]),
                    ),
                }
            }
            Field::NoKey(expression) => {
                trailing_trivia = get_expression_trailing_trivia(expression.to_owned());
                let formatted_expression = self.format_expression(expression.to_owned());
                if let FormatTriviaType::NoChange = leading_trivia {
                    Field::NoKey(formatted_expression)
                } else {
                    Field::NoKey(trivia_formatter::expression_add_trailing_trivia(
                        trivia_formatter::expression_add_leading_trivia(
                            formatted_expression,
                            leading_trivia,
                        ),
                        FormatTriviaType::Replace(vec![]),
                    ))
                }
            }
        };

        (field, trailing_trivia)
    }

    pub fn create_table_braces<'ast>(
        &self,
        start_brace: &TokenReference<'ast>,
        end_brace: &TokenReference<'ast>,
        table_type: TableType,
        additional_indent_level: Option<usize>,
    ) -> ContainedSpan<'ast> {
        match table_type {
            TableType::MultiLine => {
                // Format start and end brace properly with correct trivia
                let end_brace_leading_trivia =
                    vec![self.create_indent_trivia(additional_indent_level)];

                // Add new_line trivia to start_brace
                let start_brace_token = crate::fmt_symbol!(self, start_brace.to_owned(), "{");
                let start_brace_token = trivia_formatter::token_reference_add_trivia(
                    start_brace_token.into_owned(),
                    FormatTriviaType::NoChange,
                    FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                );

                let end_brace_token = TokenReference::new(
                    end_brace_leading_trivia,
                    Token::new(TokenType::Symbol {
                        symbol: Symbol::RightBrace,
                    }),
                    vec![],
                );
                ContainedSpan::new(
                    Cow::Owned(start_brace_token),
                    self.format_symbol(end_brace.to_owned(), end_brace_token),
                )
            }

            TableType::SingleLine => ContainedSpan::new(
                crate::fmt_symbol!(self, start_brace.to_owned(), "{ "),
                crate::fmt_symbol!(self, end_brace.to_owned(), " }"),
            ),

            TableType::Empty => ContainedSpan::new(
                crate::fmt_symbol!(self, start_brace.to_owned(), "{"),
                crate::fmt_symbol!(self, end_brace.to_owned(), "}"),
            ),
        }
    }

    pub fn format_table_constructor<'ast>(
        &mut self,
        table_constructor: TableConstructor<'ast>,
    ) -> TableConstructor<'ast> {
        let mut fields = Punctuated::new();
        let mut current_fields = table_constructor
            .fields()
            .to_owned()
            .into_pairs()
            .peekable();

        let (start_brace, end_brace) = table_constructor.braces().tokens();
        let braces_range = (
            start_brace.end_position().bytes(),
            end_brace.start_position().bytes(),
        );
        let is_multiline = (braces_range.1 - braces_range.0) > 30; // TODO: Properly determine this arbitrary number, and see if other factors should come into play

        let table_type = match current_fields.peek() {
            Some(_) => match is_multiline {
                true => TableType::MultiLine,
                false => TableType::SingleLine,
            },
            None => TableType::Empty,
        };
        let additional_indent_level = self.get_range_indent_increase(braces_range);
        let braces =
            self.create_table_braces(start_brace, end_brace, table_type, additional_indent_level);

        if is_multiline {
            self.add_indent_range(braces_range);
        }

        while let Some(pair) = current_fields.next() {
            let (field, punctuation) = pair.into_tuple();

            let leading_trivia = match is_multiline {
                true => {
                    let range = match field.to_owned() {
                        Field::ExpressionKey { brackets, .. } => {
                            CodeFormatter::get_token_range(brackets.tokens().0.token())
                        }
                        Field::NameKey { key, .. } => CodeFormatter::get_token_range(key.token()),
                        Field::NoKey(expr) => CodeFormatter::get_range_in_expression(&expr),
                    };
                    let additional_indent_level = self.get_range_indent_increase(range);
                    FormatTriviaType::Append(vec![
                        self.create_indent_trivia(additional_indent_level)
                    ])
                }
                false => FormatTriviaType::NoChange,
            };

            let (formatted_field, mut trailing_trivia) = self.format_field(&field, leading_trivia);
            let mut formatted_punctuation = None;

            match is_multiline {
                true => {
                    // Continue adding a comma and a new line for multiline tables
                    let mut symbol = TokenReference::symbol(",").unwrap();
                    if let Some(punctuation) = punctuation {
                        symbol = self
                            .format_symbol(punctuation.into_owned(), symbol)
                            .into_owned();
                    }
                    // Add newline trivia to the end of the symbol, and preserve any comments
                    trailing_trivia.push(self.create_newline_trivia());
                    let symbol = trivia_formatter::token_reference_add_trivia(
                        symbol,
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(trailing_trivia),
                    );
                    formatted_punctuation = Some(Cow::Owned(symbol))
                }
                false => {
                    if current_fields.peek().is_some() {
                        // Have more elements still to go
                        formatted_punctuation = match punctuation {
                            Some(punctuation) => Some(self.format_symbol(
                                punctuation.into_owned(),
                                TokenReference::symbol(", ").unwrap(),
                            )),
                            None => Some(Cow::Owned(TokenReference::symbol(", ").unwrap())),
                        }
                    };
                }
            }

            fields.push(Pair::new(formatted_field, formatted_punctuation))
        }

        TableConstructor::new()
            .with_braces(braces)
            .with_fields(fields)
    }
}
