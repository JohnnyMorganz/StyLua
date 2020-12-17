use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Assignment, BinOpRhs, Call, Do, ElseIf, Expression, FunctionArgs, FunctionBody, FunctionCall,
    FunctionDeclaration, GenericFor, If, Index, LocalAssignment, LocalFunction, MethodCall,
    NumericFor, Prefix, Repeat, Suffix, TableConstructor, Value, Var, VarExpression, While,
};
use full_moon::tokenizer::{Token, TokenKind, TokenReference, TokenType};
use std::borrow::Cow;

// Special Case for Statements
pub fn assignment_add_trivia<'ast>(
    assignment: Assignment<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Assignment<'ast> {
    let mut formatted_var_list = Punctuated::new();
    let mut iterator = assignment.var_list().pairs();

    // Retrieve first item and add indent to trailing trivia
    if let Some(first_pair) = iterator.next() {
        match first_pair {
            Pair::End(value) => {
                formatted_var_list.push(Pair::new(
                    var_add_leading_trivia(value.to_owned(), leading_trivia),
                    None,
                ));
            }
            Pair::Punctuated(value, punctuation) => {
                formatted_var_list.push(Pair::new(
                    var_add_leading_trivia(value.to_owned(), leading_trivia),
                    Some(punctuation.to_owned()),
                ));
            }
        }
    }

    for pair in iterator {
        formatted_var_list.push(pair.to_owned())
    }

    // TODO: Add leading trivia
    let mut formatted_expression_list = assignment.expr_list().to_owned();

    // Retrieve last item and add new line to it
    if let Some(last_pair) = formatted_expression_list.pop() {
        match last_pair {
            Pair::End(value) => {
                let expression = expression_add_trailing_trivia(value, trailing_trivia);
                formatted_expression_list.push(Pair::End(expression));
            }
            Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
        }
    }

    assignment
        .with_var_list(formatted_var_list)
        .with_expr_list(formatted_expression_list)
}

pub fn function_call_add_trivia<'ast>(
    function_call: FunctionCall<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> FunctionCall<'ast> {
    let prefix = prefix_add_leading_trivia(function_call.prefix().to_owned(), leading_trivia);
    function_call_add_trailing_trivia(function_call.with_prefix(prefix), trailing_trivia)
}

pub fn local_assignment_add_trivia<'ast>(
    local_assignment: LocalAssignment<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> LocalAssignment<'ast> {
    let local_token = token_reference_add_trivia(
        local_assignment.local_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );

    // TODO: Can we simplify the following?
    if local_assignment.expr_list().is_empty() {
        // Unassigned local variable
        let mut formatted_name_list = local_assignment.name_list().to_owned();

        // Retrieve last item and add new line to it
        if let Some(last_pair) = formatted_name_list.pop() {
            match last_pair {
                Pair::End(value) => {
                    let value = Cow::Owned(token_reference_add_trivia(
                        value.into_owned(),
                        None,
                        Some(trailing_trivia),
                    ));
                    formatted_name_list.push(Pair::End(value));
                }
                Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
            }
        }

        local_assignment
            .with_local_token(Cow::Owned(local_token))
            .with_name_list(formatted_name_list)
    } else {
        // Add newline at the end of LocalAssignment expression list
        // Expression list should already be formatted
        let mut formatted_expression_list = local_assignment.expr_list().to_owned();

        // Retrieve last item and add new line to it
        if let Some(last_pair) = formatted_expression_list.pop() {
            match last_pair {
                Pair::End(value) => {
                    let expression = expression_add_trailing_trivia(value, trailing_trivia);
                    formatted_expression_list.push(Pair::End(expression));
                }
                Pair::Punctuated(_, _) => (), // TODO: Is it possible for this to happen? Do we need to account for it?
            }
        }

        local_assignment
            .with_local_token(Cow::Owned(local_token))
            .with_expr_list(formatted_expression_list)
    }
}

