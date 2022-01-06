#[cfg(feature = "luau")]
use full_moon::ast::types::{
    GenericDeclaration, GenericDeclarationParameter, IfExpression, IndexedTypeInfo, TypeArgument,
    TypeAssertion, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
};
use full_moon::ast::{
    punctuated::Punctuated, span::ContainedSpan, BinOp, Call, Expression, FunctionArgs,
    FunctionBody, FunctionCall, FunctionName, Index, MethodCall, Parameter, Prefix, Suffix,
    TableConstructor, UnOp, Value, Var, VarExpression,
};
use full_moon::tokenizer::{Token, TokenReference};

/// Enum to determine how trivia should be added when using trivia formatter functions
#[derive(Clone, Debug)]
pub enum FormatTriviaType {
    /// Trivia will be added to the end of the current trivia
    Append(Vec<Token>),
    /// The current trivia will be replaced with the new trivia
    Replace(Vec<Token>),
    /// Trivia will not be changed
    NoChange,
}

/// Strips all leading and trailing trivia from a specific node.
/// This is useful if we need to use the node to calculate sizing, whilst we do not want trivia included
pub fn strip_trivia<T>(item: &T) -> T
where
    T: UpdateLeadingTrivia + UpdateTrailingTrivia,
{
    item.update_leading_trivia(FormatTriviaType::Replace(vec![]))
        .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
}

pub fn strip_leading_trivia<T>(item: &T) -> T
where
    T: UpdateLeadingTrivia,
{
    item.update_leading_trivia(FormatTriviaType::Replace(vec![]))
}

pub fn strip_trailing_trivia<T>(item: &T) -> T
where
    T: UpdateTrailingTrivia,
{
    item.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
}

pub trait UpdateLeadingTrivia {
    fn update_leading_trivia(&self, leading_trivia: FormatTriviaType) -> Self;
}

pub trait UpdateTrailingTrivia {
    fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType) -> Self;
}

pub trait UpdateTrivia {
    fn update_trivia(
        &self,
        leading_trivia: FormatTriviaType,
        trailing_trivia: FormatTriviaType,
    ) -> Self;
}

impl<T> UpdateLeadingTrivia for T
where
    T: UpdateTrivia,
{
    fn update_leading_trivia(&self, leading_trivia: FormatTriviaType) -> Self
    where
        Self: std::marker::Sized,
    {
        self.update_trivia(leading_trivia, FormatTriviaType::NoChange)
    }
}

impl<T> UpdateTrailingTrivia for T
where
    T: UpdateTrivia,
{
    fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType) -> Self
    where
        Self: std::marker::Sized,
    {
        self.update_trivia(FormatTriviaType::NoChange, trailing_trivia)
    }
}

impl UpdateTrivia for TokenReference {
    fn update_trivia(
        &self,
        leading_trivia: FormatTriviaType,
        trailing_trivia: FormatTriviaType,
    ) -> Self {
        let added_leading_trivia = match leading_trivia {
            FormatTriviaType::Append(trivia) => {
                let mut current: Vec<Token> = self.leading_trivia().map(|x| x.to_owned()).collect();
                current.extend(trivia);
                current
            }
            FormatTriviaType::Replace(trivia) => trivia,
            FormatTriviaType::NoChange => self.leading_trivia().map(|x| x.to_owned()).collect(),
        };
        let added_trailing_trivia = match trailing_trivia {
            FormatTriviaType::Append(trivia) => {
                let mut current: Vec<Token> =
                    self.trailing_trivia().map(|x| x.to_owned()).collect();
                current.extend(trivia);
                current
            }
            FormatTriviaType::Replace(trivia) => trivia,
            FormatTriviaType::NoChange => self.trailing_trivia().map(|x| x.to_owned()).collect(),
        };
        TokenReference::new(
            added_leading_trivia,
            self.token().to_owned(),
            added_trailing_trivia,
        )
    }
}

