use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    formatters::trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
    shape::Shape,
};
#[cfg(feature = "luau")]
use full_moon::ast::span::ContainedSpan;
#[cfg(feature = "luau")]
use full_moon::ast::types::{
    GenericDeclarationParameter, GenericParameterInfo, IndexedTypeInfo, TypeArgument,
    TypeDeclaration, TypeInfo, TypeSpecifier,
};
use full_moon::{
    ast::{
        punctuated::Punctuated, BinOp, Block, Call, Expression, Field, FunctionArgs, Index,
        LastStmt, LocalAssignment, Parameter, Prefix, Stmt, Suffix, TableConstructor, UnOp, Value,
        Var, VarExpression,
    },
    node::Node,
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};

pub fn trivia_is_whitespace(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::Whitespace)
}

pub fn trivia_is_singleline_comment(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::SingleLineComment)
}

pub fn trivia_is_comment(trivia: &Token) -> bool {
    matches!(
        trivia.token_kind(),
        TokenKind::SingleLineComment | TokenKind::MultiLineComment
    )
}

pub fn trivia_is_newline(trivia: &Token) -> bool {
    if let TokenType::Whitespace { characters } = trivia.token_type() {
        if characters.find('\n').is_some() {
            return true;
        }
    }
    false
}

pub fn trivia_contains_newline<'a>(trivia_vec: impl Iterator<Item = &'a Token>) -> bool {
    for trivia in trivia_vec {
        if trivia_is_newline(trivia) {
            return true;
        }
    }
    false
}

/// Determines whether a particular node spans over multiple lines
pub fn spans_multiple_lines<T: std::fmt::Display>(item: &T) -> bool {
    let string = format!("{}", item);
    string.lines().count() > 1
}

