use crate::formatters::{expression_formatter, CodeFormatter};
use full_moon::ast::span::ContainedSpan;
use full_moon::ast::types::{
    AsAssertion, CompoundAssignment, CompoundOp, ExportedTypeDeclaration, GenericDeclaration,
    IndexedTypeInfo, TypeDeclaration, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::borrow::Cow;
use std::boxed::Box;

pub fn format_compound_op<'ast>(
    code_formatter: &CodeFormatter,
    compound_op: CompoundOp<'ast>,
) -> CompoundOp<'ast> {
    match compound_op {
        CompoundOp::PlusEqual(token) => CompoundOp::PlusEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" += ").unwrap()),
        ),
        CompoundOp::MinusEqual(token) => CompoundOp::MinusEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" -= ").unwrap()),
        ),
        CompoundOp::StarEqual(token) => CompoundOp::StarEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" *= ").unwrap()),
        ),
        CompoundOp::SlashEqual(token) => CompoundOp::SlashEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" /= ").unwrap()),
        ),
        CompoundOp::PercentEqual(token) => CompoundOp::PercentEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" %= ").unwrap()),
        ),
        CompoundOp::CaretEqual(token) => CompoundOp::CaretEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" ^= ").unwrap()),
        ),
        CompoundOp::TwoDotsEqual(token) => CompoundOp::TwoDotsEqual(
            code_formatter
                .format_symbol(token.into_owned(), TokenReference::symbol(" ..= ").unwrap()),
        ),
    }
}

pub fn format_compound_assignment<'ast>(
    code_formatter: &mut CodeFormatter,
    compound_assignment: CompoundAssignment<'ast>,
) -> CompoundAssignment<'ast> {
    let lhs =
        expression_formatter::format_var(code_formatter, compound_assignment.lhs().to_owned());
    let compound_operator = format_compound_op(
        code_formatter,
        compound_assignment.compound_operator().to_owned(),
    );
    let rhs = expression_formatter::format_expression(
        code_formatter,
        compound_assignment.rhs().to_owned(),
    );

    compound_assignment
        .with_lhs(lhs)
        .with_compound_operator(compound_operator)
        .with_rhs(rhs)
}

pub fn format_type_info<'ast>(
    code_formatter: &mut CodeFormatter,
    type_info: TypeInfo<'ast>,
) -> TypeInfo<'ast> {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let (start_brace, end_brace) = braces.tokens().to_owned();
            let braces = ContainedSpan::new(
                code_formatter.format_symbol(
                    start_brace.to_owned(),
                    TokenReference::symbol("{ ").unwrap(),
                ),
                code_formatter
                    .format_symbol(end_brace.to_owned(), TokenReference::symbol(" }").unwrap()),
            );
            let type_info = Box::new(format_type_info(code_formatter, *type_info));

            TypeInfo::Array { braces, type_info }
        }

        TypeInfo::Basic(token_reference) => {
            let token_reference = code_formatter.format_token_reference(token_reference);
            TypeInfo::Basic(token_reference)
        }

        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let parentheses = code_formatter.format_contained_span(parentheses);
            let arguments = code_formatter.format_punctuated(arguments, &format_type_info);
            let arrow = code_formatter.format_symbol(
                arrow.to_owned().into_owned(),
                TokenReference::symbol(" -> ").unwrap(),
            );
            let return_type = Box::new(format_type_info(code_formatter, *return_type));

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
            let base = code_formatter.format_token_reference(base);
            let arrows = code_formatter.format_contained_span(arrows);
            let generics = code_formatter.format_punctuated(generics, &format_type_info);

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
            let left = Box::new(format_type_info(code_formatter, *left));
            let ampersand = code_formatter.format_symbol(
                ampersand.to_owned().into_owned(),
                TokenReference::symbol(" & ").unwrap(),
            );
            let right = Box::new(format_type_info(code_formatter, *right));

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
            let module = code_formatter.format_token_reference(module);
            let punctuation = code_formatter.format_symbol(
                punctuation.to_owned().into_owned(),
                TokenReference::symbol(".").unwrap(),
            );
            let type_info = Box::new(format_indexed_type_info(code_formatter, *type_info));

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
            let base = Box::new(format_type_info(code_formatter, *base));
            let question_mark = code_formatter.format_symbol(
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
            let braces = ContainedSpan::new(
                code_formatter.format_symbol(
                    start_brace.to_owned(),
                    TokenReference::symbol("{ ").unwrap(),
                ),
                code_formatter
                    .format_symbol(end_brace.to_owned(), TokenReference::symbol(" }").unwrap()),
            );
            let fields = code_formatter.format_punctuated(fields, &format_type_field);

            TypeInfo::Table { braces, fields }
        }

        TypeInfo::Typeof {
            typeof_token,
            parentheses,
            inner,
        } => {
            let typeof_token = code_formatter.format_symbol(
                typeof_token.to_owned().into_owned(),
                TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::Owned(String::from("typeof")),
                    }),
                    vec![],
                ),
            );
            let parentheses = code_formatter.format_contained_span(parentheses);
            let inner = Box::new(expression_formatter::format_expression(
                code_formatter,
                *inner,
            ));

            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            }
        }

        TypeInfo::Tuple { parentheses, types } => {
            let parentheses = code_formatter.format_contained_span(parentheses);
            let types = code_formatter.format_punctuated(types, &format_type_info);

            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let left = Box::new(format_type_info(code_formatter, *left));
            let pipe = code_formatter.format_symbol(
                pipe.to_owned().into_owned(),
                TokenReference::symbol(" | ").unwrap(),
            );
            let right = Box::new(format_type_info(code_formatter, *right));

            TypeInfo::Union { left, pipe, right }
        }
    }
}