pub fn do_block_add_trivia<'ast>(
    do_block: Do<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Do<'ast> {
    let do_token = token_reference_add_trivia(
        do_block.do_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        do_block.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    do_block
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

pub fn generic_for_add_trivia<'ast>(
    generic_for: GenericFor<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> GenericFor<'ast> {
    let for_token = token_reference_add_trivia(
        generic_for.for_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );
    let do_token = token_reference_add_trivia(
        generic_for.do_token().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        generic_for.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    generic_for
        .with_for_token(Cow::Owned(for_token))
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

fn else_if_block_add_trivia<'ast>(
    else_if_block: ElseIf<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> ElseIf<'ast> {
    let else_if_token = token_reference_add_trivia(
        else_if_block.else_if_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );
    let then_token = token_reference_add_trivia(
        else_if_block.then_token().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );

    else_if_block
        .with_else_if_token(Cow::Owned(else_if_token))
        .with_then_token(Cow::Owned(then_token))
}

pub fn if_block_add_trivia<'ast>(
    if_block: If<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> If<'ast> {
    let if_token = token_reference_add_trivia(
        if_block.if_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );
    let then_token = token_reference_add_trivia(
        if_block.then_token().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        if_block.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );

    let else_if_block = match if_block.else_if() {
        Some(else_if) => Some(
            else_if
                .iter()
                .map(|else_if| {
                    else_if_block_add_trivia(
                        else_if.to_owned(),
                        leading_trivia.to_owned(),
                        trailing_trivia.to_owned(),
                    )
                })
                .collect(),
        ),
        None => None,
    };

    let else_token = match if_block.else_token() {
        Some(else_token) => Some(Cow::Owned(token_reference_add_trivia(
            else_token.to_owned(),
            Some(leading_trivia.to_owned()),
            Some(trailing_trivia.to_owned()),
        ))),
        None => None,
    };

    if_block
        .with_if_token(Cow::Owned(if_token))
        .with_then_token(Cow::Owned(then_token))
        .with_else_if(else_if_block)
        .with_else_token(else_token)
        .with_end_token(Cow::Owned(end_token))
}

pub fn function_declaration_add_trivia<'ast>(
    function_declaration: FunctionDeclaration<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> FunctionDeclaration<'ast> {
    let function_token = token_reference_add_trivia(
        function_declaration.function_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );

    let function_body = function_declaration.body().to_owned();
    let parameters_parentheses = contained_span_add_trailing_trivia(
        function_body.parameters_parentheses().to_owned(),
        trailing_trivia.to_owned(),
    );
    let end_token = token_reference_add_trivia(
        function_body.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );

    function_declaration
        .with_function_token(Cow::Owned(function_token))
        .with_body(
            function_body
                .with_parameters_parentheses(parameters_parentheses)
                .with_end_token(Cow::Owned(end_token)),
        )
}

pub fn local_function_add_trivia<'ast>(
    local_function: LocalFunction<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> LocalFunction<'ast> {
    let local_token = token_reference_add_trivia(
        local_function.local_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );

    let function_body = local_function.func_body().to_owned();
    let parameters_parentheses = contained_span_add_trivia(
        function_body.parameters_parentheses().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        function_body.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );

    local_function
        .with_local_token(Cow::Owned(local_token))
        .with_func_body(
            function_body
                .with_parameters_parentheses(parameters_parentheses)
                .with_end_token(Cow::Owned(end_token)),
        )
}

pub fn numeric_for_add_trivia<'ast>(
    numeric_for: NumericFor<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> NumericFor<'ast> {
    // TODO: This is a copy of generic_for, can we reduce this?
    let for_token = token_reference_add_trivia(
        numeric_for.for_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );
    let do_token = token_reference_add_trivia(
        numeric_for.do_token().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        numeric_for.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    numeric_for
        .with_for_token(Cow::Owned(for_token))
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

pub fn repeat_block_add_trivia<'ast>(
    repeat_block: Repeat<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Repeat<'ast> {
    let repeat_token = token_reference_add_trivia(
        repeat_block.repeat_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    let until_token = token_reference_add_trivia(
        repeat_block.until_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    repeat_block
        .with_repeat_token(Cow::Owned(repeat_token))
        .with_until_token(Cow::Owned(until_token))
}

pub fn while_block_add_trivia<'ast>(
    while_block: While<'ast>,
    leading_trivia: Vec<Token<'ast>>,
    trailing_trivia: Vec<Token<'ast>>,
) -> While<'ast> {
    let while_token = token_reference_add_trivia(
        while_block.while_token().to_owned(),
        Some(leading_trivia.to_owned()),
        None,
    );
    let do_token = token_reference_add_trivia(
        while_block.do_token().to_owned(),
        None,
        Some(trailing_trivia.to_owned()),
    );
    let end_token = token_reference_add_trivia(
        while_block.end_token().to_owned(),
        Some(leading_trivia.to_owned()),
        Some(trailing_trivia.to_owned()),
    );
    while_block
        .with_while_token(Cow::Owned(while_token))
        .with_do_token(Cow::Owned(do_token))
        .with_end_token(Cow::Owned(end_token))
}

// Remainder of Nodes

