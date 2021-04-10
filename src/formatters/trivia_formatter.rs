use crate::{
    formatters::{trivia_util, CodeFormatter},
    IndentType,
};
#[cfg(feature = "luau")]
use full_moon::ast::types::{IndexedTypeInfo, TypeAssertion, TypeInfo, TypeSpecifier};
use full_moon::ast::{
    punctuated::Punctuated, span::ContainedSpan, BinOp, Call, Expression, FunctionArgs,
    FunctionBody, FunctionCall, Index, MethodCall, Parameter, Prefix, Suffix, TableConstructor,
    UnOp, Value, Var, VarExpression,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};

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

/// Strips all leading and trailing trivia from a specific node.
/// This is useful if we need to use the node to calculate sizing, whilst we do not want trivia included
pub fn strip_trivia<'ast, T>(item: &T) -> T
where
    T: UpdateLeadingTrivia<'ast> + UpdateTrailingTrivia<'ast>,
{
    item.update_leading_trivia(FormatTriviaType::Replace(vec![]))
        .update_trailing_trivia(FormatTriviaType::Replace(vec![]))
}

impl CodeFormatter {
    /// Creates indent trivia without including `self.indent_level`.
    /// You should pass the exact amount of indent you require to this function
    pub fn create_plain_indent_trivia<'ast>(&self, indent_level: usize) -> Token<'ast> {
        match self.config.indent_type {
            IndentType::Tabs => Token::new(TokenType::tabs(indent_level)),
            IndentType::Spaces => {
                Token::new(TokenType::spaces(indent_level * self.config.indent_width))
            }
        }
    }

