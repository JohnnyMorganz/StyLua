use crate::formatters::trivia_formatter::{self, FormatTriviaType};
#[cfg(feature = "luau")]
use full_moon::ast::types::{TypeAssertion, IndexedTypeInfo, TypeField, TypeFieldKey, TypeInfo};
use full_moon::{
    ast::{
        punctuated::Pair, span::ContainedSpan, BinOp, Call, Expression, Field, FunctionArgs, Index,
        Prefix, Stmt, Suffix, TableConstructor, UnOp, Value, Var,
    },
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};
use std::borrow::Cow;

pub fn trivia_is_newline(trivia: &Token) -> bool {
    if let TokenType::Whitespace { characters } = trivia.token_type() {
        if characters.find('\n').is_some() {
            return true;
        }
    }
    false
}

pub fn trivia_contains_newline<'ast>(trivia_vec: impl Iterator<Item = &'ast Token<'ast>>) -> bool {
    for trivia in trivia_vec {
        if trivia_is_newline(trivia) {
            return true;
        }
    }
    false
}

pub fn can_hang_expression(expression: &Expression) -> bool {
    match expression {
        Expression::Parentheses { expression, .. } => can_hang_expression(expression),
        Expression::UnaryOperator { expression, .. } => can_hang_expression(expression),
        Expression::BinaryOperator { .. } => true, // If a binop is present, then we can hang the expression
        Expression::Value { .. } => false,
        other => panic!("unknown node {:?}", other),
    }
}

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
        other => panic!("unknown node {:?}", other),
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
            other => panic!("unknown node {:?}", other),
        },
        Suffix::Call(call) => match call {
            Call::AnonymousCall(function_args) => function_args_trailing_trivia(function_args),
            Call::MethodCall(method_call) => function_args_trailing_trivia(method_call.args()),
            other => panic!("unknown node {:?}", other),
        },
        other => panic!("unknown node {:?}", other),
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
        other => panic!("unknown node {:?}", other),
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

        other => panic!("unknown node {:?}", other),
    }
}

