use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    formatters::trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
    shape::Shape,
};
#[cfg(feature = "luau")]
use full_moon::ast::types::{
    GenericDeclarationParameter, GenericParameterInfo, IndexedTypeInfo, TypeArgument,
    TypeDeclaration, TypeInfo, TypeSpecifier,
};
use full_moon::{
    ast::{
        punctuated::Punctuated, BinOp, Block, Call, Expression, Field, FunctionArgs, Index,
        LastStmt, LocalAssignment, Parameter, Prefix, Stmt, Suffix, TableConstructor, UnOp, Var,
        VarExpression,
    },
    node::Node,
    tokenizer::{Token, TokenKind, TokenReference, TokenType},
};

// TODO: can we change this from returning a Vec to just a plain iterator?
pub trait GetLeadingTrivia {
    fn leading_trivia(&self) -> Vec<Token>;

    fn has_leading_comments(&self, search: CommentSearch) -> bool {
        trivia_contains_comments(self.leading_trivia().iter(), search)
    }

    fn leading_comments(&self) -> Vec<Token> {
        self.leading_trivia()
            .iter()
            .filter(|token| trivia_is_comment(token))
            .cloned()
            .collect()
    }
}

pub trait GetTrailingTrivia {
    fn trailing_trivia(&self) -> Vec<Token>;

    fn has_trailing_comments(&self, search: CommentSearch) -> bool {
        trivia_contains_comments(self.trailing_trivia().iter(), search)
    }

    // Retrieves all the trailing comments from the token
    // Prepends a space before each comment
    fn trailing_comments_search(&self, search: CommentSearch) -> Vec<Token> {
        self.trailing_trivia()
            .iter()
            .filter(|token| trivia_is_comment_search(token, search))
            .flat_map(|x| {
                // Prepend a single space beforehand
                vec![Token::new(TokenType::spaces(1)), x.to_owned()]
            })
            .collect()
    }

    fn trailing_comments(&self) -> Vec<Token> {
        self.trailing_comments_search(CommentSearch::All)
    }
}

pub fn trivia_is_whitespace(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::Whitespace)
}

pub fn trivia_is_singleline_comment(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::SingleLineComment)
}

fn trivia_is_multiline_comment(trivia: &Token) -> bool {
    matches!(trivia.token_kind(), TokenKind::MultiLineComment)
}

pub fn trivia_is_comment(trivia: &Token) -> bool {
    matches!(
        trivia.token_kind(),
        TokenKind::SingleLineComment | TokenKind::MultiLineComment
    )
}

fn trivia_is_comment_search(trivia: &Token, search: CommentSearch) -> bool {
    match search {
        CommentSearch::Single => trivia_is_singleline_comment(trivia),
        CommentSearch::Multiline => trivia_is_multiline_comment(trivia),
        CommentSearch::All => trivia_is_comment(trivia),
    }
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
    let string = format!("{item}");
    string.lines().count() > 1
}