    /// Pushes a BinOp onto a newline, and indent its depending on indent_level. Moves trailing comments to before the BinOp.
    /// Does not hang if the BinOp is a relational operator.
    fn hang_binop<'ast>(&self, binop: BinOp<'ast>, indent_level: usize) -> BinOp<'ast> {
        match binop {
            // Don't add the trivia if the binop is binding
            BinOp::GreaterThan(_)
            | BinOp::GreaterThanEqual(_)
            | BinOp::LessThan(_)
            | BinOp::LessThanEqual(_)
            | BinOp::TildeEqual(_)
            | BinOp::TwoEqual(_) => {
                // Return original binop
                binop
            }
            _ => {
                // If there are any comments trailing the BinOp, we need to move them to before the BinOp
                let mut trailing_comments = trivia_util::binop_trailing_comments(&binop);
                // Create a newline just before the BinOp, and preserve the indentation
                trailing_comments.push(self.create_newline_trivia());
                trailing_comments.push(self.create_plain_indent_trivia(indent_level));

                binop.update_trivia(
                    FormatTriviaType::Replace(trailing_comments),
                    FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]),
                )
            }
        }
    }

    /// Finds the length of the expression which matches the precedence level of the provided binop
    fn binop_expression_length<'ast>(
        &self,
        expression: &Expression<'ast>,
        top_binop: &BinOp<'ast>,
    ) -> usize {
        match expression {
            Expression::BinaryOperator { lhs, binop, rhs } => {
                if binop.precedence() == top_binop.precedence()
                    && binop.is_right_associative() == top_binop.is_right_associative()
                {
                    if binop.is_right_associative() {
                        self.binop_expression_length(rhs, top_binop)
                            + binop.to_string().len()
                            + lhs.to_string().len()
                    } else {
                        self.binop_expression_length(lhs, top_binop)
                            + binop.to_string().len()
                            + rhs.to_string().len()
                    }
                } else {
                    0
                }
            }
            _ => expression.to_string().len(),
        }
    }

    fn hang_binop_expression<'ast>(
        &self,
        expression: Expression<'ast>,
        top_binop: BinOp<'ast>,
        indent_level: usize,
    ) -> Expression<'ast> {
        let full_expression = expression.to_owned();

        match expression {
            Expression::BinaryOperator { lhs, binop, rhs } => {
                // Keep grouping together all operators with the same precedence level as the main BinOp
                // They should also have the same associativity
                let same_op_level = binop.precedence() == top_binop.precedence()
                    && binop.is_right_associative() == top_binop.is_right_associative();
                let is_right_associative = top_binop.is_right_associative();

                let indent_level = if same_op_level {
                    indent_level
                } else {
                    indent_level + 1
                };

                let side_to_use = if is_right_associative {
                    rhs.to_owned()
                } else {
                    lhs.to_owned()
                };

                let over_column_width = indent_level * self.config.indent_width
                    + self.binop_expression_length(&full_expression, &binop)
                    > self.config.column_width;

                let (binop, updated_side) = if same_op_level || over_column_width {
                    let op = self.hang_binop(binop.to_owned(), indent_level);

                    let side = self.hang_binop_expression(
                        *side_to_use,
                        if same_op_level { top_binop } else { binop },
                        indent_level,
                    );

                    (op, side)
                } else {
                    (binop, *side_to_use)
                };

                if is_right_associative {
                    Expression::BinaryOperator {
                        lhs,
                        binop,
                        rhs: Box::new(updated_side),
                    }
                } else {
                    Expression::BinaryOperator {
                        lhs: Box::new(updated_side),
                        binop,
                        rhs,
                    }
                }
            }
            // Base case: no more binary operators - just return to normal splitting
            _ => self.expression_split_binop(expression, indent_level),
        }
    }

    fn expression_split_binop<'ast>(
        &self,
        expression: Expression<'ast>,
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
                    < self.config.column_width
                {
                    // The expression inside the parentheses is small, we do not need to break it down further
                    return Expression::Parentheses {
                        contained,
                        expression,
                    };
                }

                // Modify the parentheses to hang the expression
                let (start_token, end_token) = contained.tokens();
                // Create a newline after the start brace and before the end brace
                // Also, indent enough for the first expression in the start brace
                let contained = ContainedSpan::new(
                    start_token.update_trailing_trivia(FormatTriviaType::Append(vec![
                        self.create_newline_trivia(),
                        self.create_plain_indent_trivia(indent_increase + 1),
                    ])),
                    end_token.update_leading_trivia(FormatTriviaType::Append(vec![
                        self.create_newline_trivia(),
                        self.create_plain_indent_trivia(indent_increase),
                    ])),
                );

                Expression::Parentheses {
                    contained,
                    expression: Box::new(self.expression_split_binop(
                        *expression,
                        indent_increase + 1, // Apply indent increase
                    )),
                }
            }
            Expression::UnaryOperator { unop, expression } => Expression::UnaryOperator {
                unop,
                expression: Box::new(self.expression_split_binop(*expression, indent_increase)),
            },
            Expression::BinaryOperator { lhs, binop, rhs } => {
                let lhs =
                    Box::new(self.hang_binop_expression(*lhs, binop.to_owned(), indent_increase));
                let rhs =
                    Box::new(self.hang_binop_expression(*rhs, binop.to_owned(), indent_increase));
                let binop = self.hang_binop(binop, indent_increase);

                Expression::BinaryOperator { lhs, binop, rhs }
            }

            Expression::Value {
                value,
                #[cfg(feature = "luau")]
                type_assertion,
            } => Expression::Value {
                value: match *value {
                    Value::ParenthesesExpression(expression) => {
                        Box::new(Value::ParenthesesExpression(
                            self.expression_split_binop(expression, indent_increase),
                        ))
                    }
                    _ => value,
                },
                #[cfg(feature = "luau")]
                type_assertion,
            },

            // Can't hang anything else, so just return the original expression
            _ => expression,
        }
    }

    pub fn hang_expression_no_trailing_newline<'ast>(
        &self,
        expression: Expression<'ast>,
        additional_indent_level: Option<usize>,
        hang_level: Option<usize>,
    ) -> Expression<'ast> {
        let additional_indent_level =
            additional_indent_level.unwrap_or(0) + hang_level.unwrap_or(0);
        let hang_level = self.indent_level + additional_indent_level;

        self.expression_split_binop(expression, hang_level)
    }

    pub fn hang_expression<'ast>(
        &self,
        expression: Expression<'ast>,
        additional_indent_level: Option<usize>,
        hang_level: Option<usize>,
    ) -> Expression<'ast> {
        self.hang_expression_no_trailing_newline(expression, additional_indent_level, hang_level)
            .update_trailing_trivia(FormatTriviaType::Append(vec![self.create_newline_trivia()]))
    }
}

pub trait UpdateLeadingTrivia<'ast> {
    fn update_leading_trivia(&self, leading_trivia: FormatTriviaType<'ast>) -> Self;
}

