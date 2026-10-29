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
use full_moon::ast::luau::TypeInfo;

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
                #[cfg(feature = "luajit")]
                let text = text
                    .trim_end_matches("ULL")
                    .trim_end_matches("LL")
                    .to_string();

                let number = match text.as_str().parse::<f64>() {
                    Ok(num) => num.to_string(),
                    // Try parsing as Hex (0x)
                    Err(_) => match u64::from_str_radix(&text.as_str()[2..], 16) {
                        Ok(num) => num.to_string(),
                        // If in Luau, try parsing as binary (0b)
                        #[cfg(feature = "luau")]
                        Err(_) => match u64::from_str_radix(&text.as_str()[2..], 2) {
                            Ok(num) => num.to_string(),
                            Err(_) => normalise_hex_float(&text),
                        },
                        #[cfg(not(feature = "luau"))]
                        Err(_) => normalise_hex_float(&text),
                    },
                };

                TokenType::Number {
                    text: number.into(),
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
                multi_line_depth,
                ..
            } => TokenType::StringLiteral {
                literal: literal.to_owned().replace('\\', "").into(),
                multi_line_depth: multi_line_depth.to_owned(),
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

/// Normalises a hexadecimal number with a fraction or a binary exponent, such as `0xA.8p0` (Lua 5.2+),
/// or a hexadecimal integer too large for 64 bits. Anything else is kept as written.
fn normalise_hex_float(text: &str) -> String {
    let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) else {
        return text.to_string();
    };
    let (mantissa, exponent) = digits.split_once(['p', 'P']).unwrap_or((digits, "0"));
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));

    let mut value = 0.0;
    for digit in integer.chars().chain(fraction.chars()) {
        match digit.to_digit(16) {
            Some(digit) => value = value * 16.0 + f64::from(digit),
            None => return text.to_string(),
        }
    }
    match exponent.parse::<i32>() {
        Ok(exponent) => (value * 2f64.powi(exponent - 4 * fraction.len() as i32)).to_string(),
        Err(_) => text.to_string(),
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
    #[cfg(feature = "luau")]
    fn test_equivalent_hex_numbers_with_separators() {
        let input_ast = full_moon::parse("local x = 0xffff_ffc0").unwrap();
        let output_ast = full_moon::parse("local x = 0xffffffc0").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_hex_numbers_very_large_number() {
        let input_ast = full_moon::parse("max = max or 0xffffffff").unwrap();
        let output_ast = full_moon::parse("max = max or 0xffffffff").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    fn test_equivalent_hex_numbers_beyond_i64() {
        let input_ast = full_moon::parse("local x = 0xffffffffffffffff").unwrap();
        let output_ast = full_moon::parse("local x = 0xFFFFFFFFFFFFFFFF").unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "lua52")]
    fn test_equivalent_hex_float_numbers() {
        use full_moon::LuaVersion;

        let input_ast = full_moon::parse_fallible(
            "local x = 0x1p4 + 0xA.8p0 + 0x.1p1 + 0x1.8 + 0x1p-1",
            LuaVersion::lua52(),
        )
        .into_result()
        .unwrap();
        let output_ast = full_moon::parse_fallible(
            "local x = 0X1P4 + 0xa.8P0 + 0x.1p1 + 0x1.8 + 0X1P-1",
            LuaVersion::lua52(),
        )
        .into_result()
        .unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "lua52")]
    fn test_different_hex_float_numbers() {
        use full_moon::LuaVersion;

        let input_ast = full_moon::parse_fallible("local x = 0x1p4", LuaVersion::lua52())
            .into_result()
            .unwrap();
        let output_ast = full_moon::parse_fallible("local x = 0x1p5", LuaVersion::lua52())
            .into_result()
            .unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(!ast_verifier.compare(input_ast, output_ast));
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
        use full_moon::LuaVersion;

        let input_ast = full_moon::parse_fallible("local x = 0B10101", LuaVersion::luau())
            .into_result()
            .unwrap();
        let output_ast = full_moon::parse_fallible("local x = 0b10101", LuaVersion::luau())
            .into_result()
            .unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "luau")]
    fn test_different_binary_numbers() {
        use full_moon::LuaVersion;

        let input_ast = full_moon::parse_fallible("local x = 0b1111", LuaVersion::luau())
            .into_result()
            .unwrap();
        let output_ast = full_moon::parse_fallible("local x = 0b1110", LuaVersion::luau())
            .into_result()
            .unwrap();

        let mut ast_verifier = AstVerifier::new();
        assert!(!ast_verifier.compare(input_ast, output_ast));
    }

    #[test]
    #[cfg(feature = "luajit")]
    fn test_equivalent_luajit_numbers() {
        use full_moon::LuaVersion;

        let input_ast = full_moon::parse_fallible("local x = 2 ^ 63LL", LuaVersion::luajit())
            .into_result()
            .unwrap();
        let output_ast = full_moon::parse_fallible("local x = 2 ^ 63", LuaVersion::luajit())
            .into_result()
            .unwrap();

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