pub fn can_hang_expression(expression: &Expression) -> bool {
    match expression {
        Expression::Parentheses { .. } => true, // Can always hang parentheses if necessary
        Expression::UnaryOperator { expression, .. } => can_hang_expression(expression),
        Expression::BinaryOperator { .. } => true, // If a binop is present, then we can hang the expression
        Expression::FunctionCall(function_call) => match function_call.prefix() {
            Prefix::Expression(expression) => can_hang_expression(expression),
            _ => false,
        },
        Expression::Var(Var::Expression(expression)) => match expression.prefix() {
            Prefix::Expression(expression) => can_hang_expression(expression),
            _ => false,
        },
        #[cfg(feature = "luau")]
        Expression::TypeAssertion { expression, .. } => can_hang_expression(expression),
        _ => false,
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
impl GetTrailingTrivia for FunctionArgs {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
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
}

impl GetTrailingTrivia for Prefix {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Prefix::Name(name) => GetTrailingTrivia::trailing_trivia(name),
            Prefix::Expression(expression) => expression.trailing_trivia(),
            other => panic!("unknown node {:?}", other),
        }
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

impl GetLeadingTrivia for Suffix {
    fn leading_trivia(&self) -> Vec<Token> {
        suffix_leading_trivia(self).cloned().collect()
    }
}

impl GetTrailingTrivia for Suffix {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Suffix::Index(index) => match index {
                Index::Brackets { brackets, .. } => {
                    let (_, end_brace) = brackets.tokens();
                    end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
                }
                Index::Dot { name, .. } => name.trailing_trivia().map(|x| x.to_owned()).collect(),
                other => panic!("unknown node {:?}", other),
            },
            Suffix::Call(call) => match call {
                Call::AnonymousCall(function_args) => function_args.trailing_trivia(),
                Call::MethodCall(method_call) => method_call.args().trailing_trivia(),
                other => panic!("unknown node {:?}", other),
            },
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[cfg(feature = "luau")]
impl GetTrailingTrivia for GenericDeclarationParameter {
    fn trailing_trivia(&self) -> Vec<Token> {
        if let Some(default_type) = self.default_type() {
            default_type.trailing_trivia()
        } else {
            match self.parameter() {
                GenericParameterInfo::Name(token) => token.trailing_trivia().cloned().collect(),
                GenericParameterInfo::Variadic { ellipse, .. } => {
                    ellipse.trailing_trivia().cloned().collect()
                }
                other => panic!("unknown node {:?}", other),
            }
        }
    }
}

impl GetTrailingTrivia for VarExpression {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.suffixes().last().map_or_else(
            || self.prefix().trailing_trivia(),
            GetTrailingTrivia::trailing_trivia,
        )
    }
}

impl GetTrailingTrivia for Var {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Var::Name(token_reference) => GetTrailingTrivia::trailing_trivia(token_reference),
            Var::Expression(var_expr) => var_expr.trailing_trivia(),
            other => panic!("unknown node {:?}", other),
        }
    }
}

impl GetLeadingTrivia for Expression {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
            Expression::Parentheses { contained, .. } => {
                GetLeadingTrivia::leading_trivia(contained.tokens().0)
            }
            Expression::UnaryOperator { unop, .. } => match unop {
                UnOp::Minus(token_ref) | UnOp::Not(token_ref) | UnOp::Hash(token_ref) => {
                    GetLeadingTrivia::leading_trivia(token_ref)
                }
                #[cfg(feature = "lua53")]
                UnOp::Tilde(token_ref) => GetLeadingTrivia::leading_trivia(token_ref),
                other => panic!("unknown node {:?}", other),
            },
            Expression::BinaryOperator { lhs, .. } => lhs.leading_trivia(),
            Expression::Function((token_ref, _)) => GetLeadingTrivia::leading_trivia(token_ref),
            Expression::FunctionCall(function_call) => function_call.prefix().leading_trivia(),
            #[cfg(feature = "luau")]
            Expression::IfExpression(if_expression) => {
                GetLeadingTrivia::leading_trivia(if_expression.if_token())
            }
            #[cfg(feature = "luau")]
            Expression::InterpolatedString(interpolated_string) => {
                interpolated_string.segments().next().map_or_else(
                    || GetLeadingTrivia::leading_trivia(interpolated_string.last_string()),
                    |segment| GetLeadingTrivia::leading_trivia(&segment.literal),
                )
            }
            Expression::TableConstructor(table) => {
                GetLeadingTrivia::leading_trivia(table.braces().tokens().0)
            }
            Expression::Number(token_ref) => GetLeadingTrivia::leading_trivia(token_ref),
            Expression::String(token_ref) => GetLeadingTrivia::leading_trivia(token_ref),
            Expression::Symbol(token_ref) => GetLeadingTrivia::leading_trivia(token_ref),
            Expression::Var(var) => var.leading_trivia(),
            #[cfg(feature = "luau")]
            Expression::TypeAssertion { expression, .. } => expression.leading_trivia(),
            other => panic!("unknown node {:?}", other),
        }
    }
}

