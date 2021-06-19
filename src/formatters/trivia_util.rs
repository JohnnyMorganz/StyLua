use crate::formatters::trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia};
#[cfg(feature = "luau")]
use full_moon::ast::span::ContainedSpan;
#[cfg(feature = "luau")]
use full_moon::ast::types::{IndexedTypeInfo, TypeDeclaration, TypeField, TypeInfo};
use full_moon::ast::{Block, FunctionBody};
use full_moon::{
    ast::{
        BinOp, Call, Expression, Field, FunctionArgs, Index, LastStmt, Prefix, Stmt, Suffix,
        TableConstructor, UnOp, Value, Var,
    },
    node::Node,
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};

pub fn trivia_is_whitespace(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::Whitespace)
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

pub fn is_function_empty(function_body: &FunctionBody) -> bool {
    is_block_empty(function_body.block())
        && !function_body
            .parameters_parentheses()
            .tokens()
            .1
            .trailing_trivia()
            .any(trivia_is_comment)
        && !function_body
            .end_token()
            .leading_trivia()
            .any(trivia_is_comment)
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
        TypeInfo::Variadic { type_info, .. } => type_info_trailing_trivia(type_info),

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

pub fn binop_leading_comments<'ast>(binop: &BinOp<'ast>) -> Vec<Token<'ast>> {
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
        other => panic!("unknown node {:?}", other),
    }
}

pub fn binop_trailing_comments<'ast>(binop: &BinOp<'ast>) -> Vec<Token<'ast>> {
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
        other => panic!("unknown node {:?}", other),
    }
}

pub fn expression_leading_comments<'ast>(expression: &Expression<'ast>) -> Vec<Token<'ast>> {
    get_expression_leading_trivia(expression)
        .iter()
        .filter(|token| trivia_is_comment(token))
        .map(|x| x.to_owned())
        .collect()
}

pub fn take_expression_leading_comments<'ast>(
    expression: &Expression<'ast>,
) -> (Expression<'ast>, Vec<Token<'ast>>) {
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

pub fn take_expression_trailing_comments<'ast>(
    expression: &Expression<'ast>,
) -> (Expression<'ast>, Vec<Token<'ast>>) {
    let trailing_comments = get_expression_trailing_trivia(expression)
        .iter()
        .filter(|token| trivia_is_comment(token))
        .map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .flatten()
        .collect();

    (
        expression.update_trailing_trivia(
            FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
        ),
        trailing_comments,
    )
}

#[cfg(feature = "luau")]
pub fn take_type_field_trailing_comments(
    type_field: TypeField<'_>,
) -> (TypeField<'_>, Vec<Token<'_>>) {
    let trailing_comments = type_info_trailing_trivia(type_field.value())
        .iter()
        .filter(|token| trivia_is_comment(token))
        .map(|x| {
            // Prepend a single space beforehand
            vec![Token::new(TokenType::spaces(1)), x.to_owned()]
        })
        .flatten()
        .collect();

    (
        type_field.update_trailing_trivia(
            FormatTriviaType::Replace(vec![]), // TODO: Do we need to keep some trivia?
        ),
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
        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let (return_type, trailing_trivia) = get_type_info_trailing_trivia(*return_type);
            (
                TypeInfo::Callback {
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
        other => panic!("unknown node {:?}", other),
    }
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
                local_assignment.with_names(formatted_name_list)
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
                None => Vec::new(),
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

pub fn get_last_stmt_trailing_trivia(last_stmt: LastStmt) -> (LastStmt, Vec<Token>) {
    match last_stmt {
        LastStmt::Return(ret) => {
            let mut return_token = ret.token().to_owned();
            let mut formatted_expression_list = ret.returns().to_owned();
            let mut trailing_trivia = Vec::new();

            // Retrieve last item and remove trailing trivia
            if let Some(last_pair) = formatted_expression_list.pop() {
                let pair = last_pair.map(|value| {
                    trailing_trivia = get_expression_trailing_trivia(&value);
                    value.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
                });
                formatted_expression_list.push(pair);
            } else {
                trailing_trivia = return_token
                    .trailing_trivia()
                    .map(|x| x.to_owned())
                    .collect();
                return_token =
                    return_token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));
            }

            (
                LastStmt::Return(
                    ret.with_token(return_token)
                        .with_returns(formatted_expression_list),
                ),
                trailing_trivia,
            )
        }
        LastStmt::Break(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

            (LastStmt::Break(token), trailing_trivia)
        }
        #[cfg(feature = "luau")]
        LastStmt::Continue(token) => {
            let trailing_trivia = token.trailing_trivia().map(|x| x.to_owned()).collect();
            let token = token.update_trailing_trivia(FormatTriviaType::Replace(vec![]));

            (LastStmt::Continue(token), trailing_trivia)
        }
        other => panic!("unknown node {:?}", other),
    }
}

pub fn token_trivia_contains_comments<'ast>(
    trivia: impl Iterator<Item = &'ast Token<'ast>>,
) -> bool {
    for trivia in trivia {
        if trivia_is_comment(trivia) {
            return true;
        }
    }
    false
}

pub fn token_contains_leading_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.leading_trivia())
}

pub fn token_contains_trailing_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.trailing_trivia())
}

pub fn token_contains_comments(token_ref: &TokenReference) -> bool {
    token_trivia_contains_comments(token_ref.leading_trivia())
        || token_trivia_contains_comments(token_ref.trailing_trivia())
}

pub fn contains_comments<'ast>(node: impl Node<'ast>) -> bool {
    node.tokens().into_iter().any(token_contains_comments)
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

// Checks to see whether an expression contains comments inline inside of it
// This can only happen if the expression is a BinOp
// We should ignore any comments which are trailing for the whole expression, as they are not inline
pub fn expression_contains_inline_comments(expression: &Expression) -> bool {
    match expression {
        Expression::BinaryOperator { lhs, binop, rhs } => {
            contains_comments(binop) || contains_comments(lhs)
            // Check if the binop chain still continues
            // If so, we should keep checking the expresion
            // Otherwise, stop checking
            || match &**rhs {
                Expression::BinaryOperator { .. } => expression_contains_inline_comments(rhs),
                Expression::UnaryOperator { unop, expression } => {
                    let op_contains_comments = match unop {
                        UnOp::Minus(token) | UnOp::Not(token) | UnOp::Hash(token) => contains_comments(token),
                        other => panic!("unknown node {:?}", other)
                    };
                    op_contains_comments || expression_contains_inline_comments(expression)
                }
                Expression::Value{ .. } => false,
                Expression::Parentheses { .. } => contains_comments(rhs),
                other => panic!("unknown node {:?}", other),
            }
        }
        _ => false,
    }
}