fn var_trailing_trivia<'ast>(var: &Var<'ast>) -> Vec<Token<'ast>> {
    match var {
        Var::Name(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Var::Expression(var_expr) => {
            if let Some(last_suffix) = var_expr.suffixes().last() {
                suffix_trailing_trivia(last_suffix)
            } else {
                // TODO: is it possible for this to happen?
                vec![]
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_value_trailing_trivia<'ast>(value: &Value<'ast>) -> Vec<Token<'ast>> {
    match value {
        Value::Function((_, function_body)) => function_body
            .end_token()
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Value::FunctionCall(function_call) => {
            if let Some(last_suffix) = function_call.suffixes().last() {
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
        Value::ParenthesesExpression(expr) => get_expression_trailing_trivia(&expr),
        Value::Symbol(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Value::Var(var) => var_trailing_trivia(var),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_expression_trailing_trivia<'ast>(expression: &Expression<'ast>) -> Vec<Token<'ast>> {
    match expression {
        Expression::Parentheses { contained, .. } => {
            let (_, end_parentheses) = contained.tokens();
            end_parentheses
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect()
        }
        Expression::UnaryOperator { expression, .. } => get_expression_trailing_trivia(expression),
        Expression::BinaryOperator { rhs, .. } => get_expression_trailing_trivia(rhs),
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => {
            #[cfg(feature = "luau")]
            if let Some(type_assertion) = type_assertion {
                return type_info_trailing_trivia(type_assertion.cast_to());
            }

            get_value_trailing_trivia(value)
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_expression_leading_trivia<'ast>(expression: &Expression<'ast>) -> Vec<Token<'ast>> {
    match expression {
        Expression::Parentheses { contained, .. } => contained
            .tokens()
            .0
            .leading_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Expression::UnaryOperator { unop, .. } => match unop {
            UnOp::Minus(token_ref) | UnOp::Not(token_ref) | UnOp::Hash(token_ref) => {
                token_ref.leading_trivia().map(|x| x.to_owned()).collect()
            }
            other => panic!("unknown node {:?}", other),
        },
        Expression::BinaryOperator { lhs, .. } => get_expression_leading_trivia(lhs),
        Expression::Value { value, .. } => match &**value {
            Value::Function((token_ref, _)) => {
                token_ref.leading_trivia().map(|x| x.to_owned()).collect()
            }
            Value::FunctionCall(function_call) => match function_call.prefix() {
                Prefix::Name(token_ref) => {
                    token_ref.leading_trivia().map(|x| x.to_owned()).collect()
                }
                Prefix::Expression(expr) => get_expression_leading_trivia(expr),
                other => panic!("unknown node {:?}", other),
            },
            Value::TableConstructor(table) => table
                .braces()
                .tokens()
                .0
                .leading_trivia()
                .map(|x| x.to_owned())
                .collect(),
            Value::Number(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::ParenthesesExpression(expr) => get_expression_leading_trivia(&expr),
            Value::String(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::Symbol(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::Var(var) => match var {
                Var::Name(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
                Var::Expression(var_expr) => match var_expr.prefix() {
                    Prefix::Name(token_ref) => {
                        token_ref.leading_trivia().map(|x| x.to_owned()).collect()
                    }
                    Prefix::Expression(expr) => get_expression_leading_trivia(expr),
                    other => panic!("unknown node {:?}", other),
                },
                other => panic!("unknown node {:?}", other),
            },
            other => panic!("unknown node {:?}", other),
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_field_leading_trivia<'ast>(field: &Field<'ast>) -> Vec<Token<'ast>> {
    match field {
        Field::ExpressionKey { brackets, .. } => brackets
            .tokens()
            .0
            .leading_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Field::NameKey { key, .. } => key.leading_trivia().map(|x| x.to_owned()).collect(),
        Field::NoKey(expression) => get_expression_leading_trivia(expression),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_value_trailing_comments<'ast>(value: &Value<'ast>) -> Vec<Token<'ast>> {
    get_value_trailing_trivia(value)
        .iter()
        .filter(|token| {
            token.token_kind() == TokenKind::SingleLineComment
                || token.token_kind() == TokenKind::MultiLineComment
        })
        .map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .flatten()
        .collect()
}

pub fn get_expression_trailing_comments<'ast>(
    expression: &Expression<'ast>,
) -> (Expression<'ast>, Vec<Token<'ast>>) {
    let trailing_comments = get_expression_trailing_trivia(expression)
        .iter()
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

    let new_expression = trivia_formatter::expression_add_trailing_trivia(
        expression.to_owned(),
        FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
    );

    (new_expression, trailing_comments)
}

pub fn get_stmt_trailing_trivia(stmt: Stmt) -> (Stmt, Vec<Token>) {
    let mut trailing_trivia = Vec::new();
    let updated_stmt = match stmt {
        Stmt::Assignment(assignment) => {
            let mut formatted_expression_list = assignment.expressions().to_owned();
            if let Some(last_pair) = formatted_expression_list.pop() {
                match last_pair {
                    Pair::End(value) => {
                        trailing_trivia = get_expression_trailing_trivia(&value);
                        let expression = trivia_formatter::expression_add_trailing_trivia(
                            value,
                            FormatTriviaType::Replace(vec![]),
                        );
                        formatted_expression_list.push(Pair::End(expression));
                    }
                    Pair::Punctuated(_, _) => {
                        panic!("we got a punctuated as the last sequence in expression")
                    }
                }
            }

            Stmt::Assignment(assignment.with_expressions(formatted_expression_list))
        }

        Stmt::LocalAssignment(local_assignment) => {
            let new_assignment = if local_assignment.expressions().is_empty() {
                // Unassigned local variable
                let mut formatted_name_list = local_assignment.names().to_owned();

                // Retrieve last item and take its trailing comments
                if let Some(last_pair) = formatted_name_list.pop() {
                    match last_pair {
                        Pair::End(value) => {
                            trailing_trivia =
                                value.trailing_trivia().map(|x| x.to_owned()).collect();
                            let value = Cow::Owned(trivia_formatter::token_reference_add_trivia(
                                value.into_owned(),
                                FormatTriviaType::NoChange,
                                FormatTriviaType::Replace(vec![]),
                            ));
                            formatted_name_list.push(Pair::End(value));
                        }
                        Pair::Punctuated(_, _) => {
                            panic!("punctuated sequence not ended with a Pair::End")
                        }
                    }
                }
                local_assignment.with_names(formatted_name_list)
            } else {
                // Add newline at the end of LocalAssignment expression list
                // Expression list should already be formatted
                let mut formatted_expression_list = local_assignment.expressions().to_owned();

                // Retrieve last item and remove trailing trivia
                if let Some(last_pair) = formatted_expression_list.pop() {
                    match last_pair {
                        Pair::End(value) => {
                            trailing_trivia = get_expression_trailing_trivia(&value);
                            let expression = trivia_formatter::expression_add_trailing_trivia(
                                value,
                                FormatTriviaType::Replace(vec![]),
                            );
                            formatted_expression_list.push(Pair::End(expression));
                        }
                        Pair::Punctuated(_, _) => {
                            panic!("got a last pair which was punctuated")
                        }
                    }
                }

                local_assignment.with_expressions(formatted_expression_list)
            };

            Stmt::LocalAssignment(new_assignment)
        }

        Stmt::FunctionCall(function_call) => {
            let last_suffix = function_call.suffixes().last();
            trailing_trivia = match last_suffix {
                Some(suffix) => suffix_trailing_trivia(suffix),
                None => Vec::new(),
            };

            Stmt::FunctionCall(trivia_formatter::function_call_add_trailing_trivia(
                function_call,
                FormatTriviaType::Replace(vec![]),
            ))
        }
        Stmt::Repeat(repeat_block) => {
            trailing_trivia = get_expression_trailing_trivia(repeat_block.until());
            let until_expr = trivia_formatter::expression_add_trailing_trivia(
                repeat_block.until().to_owned(),
                FormatTriviaType::Replace(vec![]),
            );

            Stmt::Repeat(repeat_block.with_until(until_expr))
        }
        _ => panic!("stmt trailing comments not implemented"),
    };

    (updated_stmt, trailing_trivia)
}

pub fn token_trivia_contains_comments<'ast>(
    trivia: impl Iterator<Item = &'ast Token<'ast>>,
) -> bool {
    for trivia in trivia {
        if trivia.token_kind() == TokenKind::SingleLineComment
            || trivia.token_kind() == TokenKind::MultiLineComment
        {
            return true;
        }
    }
    false
}

pub fn token_contains_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.leading_trivia())
        || token_trivia_contains_comments(token_ref.trailing_trivia())
}

pub fn table_fields_contains_comments(table_constructor: &TableConstructor) -> bool {
    table_constructor.fields().pairs().any(|field| {
        let mut contains_comments = match field.value() {
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            } => {
                let (start, end) = brackets.tokens();
                token_contains_comments(start)
                    || token_contains_comments(end)
                    || token_contains_comments(equal)
                    || expression_contains_comments(value)
                    || expression_contains_comments(key)
            }
            Field::NameKey { key, equal, value } => {
                token_contains_comments(equal)
                    || token_contains_comments(key)
                    || expression_contains_comments(value)
            }
            Field::NoKey(expression) => expression_contains_comments(expression),
            other => panic!("unknown node {:?}", other),
        };

        if let Some(punctuation) = field.punctuation() {
            if token_contains_comments(punctuation) {
                contains_comments = true;
            }
        }

        contains_comments
    })
}

fn table_constructor_contains_comments(table_constructor: &TableConstructor) -> bool {
    let (start, end) = table_constructor.braces().tokens();
    token_contains_comments(start)
        || token_contains_comments(end)
        || table_fields_contains_comments(table_constructor)
}

fn function_args_contains_comments(function_args: &FunctionArgs) -> bool {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => {
            let (start, end) = parentheses.tokens();
            if token_contains_comments(start) || token_contains_comments(end) {
                true
            } else {
                let mut contains_comments = false;
                for argument in arguments.pairs() {
                    contains_comments = expression_contains_comments(argument.value());
                    if let Some(punctuation) = argument.punctuation() {
                        if token_contains_comments(punctuation) {
                            contains_comments = true;
                        }
                    }
                    if contains_comments {
                        break;
                    }
                }
                contains_comments
            }
        }
        FunctionArgs::String(token) => token_contains_comments(token),
        FunctionArgs::TableConstructor(table_constructor) => {
            table_constructor_contains_comments(table_constructor)
        }
        other => panic!("unknown node {:?}", other),
    }
}

fn suffix_contains_comments(suffix: &Suffix) -> bool {
    match suffix {
        Suffix::Call(call) => match call {
            Call::AnonymousCall(function_args) => function_args_contains_comments(function_args),
            Call::MethodCall(method_call) => {
                token_contains_comments(method_call.name())
                    || token_contains_comments(method_call.colon_token())
                    || function_args_contains_comments(method_call.args())
            }
            other => panic!("unknown node {:?}", other),
        },
        Suffix::Index(index) => match index {
            Index::Brackets {
                brackets,
                expression,
            } => {
                let (start, end) = brackets.tokens();
                token_contains_comments(start)
                    || token_contains_comments(end)
                    || expression_contains_comments(expression)
            }
            Index::Dot { dot, name } => {
                token_contains_comments(dot) || token_contains_comments(name)
            }
            other => panic!("unknown node {:?}", other),
        },
        other => panic!("unknown node {:?}", other),
    }
}

fn contained_span_contains_comments(contained_span: &ContainedSpan) -> bool {
    let (start, end) = contained_span.tokens();
    token_contains_comments(start) || token_contains_comments(end)
}

#[cfg(feature = "luau")]
fn type_info_contains_comments<'ast>(type_info: &TypeInfo<'ast>) -> bool {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            contained_span_contains_comments(braces) || type_info_contains_comments(type_info)
        }
        TypeInfo::Basic(token) => token_contains_comments(token),
        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            contained_span_contains_comments(parentheses)
                || token_contains_comments(arrow)
                || type_info_contains_comments(return_type)
                || arguments.pairs().any(|pair| {
                    type_info_contains_comments(pair.value())
                        || pair
                            .punctuation()
                            .map_or(false, |punc| token_contains_comments(punc))
                })
        }
        TypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            token_contains_comments(base)
                || contained_span_contains_comments(arrows)
                || generics.pairs().any(|pair| {
                    type_info_contains_comments(pair.value())
                        || pair
                            .punctuation()
                            .map_or(false, |punc| token_contains_comments(punc))
                })
        }
        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => {
            type_info_contains_comments(left)
                || token_contains_comments(ampersand)
                || type_info_contains_comments(right)
        }
        TypeInfo::Module {
            module,
            punctuation,
            type_info,
        } => {
            token_contains_comments(module)
                || token_contains_comments(punctuation)
                || indexed_type_info_contains_comments(type_info)
        }
        TypeInfo::Optional {
            base,
            question_mark,
        } => type_info_contains_comments(base) || token_contains_comments(question_mark),
        TypeInfo::Table { braces, fields } => {
            contained_span_contains_comments(braces)
                || fields.pairs().any(|pair| {
                    type_field_contains_comments(pair.value())
                        || pair
                            .punctuation()
                            .map_or(false, |punc| token_contains_comments(punc))
                })
        }
        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => {
            token_contains_comments(typeof_token)
                || contained_span_contains_comments(parentheses)
                || expression_contains_comments(inner)
        }
        TypeInfo::Tuple { parentheses, types } => {
            contained_span_contains_comments(parentheses)
                || types.pairs().any(|pair| {
                    type_info_contains_comments(pair.value())
                        || pair
                            .punctuation()
                            .map_or(false, |punc| token_contains_comments(punc))
                })
        }
        TypeInfo::Union { left, pipe, right } => {
            type_info_contains_comments(left)
                || token_contains_comments(pipe)
                || type_info_contains_comments(right)
        }
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
fn indexed_type_info_contains_comments<'ast>(type_info: &IndexedTypeInfo<'ast>) -> bool {
    match type_info {
        IndexedTypeInfo::Basic(token) => token_contains_comments(token),
        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            token_contains_comments(base)
                || contained_span_contains_comments(arrows)
                || generics.pairs().any(|pair| {
                    type_info_contains_comments(pair.value())
                        || pair
                            .punctuation()
                            .map_or(false, |punc| token_contains_comments(punc))
                })
        }
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
fn type_field_contains_comments<'ast>(type_field: &TypeField<'ast>) -> bool {
    type_field_key_contains_comments(type_field.key())
        || token_contains_comments(type_field.colon_token())
        || type_info_contains_comments(type_field.value())
}

#[cfg(feature = "luau")]
fn type_field_key_contains_comments<'ast>(type_field_key: &TypeFieldKey<'ast>) -> bool {
    match type_field_key {
        TypeFieldKey::Name(token) => token_contains_comments(token),
        TypeFieldKey::IndexSignature { brackets, inner } => {
            contained_span_contains_comments(brackets) || type_info_contains_comments(inner)
        }
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
fn type_assertion_contains_comments<'ast>(type_assertion: &TypeAssertion<'ast>) -> bool {
    token_contains_comments(type_assertion.assertion_op())
        || type_info_contains_comments(type_assertion.cast_to())
}

fn value_contains_comments(value: &Value) -> bool {
    match value {
        Value::Function((token, body)) => {
            if token_contains_comments(token) {
                true
            } else {
                contained_span_contains_comments(body.parameters_parentheses())
                // TODO: Do we need to do any more?
            }
        }
        Value::FunctionCall(function_call) => {
            let contained = match function_call.prefix() {
                Prefix::Name(token) => token_contains_comments(token),
                Prefix::Expression(expression) => expression_contains_comments(expression),
                other => panic!("unknown node {:?}", other),
            };

            if contained {
                true
            } else {
                let mut contained_comments = false;
                for suffix in function_call.suffixes() {
                    contained_comments = suffix_contains_comments(suffix);
                    if contained_comments {
                        break;
                    }
                }
                contained_comments
            }
        }
        Value::TableConstructor(table_constructor) => {
            table_constructor_contains_comments(table_constructor)
        }
        Value::Number(token) => token_contains_comments(token),
        Value::ParenthesesExpression(expression) => expression_contains_comments(expression),
        Value::String(token) => token_contains_comments(token),
        Value::Symbol(token) => token_contains_comments(token),
        Value::Var(var) => match var {
            Var::Name(token) => token_contains_comments(token),
            Var::Expression(var_expr) => {
                let contained = match var_expr.prefix() {
                    Prefix::Name(token) => token_contains_comments(token),
                    Prefix::Expression(expression) => expression_contains_comments(expression),
                    other => panic!("unknown node {:?}", other),
                };

                if contained {
                    true
                } else {
                    let mut contained_comments = false;
                    for suffix in var_expr.suffixes() {
                        contained_comments = suffix_contains_comments(suffix);
                        if contained_comments {
                            break;
                        }
                    }
                    contained_comments
                }
            }
            other => panic!("unknown node {:?}", other),
        },
        other => panic!("unknown node {:?}", other),
    }
}

fn binop_contains_comments(binop: &BinOp) -> bool {
    match binop {
        BinOp::And(t)
        | BinOp::Caret(t)
        | BinOp::GreaterThan(t)
        | BinOp::GreaterThanEqual(t)
        | BinOp::LessThan(t)
        | BinOp::LessThanEqual(t)
        | BinOp::Minus(t)
        | BinOp::Or(t)
        | BinOp::Percent(t)
        | BinOp::Plus(t)
        | BinOp::Slash(t)
        | BinOp::Star(t)
        | BinOp::TildeEqual(t)
        | BinOp::TwoDots(t)
        | BinOp::TwoEqual(t) => token_contains_comments(t),
        other => panic!("unknown node {:?}", other),
    }
}

// Check whether any comments are present within an Expression
pub fn expression_contains_comments(expression: &Expression) -> bool {
    match expression {
        Expression::Parentheses {
            contained,
            expression,
        } => {
            contained_span_contains_comments(contained) || expression_contains_comments(expression)
        }
        Expression::UnaryOperator { unop, expression } => {
            match unop {
                UnOp::Minus(token) | UnOp::Not(token) | UnOp::Hash(token) => {
                    if token_contains_comments(token) {
                        return true;
                    }
                }
                other => panic!("unknown node {:?}", other),
            }

            expression_contains_comments(expression)
        }
        Expression::BinaryOperator { lhs, binop, rhs } =>
            binop_contains_comments(binop)
                || expression_contains_comments(lhs)
                || expression_contains_comments(rhs),
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => {
            #[cfg(feature = "luau")]
            {
                return value_contains_comments(value)
                    || type_assertion
                        .as_ref()
                        .map_or(false, |x| type_assertion_contains_comments(x));
            }

            #[cfg(not(feature = "luau"))]
            value_contains_comments(value)
        }
    }
}

// Checks to see whether an expression contains comments inline inside of it
// This can only happen if the expression is a BinOp
// We should ignore any comments which are trailing for the whole expression, as they are not inline
pub fn expression_contains_inline_comments(expression: &Expression) -> bool {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {            
            binop_contains_comments(binop) || expression_contains_comments(lhs) 
            // Check if the binop chain still continues
            // If so, we should keep checking the expresion
            // Otherwise, stop checking
            || match &**rhs {
                Expression::BinaryOperator { .. } => expression_contains_inline_comments(rhs),
                Expression::UnaryOperator { unop, expression } => {
                    let op_contains_comments = match unop {
                        UnOp::Minus(token) | UnOp::Not(token) | UnOp::Hash(token) => token_contains_comments(token)
                    };
                    op_contains_comments || expression_contains_inline_comments(expression)
                }
                Expression::Value{ .. } => false,
                other => panic!("unknown node {:?}", other),
            }
        }
        _ => false,
    }
}