impl GetTrailingTrivia for Expression {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Expression::Parentheses { contained, .. } => {
                let (_, end_parentheses) = contained.tokens();
                GetTrailingTrivia::trailing_trivia(end_parentheses)
            }
            Expression::UnaryOperator { expression, .. } => expression.trailing_trivia(),
            Expression::BinaryOperator { rhs, .. } => rhs.trailing_trivia(),
            Expression::Function((_, function_body)) => {
                GetTrailingTrivia::trailing_trivia(function_body.end_token())
            }
            Expression::FunctionCall(function_call) => function_call
                .suffixes()
                .last()
                .map_or_else(Vec::new, GetTrailingTrivia::trailing_trivia),
            Expression::String(token_reference) => {
                GetTrailingTrivia::trailing_trivia(token_reference)
            }
            Expression::TableConstructor(table_constructor) => {
                let (_, end_brace) = table_constructor.braces().tokens();
                end_brace.trailing_trivia().map(|x| x.to_owned()).collect()
            }
            Expression::Number(token_reference) => {
                GetTrailingTrivia::trailing_trivia(token_reference)
            }
            Expression::Symbol(token_reference) => {
                GetTrailingTrivia::trailing_trivia(token_reference)
            }
            Expression::Var(var) => var.trailing_trivia(),
            #[cfg(feature = "luau")]
            Expression::IfExpression(if_expression) => {
                if_expression.else_expression().trailing_trivia()
            }
            #[cfg(feature = "luau")]
            Expression::InterpolatedString(interpolated_string) => {
                GetTrailingTrivia::trailing_trivia(interpolated_string.last_string())
            }
            #[cfg(feature = "luau")]
            Expression::TypeAssertion { type_assertion, .. } => {
                type_assertion.cast_to().trailing_trivia()
            }
            other => panic!("unknown node {:?}", other),
        }
    }
}

impl GetLeadingTrivia for BinOp {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
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
            | BinOp::TwoEqual(token) => GetLeadingTrivia::leading_trivia(token),
            #[cfg(feature = "lua53")]
            BinOp::Ampersand(token)
            | BinOp::DoubleSlash(token)
            | BinOp::DoubleLessThan(token)
            | BinOp::Pipe(token)
            | BinOp::DoubleGreaterThan(token)
            | BinOp::Tilde(token) => GetLeadingTrivia::leading_trivia(token),
            other => panic!("unknown node {:?}", other),
        }
    }
}

impl GetTrailingTrivia for BinOp {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
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
            | BinOp::TwoEqual(token) => GetTrailingTrivia::trailing_trivia(token),
            #[cfg(feature = "lua53")]
            BinOp::Ampersand(token)
            | BinOp::DoubleSlash(token)
            | BinOp::DoubleLessThan(token)
            | BinOp::Pipe(token)
            | BinOp::DoubleGreaterThan(token)
            | BinOp::Tilde(token) => GetTrailingTrivia::trailing_trivia(token),
            other => panic!("unknown node {:?}", other),
        }
    }
}

pub fn take_leading_comments<T: GetLeadingTrivia + UpdateLeadingTrivia>(
    node: &T,
) -> (T, Vec<Token>) {
    let leading_comments = node.leading_comments();
    (
        node.update_leading_trivia(FormatTriviaType::Replace(vec![])),
        leading_comments,
    )
}

#[cfg(feature = "luau")]
pub fn take_trailing_trivia<T: GetTrailingTrivia + UpdateTrailingTrivia>(
    node: &T,
) -> (T, Vec<Token>) {
    let trailing_comments = node.trailing_trivia();
    (
        node.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
        trailing_comments,
    )
}

pub fn take_trailing_comments<T: GetTrailingTrivia + UpdateTrailingTrivia>(
    node: &T,
) -> (T, Vec<Token>) {
    let trailing_comments = node.trailing_comments();
    (
        node.update_trailing_trivia(FormatTriviaType::Replace(vec![])),
        trailing_comments,
    )
}