/// Adds trailing trivia at the end of a BinOpRhs expression
pub fn binop_rhs_add_trailing_trivia<'ast>(
    binop_rhs: BinOpRhs<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
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
    leading_trivia: Option<Vec<Token<'ast>>>,
    trailing_trivia: Option<Vec<Token<'ast>>>,
) -> ContainedSpan<'ast> {
    let (start_token, end_token) = contained_span.tokens();
    ContainedSpan::new(
        Cow::Owned(token_reference_add_trivia(
            start_token.to_owned(),
            leading_trivia,
            None,
        )),
        Cow::Owned(token_reference_add_trivia(
            end_token.to_owned(),
            None,
            trailing_trivia,
        )),
    )
}

#[deprecated]
pub fn contained_span_add_trailing_trivia<'ast>(
    contained_span: ContainedSpan<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> ContainedSpan<'ast> {
    contained_span_add_trivia(contained_span, None, Some(trailing_trivia))
}

/// Adds trailing trivia at the end of a Call node
pub fn call_add_trailing_trivia<'ast>(
    call: Call<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
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

/// Adds traviling trivia at the end of an Expression node
pub fn expression_add_trailing_trivia<'ast>(
    expression: Expression<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Expression<'ast> {
    match expression {
        Expression::Value { value, binop } => {
            if let Some(binop) = binop {
                Expression::Value {
                    value,
                    binop: Some(binop_rhs_add_trailing_trivia(binop, trailing_trivia)),
                }
            } else {
                Expression::Value {
                    value: Box::new(value_add_trailing_trivia(*value, trailing_trivia)),
                    binop,
                }
            }
        }

        // Add trailing trivia to the end of parentheses
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: contained_span_add_trailing_trivia(contained, trailing_trivia),
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
    trailing_trivia: Vec<Token<'ast>>,
) -> FunctionArgs<'ast> {
    match function_args {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => FunctionArgs::Parentheses {
            parentheses: contained_span_add_trailing_trivia(parentheses, trailing_trivia),
            arguments,
        },

        // Add for completeness
        FunctionArgs::String(token_reference) => FunctionArgs::String(Cow::Owned(
            token_reference_add_trailing_trivia(token_reference.into_owned(), trailing_trivia),
        )),
        FunctionArgs::TableConstructor(table_constructor) => FunctionArgs::TableConstructor(
            table_constructor_add_trailing_trivia(table_constructor, trailing_trivia),
        ),
    }
}

/// Adds trailing trivia at the end of a FunctionBody node
pub fn function_body_add_trailing_trivia<'ast>(
    function_body: FunctionBody<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> FunctionBody<'ast> {
    let function_body_token = function_body.end_token().to_owned();
    function_body.with_end_token(Cow::Owned(token_reference_add_trailing_trivia(
        function_body_token,
        trailing_trivia,
    )))
}

/// Adds trailing trivia at the end of a FunctionCall node
pub fn function_call_add_trailing_trivia<'ast>(
    function_call: FunctionCall<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
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
    trailing_trivia: Vec<Token<'ast>>,
) -> Index<'ast> {
    match index {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: contained_span_add_trailing_trivia(brackets, trailing_trivia),
            expression,
        },
        Index::Dot { dot, name } => Index::Dot {
            dot,
            name: Cow::Owned(token_reference_add_trailing_trivia(
                name.into_owned(),
                trailing_trivia,
            )),
        },
    }
}

/// Adds trailing trivia at the end of a MethodCall node
pub fn method_call_add_trailing_trivia<'ast>(
    method_call: MethodCall<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> MethodCall<'ast> {
    let method_call_args = method_call.args().to_owned();
    method_call.with_args(function_args_add_trailing_trivia(
        method_call_args,
        trailing_trivia,
    ))
}

/// Adds leading trivia to the start of a Prefix node
pub fn prefix_add_leading_trivia<'ast>(
    prefix: Prefix<'ast>,
    leading_trivia: Vec<Token<'ast>>,
) -> Prefix<'ast> {
    match prefix {
        Prefix::Name(token_reference) => Prefix::Name(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            Some(leading_trivia),
            None,
        ))),
        Prefix::Expression(expression) => {
            println!(
                "WARNING: Prefix(Expression) leading trivia not implemented for {}",
                expression
            ); // TODO: Implement
            Prefix::Expression(expression)
        }
    }
}

/// Adds trailing trivia at the end of a Suffix node
pub fn suffix_add_trailing_trivia<'ast>(
    suffix: Suffix<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Suffix<'ast> {
    match suffix {
        Suffix::Call(call) => Suffix::Call(call_add_trailing_trivia(call, trailing_trivia)),
        Suffix::Index(index) => Suffix::Index(index_add_trailing_trivia(index, trailing_trivia)),
    }
}