macro_rules! define_update_trivia {
    ($node:ident, |$self:ident, $leading_trivia:ident, $trailing_trivia:ident| $body:expr) => {
        define_update_trivia! {$node, |$self:&$node, $leading_trivia: FormatTriviaType, $trailing_trivia: FormatTriviaType| $body}
    };
    ($node:ident, $body:expr) => {
        impl UpdateTrivia for $node {
            fn update_trivia(&self, leading_trivia: FormatTriviaType, trailing_trivia: FormatTriviaType) -> Self {
                $body(&self, leading_trivia, trailing_trivia)
            }
        }
    };
}

macro_rules! define_update_leading_trivia {
    ($node:ident, |$self:ident, $leading_trivia:ident| $body:expr) => {
        define_update_leading_trivia! {$node, |$self:&$node, $leading_trivia: FormatTriviaType| $body}
    };
    ($node:ident, $body:expr) => {
        impl UpdateLeadingTrivia for $node {
            fn update_leading_trivia(&self, leading_trivia: FormatTriviaType) -> Self {
                $body(&self, leading_trivia)
            }
        }
    };
}

macro_rules! define_update_trailing_trivia {
    ($node:ident, |$self:ident, $trailing_trivia:ident| $body:expr) => {
        define_update_trailing_trivia! {$node, |$self:&$node, $trailing_trivia: FormatTriviaType| $body}
    };
    ($node:ident, $body:expr) => {
        impl UpdateTrailingTrivia for $node {
            fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType) -> Self {
                $body(&self, trailing_trivia)
            }
        }
    };
}

macro_rules! binop_trivia {
    ($enum:ident, $value:ident, $leading_trivia:ident, $trailing_trivia:ident, { $($operator:ident,)+ }) => {
        match $value {
            $(
                $enum::$operator(token) => $enum::$operator(token.update_trivia($leading_trivia, $trailing_trivia)),
            )+
            other => panic!("unknown node {:?}", other),
        }
    };
}