impl GetTrailingTrivia for Parameter {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Parameter::Name(token) | Parameter::Ellipse(token) => {
                GetTrailingTrivia::trailing_trivia(token)
            }
            other => panic!("unknown node {:?}", other),
        }
    }
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
impl GetTrailingTrivia for IndexedTypeInfo {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            IndexedTypeInfo::Basic(token) => GetTrailingTrivia::trailing_trivia(token),
            IndexedTypeInfo::Generic { arrows, .. } => {
                GetTrailingTrivia::trailing_trivia(arrows.tokens().1)
            }
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[cfg(feature = "luau")]
impl GetTrailingTrivia for TypeInfo {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            TypeInfo::Array { braces, .. } => GetTrailingTrivia::trailing_trivia(braces.tokens().1),
            TypeInfo::Basic(token) => GetTrailingTrivia::trailing_trivia(token),
            TypeInfo::String(token) => GetTrailingTrivia::trailing_trivia(token),
            TypeInfo::Boolean(token) => GetTrailingTrivia::trailing_trivia(token),
            TypeInfo::Callback { return_type, .. } => return_type.trailing_trivia(),
            TypeInfo::Generic { arrows, .. } => {
                GetTrailingTrivia::trailing_trivia(arrows.tokens().1)
            }
            TypeInfo::GenericPack { ellipse, .. } => GetTrailingTrivia::trailing_trivia(ellipse),
            TypeInfo::Intersection { right, .. } => right.trailing_trivia(),
            TypeInfo::Module { type_info, .. } => type_info.trailing_trivia(),
            TypeInfo::Optional { question_mark, .. } => {
                GetTrailingTrivia::trailing_trivia(question_mark)
            }
            TypeInfo::Table { braces, .. } => GetTrailingTrivia::trailing_trivia(braces.tokens().1),
            TypeInfo::Typeof { parentheses, .. } => {
                GetTrailingTrivia::trailing_trivia(parentheses.tokens().1)
            }
            TypeInfo::Tuple { parentheses, .. } => {
                GetTrailingTrivia::trailing_trivia(parentheses.tokens().1)
            }
            TypeInfo::Union { right, .. } => right.trailing_trivia(),
            TypeInfo::Variadic { type_info, .. } => type_info.trailing_trivia(),
            TypeInfo::VariadicPack { name, .. } => GetTrailingTrivia::trailing_trivia(name),
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[cfg(feature = "luau")]
impl GetLeadingTrivia for TypeInfo {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
            TypeInfo::Array { braces, .. } => GetLeadingTrivia::leading_trivia(braces.tokens().0),
            TypeInfo::Basic(token) | TypeInfo::String(token) | TypeInfo::Boolean(token) => {
                GetLeadingTrivia::leading_trivia(token)
            }
            TypeInfo::Callback {
                generics,
                parentheses,
                ..
            } => match generics {
                Some(generics) => GetLeadingTrivia::leading_trivia(generics.arrows().tokens().0),
                None => GetLeadingTrivia::leading_trivia(parentheses.tokens().0),
            },
            TypeInfo::Generic { base, .. } => GetLeadingTrivia::leading_trivia(base),
            TypeInfo::GenericPack { name, .. } => GetLeadingTrivia::leading_trivia(name),
            TypeInfo::Intersection { left, .. } => left.leading_trivia(),
            TypeInfo::Module { module, .. } => GetLeadingTrivia::leading_trivia(module),
            TypeInfo::Optional { base, .. } => base.leading_trivia(),
            TypeInfo::Table { braces, .. } => GetLeadingTrivia::leading_trivia(braces.tokens().0),
            TypeInfo::Typeof { typeof_token, .. } => GetLeadingTrivia::leading_trivia(typeof_token),
            TypeInfo::Tuple { parentheses, .. } => {
                GetLeadingTrivia::leading_trivia(parentheses.tokens().0)
            }
            TypeInfo::Union { left, .. } => left.leading_trivia(),
            TypeInfo::Variadic { ellipse, .. } => GetLeadingTrivia::leading_trivia(ellipse),
            TypeInfo::VariadicPack { ellipse, .. } => GetLeadingTrivia::leading_trivia(ellipse),
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[cfg(feature = "luau")]
impl GetTrailingTrivia for TypeArgument {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.type_info().trailing_trivia()
    }
}

#[cfg(feature = "luau")]
impl GetTrailingTrivia for TypeDeclaration {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.type_definition().trailing_trivia()
    }
}

#[cfg(feature = "luau")]
impl GetTrailingTrivia for TypeSpecifier {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.type_info().trailing_trivia()
    }
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
            trailing_trivia = type_specifier.trailing_trivia();

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
                    trailing_trivia = value.trailing_trivia();
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
                        trailing_trivia = value.trailing_trivia();
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
                Some(suffix) => suffix.trailing_trivia(),
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
            let trailing_trivia = repeat_block.until().trailing_trivia();
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
            let trailing_trivia = stmt.rhs().trailing_trivia();
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
            let (type_declaration, trailing_trivia) = take_trailing_trivia(stmt.type_declaration());
            (
                Stmt::ExportedTypeDeclaration(stmt.with_type_declaration(type_declaration)),
                trailing_trivia,
            )
        }
        #[cfg(feature = "luau")]
        Stmt::TypeDeclaration(stmt) => {
            let (type_declaration, trailing_trivia) = take_trailing_trivia(&stmt);
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

impl GetTrailingTrivia for LastStmt {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            LastStmt::Return(ret) => {
                if ret.returns().is_empty() {
                    GetTrailingTrivia::trailing_trivia(ret.token())
                } else {
                    ret.returns().trailing_trivia()
                }
            }
            LastStmt::Break(token) => GetTrailingTrivia::trailing_trivia(token),
            #[cfg(feature = "luau")]
            LastStmt::Continue(token) => GetTrailingTrivia::trailing_trivia(token),
            other => panic!("unknown node {:?}", other),
        }
    }
}