pub fn format_indexed_type_info<'ast>(
    code_formatter: &mut CodeFormatter,
    indexed_type_info: IndexedTypeInfo<'ast>,
) -> IndexedTypeInfo<'ast> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(token_reference) => {
            IndexedTypeInfo::Basic(code_formatter.format_token_reference(token_reference))
        }

        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let base = code_formatter.format_token_reference(base);
            let arrows = code_formatter.format_contained_span(arrows);
            let generics = code_formatter.format_punctuated(generics, &format_type_info);

            IndexedTypeInfo::Generic {
                base,
                arrows,
                generics,
            }
        }
    }
}

pub fn format_type_field<'ast>(
    code_formatter: &mut CodeFormatter,
    type_field: TypeField<'ast>,
) -> TypeField<'ast> {
    let key = format_type_field_key(code_formatter, type_field.key().to_owned());
    let colon_token = code_formatter.format_symbol(
        type_field.colon_token().to_owned(),
        TokenReference::symbol(": ").unwrap(),
    );
    let value = format_type_info(code_formatter, type_field.value().to_owned());

    type_field
        .with_key(key)
        .with_colon_token(colon_token)
        .with_value(value)
}

pub fn format_type_field_key<'ast>(
    code_formatter: &mut CodeFormatter,
    type_field_key: TypeFieldKey<'ast>,
) -> TypeFieldKey<'ast> {
    match type_field_key {
        TypeFieldKey::Name(token) => {
            TypeFieldKey::Name(code_formatter.format_token_reference(token))
        }
        TypeFieldKey::IndexSignature { brackets, inner } => {
            let brackets = code_formatter.format_contained_span(brackets);
            let inner = format_type_info(code_formatter, inner);

            TypeFieldKey::IndexSignature { brackets, inner }
        }
    }
}

pub fn format_as_assertion<'ast>(
    code_formatter: &mut CodeFormatter,
    as_assertion: AsAssertion<'ast>,
) -> AsAssertion<'ast> {
    let as_token = code_formatter.format_symbol(
        as_assertion.as_token().to_owned(),
        TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::Owned(String::from("as")),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
    );
    let cast_to = format_type_info(code_formatter, as_assertion.cast_to().to_owned());

    as_assertion.with_as_token(as_token).with_cast_to(cast_to)
}

pub fn format_type_declaration<'ast>(
    code_formatter: &mut CodeFormatter,
    type_declaration: TypeDeclaration<'ast>,
) -> TypeDeclaration<'ast> {
    let type_token = code_formatter.format_symbol(
        type_declaration.type_token().to_owned(),
        TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::Owned(String::from("type")),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
    );
    let type_name = Cow::Owned(
        code_formatter.format_plain_token_reference(type_declaration.type_name().to_owned()),
    );
    let generics = match type_declaration.generics() {
        Some(generics) => Some(format_generic_declaration(
            code_formatter,
            generics.to_owned(),
        )),
        None => None,
    };
    let equal_token = code_formatter.format_symbol(
        type_declaration.equal_token().to_owned(),
        TokenReference::symbol(" = ").unwrap(),
    );
    let type_definition = format_type_info(
        code_formatter,
        type_declaration.type_definition().to_owned(),
    );

    type_declaration
        .with_type_token(type_token)
        .with_type_name(type_name)
        .with_generics(generics)
        .with_equal_token(equal_token)
        .with_type_definition(type_definition)
}

pub fn format_generic_declaration<'ast>(
    code_formatter: &mut CodeFormatter,
    generic_declaration: GenericDeclaration<'ast>,
) -> GenericDeclaration<'ast> {
    let arrows = code_formatter.format_contained_span(generic_declaration.arrows().to_owned());
    let generics = code_formatter.format_punctuated(
        generic_declaration.generics().to_owned(),
        &CodeFormatter::format_token_reference_mut,
    );

    generic_declaration
        .with_arrows(arrows)
        .with_generics(generics)
}

pub fn format_type_specifier<'ast>(
    code_formatter: &mut CodeFormatter,
    type_specifier: TypeSpecifier<'ast>,
) -> TypeSpecifier<'ast> {
    let punctuation = code_formatter.format_symbol(
        type_specifier.punctuation().to_owned(),
        TokenReference::symbol(": ").unwrap(),
    );
    let type_info = format_type_info(code_formatter, type_specifier.type_info().to_owned());

    type_specifier
        .with_punctuation(punctuation)
        .with_type_info(type_info)
}

pub fn format_exported_type_declaration<'ast>(
    code_formatter: &mut CodeFormatter,
    exported_type_declaration: ExportedTypeDeclaration<'ast>,
) -> ExportedTypeDeclaration<'ast> {
    let export_token = code_formatter.format_symbol(
        exported_type_declaration.export_token().to_owned(),
        TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::Owned(String::from("export")),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
    );
    let type_declaration = format_type_declaration(
        code_formatter,
        exported_type_declaration.type_declaration().to_owned(),
    );

    exported_type_declaration
        .with_export_token(export_token)
        .with_type_declaration(type_declaration)
}