define_update_trivia!(BinOp, |this, leading, trailing| {
    binop_trivia!(BinOp, this, leading, trailing, {
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
});

define_update_trivia!(ContainedSpan, |this, leading, trailing| {
    let (start_token, end_token) = this.tokens();
    ContainedSpan::new(
        start_token.update_leading_trivia(leading),
        end_token.update_trailing_trivia(trailing),
    )
});

define_update_trivia!(Call, |this, leading, trailing| {
    match this {
        Call::AnonymousCall(function_args) => {
            Call::AnonymousCall(function_args.update_trivia(leading, trailing))
        }
        Call::MethodCall(method_call) => {
            Call::MethodCall(method_call.update_trivia(leading, trailing))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_leading_trivia!(Expression, |this, leading| {
    match this {
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: contained.update_leading_trivia(leading),
            expression: expression.to_owned(),
        },
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop: unop.update_leading_trivia(leading),
            expression: expression.to_owned(),
        },
        Expression::BinaryOperator { lhs, binop, rhs } => Expression::BinaryOperator {
            lhs: Box::new(lhs.update_leading_trivia(leading)),
            binop: binop.to_owned(),
            rhs: rhs.to_owned(),
        },
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => Expression::Value {
            value: Box::new(value.update_leading_trivia(leading)),
            #[cfg(feature = "luau")]
            type_assertion: type_assertion.to_owned(),
        },
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trailing_trivia!(Expression, |this, trailing| {
    match this {
        Expression::Value {
            value,
            #[cfg(feature = "luau")]
            type_assertion,
        } => {
            #[cfg(feature = "luau")]
            if let Some(as_assertion) = type_assertion {
                return Expression::Value {
                    value: value.to_owned(),
                    type_assertion: Some(as_assertion.update_trailing_trivia(trailing)),
                };
            }

            Expression::Value {
                value: Box::new(value.update_trailing_trivia(trailing)),
                #[cfg(feature = "luau")]
                type_assertion: type_assertion.to_owned(),
            }
        }

        // Add trailing trivia to the end of parentheses
        Expression::Parentheses {
            contained,
            expression,
        } => Expression::Parentheses {
            contained: contained.update_trailing_trivia(trailing),
            expression: expression.to_owned(),
        },

        // Keep recursing down until we find an Expression::Value
        Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
            unop: unop.to_owned(),
            expression: Box::new(expression.update_trailing_trivia(trailing)),
        },

        Expression::BinaryOperator { lhs, binop, rhs } => Expression::BinaryOperator {
            lhs: lhs.to_owned(),
            binop: binop.to_owned(),
            rhs: Box::new(rhs.update_trailing_trivia(trailing)),
        },
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trivia!(FunctionArgs, |this, leading, trailing| {
    match this {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => FunctionArgs::Parentheses {
            parentheses: parentheses.update_trivia(leading, trailing),
            arguments: arguments.to_owned(),
        },
        FunctionArgs::String(token_reference) => {
            FunctionArgs::String(token_reference.update_trivia(leading, trailing))
        }
        FunctionArgs::TableConstructor(table_constructor) => {
            FunctionArgs::TableConstructor(table_constructor.update_trivia(leading, trailing))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trailing_trivia!(FunctionBody, |this, trailing| {
    this.to_owned()
        .with_end_token(this.end_token().update_trailing_trivia(trailing))
});

define_update_trivia!(FunctionCall, |this, leading, trailing| {
    let prefix = match leading {
        FormatTriviaType::NoChange => this.prefix().to_owned(),
        _ => this.prefix().update_leading_trivia(leading),
    };

    let mut suffixes: Vec<Suffix> = this.suffixes().map(|x| x.to_owned()).collect();
    match trailing {
        FormatTriviaType::NoChange => (),
        _ => {
            if let Some(suffix) = suffixes.pop() {
                suffixes.push(suffix.update_trailing_trivia(trailing))
            }
        }
    };

    this.to_owned().with_prefix(prefix).with_suffixes(suffixes)
});

define_update_trivia!(FunctionName, |this, leading, trailing| {
    if let Some(method_name) = this.method_name() {
        let names = this.names().update_leading_trivia(leading);
        let method_name = method_name.update_trailing_trivia(trailing);
        this.to_owned()
            .with_names(names)
            .with_method(Some((this.method_colon().unwrap().to_owned(), method_name)))
    } else {
        let names = this
            .names()
            .update_leading_trivia(leading)
            .update_trailing_trivia(trailing);
        this.to_owned().with_names(names)
    }
});

define_update_trivia!(Index, |this, leading, trailing| {
    match this {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: brackets.update_trivia(leading, trailing),
            expression: expression.to_owned(),
        },
        Index::Dot { dot, name } => Index::Dot {
            dot: dot.update_leading_trivia(leading),
            name: name.update_trailing_trivia(trailing),
        },
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trivia!(MethodCall, |this, leading, trailing| {
    this.to_owned()
        .with_colon_token(this.colon_token().update_leading_trivia(leading))
        .with_args(this.args().update_trailing_trivia(trailing))
});

define_update_trivia!(Parameter, |this, leading, trailing| {
    match this {
        Parameter::Ellipse(token) => Parameter::Ellipse(token.update_trivia(leading, trailing)),
        Parameter::Name(token) => Parameter::Name(token.update_trivia(leading, trailing)),
        other => panic!("unknown node {:?}", other),
    }
});

define_update_leading_trivia!(Prefix, |this, leading| {
    match this {
        Prefix::Name(token_reference) => {
            Prefix::Name(token_reference.update_leading_trivia(leading))
        }
        Prefix::Expression(expression) => {
            Prefix::Expression(expression.update_leading_trivia(leading))
        }
        other => panic!("unknown node {:?}", other),
    }
});

impl<T> UpdateLeadingTrivia for Punctuated<T>
where
    T: UpdateLeadingTrivia + Clone,
{
    fn update_leading_trivia(&self, leading: FormatTriviaType) -> Self {
        let mut punctuated = Punctuated::new();
        let mut pairs = self.to_owned().into_pairs();

        // Retrieve first item and add leading trivia
        if let Some(first_pair) = pairs.next() {
            let updated_pair = first_pair.map(|value| value.update_leading_trivia(leading));
            punctuated.push(updated_pair);
        };

        // Add back the rest of the values
        for pair in pairs {
            punctuated.push(full_moon::ast::punctuated::Pair::new(
                pair.value().clone(),
                pair.punctuation().map(|x| x.to_owned()),
            ))
        }

        punctuated
    }
}

impl<T> UpdateTrailingTrivia for Punctuated<T>
where
    T: UpdateTrailingTrivia + Clone,
{
    fn update_trailing_trivia(&self, trailing: FormatTriviaType) -> Self {
        let mut punctuated = self.to_owned();

        // Add any trailing trivia to the end of the punctuated list
        if let Some(pair) = punctuated.pop() {
            let pair = pair.map(|value| value.update_trailing_trivia(trailing));
            punctuated.push(pair);
        }

        punctuated
    }
}

define_update_trivia!(Suffix, |this, leading, trailing| {
    match this {
        Suffix::Call(call) => Suffix::Call(call.update_trivia(leading, trailing)),
        Suffix::Index(index) => Suffix::Index(index.update_trivia(leading, trailing)),
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trivia!(TableConstructor, |this, leading, trailing| {
    this.to_owned()
        .with_braces(this.braces().update_trivia(leading, trailing))
});

define_update_leading_trivia!(UnOp, |this, leading| {
    match this {
        UnOp::Hash(token_reference) => UnOp::Hash(token_reference.update_leading_trivia(leading)),
        UnOp::Minus(token_reference) => UnOp::Minus(token_reference.update_leading_trivia(leading)),
        UnOp::Not(token_reference) => UnOp::Not(token_reference.update_leading_trivia(leading)),
        other => panic!("unknown node {:?}", other),
    }
});

define_update_leading_trivia!(Value, |this, leading| {
    match this {
        Value::Function((token, function_body)) => Value::Function((
            token.update_leading_trivia(leading),
            function_body.to_owned(),
        )),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(function_call.update_leading_trivia(leading))
        }
        Value::Number(token_reference) => {
            Value::Number(token_reference.update_leading_trivia(leading))
        }
        Value::ParenthesesExpression(expression) => {
            Value::ParenthesesExpression(expression.update_leading_trivia(leading))
        }
        Value::String(token_reference) => {
            Value::String(token_reference.update_leading_trivia(leading))
        }
        Value::Symbol(token_reference) => {
            Value::Symbol(token_reference.update_leading_trivia(leading))
        }
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(table_constructor.update_leading_trivia(leading))
        }
        Value::Var(var) => Value::Var(var.update_leading_trivia(leading)),
        #[cfg(feature = "luau")]
        Value::IfExpression(if_expression) => {
            Value::IfExpression(if_expression.update_leading_trivia(leading))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trailing_trivia!(Value, |this, trailing| {
    match this {
        Value::Function((token, function_body)) => Value::Function((
            token.to_owned(),
            function_body.update_trailing_trivia(trailing),
        )),
        Value::FunctionCall(function_call) => {
            Value::FunctionCall(function_call.update_trailing_trivia(trailing))
        }
        Value::Number(token_reference) => {
            Value::Number(token_reference.update_trailing_trivia(trailing))
        }
        Value::ParenthesesExpression(expression) => {
            Value::ParenthesesExpression(expression.update_trailing_trivia(trailing))
        }
        Value::String(token_reference) => {
            Value::String(token_reference.update_trailing_trivia(trailing))
        }
        Value::Symbol(token_reference) => {
            Value::Symbol(token_reference.update_trailing_trivia(trailing))
        }
        Value::TableConstructor(table_constructor) => {
            Value::TableConstructor(table_constructor.update_trailing_trivia(trailing))
        }
        Value::Var(var) => Value::Var(var.update_trailing_trivia(trailing)),
        #[cfg(feature = "luau")]
        Value::IfExpression(if_expression) => {
            Value::IfExpression(if_expression.update_trailing_trivia(trailing))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_leading_trivia!(Var, |this, leading| {
    match this {
        Var::Name(token_reference) => Var::Name(token_reference.update_leading_trivia(leading)),
        Var::Expression(var_expresion) => {
            Var::Expression(var_expresion.update_leading_trivia(leading))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trailing_trivia!(Var, |this, trailing| {
    match this {
        Var::Name(token_reference) => Var::Name(token_reference.update_trailing_trivia(trailing)),
        Var::Expression(var_expression) => {
            Var::Expression(var_expression.update_trailing_trivia(trailing))
        }
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trivia!(VarExpression, |this, leading, trailing| {
    let prefix = match leading {
        FormatTriviaType::NoChange => this.prefix().to_owned(),
        _ => this.prefix().update_leading_trivia(leading),
    };

    let mut suffixes: Vec<Suffix> = this.suffixes().map(|x| x.to_owned()).collect();
    match trailing {
        FormatTriviaType::NoChange => (),
        _ => {
            if let Some(suffix) = suffixes.pop() {
                suffixes.push(suffix.update_trailing_trivia(trailing))
            }
        }
    };

    this.to_owned().with_prefix(prefix).with_suffixes(suffixes)
});

#[cfg(feature = "luau")]
define_update_trivia!(TypeInfo, |this, leading, trailing| {
    match this {
        TypeInfo::Array { braces, type_info } => TypeInfo::Array {
            braces: braces.update_trivia(leading, trailing),
            type_info: type_info.to_owned(),
        },
        TypeInfo::Basic(token_reference) => {
            TypeInfo::Basic(token_reference.update_trivia(leading, trailing))
        }
        TypeInfo::Callback {
            generics,
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let (generics, parentheses) = if let Some(generics) = generics {
                (
                    Some(generics.update_leading_trivia(leading)),
                    parentheses.to_owned(),
                )
            } else {
                (
                    generics.to_owned(),
                    parentheses.update_leading_trivia(leading),
                )
            };
            TypeInfo::Callback {
                generics,
                parentheses,
                arguments: arguments.to_owned(),
                arrow: arrow.to_owned(),
                return_type: Box::new(return_type.update_trailing_trivia(trailing)),
            }
        }
        TypeInfo::Generic {
            base,
            arrows,
            generics,
        } => TypeInfo::Generic {
            base: base.update_leading_trivia(leading),
            arrows: arrows.update_trailing_trivia(trailing),
            generics: generics.to_owned(),
        },

        TypeInfo::GenericVariadic { name, ellipse } => TypeInfo::GenericVariadic {
            name: name.update_leading_trivia(leading),
            ellipse: ellipse.update_trailing_trivia(trailing),
        },

        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => TypeInfo::Intersection {
            left: Box::new(left.update_leading_trivia(leading)),
            ampersand: ampersand.to_owned(),
            right: Box::new(right.update_trailing_trivia(trailing)),
        },

        TypeInfo::Module {
            module,
            punctuation,
            type_info,
        } => TypeInfo::Module {
            module: module.update_leading_trivia(leading),
            punctuation: punctuation.to_owned(),
            type_info: Box::new(type_info.update_trailing_trivia(trailing)),
        },

        TypeInfo::Optional {
            base,
            question_mark,
        } => TypeInfo::Optional {
            base: Box::new(base.update_leading_trivia(leading)),
            question_mark: question_mark.update_trailing_trivia(trailing),
        },

        TypeInfo::Table { braces, fields } => TypeInfo::Table {
            braces: braces.update_trivia(leading, trailing),
            fields: fields.to_owned(),
        },

        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => TypeInfo::Typeof {
            typeof_token: typeof_token.update_leading_trivia(leading),
            parentheses: parentheses.update_trailing_trivia(trailing),
            inner: inner.to_owned(),
        },

        TypeInfo::Tuple { parentheses, types } => TypeInfo::Tuple {
            parentheses: parentheses.update_trivia(leading, trailing),
            types: types.to_owned(),
        },

        TypeInfo::Union { left, pipe, right } => TypeInfo::Union {
            left: Box::new(left.update_leading_trivia(leading)),
            pipe: pipe.to_owned(),
            right: Box::new(right.update_trailing_trivia(trailing)),
        },

        TypeInfo::Variadic { ellipse, type_info } => TypeInfo::Variadic {
            ellipse: ellipse.update_leading_trivia(leading),
            type_info: Box::new(type_info.update_trailing_trivia(trailing)),
        },

        other => panic!("unknown node {:?}", other),
    }
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(IndexedTypeInfo, |this, trailing| {
    match this {
        IndexedTypeInfo::Basic(token_reference) => {
            IndexedTypeInfo::Basic(token_reference.update_trailing_trivia(trailing))
        }
        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => IndexedTypeInfo::Generic {
            base: base.to_owned(),
            arrows: arrows.update_trailing_trivia(trailing),
            generics: generics.to_owned(),
        },

        other => panic!("unknown node {:?}", other),
    }
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(TypeArgument, |this, leading| {
    if let Some((name, colon)) = this.name() {
        this.to_owned().with_name(Some((
            name.update_leading_trivia(leading),
            colon.to_owned(),
        )))
    } else {
        this.to_owned()
            .with_type_info(this.type_info().update_leading_trivia(leading))
    }
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(TypeAssertion, |this, trailing| {
    this.to_owned()
        .with_cast_to(this.cast_to().update_trailing_trivia(trailing))
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(TypeField, |this, leading| {
    this.to_owned()
        .with_key(this.key().update_leading_trivia(leading))
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(TypeField, |this, trailing| {
    this.to_owned()
        .with_value(this.value().update_trailing_trivia(trailing))
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(TypeFieldKey, |this, leading| {
    match this {
        TypeFieldKey::Name(token) => TypeFieldKey::Name(token.update_leading_trivia(leading)),
        TypeFieldKey::IndexSignature { brackets, inner } => TypeFieldKey::IndexSignature {
            brackets: brackets.update_leading_trivia(leading),
            inner: inner.to_owned(),
        },
        other => panic!("unknown node {:?}", other),
    }
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(TypeSpecifier, |this, trailing| {
    this.to_owned()
        .with_type_info(this.type_info().update_trailing_trivia(trailing))
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(GenericDeclaration, |this, leading| {
    this.to_owned()
        .with_arrows(this.arrows().update_leading_trivia(leading))
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(GenericDeclarationParameter, |this, leading| {
    match this {
        GenericDeclarationParameter::Name(token) => {
            GenericDeclarationParameter::Name(token.update_leading_trivia(leading))
        }
        GenericDeclarationParameter::Variadic { name, ellipse } => {
            GenericDeclarationParameter::Variadic {
                name: name.update_leading_trivia(leading),
                ellipse: ellipse.to_owned(),
            }
        }
        other => panic!("unknown node {:?}", other),
    }
});

#[cfg(feature = "luau")]
define_update_leading_trivia!(IfExpression, |this, leading| {
    this.to_owned()
        .with_if_token(this.if_token().update_leading_trivia(leading))
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(IfExpression, |this, trailing| {
    this.to_owned()
        .with_else(this.else_expression().update_trailing_trivia(trailing))
});
