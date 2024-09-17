use full_moon::{
    ast::{
        punctuated::{Pair, Punctuated},
        span::ContainedSpan,
        Ast, Block, Expression, FunctionArgs, TableConstructor,
    },
    node::Node,
    tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType},
    visitors::VisitorMut,
};

#[cfg(feature = "luau")]
use full_moon::ast::types::TypeInfo;

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
        _ => expression,
    }
}

#[cfg(feature = "luau")]
fn remove_type_parentheses(type_info: TypeInfo) -> TypeInfo {
    match type_info {
        TypeInfo::Tuple { ref types, .. } => {
            if types.len() == 1 {
                types.into_iter().next().unwrap().clone()
            } else {
                type_info
            }
        }
        _ => type_info,
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
                arguments: std::iter::once(Pair::End(Expression::String(string))).collect(),
            },
            FunctionArgs::TableConstructor(table) => FunctionArgs::Parentheses {
                parentheses: ContainedSpan::new(
                    TokenReference::symbol("(").unwrap(),
                    TokenReference::symbol(")").unwrap(),
                ),
                arguments: std::iter::once(Pair::End(Expression::TableConstructor(table)))
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
        // We change the formatting of number literals
        // We will normalise all numbers by running `str::parse(number).to_string()` and replacing the Token with this output.
        // This will help highlight any differences, as it would lead to a different parsed output

        let token_type = match token.token_type() {
            TokenType::Number { text } => {
                // Luau: cleanse number of any digit separators
                #[cfg(feature = "luau")]
                let text = text.replace('_', "");
                // LuaJIT (Lua52): remove suffixes
                #[cfg(feature = "lua52")]
                let text = text
                    .trim_end_matches("ULL")
                    .trim_end_matches("LL")
                    .to_string();

                let number = match text.as_str().parse::<f64>() {
                    Ok(num) => num,
                    // Try parsing as Hex (0x)
                    Err(_) => match i32::from_str_radix(&text.as_str()[2..], 16) {
                        Ok(num) => num.into(),
                        // If in Luau, try parsing as binary (0b)
                        #[cfg(feature = "luau")]
                        Err(_) => match i32::from_str_radix(&text.as_str()[2..], 2) {
                            Ok(num) => num.into(),
                            Err(_) => unreachable!(),
                        },
                        #[cfg(not(feature = "luau"))]
                        Err(_) => unreachable!(),
                    },
                };

                TokenType::Number {
                    text: number.to_string().into(),
                }
            }
            _ => unreachable!(),
        };

        Token::new(token_type)
    }

    fn visit_string_literal(&mut self, token: Token) -> Token {
        // We change the string quotes of our program.
        // Convert all string literals to brackets quotes, and remove any escapes.
        let token_type = match token.token_type() {
            TokenType::StringLiteral {
                literal,
                multi_line,
                ..
            } => TokenType::StringLiteral {
                literal: literal.to_owned().replace('\\', "").into(),
                multi_line: multi_line.to_owned(),
                quote_type: StringLiteralQuoteType::Brackets,
            },
            _ => unreachable!(),
        };

        Token::new(token_type)
    }

    #[cfg(feature = "luau")]
    fn visit_type_info(&mut self, type_info: TypeInfo) -> TypeInfo {
        remove_type_parentheses(type_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equivalent_asts() {
        let input_ast = full_moon::parse("local x = 1").unwrap();
        let output_ast = full_moon::parse("local x = 1").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_different_asts() {
        let input_ast = full_moon::parse("local x = 1").unwrap();
        let output_ast = full_moon::parse("local x = 2").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(!ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_stmt_semicolons() {
        let input_ast = full_moon::parse("local x = 1;").unwrap();
        let output_ast = full_moon::parse("local x = 1").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_string_quote_types() {
        // Should not flag different quotes as incorrect
        let input_ast = full_moon::parse("local x = '1'").unwrap();
        let output_ast = full_moon::parse("local x = \"1\"").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_string_escapes() {
        // Should not flag cleansed escapes as incorrect
        let input_ast = full_moon::parse("local x = '\\q'").unwrap();
        let output_ast = full_moon::parse("local x = 'q'").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_numbers() {
        let input_ast = full_moon::parse("local x = .1").unwrap();
        let output_ast = full_moon::parse("local x = 0.1").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_numbers_2() {
        let input_ast = full_moon::parse("local x = -.1").unwrap();
        let output_ast = full_moon::parse("local x = -0.1").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_hex_numbers() {
        let input_ast = full_moon::parse("local x = 0XFFFF").unwrap();
        let output_ast = full_moon::parse("local x = 0xFFFF").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_different_hex_numbers() {
        let input_ast = full_moon::parse("local x = 0xFFAA").unwrap();
        let output_ast = full_moon::parse("local x = 0xFFFF").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(!ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "luau")]
    fn test_equivalent_binary_numbers() {
        let input_ast = full_moon::parse("local x = 0B10101").unwrap();
        let output_ast = full_moon::parse("local x = 0b10101").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "luau")]
    fn test_different_binary_numbers() {
        let input_ast = full_moon::parse("local x = 0b1111").unwrap();
        let output_ast = full_moon::parse("local x = 0b1110").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(!ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "lua52")]
    fn test_equivalent_luajit_numbers() {
        let input_ast = full_moon::parse("local x = 2 ^ 63LL").unwrap();
        let output_ast = full_moon::parse("local x = 2 ^ 63").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_table_separators() {
        let input_ast = full_moon::parse("local x = {'a'; 'b'; 'c';}").unwrap();
        let output_ast = full_moon::parse("local x = {'a', 'b', 'c'}").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_function_calls() {
        let input_ast = full_moon::parse("local x = call'foo'").unwrap();
        let output_ast = full_moon::parse("local x = call('foo')").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_function_calls_2() {
        let input_ast = full_moon::parse("local x = call{'foo'}").unwrap();
        let output_ast = full_moon::parse("local x = call({'foo'})").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_conditions() {
        let input_ast = full_moon::parse("if (true) then return end").unwrap();
        let output_ast = full_moon::parse("if true then return end").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "luau")]
    fn test_equivalent_types_removed_parentheses() {
        let input_ast = full_moon::parse("type Foo = (number)").unwrap();
        let output_ast = full_moon::parse("type Foo = number").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }
}