#[derive(Clone, Copy)]
pub enum CommentSearch {
    // Only care about singleline comments
    Single,
    // Only care about multiline comments
    Multiline,
    // Looking for all comments
    All,
}

fn trivia_contains_comments<'a>(
    mut trivia: impl Iterator<Item = &'a Token>,
    search: CommentSearch,
) -> bool {
    let tester = match search {
        CommentSearch::Single => trivia_is_singleline_comment,
        CommentSearch::Multiline => trivia_is_multiline_comment,
        CommentSearch::All => trivia_is_comment,
    };

    trivia.any(tester)
}

fn token_contains_comments_search(token: &TokenReference, search: CommentSearch) -> bool {
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

impl GetTrailingTrivia for Field {
    fn trailing_trivia(&self) -> Vec<Token> {
        match self {
            Field::ExpressionKey { value, .. } => value.trailing_trivia(),
            Field::NameKey { value, .. } => value.trailing_trivia(),
            Field::NoKey(expression) => expression.trailing_trivia(),
            other => panic!("unknown node {:?}", other),
        }
    }
}

pub fn punctuated_inline_comments<T: GetLeadingTrivia + GetTrailingTrivia + HasInlineComments>(
    punctuated: &Punctuated<T>,
    include_leading: bool,
) -> bool {
    let mut iter = punctuated.pairs().peekable();
    while let Some(pair) = iter.next() {
        // Only check trailing comments on the expression if this is not the last pair
        if iter.peek().is_some() && !pair.value().trailing_comments().is_empty() {
            return true;
        }

        if pair.punctuation().map_or(false, token_contains_comments)
            || (include_leading && !pair.value().leading_comments().is_empty())
            || pair.value().has_inline_comments()
        {
            return true;
        }
    }

    false
}

impl GetLeadingTrivia for TokenReference {
    fn leading_trivia(&self) -> Vec<Token> {
        self.leading_trivia().cloned().collect()
    }
}

impl<T: GetLeadingTrivia> GetLeadingTrivia for Punctuated<T> {
    fn leading_trivia(&self) -> Vec<Token> {
        self.iter()
            .next()
            .map_or_else(Vec::new, GetLeadingTrivia::leading_trivia)
    }
}

impl GetLeadingTrivia for Var {
    fn leading_trivia(&self) -> Vec<Token> {
        match self {
            Var::Name(token_reference) => GetLeadingTrivia::leading_trivia(token_reference),
            Var::Expression(var_expr) => var_expr.prefix().leading_trivia(),
            other => panic!("unknown node {:?}", other),
        }
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

impl GetTrailingTrivia for TokenReference {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.trailing_trivia().cloned().collect()
    }
}

impl<T: GetTrailingTrivia> GetTrailingTrivia for Punctuated<T> {
    fn trailing_trivia(&self) -> Vec<Token> {
        self.iter()
            .last()
            .map_or_else(Vec::new, GetTrailingTrivia::trailing_trivia)
    }
}

pub trait HasInlineComments {
    fn has_inline_comments(&self) -> bool {
        false
    }
}

impl HasInlineComments for Expression {
    // Checks to see whether an expression contains comments inline inside of it
    // This can only happen if the expression is a BinOp
    // We should ignore any comments which are trailing for the whole expression, as they are not inline
    fn has_inline_comments(&self) -> bool {
        match self {
            Expression::BinaryOperator { lhs, binop, rhs } => {
                contains_comments(binop) || contains_comments(lhs) || rhs.has_inline_comments()
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
                op_contains_comments || expression.has_inline_comments()
            }
            Expression::Parentheses {
                contained,
                expression,
            } => {
                contained
                    .tokens()
                    .0
                    .has_trailing_comments(CommentSearch::All)
                    || contained
                        .tokens()
                        .1
                        .has_leading_comments(CommentSearch::All)
                    || contains_comments(expression)
            }
            #[cfg(feature = "luau")]
            Expression::TypeAssertion { expression, .. } => expression.has_inline_comments(),
            _ => false,
        }
    }
}

impl HasInlineComments for Var {}
impl HasInlineComments for TokenReference {}

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
