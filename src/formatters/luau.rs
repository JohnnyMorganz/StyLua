use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_op, fmt_symbol,
    formatters::{
        expression::{format_expression, format_var},
        general::{
            format_contained_span, format_end_token, format_symbol, format_token_reference,
            try_format_punctuated, EndTokenType,
        },
        table::{create_table_braces, TableType},
        trivia::{
            strip_leading_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
        trivia_util::{contains_comments, take_type_field_trailing_comments},
    },
    shape::Shape,
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

pub fn format_compound_op<'ast>(
    ctx: &Context,
    compound_op: &CompoundOp<'ast>,
    shape: Shape,
) -> CompoundOp<'ast> {
    fmt_op!(ctx, CompoundOp, compound_op, shape, {
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
    ctx: &Context,
    compound_assignment: &CompoundAssignment<'ast>,
    shape: Shape,
) -> CompoundAssignment<'ast> {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let lhs = format_var(ctx, compound_assignment.lhs(), shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let compound_operator = format_compound_op(ctx, compound_assignment.compound_operator(), shape);
    let shape = shape
        + (strip_leading_trivia(&lhs).to_string().len() + compound_operator.to_string().len());

    let rhs = format_expression(ctx, compound_assignment.rhs(), shape)
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    CompoundAssignment::new(lhs, compound_operator, rhs)
}

