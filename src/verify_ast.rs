use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Ast, Block, Expression, FunctionArgs, TableConstructor, Value,
    },
    node::Node,
    tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType},
    visitors::VisitorMut,
};

pub struct AstVerifier {}

impl AstVerifier {
    pub fn new() -> Self {
        Self {}
    }

    /// Compares two ASTs to see if they are similar.
    /// Firstly massages the AST to ensure we don't trigger false positives
    pub fn compare(&mut self, input_ast: Ast, reparsed_output: Ast) -> bool {
        let massaged_input = self.visit_ast(input_ast);
        let massaged_output = self.visit_ast(reparsed_output);

        massaged_input.similar(&massaged_output)
    }
}

fn remove_parentheses(expression: Expression) -> Expression {
    match expression {
        Expression::Parentheses { expression, .. } => *expression,
        Expression::Value { value, .. } => Expression::Value {
            value: match *value {
                Value::ParenthesesExpression(expression) => return remove_parentheses(expression),
                _ => value,
            },
            #[cfg(feature = "luau")]
            type_assertion: None,
        },
        _ => expression,
    }
}

// Massages the AST so that structures we have changed in Nodes remain constant.
// Note, the massaged AST may not actually be valid syntax if we print it back out, but we have already checked
// the validity of the output, so any invalid syntax output would already have been flagged.
// The AST massager's primary job is so that changes we have explicitly done are not flagged when checking AST similarity.
impl VisitorMut for AstVerifier {
    fn visit_block(&mut self, node: Block) -> Block {
        // We remove unnecessary semicolons at the end of statements.
        // We will remove all semicolons that are still present in statements.
        let stmts = node
            .stmts_with_semicolon()
            .map(|(stmt, _semicolon)| (stmt.to_owned(), None))
            .collect();
        let last_stmt = node
            .last_stmt_with_semicolon()
            .map(|(last_stmt, _semicolon)| (last_stmt.to_owned(), None));

        node.with_stmts(stmts).with_last_stmt(last_stmt)
    }

    fn visit_table_constructor(&mut self, node: TableConstructor) -> TableConstructor {
        // We change semicolon field separators to commas
        // We will replace all field separators with commas, and include a trailing comma

        let current_fields = node.fields();
        let mut fields = Punctuated::new();
        for field in current_fields.to_owned().into_pairs() {
            let pair = match field {
                Pair::Punctuated(field, _) | Pair::End(field) => {
                    Pair::Punctuated(field, TokenReference::symbol(",").unwrap())
                }
            };
            fields.push(pair)
        }

        node.with_fields(fields)
    }

    fn visit_function_args(
        &mut self,
        node: full_moon::ast::FunctionArgs,
    ) -> full_moon::ast::FunctionArgs {
        // We change the parentheses around function arguments
        // We will normalise all function args so that they are wrapped around in parentheses
        match node {
            FunctionArgs::String(string) => FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    TokenReference::symbol("(").unwrap(),
                    TokenReference::symbol(")").unwrap(),
                ),
                arguments: std::iter::once(Pair::End(Expression::Value {
                    value: Box::new(Value::String(string)),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                }))
                .collect(),
            },
            FunctionArgs::TableConstructor(table) => FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    TokenReference::symbol("(").unwrap(),
                    TokenReference::symbol(")").unwrap(),
                ),
                arguments: std::iter::once(Pair::End(Expression::Value {
                    value: Box::new(Value::TableConstructor(table)),
                    #[cfg(feature = "luau")]
                    type_assertion: None,
                }))
                .collect(),
            },
            _ => node,
        }
    }

    fn visit_expression(&mut self, node: Expression) -> Expression {
        // There are places where we remove parentheses.
        // TODO: is this too eager? will we lose out in finding differences by doing this?
        remove_parentheses(node)
    }

    fn visit_number(&mut self, token: Token) -> Token {
        // TODO:
        // We change the formatting of number literals
        // We will normalise all numbers by running `str::parse(number).to_string()` and replacing the Token with this output.
        // This will help highlight any differences, as it would lead to a different parsed output

        // let token_type = match token.token_type() {
        //     TokenType::Number { text } => {
        //         // Luau: cleanse number of any digit separators
        //         #[cfg(feature = "luau")]
        //         let text = text.replace("_", "");

        //         let number = match i32::from_str_radix(text.as_str(), 10) {
        //             Ok(num) => num,
        //             Err(_) => match i32::from_str_radix(&text.as_str()[2..], 16) {
        //                 Ok(num) => num,
        //                 Err(_) => match i32::from_str_radix(&text.as_str()[2..], 2) {
        //                     Ok(num) => num,
        //                     Err(_) => unreachable!(),
        //                 },
        //             },
        //         };

        //         TokenType::Number {
        //             text: number.to_string().into(),
        //         }
        //     }
        //     _ => unreachable!(),
        // };

        // Token::new(token_type)
        token
    }

    fn visit_string_literal(&mut self, token: Token) -> Token {
        // We change the string quotes of our progrem.
        // Convert all string literals to brackets quotes, and remove any quote escapes.
        let token_type = match token.token_type() {
            TokenType::StringLiteral {
                literal,
                multi_line,
                ..
            } => TokenType::StringLiteral {
                literal: literal
                    .to_owned()
                    .replace("\\\"", "\"")
                    .replace("\\'", "'")
                    .into(),
                multi_line: multi_line.to_owned(),
                quote_type: StringLiteralQuoteType::Brackets,
            },
            _ => unreachable!(),
        };

        Token::new(token_type)
    }
}
