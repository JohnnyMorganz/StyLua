use crate::formatters::{
    table_formatter::TableType,
    trivia_formatter::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
    CodeFormatter,
};
use full_moon::ast::types::{
    CompoundAssignment, CompoundOp, ExportedTypeDeclaration, GenericDeclaration, IndexedTypeInfo,
    TypeAssertion, TypeDeclaration, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::borrow::Cow;
use std::boxed::Box;

impl CodeFormatter {
    pub fn format_compound_op<'ast>(&self, compound_op: &CompoundOp<'ast>) -> CompoundOp<'ast> {
        crate::fmt_op!(self, CompoundOp, compound_op, {
            PlusEqual = " += ",
            MinusEqual = " -= ",
            StarEqual = " *= ",
            SlashEqual = " /= ",
            PercentEqual = " %= ",
            CaretEqual = " ^= ",
            TwoDotsEqual = " ..= ",
        })
    }

    pub fn format_compound_assignment<'ast>(
        &mut self,
        compound_assignment: &CompoundAssignment<'ast>,
    ) -> CompoundAssignment<'ast> {
        // Calculate trivia
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_range_in_expression(compound_assignment.rhs()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let lhs = self
            .format_var(compound_assignment.lhs())
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
        let compound_operator = self.format_compound_op(compound_assignment.compound_operator());
        let rhs = self
            .format_expression(compound_assignment.rhs())
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        CompoundAssignment::new(lhs, compound_operator, rhs)
    }

    pub fn format_type_info<'ast>(&mut self, type_info: &TypeInfo<'ast>) -> TypeInfo<'ast> {
        match type_info {
            TypeInfo::Array { braces, type_info } => {
                let (start_brace, end_brace) = braces.tokens().to_owned();
                let braces = ContainedSpan::new(
                    crate::fmt_symbol!(self, start_brace, "{ "),
                    crate::fmt_symbol!(self, end_brace, " }"),
                );
                let type_info = Box::new(self.format_type_info(type_info));

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
                let arguments = self
                    .format_punctuated(arguments, &CodeFormatter::format_type_info)
                    .0;
                let arrow = crate::fmt_symbol!(self, arrow, " -> ");
                let return_type = Box::new(self.format_type_info(return_type));

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
                let generics = self
                    .format_punctuated(generics, &CodeFormatter::format_type_info)
                    .0;

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
                let left = Box::new(self.format_type_info(left));
                let ampersand = crate::fmt_symbol!(self, ampersand, " & ");
                let right = Box::new(self.format_type_info(right));

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
                let punctuation = crate::fmt_symbol!(self, punctuation, ".");
                let type_info = Box::new(self.format_indexed_type_info(type_info));

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
                let base = Box::new(self.format_type_info(base));
                let question_mark = crate::fmt_symbol!(self, question_mark, "?");

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

                let mut current_fields = fields.to_owned().into_pairs().peekable();
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

                    let formatted_field = self.format_type_field(&field, leading_trivia);
                    let mut formatted_punctuation = None;

                    match is_multiline {
                        true => {
                            // Continue adding a comma and a new line for multiline tables
                            // Add newline trivia to the end of the symbol
                            let symbol = match punctuation {
                                Some(punctuation) => crate::fmt_symbol!(self, &punctuation, ","),
                                None => TokenReference::symbol(",").unwrap(),
                            }
                            .update_trailing_trivia(
                                FormatTriviaType::Append(vec![self.create_newline_trivia()]),
                            );
                            formatted_punctuation = Some(symbol)
                        }

                        false => {
                            if current_fields.peek().is_some() {
                                // Have more elements still to go
                                formatted_punctuation = match punctuation {
                                    Some(punctuation) => {
                                        Some(crate::fmt_symbol!(self, &punctuation, ", "))
                                    }
                                    None => Some(TokenReference::symbol(", ").unwrap()),
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
                    typeof_token,
                    &TokenReference::new(
                        vec![],
                        Token::new(TokenType::Identifier {
                            identifier: Cow::Owned(String::from("typeof")),
                        }),
                        vec![],
                    ),
                );
                let parentheses = self.format_contained_span(parentheses);
                let inner = Box::new(self.format_expression(inner));

                TypeInfo::Typeof {
                    typeof_token,
                    parentheses,
                    inner,
                }
            }

            TypeInfo::Tuple { parentheses, types } => {
                let parentheses = self.format_contained_span(parentheses);
                let types = self
                    .format_punctuated(types, &CodeFormatter::format_type_info)
                    .0;

                TypeInfo::Tuple { parentheses, types }
            }

            TypeInfo::Union { left, pipe, right } => {
                let left = Box::new(self.format_type_info(left));
                let pipe = crate::fmt_symbol!(self, pipe, " | ");
                let right = Box::new(self.format_type_info(right));

                TypeInfo::Union { left, pipe, right }
            }

            other => panic!("unknown node {:?}", other),
        }
    }

    pub fn format_indexed_type_info<'ast>(
        &mut self,
        indexed_type_info: &IndexedTypeInfo<'ast>,
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
                let generics = self
                    .format_punctuated(generics, &CodeFormatter::format_type_info)
                    .0;

                IndexedTypeInfo::Generic {
                    base,
                    arrows,
                    generics,
                }
            }

            other => panic!("unknown node {:?}", other),
        }
    }

    pub fn format_type_field<'ast>(
        &mut self,
        type_field: &TypeField<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> TypeField<'ast> {
        let key = self.format_type_field_key(type_field.key(), leading_trivia);
        let colon_token = crate::fmt_symbol!(self, type_field.colon_token(), ": ");
        let value = self.format_type_info(type_field.value());

        type_field
            .to_owned()
            .with_key(key)
            .with_colon_token(colon_token)
            .with_value(value)
    }

    pub fn format_type_field_key<'ast>(
        &mut self,
        type_field_key: &TypeFieldKey<'ast>,
        leading_trivia: FormatTriviaType<'ast>,
    ) -> TypeFieldKey<'ast> {
        match type_field_key {
            TypeFieldKey::Name(token) => TypeFieldKey::Name(
                self.format_token_reference(token)
                    .update_leading_trivia(leading_trivia),
            ),
            TypeFieldKey::IndexSignature { brackets, inner } => TypeFieldKey::IndexSignature {
                brackets: self
                    .format_contained_span(brackets)
                    .update_leading_trivia(leading_trivia),
                inner: self.format_type_info(inner),
            },
            other => panic!("unknown node {:?}", other),
        }
    }

    pub fn format_type_assertion<'ast>(
        &mut self,
        type_assertion: &TypeAssertion<'ast>,
    ) -> TypeAssertion<'ast> {
        let assertion_op = crate::fmt_symbol!(self, type_assertion.assertion_op(), " :: ");
        let cast_to = self.format_type_info(type_assertion.cast_to());

        TypeAssertion::new(cast_to).with_assertion_op(assertion_op)
    }

    fn format_type_declaration<'ast>(
        &mut self,
        type_declaration: &TypeDeclaration<'ast>,
        add_leading_trivia: bool,
    ) -> TypeDeclaration<'ast> {
        // Calculate trivia
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(type_declaration.type_token()),
        );
        let trailing_trivia = vec![self.create_newline_trivia()];

        let mut type_token = self.format_symbol(
            type_declaration.type_token(),
            &TokenReference::new(
                vec![],
                Token::new(TokenType::Identifier {
                    identifier: Cow::Owned(String::from("type")),
                }),
                vec![Token::new(TokenType::spaces(1))],
            ),
        );

        if add_leading_trivia {
            let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
            type_token = type_token.update_leading_trivia(FormatTriviaType::Append(leading_trivia))
        }

        let type_name = self.format_token_reference(type_declaration.type_name());
        let generics = match type_declaration.generics() {
            Some(generics) => Some(self.format_generic_declaration(generics)),
            None => None,
        };
        let equal_token = crate::fmt_symbol!(self, type_declaration.equal_token(), " = ");
        let type_definition = self
            .format_type_info(type_declaration.type_definition())
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        type_declaration
            .to_owned()
            .with_type_token(type_token)
            .with_type_name(type_name)
            .with_generics(generics)
            .with_equal_token(equal_token)
            .with_type_definition(type_definition)
    }

    /// Wrapper around `format_type_declaration` for statements
    /// This is required as `format_type_declaration` is also used for ExportedTypeDeclaration, and we don't want leading trivia there
    pub fn format_type_declaration_stmt<'ast>(
        &mut self,
        type_declaration: &TypeDeclaration<'ast>,
    ) -> TypeDeclaration<'ast> {
        self.format_type_declaration(type_declaration, true)
    }

    pub fn format_generic_declaration<'ast>(
        &mut self,
        generic_declaration: &GenericDeclaration<'ast>,
    ) -> GenericDeclaration<'ast> {
        let arrows = self.format_contained_span(generic_declaration.arrows());
        let generics = self
            .format_punctuated(
                generic_declaration.generics(),
                &CodeFormatter::format_token_reference_mut,
            )
            .0;

        generic_declaration
            .to_owned()
            .with_arrows(arrows)
            .with_generics(generics)
    }

    pub fn format_type_specifier<'ast>(
        &mut self,
        type_specifier: &TypeSpecifier<'ast>,
    ) -> TypeSpecifier<'ast> {
        let punctuation = crate::fmt_symbol!(self, type_specifier.punctuation(), ": ");
        let type_info = self.format_type_info(type_specifier.type_info());

        type_specifier
            .to_owned()
            .with_punctuation(punctuation)
            .with_type_info(type_info)
    }

    pub fn format_exported_type_declaration<'ast>(
        &mut self,
        exported_type_declaration: &ExportedTypeDeclaration<'ast>,
    ) -> ExportedTypeDeclaration<'ast> {
        // Calculate trivia
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(exported_type_declaration.export_token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];

        let export_token = self
            .format_symbol(
                exported_type_declaration.export_token(),
                &TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::Owned(String::from("export")),
                    }),
                    vec![Token::new(TokenType::spaces(1))],
                ),
            )
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
        let type_declaration =
            self.format_type_declaration(exported_type_declaration.type_declaration(), false);

        exported_type_declaration
            .to_owned()
            .with_export_token(export_token)
            .with_type_declaration(type_declaration)
    }
}