pub fn format_type_info<'ast>(
    ctx: &Context,
    type_info: &TypeInfo<'ast>,
    shape: Shape,
) -> TypeInfo<'ast> {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let (start_brace, end_brace) = braces.tokens().to_owned();
            let braces = ContainedSpan::new(
                fmt_symbol!(ctx, start_brace, "{ ", shape),
                fmt_symbol!(ctx, end_brace, " }", shape),
            );
            let type_info = Box::new(format_type_info(ctx, type_info, shape));

            TypeInfo::Array { braces, type_info }
        }

        TypeInfo::Basic(token_reference) => {
            let token_reference = format_token_reference(ctx, token_reference, shape);
            TypeInfo::Basic(token_reference)
        }

        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let parentheses = format_contained_span(ctx, parentheses, shape);
            let arguments = try_format_punctuated(ctx, arguments, shape, format_type_info, None);
            let arrow = fmt_symbol!(ctx, arrow, " -> ", shape);
            let return_type = Box::new(format_type_info(ctx, return_type, shape));
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
            let base = format_token_reference(ctx, base, shape);
            let arrows = format_contained_span(ctx, arrows, shape);
            let generics = try_format_punctuated(ctx, generics, shape, format_type_info, None);
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
            let left = Box::new(format_type_info(ctx, left, shape));
            let ampersand = fmt_symbol!(ctx, ampersand, " & ", shape);
            let right = Box::new(format_type_info(ctx, right, shape));
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
            let module = format_token_reference(ctx, module, shape);
            let punctuation = fmt_symbol!(ctx, punctuation, ".", shape);
            let type_info = Box::new(format_indexed_type_info(ctx, type_info, shape));
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
            let base = Box::new(format_type_info(ctx, base, shape));
            let question_mark = fmt_symbol!(ctx, question_mark, "?", shape);
            TypeInfo::Optional {
                base,
                question_mark,
            }
        }

        TypeInfo::Table { braces, fields } => {
            let (start_brace, end_brace) = braces.tokens().to_owned();
            let braces_range = (
                start_brace.token().end_position().bytes(),
                end_brace.token().start_position().bytes(),
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

            let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);

            let shape = if is_multiline {
                shape.increment_additional_indent()
            } else {
                shape
            };

            let mut fields = Punctuated::new();

            while let Some(pair) = current_fields.next() {
                let (field, punctuation) = pair.into_tuple();

                let leading_trivia = match is_multiline {
                    true => FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
                    false => FormatTriviaType::NoChange,
                };

                let mut formatted_field = format_type_field(ctx, &field, leading_trivia, shape);
                let mut formatted_punctuation = None;

                match is_multiline {
                    true => {
                        // Continue adding a comma and a new line for multiline tables
                        // Add newline trivia to the end of the symbol

                        let (field, mut trailing_comments) =
                            take_type_field_trailing_comments(formatted_field);
                        formatted_field = field;
                        trailing_comments.push(create_newline_trivia(ctx));

                        let symbol = match punctuation {
                            Some(punctuation) => fmt_symbol!(ctx, &punctuation, ",", shape),
                            None => TokenReference::symbol(",").unwrap(),
                        }
                        .update_trailing_trivia(FormatTriviaType::Append(trailing_comments));
                        formatted_punctuation = Some(symbol)
                    }

                    false => {
                        if current_fields.peek().is_some() {
                            // Have more elements still to go
                            formatted_punctuation = match punctuation {
                                Some(punctuation) => {
                                    Some(fmt_symbol!(ctx, &punctuation, ", ", shape))
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
            let typeof_token = format_symbol(
                ctx,
                typeof_token,
                &TokenReference::new(
                    vec![],
                    Token::new(TokenType::Identifier {
                        identifier: Cow::Owned(String::from("typeof")),
                    }),
                    vec![],
                ),
                shape,
            );
            let parentheses = format_contained_span(ctx, parentheses, shape);
            let inner = Box::new(format_expression(ctx, inner, shape));
            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            }
        }

        TypeInfo::Tuple { parentheses, types } => {
            let parentheses = format_contained_span(ctx, parentheses, shape);
            let types = try_format_punctuated(ctx, types, shape, format_type_info, None);

            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let left = Box::new(format_type_info(ctx, left, shape));
            let pipe = fmt_symbol!(ctx, pipe, " | ", shape);
            let right = Box::new(format_type_info(ctx, right, shape));

            TypeInfo::Union { left, pipe, right }
        }

        TypeInfo::Variadic { ellipse, type_info } => {
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);
            let type_info = Box::new(format_type_info(ctx, type_info, shape));

            TypeInfo::Variadic { ellipse, type_info }
        }

        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_indexed_type_info<'ast>(
    ctx: &Context,
    indexed_type_info: &IndexedTypeInfo<'ast>,
    shape: Shape,
) -> IndexedTypeInfo<'ast> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(token_reference) => {
            IndexedTypeInfo::Basic(format_token_reference(ctx, token_reference, shape))
        }

        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let base = format_token_reference(ctx, base, shape);
            let arrows = format_contained_span(ctx, arrows, shape);
            let generics = try_format_punctuated(ctx, generics, shape, format_type_info, None);
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
    ctx: &Context,
    type_field: &TypeField<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    shape: Shape,
) -> TypeField<'ast> {
    let key = format_type_field_key(ctx, type_field.key(), leading_trivia, shape);
    let colon_token = fmt_symbol!(ctx, type_field.colon_token(), ": ", shape);
    let value = format_type_info(ctx, type_field.value(), shape);

    type_field
        .to_owned()
        .with_key(key)
        .with_colon_token(colon_token)
        .with_value(value)
}

pub fn format_type_field_key<'ast>(
    ctx: &Context,
    type_field_key: &TypeFieldKey<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
    shape: Shape,
) -> TypeFieldKey<'ast> {
    match type_field_key {
        TypeFieldKey::Name(token) => TypeFieldKey::Name(
            format_token_reference(ctx, token, shape).update_leading_trivia(leading_trivia),
        ),
        TypeFieldKey::IndexSignature { brackets, inner } => TypeFieldKey::IndexSignature {
            brackets: format_contained_span(ctx, brackets, shape)
                .update_leading_trivia(leading_trivia),
            inner: format_type_info(ctx, inner, shape),
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_type_assertion<'ast>(
    ctx: &Context,
    type_assertion: &TypeAssertion<'ast>,
    shape: Shape,
) -> TypeAssertion<'ast> {
    let assertion_op = fmt_symbol!(ctx, type_assertion.assertion_op(), " :: ", shape);
    let cast_to = format_type_info(ctx, type_assertion.cast_to(), shape);

    TypeAssertion::new(cast_to).with_assertion_op(assertion_op)
}