pub trait UpdateTrailingTrivia<'ast> {
    fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType<'ast>) -> Self;
}

pub trait UpdateTrivia<'ast> {
    fn update_trivia(
        &self,
        leading_trivia: FormatTriviaType<'ast>,
        trailing_trivia: FormatTriviaType<'ast>,
    ) -> Self;
}

impl<'ast, T> UpdateLeadingTrivia<'ast> for T
where
    T: UpdateTrivia<'ast>,
{
    fn update_leading_trivia(&self, leading_trivia: FormatTriviaType<'ast>) -> Self
    where
        Self: std::marker::Sized,
    {
        self.update_trivia(leading_trivia, FormatTriviaType::NoChange)
    }
}

impl<'ast, T> UpdateTrailingTrivia<'ast> for T
where
    T: UpdateTrivia<'ast>,
{
    fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType<'ast>) -> Self
    where
        Self: std::marker::Sized,
    {
        self.update_trivia(FormatTriviaType::NoChange, trailing_trivia)
    }
}

impl<'ast> UpdateTrivia<'ast> for TokenReference<'ast> {
    fn update_trivia(
        &self,
        leading_trivia: FormatTriviaType<'ast>,
        trailing_trivia: FormatTriviaType<'ast>,
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
        define_update_trivia! {$node, |$self:&$node<'ast>, $leading_trivia: FormatTriviaType<'ast>, $trailing_trivia: FormatTriviaType<'ast>| $body}
    };
    ($node:ident, $body:expr) => {
        impl<'ast> UpdateTrivia<'ast> for $node<'ast> {
            fn update_trivia(&self, leading_trivia: FormatTriviaType<'ast>, trailing_trivia: FormatTriviaType<'ast>) -> Self {
                $body(&self, leading_trivia, trailing_trivia)
            }
        }
    };
}

macro_rules! define_update_leading_trivia {
    ($node:ident, |$self:ident, $leading_trivia:ident| $body:expr) => {
        define_update_leading_trivia! {$node, |$self:&$node<'ast>, $leading_trivia: FormatTriviaType<'ast>| $body}
    };
    ($node:ident, $body:expr) => {
        impl<'ast> UpdateLeadingTrivia<'ast> for $node<'ast> {
            fn update_leading_trivia(&self, leading_trivia: FormatTriviaType<'ast>) -> Self {
                $body(&self, leading_trivia)
            }
        }
    };
}