/// Adds trailing trivia at the end of a TableConstructor node
pub fn table_constructor_add_trailing_trivia<'ast>(
    table_constructor: TableConstructor<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> TableConstructor<'ast> {
    let table_constructor_braces = table_constructor.braces().to_owned();
    table_constructor.with_braces(contained_span_add_trailing_trivia(
        table_constructor_braces,
        trailing_trivia,
    ))
}

/// Adds trivia to a TokenReferenece
pub fn token_reference_add_trivia<'ast>(
    token_reference: TokenReference<'ast>,
    leading_trivia: Option<Vec<Token<'ast>>>,
    trailing_trivia: Option<Vec<Token<'ast>>>,
) -> TokenReference<'ast> {
    let added_leading_trivia = match leading_trivia {
        Some(trivia) => {
            let mut current: Vec<Token<'ast>> = token_reference
                .leading_trivia()
                .filter(|x| x.token_kind() == TokenKind::SingleLineComment)
                .map(|x| x.to_owned())
                .collect();
            current.extend(trivia);
            current
        }
        None => token_reference
            .leading_trivia()
            .map(|x| x.to_owned())
            .collect(),
    };

    let added_trailing_trivia = match trailing_trivia {
        Some(trivia) => {
            let mut current: Vec<Token<'ast>> = token_reference
                .trailing_trivia()
                .filter(|x| x.token_kind() == TokenKind::SingleLineComment)
                .map(|x| x.to_owned())
                .collect();
            current.extend(trivia);
            current
        }
        None => token_reference
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

#[deprecated]
pub fn token_reference_add_trailing_trivia<'ast>(
    token_reference: TokenReference<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> TokenReference<'ast> {
    token_reference_add_trivia(token_reference, None, Some(trailing_trivia))
}

/// Adds trailing trivia at the end of a Value node
pub fn value_add_trailing_trivia<'ast>(
    value: Value<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
) -> Value<'ast> {
    match value {
        Value::String(token_reference) => Value::String(Cow::Owned(
            token_reference_add_trailing_trivia(token_reference.into_owned(), trailing_trivia),
        )),
        Value::Number(token_reference) => Value::Number(Cow::Owned(
            token_reference_add_trailing_trivia(token_reference.into_owned(), trailing_trivia),
        )),
        Value::Symbol(token_reference) => Value::Symbol(Cow::Owned(
            token_reference_add_trailing_trivia(token_reference.into_owned(), trailing_trivia),
        )),
        Value::ParseExpression(expression) => {
            Value::ParseExpression(expression_add_trailing_trivia(expression, trailing_trivia))
        }
        Value::FunctionCall(function_call) => Value::FunctionCall(
            function_call_add_trailing_trivia(function_call, trailing_trivia),
        ),
        Value::TableConstructor(table_constructor) => Value::TableConstructor(
            table_constructor_add_trailing_trivia(table_constructor, trailing_trivia),
        ),
        Value::Var(var) => Value::Var(var_add_trailing_trivia(var, trailing_trivia)),
        Value::Function((token, function_body)) => Value::Function((
            token,
            function_body_add_trailing_trivia(function_body, trailing_trivia),
        )),
    }
}

pub fn var_add_leading_trivia<'ast>(var: Var<'ast>, leading_trivia: Vec<Token<'ast>>) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(Cow::Owned(token_reference_add_trivia(
            token_reference.into_owned(),
            Some(leading_trivia),
            None,
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
    trailing_trivia: Vec<Token<'ast>>,
) -> Var<'ast> {
    match var {
        Var::Name(token_reference) => Var::Name(Cow::Owned(token_reference_add_trailing_trivia(
            token_reference.into_owned(),
            trailing_trivia,
        ))),
        Var::Expression(var_expression) => Var::Expression(var_expression_add_trailing_trivia(
            var_expression,
            trailing_trivia,
        )),
    }
}

pub fn var_expression_add_leading_trivia<'ast>(
    var_expresion: VarExpression<'ast>,
    leading_trivia: Vec<Token<'ast>>,
) -> VarExpression<'ast> {
    let prefix = prefix_add_leading_trivia(var_expresion.prefix().to_owned(), leading_trivia);
    var_expresion.with_prefix(prefix)
}

/// Adds trailing trivia at the end of a VarExpression node
pub fn var_expression_add_trailing_trivia<'ast>(
    var_expression: VarExpression<'ast>,
    trailing_trivia: Vec<Token<'ast>>,
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
