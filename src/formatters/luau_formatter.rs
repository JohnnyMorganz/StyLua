use crate::formatters::{
    table_formatter::TableType,
    trivia_formatter::{self, FormatTriviaType},
    CodeFormatter,
};
use full_moon::ast::types::{
    AsAssertion, CompoundAssignment, CompoundOp, ExportedTypeDeclaration, GenericDeclaration,
    IndexedTypeInfo, TypeDeclaration, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::borrow::Cow;
use std::boxed::Box;

impl CodeFormatter {
    pub fn format_compound_op<'ast>(&self, compound_op: CompoundOp<'ast>) -> CompoundOp<'ast> {
        match compound_op {
            CompoundOp::PlusEqual(token) => CompoundOp::PlusEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" += ").unwrap()),
            ),
            CompoundOp::MinusEqual(token) => CompoundOp::MinusEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" -= ").unwrap()),
            ),
            CompoundOp::StarEqual(token) => CompoundOp::StarEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" *= ").unwrap()),
            ),
            CompoundOp::SlashEqual(token) => CompoundOp::SlashEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" /= ").unwrap()),
            ),
            CompoundOp::PercentEqual(token) => CompoundOp::PercentEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" %= ").unwrap()),
            ),
            CompoundOp::CaretEqual(token) => CompoundOp::CaretEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" ^= ").unwrap()),
            ),
            CompoundOp::TwoDotsEqual(token) => CompoundOp::TwoDotsEqual(
                self.format_symbol(token.into_owned(), TokenReference::symbol(" ..= ").unwrap()),
            ),
        }
    }

    pub fn format_compound_assignment<'ast>(
        &mut self,
        compound_assignment: CompoundAssignment<'ast>,
    ) -> CompoundAssignment<'ast> {
        let lhs = self.format_var(compound_assignment.lhs().to_owned());
        let compound_operator =
            self.format_compound_op(compound_assignment.compound_operator().to_owned());
        let rhs = self.format_expression(compound_assignment.rhs().to_owned());

        compound_assignment
            .with_lhs(lhs)
            .with_compound_operator(compound_operator)
            .with_rhs(rhs)
    }

    pub fn format_type_info<'ast>(&mut self, type_info: TypeInfo<'ast>) -> TypeInfo<'ast> {
        match type_info {
            TypeInfo::Array { braces, type_info } => {
                let (start_brace, end_brace) = braces.tokens().to_owned();
                let braces = ContainedSpan::new(
                    self.format_symbol(
                        start_brace.to_owned(),
                        TokenReference::symbol("{ ").unwrap(),
                    ),
                    self.format_symbol(end_brace.to_owned(), TokenReference::symbol(" }").unwrap()),
                );
                let type_info = Box::new(self.format_type_info(*type_info));

                TypeInfo::Array { braces, type_info }
            }

            TypeInfo::Basic(token_reference) => {
                let token_reference = self.format_token_reference(token_reference);
                TypeInfo::Basic(token_reference)
            }

            TypeInfo::Callback {
                parentheses,
                arguments,
                arrow,
                return_type,
            } => {
                let parentheses = self.format_contained_span(parentheses);
                let arguments = self.format_punctuated(arguments, &CodeFormatter::format_type_info);
                let arrow = self.format_symbol(
                    arrow.to_owned().into_owned(),
                    TokenReference::symbol(" -> ").unwrap(),
                );
                let return_type = Box::new(self.format_type_info(*return_type));

                TypeInfo::Callback {
                    parentheses,
                    arguments,
                    arrow,
                    return_type,
                }
            }

            TypeInfo::Generic {
                base,
                arrows,
                generics,
            } => {
                let base = self.format_token_reference(base);
                let arrows = self.format_contained_span(arrows);
                let generics = self.format_punctuated(generics, &CodeFormatter::format_type_info);

                TypeInfo::Generic {
                    base,
                    arrows,
                    generics,
                }
            }

            TypeInfo::Intersection {
                left,
                ampersand,
                right,
            } => {
                let left = Box::new(self.format_type_info(*left));
                let ampersand = self.format_symbol(
                    ampersand.to_owned().into_owned(),
                    TokenReference::symbol(" & ").unwrap(),
                );
                let right = Box::new(self.format_type_info(*right));

                TypeInfo::Intersection {
                    left,
                    ampersand,
                    right,
                }
            }

            TypeInfo::Module {
                module,
                punctuation,
                type_info,
            } => {
                let module = self.format_token_reference(module);
                let punctuation = self.format_symbol(
                    punctuation.to_owned().into_owned(),
                    TokenReference::symbol(".").unwrap(),
                );
                let type_info = Box::new(self.format_indexed_type_info(*type_info));

                TypeInfo::Module {
                    module,
                    punctuation,
                    type_info,
                }
            }

            TypeInfo::Optional {
                base,
                question_mark,
            } => {
                let base = Box::new(self.format_type_info(*base));
                let question_mark = self.format_symbol(
                    question_mark.to_owned().into_owned(),
                    TokenReference::symbol("?").unwrap(),
                );

                TypeInfo::Optional {
                    base,
                    question_mark,
                }
            }

            TypeInfo::Table { braces, fields } => {
                let (start_brace, end_brace) = braces.tokens().to_owned();
                let braces_range = (
                    start_brace.end_position().bytes(),
                    end_brace.start_position().bytes(),
                );

                let mut current_fields = fields.into_pairs().peekable();
                let is_multiline = (braces_range.1 - braces_range.0) > 30; // TODO: Properly determine this arbitrary number, and see if other factors should come into play
                let table_type = match current_fields.peek() {
                    Some(_) => match is_multiline {
                        true => TableType::MultiLine,
                        false => TableType::SingleLine,
                    },
                    None => TableType::Empty,
                };
                let additional_indent_level = self.get_range_indent_increase(braces_range);
                let braces = self.create_table_braces(
                    start_brace,
                    end_brace,
                    table_type,
                    additional_indent_level,
                );
                if is_multiline {
                    self.add_indent_range(braces_range);
                }

                let mut fields = Punctuated::new();

                while let Some(pair) = current_fields.next() {
                    let (field, punctuation) = pair.into_tuple();

                    let leading_trivia = match is_multiline {
                        true => {
                            let range = CodeFormatter::get_token_range(field.colon_token().token());
                            let additional_indent_level = self.get_range_indent_increase(range);
                            FormatTriviaType::Append(vec![
                                self.create_indent_trivia(additional_indent_level)
                            ])
                        }
                        false => FormatTriviaType::NoChange,
                    };

                    let formatted_field = self.format_type_field(field, leading_trivia);
                    let mut formatted_punctuation = None;

                    match is_multiline {
                        true => {
                            // Continue adding a comma and a new line for multiline tables
                            let mut symbol = TokenReference::symbol(",").unwrap();
                            if let Some(punctuation) = punctuation {
                                symbol = self
                                    .format_symbol(punctuation.into_owned(), symbol)
                                    .into_owned();
                            }
                            // Add newline trivia to the end of the symbol
                            let symbol = trivia_formatter::token_reference_add_trivia(
                                symbol,
                                FormatTriviaType::NoChange,
                                FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                            );
                            formatted_punctuation = Some(Cow::Owned(symbol))
                        }

                        false => {
                            if current_fields.peek().is_some() {
                                // Have more elements still to go
                                formatted_punctuation = match punctuation {
                                    Some(punctuation) => Some(self.format_symbol(
                                        punctuation.into_owned(),
                                        TokenReference::symbol(", ").unwrap(),
                                    )),
                                    None => Some(Cow::Owned(TokenReference::symbol(", ").unwrap())),
                                }
                            };
                        }
                    }

                    fields.push(Pair::new(formatted_field, formatted_punctuation));
                }

                TypeInfo::Table { braces, fields }
            }

            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            } => {
                let typeof_token = self.format_symbol(
                    typeof_token.to_owned().into_owned(),
                    TokenReference::new(
                        vec![],
                        Token::new(TokenType::Identifier {
                            identifier: Cow::Owned(String::from("typeof")),
                        }),
                        vec![],
                    ),
                );
                let parentheses = self.format_contained_span(parentheses);
                let inner = Box::new(self.format_expression(*inner));

                TypeInfo::Typeof {
                    typeof_token,
                    parentheses,
                    inner,
                }
            }

            TypeInfo::Tuple { parentheses, types } => {
                let parentheses = self.format_contained_span(parentheses);
                let types = self.format_punctuated(types, &CodeFormatter::format_type_info);

                TypeInfo::Tuple { parentheses, types }
            }

            TypeInfo::Union { left, pipe, right } => {
                let left = Box::new(self.format_type_info(*left));
                let pipe = self.format_symbol(
                    pipe.to_owned().into_owned(),
                    TokenReference::symbol(" | ").unwrap(),
                );
                let right = Box::new(self.format_type_info(*right));

                TypeInfo::Union { left, pipe, right }
            }
        }
    }

    pub fn format_indexed_type_info<'ast>(
        &mut self,
        indexed_type_info: IndexedTypeInfo<'ast>,
    ) -> IndexedTypeInfo<'ast> {
        match indexed_type_info {
            IndexedTypeInfo::Basic(token_reference) => {
                IndexedTypeInfo::Basic(self.format_token_reference(token_reference))
            }

            IndexedTypeInfo::Generic {
                base,
                arrows,
                generics,
            } => {
                let base = self.format_token_reference(base);
                let arrows = self.format_contained_span(arrows);
                let generics = self.format_punctuated(generics, &CodeFormatter::format_type_info);

                IndexedTypeInfo::Generic {
                    base,
                    arrows,
                    generics,
                }
            }
        }
    }

    pub fn format_type_field<'ast>(
        &mut self,
        type_field: TypeField<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> TypeField<'ast> {
        let key = self.format_type_field_key(type_field.key().to_owned(), leading_trivia);
        let colon_token = self.format_symbol(
            type_field.colon_token().to_owned(),
            TokenReference::symbol(": ").unwrap(),
        );
        let value = self.format_type_info(type_field.value().to_owned());

        type_field
            .with_key(key)
            .with_colon_token(colon_token)
            .with_value(value)
    }

    pub fn format_type_field_key<'ast>(
        &mut self,
        type_field_key: TypeFieldKey<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> TypeFieldKey<'ast> {
        match type_field_key {
            TypeFieldKey::Name(token) => {
                TypeFieldKey::Name(Cow::Owned(trivia_formatter::token_reference_add_trivia(
                    self.format_token_reference(token).into_owned(),
                    leading_trivia,
                    FormatTriviaType::NoChange,
                )))
            }
            TypeFieldKey::IndexSignature { brackets, inner } => {
                let brackets = trivia_formatter::contained_span_add_trivia(
                    self.format_contained_span(brackets),
                    leading_trivia,
                    FormatTriviaType::NoChange,
                );
                let inner = self.format_type_info(inner);

                TypeFieldKey::IndexSignature { brackets, inner }
            }
        }
    }

    pub fn format_as_assertion<'ast>(
        &mut self,
        as_assertion: AsAssertion<'ast>,
    ) -> AsAssertion<'ast> {
        let as_token = self.format_symbol(
            as_assertion.as_token().to_owned(),
            TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("as")),
                }),
                vec![Token::new(TokenType::spaces(1))],
            ),
        );
        let cast_to = self.format_type_info(as_assertion.cast_to().to_owned());

        as_assertion.with_as_token(as_token).with_cast_to(cast_to)
    }

    pub fn format_type_declaration<'ast>(
        &mut self,
        type_declaration: TypeDeclaration<'ast>,
    ) -> TypeDeclaration<'ast> {
        let type_token = self.format_symbol(
            type_declaration.type_token().to_owned(),
            TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("type")),
                }),
                vec![Token::new(TokenType::spaces(1))],
            ),
        );
        let type_name =
            Cow::Owned(self.format_plain_token_reference(type_declaration.type_name().to_owned()));
        let generics = match type_declaration.generics() {
            Some(generics) => Some(self.format_generic_declaration(generics.to_owned())),
            None => None,
        };
        let equal_token = self.format_symbol(
            type_declaration.equal_token().to_owned(),
            TokenReference::symbol(" = ").unwrap(),
        );
        let type_definition = self.format_type_info(type_declaration.type_definition().to_owned());

        type_declaration
            .with_type_token(type_token)
            .with_type_name(type_name)
            .with_generics(generics)
            .with_equal_token(equal_token)
            .with_type_definition(type_definition)
    }

    pub fn format_generic_declaration<'ast>(
        &mut self,
        generic_declaration: GenericDeclaration<'ast>,
    ) -> GenericDeclaration<'ast> {
        let arrows = self.format_contained_span(generic_declaration.arrows().to_owned());
        let generics = self.format_punctuated(
            generic_declaration.generics().to_owned(),
            &CodeFormatter::format_token_reference_mut,
        );

        generic_declaration
            .with_arrows(arrows)
            .with_generics(generics)
    }

    pub fn format_type_specifier<'ast>(
        &mut self,
        type_specifier: TypeSpecifier<'ast>,
    ) -> TypeSpecifier<'ast> {
        let punctuation = self.format_symbol(
            type_specifier.punctuation().to_owned(),
            TokenReference::symbol(": ").unwrap(),
        );
        let type_info = self.format_type_info(type_specifier.type_info().to_owned());

        type_specifier
            .with_punctuation(punctuation)
            .with_type_info(type_info)
    }

    pub fn format_exported_type_declaration<'ast>(
        &mut self,
        exported_type_declaration: ExportedTypeDeclaration<'ast>,
    ) -> ExportedTypeDeclaration<'ast> {
        let export_token = self.format_symbol(
            exported_type_declaration.export_token().to_owned(),
            TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("export")),
                }),
                vec![Token::new(TokenType::spaces(1))],
            ),
        );
        let type_declaration =
            self.format_type_declaration(exported_type_declaration.type_declaration().to_owned());

        exported_type_declaration
            .with_export_token(export_token)
            .with_type_declaration(type_declaration)
    }
}