macro_rules! define_update_trailing_trivia {
    ($node:ident, |$self:ident, $trailing_trivia:ident| $body:expr) => {
        define_update_trailing_trivia! {$node, |$self:&$node<'ast>, $trailing_trivia: FormatTriviaType<'ast>| $body}
    };
    ($node:ident, $body:expr) => {
        impl<'ast> UpdateTrailingTrivia<'ast> for $node<'ast> {
            fn update_trailing_trivia(&self, trailing_trivia: FormatTriviaType<'ast>) -> Self {
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

define_update_trailing_trivia!(Call, |this, trailing| {
    match this {
        Call::AnonymousCall(function_args) => {
            Call::AnonymousCall(function_args.update_trailing_trivia(trailing))
        }
        Call::MethodCall(method_call) => {
            Call::MethodCall(method_call.update_trailing_trivia(trailing))
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

define_update_trailing_trivia!(FunctionArgs, |this, trailing| {
    match this {
        FunctionArgs::Parentheses {
            parentheses,
            arguments,
        } => FunctionArgs::Parentheses {
            parentheses: parentheses.update_trailing_trivia(trailing),
            arguments: arguments.to_owned(),
        },
        FunctionArgs::String(token_reference) => {
            FunctionArgs::String(token_reference.update_trailing_trivia(trailing))
        }
        FunctionArgs::TableConstructor(table_constructor) => {
            FunctionArgs::TableConstructor(table_constructor.update_trailing_trivia(trailing))
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

    let mut suffixes: Vec<Suffix<'ast>> = this.suffixes().map(|x| x.to_owned()).collect();
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

define_update_trailing_trivia!(Index, |this, trailing| {
    match this {
        Index::Brackets {
            brackets,
            expression,
        } => Index::Brackets {
            brackets: brackets.update_trailing_trivia(trailing),
            expression: expression.to_owned(),
        },
        Index::Dot { dot, name } => Index::Dot {
            dot: dot.to_owned(),
            name: name.update_trailing_trivia(trailing),
        },
        other => panic!("unknown node {:?}", other),
    }
});

define_update_trailing_trivia!(MethodCall, |this, trailing| {
    this.to_owned()
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

impl<'ast, T> UpdateLeadingTrivia<'ast> for Punctuated<'ast, T>
where
    T: UpdateLeadingTrivia<'ast> + Clone,
{
    fn update_leading_trivia(&self, leading: FormatTriviaType<'ast>) -> Self {
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

impl<'ast, T> UpdateTrailingTrivia<'ast> for Punctuated<'ast, T>
where
    T: UpdateTrailingTrivia<'ast> + Clone,
{
    fn update_trailing_trivia(&self, trailing: FormatTriviaType<'ast>) -> Self {
        let mut punctuated = self.to_owned();

        // Add any trailing trivia to the end of the punctuated list
        if let Some(pair) = punctuated.pop() {
            let pair = pair.map(|value| value.update_trailing_trivia(trailing));
            punctuated.push(pair);
        }

        punctuated
    }
}

define_update_trailing_trivia!(Suffix, |this, trailing| {
    match this {
        Suffix::Call(call) => Suffix::Call(call.update_trailing_trivia(trailing)),
        Suffix::Index(index) => Suffix::Index(index.update_trailing_trivia(trailing)),
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

    let mut suffixes: Vec<Suffix<'ast>> = this.suffixes().map(|x| x.to_owned()).collect();
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
define_update_trailing_trivia!(TypeInfo, |this, trailing| {
    match this {
        TypeInfo::Array { braces, type_info } => TypeInfo::Array {
            braces: braces.update_trailing_trivia(trailing),
            type_info: type_info.to_owned(),
        },
        TypeInfo::Basic(token_reference) => {
            TypeInfo::Basic(token_reference.update_trailing_trivia(trailing))
        }
        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => TypeInfo::Callback {
            parentheses: parentheses.to_owned(),
            arguments: arguments.to_owned(),
            arrow: arrow.to_owned(),
            return_type: Box::new(return_type.update_trailing_trivia(trailing)),
        },
        TypeInfo::Generic {
            base,
            arrows,
            generics,
        } => TypeInfo::Generic {
            base: base.to_owned(),
            arrows: arrows.update_trailing_trivia(trailing),
            generics: generics.to_owned(),
        },

        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => TypeInfo::Intersection {
            left: left.to_owned(),
            ampersand: ampersand.to_owned(),
            right: Box::new(right.update_trailing_trivia(trailing)),
        },

        TypeInfo::Module {
            module,
            punctuation,
            type_info,
        } => TypeInfo::Module {
            module: module.to_owned(),
            punctuation: punctuation.to_owned(),
            type_info: Box::new(type_info.update_trailing_trivia(trailing)),
        },

        TypeInfo::Optional {
            base,
            question_mark,
        } => TypeInfo::Optional {
            base: base.to_owned(),
            question_mark: question_mark.update_trailing_trivia(trailing),
        },

        TypeInfo::Table { braces, fields } => TypeInfo::Table {
            braces: braces.update_trailing_trivia(trailing),
            fields: fields.to_owned(),
        },

        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => TypeInfo::Typeof {
            typeof_token: typeof_token.to_owned(),
            parentheses: parentheses.update_trailing_trivia(trailing),
            inner: inner.to_owned(),
        },

        TypeInfo::Tuple { parentheses, types } => TypeInfo::Tuple {
            parentheses: parentheses.update_trailing_trivia(trailing),
            types: types.to_owned(),
        },

        TypeInfo::Union { left, pipe, right } => TypeInfo::Union {
            left: left.to_owned(),
            pipe: pipe.to_owned(),
            right: Box::new(right.update_trailing_trivia(trailing)),
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
define_update_trailing_trivia!(TypeAssertion, |this, trailing| {
    this.to_owned()
        .with_cast_to(this.cast_to().update_trailing_trivia(trailing))
});

#[cfg(feature = "luau")]
define_update_trailing_trivia!(TypeSpecifier, |this, trailing| {
    this.to_owned()
        .with_type_info(this.type_info().update_trailing_trivia(trailing))
});
