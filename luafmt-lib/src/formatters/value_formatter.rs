use full_moon::ast::Value;
use full_moon::tokenizer::{StringLiteralQuoteType, Token, TokenReference, TokenType};
use full_moon::visitors::VisitorMut;
use std::borrow::Cow;

#[derive(Default)]
pub struct ValueFormatter;

impl ValueFormatter {
    fn format_value<'ast>(&mut self, value: Value<'ast>) -> Value<'ast> {
        match value {
            Value::String(token_ref) => {
                let old_token_info = &*token_ref.token_type();

                match old_token_info {
                    TokenType::StringLiteral {
                        literal,
                        multi_line,
                        ..
                    } => {
                        let string_token = Token::new(TokenType::StringLiteral {
                            literal: literal.clone(),
                            multi_line: match multi_line {
                                Some(size) => Some(*size),
                                None => None,
                            },
                            quote_type: StringLiteralQuoteType::Double,
                        });
                        Value::String(Cow::Owned(TokenReference::new(
                            Vec::new(),
                            string_token,
                            Vec::new(),
                        )))
                    }
                    _ => panic!("have string with a non string-literal token kind"),
                }
            }
            // Return value back unformatted
            _ => value,
        }
    }
}

impl<'ast> VisitorMut<'ast> for ValueFormatter {
    fn visit_value(&mut self, node: Value<'ast>) -> Value<'ast> {
        self.format_value(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::formatters::value_formatter::ValueFormatter;
    use full_moon::visitors::VisitorMut;
    use full_moon::{parse, print};
    #[test]
    fn test_string_value_formatter() {
        let mut visitor = ValueFormatter::default();
        let ast = parse("local x = 'test'      ").unwrap();
        assert_eq!(print(&visitor.visit_ast(ast)), "local x = \"test\"");
    }
}
