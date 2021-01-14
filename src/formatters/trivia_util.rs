use crate::formatters::trivia_formatter::{self, FormatTriviaType};
use full_moon::{
    ast::{
        BinOp, Call, Expression, Field, FunctionArgs, Index, Prefix, Suffix, TableConstructor,
        UnOp, Value, Var,
    },
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};
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

fn var_trailing_trivia<'ast>(var: &Var<'ast>) -> Vec<Token<'ast>> {
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
                get_expression_trailing_trivia(binop.rhs())
            } else {
                match &**value {
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
                    Value::ParseExpression(expr) => get_expression_trailing_trivia(&expr),
                    Value::Symbol(token_reference) => token_reference
                        .trailing_trivia()
                        .map(|x| x.to_owned())
                        .collect(),
                    Value::Var(var) => var_trailing_trivia(var),
                }
            }
        }
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
        },
        Expression::Value { value, .. } => match &**value {
            Value::Function((token_ref, _)) => {
                token_ref.leading_trivia().map(|x| x.to_owned()).collect()
            }
            Value::FunctionCall(function_call) => match function_call.prefix() {
                Prefix::Name(token_ref) => {
                    token_ref.leading_trivia().map(|x| x.to_owned()).collect()
                }
                Prefix::Expression(expr) => get_expression_leading_trivia(expr),
            },
            Value::TableConstructor(table) => table
                .braces()
                .tokens()
                .0
                .leading_trivia()
                .map(|x| x.to_owned())
                .collect(),
            Value::Number(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::ParseExpression(expr) => get_expression_leading_trivia(&expr),
            Value::String(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::Symbol(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::Var(var) => match var {
                Var::Name(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
                Var::Expression(var_expr) => match var_expr.prefix() {
                    Prefix::Name(token_ref) => {
                        token_ref.leading_trivia().map(|x| x.to_owned()).collect()
                    }
                    Prefix::Expression(expr) => get_expression_leading_trivia(expr),
                },
            },
        },
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
    }
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
        .map(|x| x.to_owned())
        .collect();

    let new_expression = trivia_formatter::expression_add_trailing_trivia(
        expression.to_owned(),
        FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
    );

    return (new_expression, trailing_comments);
}

pub fn get_var_trailing_comments<'ast>(var: &Var<'ast>) -> (Var<'ast>, Vec<Token<'ast>>) {
    let trailing_comments = var_trailing_trivia(var)
        .iter()
        .filter(|token| {
            token.token_kind() == TokenKind::SingleLineComment
                || token.token_kind() == TokenKind::MultiLineComment
        })
        .map(|x| x.to_owned())
        .collect();

    let new_var = trivia_formatter::var_add_trailing_trivia(
        var.to_owned(),
        FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
    );

    return (new_var, trailing_comments);
}

pub fn get_token_ref_trailing_comments<'ast>(
    token_ref: &Cow<'ast, TokenReference<'ast>>,
) -> (Cow<'ast, TokenReference<'ast>>, Vec<Token<'ast>>) {
    println!("getting trailing comments for {:?}", token_ref);
    let trailing_comments = token_ref
        .trailing_trivia()
        .filter(|token| {
            token.token_kind() == TokenKind::SingleLineComment
                || token.token_kind() == TokenKind::MultiLineComment
        })
        .map(|x| x.to_owned())
        .collect();
    println!("{:?}", trailing_comments);
    let new_token_ref = trivia_formatter::token_reference_add_trivia(
        token_ref.to_owned().into_owned(),
        FormatTriviaType::NoChange,
        FormatTriviaType::Replace(vec![]),
    );

    return (Cow::Owned(new_token_ref), trailing_comments);
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

pub fn token_contains_comments<'ast>(token_ref: &TokenReference<'ast>) -> bool {
    token_trivia_contains_comments(token_ref.leading_trivia())
        || token_trivia_contains_comments(token_ref.trailing_trivia())
}