fn format_type_declaration<'ast>(
    ctx: &Context,
    type_declaration: &TypeDeclaration<'ast>,
    add_leading_trivia: bool,
    shape: Shape,
) -> TypeDeclaration<'ast> {
    // Calculate trivia
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let mut type_token = format_symbol(
        ctx,
        type_declaration.type_token(),
        &TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::Owned(String::from("type")),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
        shape,
    );

    if add_leading_trivia {
        let leading_trivia = vec![create_indent_trivia(ctx, shape)];
        type_token = type_token.update_leading_trivia(FormatTriviaType::Append(leading_trivia))
    }

    let type_name = format_token_reference(ctx, type_declaration.type_name(), shape);
    let generics = type_declaration
        .generics()
        .map(|generics| format_generic_declaration(ctx, generics, shape));
    let equal_token = fmt_symbol!(ctx, type_declaration.equal_token(), " = ", shape);
    let type_definition = format_type_info(ctx, type_declaration.type_definition(), shape)
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
    ctx: &Context,
    type_declaration: &TypeDeclaration<'ast>,
    shape: Shape,
) -> TypeDeclaration<'ast> {
    format_type_declaration(ctx, type_declaration, true, shape)
}

pub fn format_generic_declaration<'ast>(
    ctx: &Context,
    generic_declaration: &GenericDeclaration<'ast>,
    shape: Shape,
) -> GenericDeclaration<'ast> {
    // If the generics contains comments, then format multiline
    let (arrows, generics) = if contains_comments(generic_declaration.generics()) {
        let (start_arrow, end_arrow) = generic_declaration.arrows().tokens();

        // Format start and end arrows properly with correct trivia
        let end_arrow_leading_trivia =
            vec![create_newline_trivia(ctx), create_indent_trivia(ctx, shape)];

        // Add new_line trivia to start arrow
        let start_arrow_token = fmt_symbol!(ctx, start_arrow, "<", shape)
            .update_trailing_trivia(FormatTriviaType::Append(vec![create_newline_trivia(ctx)]));

        let end_arrow_token = format_end_token(ctx, end_arrow, EndTokenType::ClosingBrace, shape)
            .update_leading_trivia(FormatTriviaType::Append(end_arrow_leading_trivia));

        let arrows = ContainedSpan::new(start_arrow_token, end_arrow_token);

        let shape = shape.reset().increment_additional_indent();
        let generics = try_format_punctuated(
            ctx,
            generic_declaration.generics(),
            shape,
            format_token_reference,
            None,
        )
        .update_leading_trivia(FormatTriviaType::Append(vec![create_indent_trivia(
            ctx, shape,
        )]));

        (arrows, generics)
    } else {
        (
            format_contained_span(ctx, generic_declaration.arrows(), shape),
            try_format_punctuated(
                ctx,
                generic_declaration.generics(),
                shape,
                format_token_reference,
                None,
            ),
        )
    };

    generic_declaration
        .to_owned()
        .with_arrows(arrows)
        .with_generics(generics)
}

pub fn format_type_specifier<'ast>(
    ctx: &Context,
    type_specifier: &TypeSpecifier<'ast>,
    shape: Shape,
) -> TypeSpecifier<'ast> {
    let punctuation = fmt_symbol!(ctx, type_specifier.punctuation(), ": ", shape);
    let type_info = format_type_info(ctx, type_specifier.type_info(), shape);

    type_specifier
        .to_owned()
        .with_punctuation(punctuation)
        .with_type_info(type_info)
}

pub fn format_exported_type_declaration<'ast>(
    ctx: &Context,
    exported_type_declaration: &ExportedTypeDeclaration<'ast>,
    shape: Shape,
) -> ExportedTypeDeclaration<'ast> {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];

    let export_token = format_symbol(
        ctx,
        exported_type_declaration.export_token(),
        &TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: Cow::Owned(String::from("export")),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
        shape,
    )
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let type_declaration = format_type_declaration(
        ctx,
        exported_type_declaration.type_declaration(),
        false,
        shape,
    );

    exported_type_declaration
        .to_owned()
        .with_export_token(export_token)
        .with_type_declaration(type_declaration)
}