pub fn can_hang_expression(expression: &Expression) -> bool {
    match expression {
        Expression::Parentheses { .. } => true, // Can always hang parentheses if necessary
        Expression::UnaryOperator { expression, .. } => can_hang_expression(expression),
        Expression::BinaryOperator { .. } => true, // If a binop is present, then we can hang the expression
        Expression::Value { value, .. } => match &**value {
            Value::ParenthesesExpression(expression) => can_hang_expression(expression),
            Value::FunctionCall(function_call) => match function_call.prefix() {
                Prefix::Expression(expression) => can_hang_expression(expression),
                _ => false,
            },
            Value::Var(Var::Expression(expression)) => match expression.prefix() {
                Prefix::Expression(expression) => can_hang_expression(expression),
                _ => false,
            },
            _ => false,
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn is_block_empty(block: &Block) -> bool {
    block.stmts().next().is_none() && block.last_stmt().is_none()
}

pub fn is_block_simple(block: &Block) -> bool {
    (block.stmts().next().is_none() && block.last_stmt().is_some())
        || (block.stmts().count() == 1
            && block.last_stmt().is_none()
            && match block.stmts().next().unwrap() {
                Stmt::LocalAssignment(assignment)
                    if assignment.names().len() == 1 && assignment.expressions().len() <= 1 =>
                {
                    true
                }
                Stmt::Assignment(assignment)
                    if assignment.variables().len() == 1 && assignment.expressions().len() <= 1 =>
                {
                    true
                }
                Stmt::FunctionCall(_) => true,
                #[cfg(feature = "lua52")]
                Stmt::Goto(_) => true,
                _ => false,
            })
}

// TODO: Can we clean this up? A lot of this code is repeated in trivia_formatter
fn function_args_trailing_trivia(function_args: &FunctionArgs) -> Vec<Token> {
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

pub fn prefix_trailing_trivia(prefix: &Prefix) -> Vec<Token> {
    match prefix {
        Prefix::Name(name) => name.trailing_trivia().cloned().collect(),
        Prefix::Expression(expression) => get_expression_trailing_trivia(expression),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn suffix_leading_trivia(suffix: &Suffix) -> impl Iterator<Item = &Token> {
    match suffix {
        Suffix::Index(index) => match index {
            Index::Brackets { brackets, .. } => brackets.tokens().0.leading_trivia(),
            Index::Dot { dot, .. } => dot.leading_trivia(),
            other => panic!("unknown node {:?}", other),
        },
        Suffix::Call(call) => match call {
            Call::AnonymousCall(function_args) => match function_args {
                FunctionArgs::Parentheses { parentheses, .. } => {
                    parentheses.tokens().0.leading_trivia()
                }
                FunctionArgs::String(string) => string.leading_trivia(),
                FunctionArgs::TableConstructor(table_constructor) => {
                    table_constructor.braces().tokens().0.leading_trivia()
                }
                other => panic!("unknown node {:?}", other),
            },
            Call::MethodCall(method_call) => method_call.colon_token().leading_trivia(),
            other => panic!("unknown node {:?}", other),
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn suffix_trailing_trivia(suffix: &Suffix) -> Vec<Token> {
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
fn indexed_type_info_trailing_trivia(indexed_type_info: &IndexedTypeInfo) -> Vec<Token> {
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
pub fn type_info_trailing_trivia(type_info: &TypeInfo) -> Vec<Token> {
    match type_info {
        TypeInfo::Array { braces, .. } => {
            let (_, end_brace) = braces.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
        TypeInfo::Basic(token_reference)
        | TypeInfo::String(token_reference)
        | TypeInfo::Boolean(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),

        TypeInfo::Callback { return_type, .. } => type_info_trailing_trivia(return_type),
        TypeInfo::Generic { arrows, .. } => {
            let (_, end_brace) = arrows.tokens();
            end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
        }
        TypeInfo::GenericPack { ellipse, .. } => ellipse.trailing_trivia().cloned().collect(),

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
        TypeInfo::Variadic { type_info, .. } => type_info_trailing_trivia(type_info),
        TypeInfo::VariadicPack { name, .. } => name.trailing_trivia().cloned().collect(),
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
fn generic_declaration_parameter_trailing_trivia(
    parameter: &GenericDeclarationParameter,
) -> Vec<Token> {
    if let Some(default_type) = parameter.default_type() {
        type_info_trailing_trivia(default_type)
    } else {
        match parameter.parameter() {
            GenericParameterInfo::Name(token) => token.trailing_trivia().cloned().collect(),
            GenericParameterInfo::Variadic { ellipse, .. } => {
                ellipse.trailing_trivia().cloned().collect()
            }
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[cfg(feature = "luau")]
pub fn take_generic_parameter_trailing_comments(
    parameter: &GenericDeclarationParameter,
) -> (GenericDeclarationParameter, Vec<Token>) {
    let trailing_comments = generic_declaration_parameter_trailing_trivia(parameter)
        .iter()
        .filter(|x| trivia_is_comment(x))
        .flat_map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .collect();
    (
        parameter.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
        trailing_comments,
    )
}

fn var_trailing_trivia(var: &Var) -> Vec<Token> {
    match var {
        Var::Name(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Var::Expression(var_expr) => {
            if let Some(last_suffix) = var_expr.suffixes().last() {
                suffix_trailing_trivia(last_suffix)
            } else {
                unreachable!("got a VarExpression with no suffix");
            }
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_value_trailing_trivia(value: &Value) -> Vec<Token> {
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
                unreachable!("got a FunctionCall with no suffix");
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
        Value::ParenthesesExpression(expr) => get_expression_trailing_trivia(expr),
        Value::Symbol(token_reference) => token_reference
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        Value::Var(var) => var_trailing_trivia(var),
        #[cfg(feature = "luau")]
        Value::IfExpression(if_expression) => {
            get_expression_trailing_trivia(if_expression.else_expression())
        }
        #[cfg(feature = "luau")]
        Value::InterpolatedString(interpolated_string) => interpolated_string
            .last_string()
            .trailing_trivia()
            .map(|x| x.to_owned())
            .collect(),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn get_expression_trailing_trivia(expression: &Expression) -> Vec<Token> {
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

pub fn get_expression_leading_trivia(expression: &Expression) -> Vec<Token> {
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
            #[cfg(feature = "lua53")]
            UnOp::Tilde(token_ref) => token_ref.leading_trivia().cloned().collect(),
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
            #[cfg(feature = "luau")]
            Value::IfExpression(if_expression) => if_expression
                .if_token()
                .leading_trivia()
                .map(|x| x.to_owned())
                .collect(),
            #[cfg(feature = "luau")]
            Value::InterpolatedString(interpolated_string) => {
                interpolated_string.segments().next().map_or_else(
                    || {
                        interpolated_string
                            .last_string()
                            .leading_trivia()
                            .map(|x| x.to_owned())
                            .collect()
                    },
                    |segment| {
                        segment
                            .literal
                            .leading_trivia()
                            .map(|x| x.to_owned())
                            .collect()
                    },
                )
            }
            Value::TableConstructor(table) => table
                .braces()
                .tokens()
                .0
                .leading_trivia()
                .map(|x| x.to_owned())
                .collect(),
            Value::Number(token_ref) => token_ref.leading_trivia().map(|x| x.to_owned()).collect(),
            Value::ParenthesesExpression(expr) => get_expression_leading_trivia(expr),
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

pub fn punctuated_leading_trivia(punctuated: &Punctuated<Expression>) -> Vec<Token> {
    punctuated
        .iter()
        .next()
        .map_or_else(Vec::new, get_expression_leading_trivia)
}

pub fn binop_leading_comments(binop: &BinOp) -> Vec<Token> {
    match binop {
        BinOp::And(token)
        | BinOp::Caret(token)
        | BinOp::GreaterThan(token)
        | BinOp::GreaterThanEqual(token)
        | BinOp::LessThan(token)
        | BinOp::LessThanEqual(token)
        | BinOp::Minus(token)
        | BinOp::Or(token)
        | BinOp::Percent(token)
        | BinOp::Plus(token)
        | BinOp::Slash(token)
        | BinOp::Star(token)
        | BinOp::TildeEqual(token)
        | BinOp::TwoDots(token)
        | BinOp::TwoEqual(token) => token
            .leading_trivia()
            .filter(|token| trivia_is_comment(token))
            .map(|x| x.to_owned())
            .collect(),
        #[cfg(feature = "lua53")]
        BinOp::Ampersand(token)
        | BinOp::DoubleSlash(token)
        | BinOp::DoubleLessThan(token)
        | BinOp::Pipe(token)
        | BinOp::DoubleGreaterThan(token)
        | BinOp::Tilde(token) => token
            .leading_trivia()
            .filter(|token| trivia_is_comment(token))
            .cloned()
            .collect(),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn binop_trailing_comments(binop: &BinOp) -> Vec<Token> {
    match binop {
        BinOp::And(token)
        | BinOp::Caret(token)
        | BinOp::GreaterThan(token)
        | BinOp::GreaterThanEqual(token)
        | BinOp::LessThan(token)
        | BinOp::LessThanEqual(token)
        | BinOp::Minus(token)
        | BinOp::Or(token)
        | BinOp::Percent(token)
        | BinOp::Plus(token)
        | BinOp::Slash(token)
        | BinOp::Star(token)
        | BinOp::TildeEqual(token)
        | BinOp::TwoDots(token)
        | BinOp::TwoEqual(token) => {
            token
                .trailing_trivia()
                .filter(|token| trivia_is_comment(token))
                .flat_map(|x| {
                    // Prepend a single space beforehand
                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                })
                .collect()
        }
        #[cfg(feature = "lua53")]
        BinOp::Ampersand(token)
        | BinOp::DoubleSlash(token)
        | BinOp::DoubleLessThan(token)
        | BinOp::Pipe(token)
        | BinOp::DoubleGreaterThan(token)
        | BinOp::Tilde(token) => token
            .trailing_trivia()
            .filter(|token| trivia_is_comment(token))
            .flat_map(|x| {
                // Prepend a single space beforehand
                vec![Token::new(TokenType::spaces(1)), x.to_owned()]
            })
            .collect(),
        other => panic!("unknown node {:?}", other),
    }
}

pub fn expression_leading_comments(expression: &Expression) -> Vec<Token> {
    get_expression_leading_trivia(expression)
        .iter()
        .filter(|token| trivia_is_comment(token))
        .map(|x| x.to_owned())
        .collect()
}

pub fn take_expression_leading_comments(expression: &Expression) -> (Expression, Vec<Token>) {
    let trailing_comments = get_expression_leading_trivia(expression)
        .iter()
        .filter(|token| trivia_is_comment(token))
        .map(|x| x.to_owned())
        .collect();

    (
        expression.update_leading_trivia(
            FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
        ),
        trailing_comments,
    )
}

pub fn take_expression_trailing_comments(expression: &Expression) -> (Expression, Vec<Token>) {
    let trailing_comments = get_expression_trailing_trivia(expression)
        .iter()
        .filter(|token| trivia_is_comment(token))
        .flat_map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .collect();

    (
        expression.update_trailing_trivia(
            FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
        ),
        trailing_comments,
    )
}

pub fn take_parameter_trailing_comments(parameter: &Parameter) -> (Parameter, Vec<Token>) {
    let trailing_trivia = match parameter {
        Parameter::Name(token) | Parameter::Ellipse(token) => token.trailing_trivia(),
        other => panic!("unknown node {:?}", other),
    };

    // Remove any trailing comments from the parameter if present
    let trailing_comments: Vec<Token> = trailing_trivia
        .filter(|token| trivia_is_comment(token))
        .flat_map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .collect();

    (
        parameter.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
        trailing_comments,
    )
}

/// Macro for retrieving trailing trivia out of a stmt which ends with an `end` token
macro_rules! end_stmt_trailing_trivia {
    ($enum:ident, $value:ident) => {{
        let end_token = $value.end_token();
        let trailing_trivia = end_token.trailing_trivia().map(|x| x.to_owned()).collect();
        let new_end_token = end_token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

        (
            Stmt::$enum($value.with_end_token(new_end_token)),
            trailing_trivia,
        )
    }};
}

#[cfg(feature = "luau")]
fn get_indexed_type_info_trailing_trivia(
    type_info: IndexedTypeInfo,
) -> (IndexedTypeInfo, Vec<Token>) {
    match type_info {
        IndexedTypeInfo::Basic(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (IndexedTypeInfo::Basic(token), trailing_trivia)
        }
        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let (start_brace, end_brace) = arrows.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (
                IndexedTypeInfo::Generic {
                    base,
                    arrows: braces,
                    generics,
                },
                trailing_trivia,
            )
        }
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
fn get_type_info_trailing_trivia(type_info: TypeInfo) -> (TypeInfo, Vec<Token>) {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let (start_brace, end_brace) = braces.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (TypeInfo::Array { braces, type_info }, trailing_trivia)
        }
        TypeInfo::Basic(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (TypeInfo::Basic(token), trailing_trivia)
        }
        TypeInfo::String(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (TypeInfo::String(token), trailing_trivia)
        }
        TypeInfo::Boolean(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (TypeInfo::Boolean(token), trailing_trivia)
        }
        TypeInfo::Callback {
            generics,
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let (return_type, trailing_trivia) = get_type_info_trailing_trivia(*return_type);
            (
                TypeInfo::Callback {
                    generics,
                    parentheses,
                    arguments,
                    arrow,
                    return_type: Box::new(return_type),
                },
                trailing_trivia,
            )
        }
        TypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let (start_brace, end_brace) = arrows.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (
                TypeInfo::Generic {
                    base,
                    arrows: braces,
                    generics,
                },
                trailing_trivia,
            )
        }
        TypeInfo::GenericPack { name, ellipse } => {
            let trailing_trivia = ellipse.trailing_trivia().map(|x| x.to_owned()).collect();
            let ellipse = ellipse.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (TypeInfo::GenericPack { name, ellipse }, trailing_trivia)
        }
        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => {
            let (right, trailing_trivia) = get_type_info_trailing_trivia(*right);
            (
                TypeInfo::Intersection {
                    left,
                    ampersand,
                    right: Box::new(right),
                },
                trailing_trivia,
            )
        }
        TypeInfo::Module {
            module,
            punctuation,
            type_info,
        } => {
            let (type_info, trailing_trivia) = get_indexed_type_info_trailing_trivia(*type_info);
            (
                TypeInfo::Module {
                    module,
                    punctuation,
                    type_info: Box::new(type_info),
                },
                trailing_trivia,
            )
        }
        TypeInfo::Optional {
            base,
            question_mark,
        } => {
            let trailing_trivia = question_mark
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect();
            let question_mark =
                question_mark.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (
                TypeInfo::Optional {
                    base,
                    question_mark,
                },
                trailing_trivia,
            )
        }
        TypeInfo::Table { braces, fields } => {
            let (start_brace, end_brace) = braces.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (TypeInfo::Table { braces, fields }, trailing_trivia)
        }
        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => {
            let (start_brace, end_brace) = parentheses.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (
                TypeInfo::Typeof {
                    typeof_token,
                    parentheses: braces,
                    inner,
                },
                trailing_trivia,
            )
        }
        TypeInfo::Tuple { parentheses, types } => {
            let (start_brace, end_brace) = parentheses.tokens();
            let trailing_trivia = end_brace.trailing_trivia().map(|x| x.to_owned()).collect();
            let braces = ContainedSpan::new(
                start_brace.to_owned(),
                end_brace.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            );

            (
                TypeInfo::Tuple {
                    types,
                    parentheses: braces,
                },
                trailing_trivia,
            )
        }
        TypeInfo::Union { left, pipe, right } => {
            let (right, trailing_trivia) = get_type_info_trailing_trivia(*right);
            (
                TypeInfo::Union {
                    left,
                    pipe,
                    right: Box::new(right),
                },
                trailing_trivia,
            )
        }
        TypeInfo::Variadic { ellipse, type_info } => {
            let (type_info, trailing_trivia) = get_type_info_trailing_trivia(*type_info);
            (
                TypeInfo::Variadic {
                    ellipse,
                    type_info: Box::new(type_info),
                },
                trailing_trivia,
            )
        }
        TypeInfo::VariadicPack { ellipse, name } => {
            let trailing_trivia = name.trailing_trivia().map(|x| x.to_owned()).collect();
            let name = name.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (TypeInfo::VariadicPack { ellipse, name }, trailing_trivia)
        }
        other => panic!("unknown node {:?}", other),
    }
}

#[cfg(feature = "luau")]
// TODO: I tried to make this return `impl <Iterator = &Token>` but it didn't work because of the 3 recursive calls. Need to look into this to prevent alloc
pub fn type_info_leading_trivia(type_info: &TypeInfo) -> Vec<&Token> {
    match type_info {
        TypeInfo::Array { braces, .. } => braces.tokens().0.leading_trivia(),
        TypeInfo::Basic(token) | TypeInfo::String(token) | TypeInfo::Boolean(token) => {
            token.leading_trivia()
        }
        TypeInfo::Callback {
            generics,
            parentheses,
            ..
        } => match generics {
            Some(generics) => generics.arrows().tokens().0.leading_trivia(),
            None => parentheses.tokens().0.leading_trivia(),
        },
        TypeInfo::Generic { base, .. } => base.leading_trivia(),
        TypeInfo::GenericPack { name, .. } => name.leading_trivia(),
        TypeInfo::Intersection { left, .. } => return type_info_leading_trivia(left),
        TypeInfo::Module { module, .. } => module.leading_trivia(),
        TypeInfo::Optional { base, .. } => return type_info_leading_trivia(base),
        TypeInfo::Table { braces, .. } => braces.tokens().0.leading_trivia(),
        TypeInfo::Typeof { typeof_token, .. } => typeof_token.leading_trivia(),
        TypeInfo::Tuple { parentheses, .. } => parentheses.tokens().0.leading_trivia(),
        TypeInfo::Union { left, .. } => return type_info_leading_trivia(left),
        TypeInfo::Variadic { ellipse, .. } => ellipse.leading_trivia(),
        TypeInfo::VariadicPack { ellipse, .. } => ellipse.leading_trivia(),
        other => panic!("unknown node {:?}", other),
    }
    .collect()
}

#[cfg(feature = "luau")]
pub fn take_type_info_trailing_comments(type_info: &TypeInfo) -> (TypeInfo, Vec<Token>) {
    let (type_info, trailing_trivia) = get_type_info_trailing_trivia(type_info.to_owned());

    let trailing_comments = trailing_trivia
        .iter()
        .filter(|token| trivia_is_comment(token))
        .flat_map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .collect();

    (type_info, trailing_comments)
}

#[cfg(feature = "luau")]
pub fn take_type_argument_trailing_comments(
    type_argument: &TypeArgument,
) -> (TypeArgument, Vec<Token>) {
    let (type_info, trailing_comments) =
        take_type_info_trailing_comments(type_argument.type_info());

    (
        type_argument.to_owned().with_type_info(type_info),
        trailing_comments,
    )
}

#[cfg(feature = "luau")]
fn get_type_declaration_trailing_trivia(
    type_declaration: TypeDeclaration,
) -> (TypeDeclaration, Vec<Token>) {
    let (type_definition, trailing_trivia) =
        get_type_info_trailing_trivia(type_declaration.type_definition().to_owned());
    (
        type_declaration.with_type_definition(type_definition),
        trailing_trivia,
    )
}

#[cfg(feature = "luau")]
fn type_specifier_trailing_trivia(type_specifier: &TypeSpecifier) -> Vec<Token> {
    get_type_info_trailing_trivia(type_specifier.type_info().clone()).1
}

fn get_empty_local_assignment_trailing_trivia(
    local_assignment: LocalAssignment,
) -> (LocalAssignment, Vec<Token>) {
    let mut trailing_trivia = Vec::new();

    #[cfg(feature = "luau")]
    {
        let mut type_specifiers = local_assignment
            .type_specifiers()
            .map(|x| x.cloned())
            .collect::<Vec<_>>();

        if let Some(Some(type_specifier)) = type_specifiers.pop() {
            trailing_trivia = type_specifier_trailing_trivia(&type_specifier);

            type_specifiers.push(Some(
                type_specifier.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            ));

            return (
                local_assignment.with_type_specifiers(type_specifiers),
                trailing_trivia,
            );
        }
    }

    #[cfg(feature = "lua54")]
    {
        let mut attributes = local_assignment
            .attributes()
            .map(|x| x.cloned())
            .collect::<Vec<_>>();

        if let Some(Some(attribute)) = attributes.pop() {
            trailing_trivia = attribute
                .brackets()
                .tokens()
                .1
                .trailing_trivia()
                .cloned()
                .collect();

            attributes.push(Some(
                attribute.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
            ));

            return (
                local_assignment.with_attributes(attributes),
                trailing_trivia,
            );
        }
    }

    // Unassigned local variable
    let mut formatted_name_list = local_assignment.names().to_owned();
    // Retrieve last item and take its trailing comments
    if let Some(last_pair) = formatted_name_list.pop() {
        let pair = last_pair.map(|value| {
            trailing_trivia = value.trailing_trivia().map(|x| x.to_owned()).collect();
            value.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
        });
        formatted_name_list.push(pair);
    }

    (
        local_assignment.with_names(formatted_name_list),
        trailing_trivia,
    )
}

pub fn get_stmt_trailing_trivia(stmt: Stmt) -> (Stmt, Vec<Token>) {
    let (updated_stmt, trailing_trivia) = match stmt {
        Stmt::Assignment(assignment) => {
            let mut formatted_expression_list = assignment.expressions().to_owned();
            let mut trailing_trivia = Vec::new();
            if let Some(last_pair) = formatted_expression_list.pop() {
                let pair = last_pair.map(|value| {
                    trailing_trivia = get_expression_trailing_trivia(&value);
                    value.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
                });
                formatted_expression_list.push(pair);
            }

            (
                Stmt::Assignment(assignment.with_expressions(formatted_expression_list)),
                trailing_trivia,
            )
        }

        Stmt::LocalAssignment(local_assignment) => {
            let mut trailing_trivia = Vec::new();
            let new_assignment = if local_assignment.expressions().is_empty() {
                let (assignment, trivia) =
                    get_empty_local_assignment_trailing_trivia(local_assignment);
                trailing_trivia = trivia;
                assignment
            } else {
                // Add newline at the end of LocalAssignment expression list
                // Expression list should already be formatted
                let mut formatted_expression_list = local_assignment.expressions().to_owned();

                // Retrieve last item and remove trailing trivia
                if let Some(last_pair) = formatted_expression_list.pop() {
                    let pair = last_pair.map(|value| {
                        trailing_trivia = get_expression_trailing_trivia(&value);
                        value.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
                    });
                    formatted_expression_list.push(pair);
                }

                local_assignment.with_expressions(formatted_expression_list)
            };

            (Stmt::LocalAssignment(new_assignment), trailing_trivia)
        }

        Stmt::FunctionCall(function_call) => {
            let last_suffix = function_call.suffixes().last();
            let trailing_trivia = match last_suffix {
                Some(suffix) => suffix_trailing_trivia(suffix),
                None => unreachable!("got a FunctionCall with no suffix"),
            };

            (
                Stmt::FunctionCall(
                    function_call.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
                ),
                trailing_trivia,
            )
        }
        Stmt::Repeat(repeat_block) => {
            let trailing_trivia = get_expression_trailing_trivia(repeat_block.until());
            let until_expr = repeat_block
                .until()
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

            (
                Stmt::Repeat(repeat_block.with_until(until_expr)),
                trailing_trivia,
            )
        }

        Stmt::Do(stmt) => {
            end_stmt_trailing_trivia!(Do, stmt)
        }
        Stmt::GenericFor(stmt) => {
            end_stmt_trailing_trivia!(GenericFor, stmt)
        }
        Stmt::If(stmt) => {
            end_stmt_trailing_trivia!(If, stmt)
        }
        Stmt::FunctionDeclaration(stmt) => {
            let end_token = stmt.body().end_token();
            let trailing_trivia = end_token.trailing_trivia().map(|x| x.to_owned()).collect();
            let new_end_token = end_token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

            let body = stmt.body().to_owned().with_end_token(new_end_token);
            (
                Stmt::FunctionDeclaration(stmt.with_body(body)),
                trailing_trivia,
            )
        }
        Stmt::LocalFunction(stmt) => {
            let end_token = stmt.body().end_token();
            let trailing_trivia = end_token.trailing_trivia().map(|x| x.to_owned()).collect();
            let new_end_token = end_token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

            let body = stmt.body().to_owned().with_end_token(new_end_token);
            (Stmt::LocalFunction(stmt.with_body(body)), trailing_trivia)
        }
        Stmt::NumericFor(stmt) => {
            end_stmt_trailing_trivia!(NumericFor, stmt)
        }
        Stmt::While(stmt) => {
            end_stmt_trailing_trivia!(While, stmt)
        }

        #[cfg(feature = "luau")]
        Stmt::CompoundAssignment(stmt) => {
            let trailing_trivia = get_expression_trailing_trivia(stmt.rhs());
            let expr = stmt
                .rhs()
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (
                Stmt::CompoundAssignment(stmt.with_rhs(expr)),
                trailing_trivia,
            )
        }
        #[cfg(feature = "luau")]
        Stmt::ExportedTypeDeclaration(stmt) => {
            let (type_declaration, trailing_trivia) =
                get_type_declaration_trailing_trivia(stmt.type_declaration().to_owned());
            (
                Stmt::ExportedTypeDeclaration(stmt.with_type_declaration(type_declaration)),
                trailing_trivia,
            )
        }
        #[cfg(feature = "luau")]
        Stmt::TypeDeclaration(stmt) => {
            let (type_declaration, trailing_trivia) = get_type_declaration_trailing_trivia(stmt);
            (Stmt::TypeDeclaration(type_declaration), trailing_trivia)
        }
        #[cfg(feature = "lua52")]
        Stmt::Goto(stmt) => {
            let trailing_trivia = stmt
                .label_name()
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect();
            let label_name = stmt
                .label_name()
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (
                Stmt::Goto(stmt.with_label_name(label_name)),
                trailing_trivia,
            )
        }
        #[cfg(feature = "lua52")]
        Stmt::Label(stmt) => {
            let trailing_trivia = stmt
                .right_colons()
                .trailing_trivia()
                .map(|x| x.to_owned())
                .collect();
            let right_colons = stmt
                .right_colons()
                .update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            (
                Stmt::Label(stmt.with_right_colons(right_colons)),
                trailing_trivia,
            )
        }

        other => panic!("unknown node {:?}", other),
    };

    (updated_stmt, trailing_trivia)
}

pub fn last_stmt_trailing_trivia(last_stmt: &LastStmt) -> Vec<Token> {
    match last_stmt {
        LastStmt::Return(ret) => {
            if ret.returns().is_empty() {
                ret.token().trailing_trivia().cloned().collect()
            } else {
                let last_expression = ret.returns().iter().last().unwrap();
                get_expression_trailing_trivia(last_expression)
            }
        }
        LastStmt::Break(token) => token.trailing_trivia().cloned().collect(),
        #[cfg(feature = "luau")]
        LastStmt::Continue(token) => token.trailing_trivia().cloned().collect(),
        other => panic!("unknown node {:?}", other),
    }
}

#[derive(Clone, Copy)]
pub enum CommentSearch {
    // Only care about singleline comments
    #[allow(dead_code)]
    Single,
    // Looking for all comments
    All,
}

pub fn trivia_contains_comments<'a>(
    mut trivia: impl Iterator<Item = &'a Token>,
    search: CommentSearch,
) -> bool {
    let tester = match search {
        CommentSearch::Single => trivia_is_singleline_comment,
        CommentSearch::All => trivia_is_comment,
    };

    trivia.any(tester)
}

pub fn token_trivia_contains_comments<'a>(trivia: impl Iterator<Item = &'a Token>) -> bool {
    trivia_contains_comments(trivia, CommentSearch::All)
}

pub fn token_contains_leading_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.leading_trivia())
}

pub fn token_contains_trailing_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.trailing_trivia())
}

pub fn token_contains_comments_search(token: &TokenReference, search: CommentSearch) -> bool {
    trivia_contains_comments(token.leading_trivia(), search)
        || trivia_contains_comments(token.trailing_trivia(), search)
}

pub fn token_contains_comments(token: &TokenReference) -> bool {
    token_contains_comments_search(token, CommentSearch::All)
}

/// CAUTION: VERY EXPENSIVE FUNCTION FOR LARGE NODES
pub fn contains_comments(node: impl Node) -> bool {
    node.tokens().any(token_contains_comments)
}

#[allow(dead_code)]
pub fn contains_singleline_comments(node: impl Node) -> bool {
    node.tokens()
        .into_iter()
        .any(|token| token_contains_comments_search(token, CommentSearch::Single))
}

/// Checks whether any [`Field`] within a [`TableConstructor`] contains comments, without checking the braces
pub fn table_fields_contains_comments(table_constructor: &TableConstructor) -> bool {
    table_constructor.fields().pairs().any(|field| {
        let comments = match field.value() {
            Field::ExpressionKey {
                brackets,
                key,
                equal,
                value,
            } => {
                contains_comments(brackets)
                    || contains_comments(key)
                    || contains_comments(equal)
                    || contains_comments(value)
            }
            Field::NameKey { key, equal, value } => {
                contains_comments(key) || contains_comments(equal) || contains_comments(value)
            }
            Field::NoKey(expression) => contains_comments(expression),
            other => panic!("unknown node {:?}", other),
        };

        comments || field.punctuation().map_or(false, contains_comments)
    })
}

pub fn table_field_trailing_trivia(field: &Field) -> Vec<Token> {
    match field {
        Field::ExpressionKey { value, .. } => get_expression_trailing_trivia(value),
        Field::NameKey { value, .. } => get_expression_trailing_trivia(value),
        Field::NoKey(expression) => get_expression_trailing_trivia(expression),
        other => panic!("unknown node {:?}", other),
    }
}

// Checks to see whether an expression contains comments inline inside of it
// This can only happen if the expression is a BinOp
// We should ignore any comments which are trailing for the whole expression, as they are not inline
pub fn expression_contains_inline_comments(expression: &Expression) -> bool {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            contains_comments(binop)
                || contains_comments(lhs)
                || expression_contains_inline_comments(rhs)
        }
        Expression::UnaryOperator { unop, expression } => {
            let op_contains_comments = match unop {
                UnOp::Minus(token) | UnOp::Not(token) | UnOp::Hash(token) => {
                    contains_comments(token)
                }
                #[cfg(feature = "lua53")]
                UnOp::Tilde(token) => contains_comments(token),
                other => panic!("unknown node {:?}", other),
            };
            op_contains_comments || expression_contains_inline_comments(expression)
        }
        Expression::Parentheses {
            contained,
            expression,
        } => {
            token_contains_trailing_comments(contained.tokens().0)
                || token_contains_leading_comments(contained.tokens().1)
                || contains_comments(expression)
        }
        Expression::Value { value, .. } => match &**value {
            Value::ParenthesesExpression(expression) => {
                expression_contains_inline_comments(expression)
            }
            _ => false,
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn punctuated_expression_inline_comments(punctuated: &Punctuated<Expression>) -> bool {
    punctuated.pairs().any(|pair| {
        pair.punctuation().map_or(false, token_contains_comments)
            || expression_contains_inline_comments(pair.value())
    })
}

// TODO: can we change this from returning a Vec to just a plain iterator?
pub trait GetLeadingTrivia {
    fn leading_trivia(&self) -> Vec<Token>;
}

impl GetLeadingTrivia for TokenReference {
    fn leading_trivia(&self) -> Vec<Token> {
        self.leading_trivia().cloned().collect()
    }
}

impl GetLeadingTrivia for Suffix {
    fn leading_trivia(&self) -> Vec<Token> {
        suffix_leading_trivia(self).cloned().collect()
    }
}

impl GetLeadingTrivia for Expression {
    fn leading_trivia(&self) -> Vec<Token> {
        get_expression_leading_trivia(self)
    }
}

impl GetLeadingTrivia for Punctuated<Expression> {
    fn leading_trivia(&self) -> Vec<Token> {
        punctuated_leading_trivia(self)
    }
}

impl GetLeadingTrivia for Var {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
            Var::Name(token_reference) => GetLeadingTrivia::leading_trivia(token_reference),
            Var::Expression(var_expr) => {
                if let Some(last_suffix) = var_expr.suffixes().last() {
                    suffix_trailing_trivia(last_suffix)
                } else {
                    unreachable!("got a VarExpression with no suffix");
                }
            }
            other => panic!("unknown node {:?}", other),
        }
    }
}

impl GetLeadingTrivia for VarExpression {
    fn leading_trivia(&self) -> Vec<Token> {
        self.prefix().leading_trivia()
    }
}

impl GetLeadingTrivia for Prefix {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
            Prefix::Name(token) => GetLeadingTrivia::leading_trivia(token),
            Prefix::Expression(expression) => expression.leading_trivia(),
            other => unreachable!("unknown prefix {:?}", other),
        }
    }
}

// Commonly, we update trivia to add in a newline and indent trivia to the leading trivia of a token/node.
// An issue with this is if we do not properly take into account comments. This function also handles any comments present
// by also interspersing them with the required newline and indentation, so they are aligned correctly.
pub fn prepend_newline_indent<T>(ctx: &Context, node: &T, shape: Shape) -> T
where
    T: GetLeadingTrivia + UpdateLeadingTrivia,
{
    // Take all the leading trivia comments, and indent them accordingly
    let leading_trivia: Vec<_> = node
        .leading_trivia()
        .iter()
        .filter(|token| trivia_is_comment(token))
        .cloned()
        .flat_map(|trivia| {
            // Prepend an indent before the comment, and append a newline after the comments
            vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
                trivia,
            ]
        })
        // Add in the newline and indentation for the actual node
        .chain(std::iter::once(create_newline_trivia(ctx)))
        .chain(std::iter::once(create_indent_trivia(ctx, shape)))
        .collect();

    node.update_leading_trivia(FormatTriviaType::Replace(leading_trivia))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_contains_singleline_comments() {
        let token = TokenReference::new(
            vec![],
            Token::new(TokenType::Symbol {
                symbol: full_moon::tokenizer::Symbol::And,
            }),
            vec![Token::new(TokenType::SingleLineComment {
                comment: "hello".into(),
            })],
        );
        assert!(contains_singleline_comments(token))
    }

    #[test]
    fn test_token_contains_no_singleline_comments() {
        let token = TokenReference::new(
            vec![],
            Token::new(TokenType::Symbol {
                symbol: full_moon::tokenizer::Symbol::And,
            }),
            vec![],
        );
        assert!(!contains_singleline_comments(token))
    }

    #[test]
    fn test_token_contains_no_singleline_comments_2() {
        let token = TokenReference::new(
            vec![],
            Token::new(TokenType::Symbol {
                symbol: full_moon::tokenizer::Symbol::And,
            }),
            vec![Token::new(TokenType::MultiLineComment {
                comment: "hello".into(),
                blocks: 1,
            })],
        );
        assert!(!contains_singleline_comments(token))
    }
}