fn table_constructor_contains_comments<'ast>(table_constructor: &TableConstructor) -> bool {
    let (start, end) = table_constructor.braces().tokens();
    if token_contains_comments(start) || token_contains_comments(end) {
        true
    } else {
        let mut contains_comments = false;

        for field in table_constructor.fields().pairs() {
            contains_comments = match field.value() {
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
            };

            if let Some(punctuation) = field.punctuation() {
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

fn function_args_contains_comments<'ast>(function_args: &FunctionArgs) -> bool {
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
    }
}

fn suffix_contains_comments<'ast>(suffix: &Suffix) -> bool {
    match suffix {
        Suffix::Call(call) => match call {
            Call::AnonymousCall(function_args) => function_args_contains_comments(function_args),
            Call::MethodCall(method_call) => {
                token_contains_comments(method_call.name())
                    || token_contains_comments(method_call.colon_token())
                    || function_args_contains_comments(method_call.args())
            }
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
        },
    }
}

// Check whether any comments are present within an Expression
pub fn expression_contains_comments<'ast>(expression: &Expression<'ast>) -> bool {
    match expression {
        Expression::Parentheses {
            contained,
            expression,
        } => {
            let (start, end) = contained.tokens();
            if token_contains_comments(start) {
                true
            } else if token_contains_comments(end) {
                true
            } else {
                expression_contains_comments(expression)
            }
        }
        Expression::UnaryOperator { unop, expression } => {
            match unop {
                UnOp::Minus(token) | UnOp::Not(token) | UnOp::Hash(token) => {
                    if token_contains_comments(token) {
                        return true;
                    }
                }
            }

            expression_contains_comments(expression)
        }
        Expression::Value { value, binop } => {
            let contains_comments = match &**value {
                Value::Function((token, body)) => {
                    if token_contains_comments(token) {
                        true
                    } else {
                        let (start, end) = body.parameters_parentheses().tokens();
                        if token_contains_comments(start) || token_contains_comments(end) {
                            true
                        } else {
                            // TODO: do we need any more?
                            false
                        }
                    }
                }
                Value::FunctionCall(function_call) => {
                    let contained = match function_call.prefix() {
                        Prefix::Name(token) => token_contains_comments(token),
                        Prefix::Expression(expression) => expression_contains_comments(expression),
                    };

                    if contained {
                        true
                    } else {
                        let mut contained_comments = false;
                        for suffix in function_call.iter_suffixes() {
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
                Value::ParseExpression(expression) => expression_contains_comments(expression),
                Value::String(token) => token_contains_comments(token),
                Value::Symbol(token) => token_contains_comments(token),
                Value::Var(var) => match var {
                    Var::Name(token) => token_contains_comments(token),
                    Var::Expression(var_expr) => {
                        let contained = match var_expr.prefix() {
                            Prefix::Name(token) => token_contains_comments(token),
                            Prefix::Expression(expression) => {
                                expression_contains_comments(expression)
                            }
                        };

                        if contained {
                            true
                        } else {
                            let mut contained_comments = false;
                            for suffix in var_expr.iter_suffixes() {
                                contained_comments = suffix_contains_comments(suffix);
                                if contained_comments {
                                    break;
                                }
                            }
                            contained_comments
                        }
                    }
                },
            };

            if contains_comments {
                true
            } else {
                match binop {
                    Some(binop) => {
                        let contains = match binop.bin_op() {
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
                        };

                        contains || expression_contains_comments(binop.rhs())
                    }
                    None => false,
                }
            }
        }
    }
}

// Checks to see whether an expression contains comments inline inside of it
// This can only happen if the expression is a BinOp
pub fn expression_contains_inline_comments<'ast>(expression: &Expression<'ast>) -> bool {
    match expression {
        Expression::Value { binop, .. } => {
            if binop.is_some() {
                expression_contains_comments(expression)
            } else {
                false
            }
        }
        _ => false,
    }
}
