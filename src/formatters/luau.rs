use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_op, fmt_symbol,
    formatters::{
        expression::{format_expression, format_var},
        general::{
            format_contained_span, format_symbol, format_token_reference,
            format_token_reference_mut, try_format_punctuated,
        },
        table::{create_table_braces, TableType},
        trivia::{
            strip_leading_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
        util::{expression_range, token_range},
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

pub fn format_compound_op<'ast>(ctx: &Context, compound_op: &CompoundOp<'ast>) -> CompoundOp<'ast> {
    fmt_op!(ctx, CompoundOp, compound_op, {
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
    ctx: &mut Context,
    compound_assignment: &CompoundAssignment<'ast>,
    shape: Shape,
) -> CompoundAssignment<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(expression_range(compound_assignment.rhs()));
    let shape = shape.with_additional_indent(additional_indent_level);
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let lhs = format_var(ctx, compound_assignment.lhs(), shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let compound_operator = format_compound_op(ctx, compound_assignment.compound_operator());
    let shape = shape
        + (strip_leading_trivia(&lhs).to_string().len() + compound_operator.to_string().len());

    let rhs = format_expression(ctx, compound_assignment.rhs(), shape)
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    CompoundAssignment::new(lhs, compound_operator, rhs)
}

fn format_type_info_shape<'ast>(
    ctx: &mut Context,
    type_info: &TypeInfo<'ast>,
    _shape: Shape,
) -> TypeInfo<'ast> {
    format_type_info(ctx, type_info)
}

pub fn format_type_info<'ast>(ctx: &mut Context, type_info: &TypeInfo<'ast>) -> TypeInfo<'ast> {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let (start_brace, end_brace) = braces.tokens().to_owned();
            let braces = ContainedSpan::new(
                fmt_symbol!(ctx, start_brace, "{ "),
                fmt_symbol!(ctx, end_brace, " }"),
            );
            let type_info = Box::new(format_type_info(ctx, type_info));

            TypeInfo::Array { braces, type_info }
        }

        TypeInfo::Basic(token_reference) => {
            let token_reference = format_token_reference(ctx, token_reference);
            TypeInfo::Basic(token_reference)
        }

        TypeInfo::Callback {
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            let parentheses = format_contained_span(ctx, parentheses);
            let arguments = try_format_punctuated(
                ctx,
                arguments,
                Shape::from_context(ctx),
                format_type_info_shape,
            );
            let arrow = fmt_symbol!(ctx, arrow, " -> ");
            let return_type = Box::new(format_type_info(ctx, return_type));
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
            let base = format_token_reference(ctx, base);
            let arrows = format_contained_span(ctx, arrows);
            let generics = try_format_punctuated(
                ctx,
                generics,
                Shape::from_context(ctx),
                format_type_info_shape,
            );
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
            let left = Box::new(format_type_info(ctx, left));
            let ampersand = fmt_symbol!(ctx, ampersand, " & ");
            let right = Box::new(format_type_info(ctx, right));
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
            let module = format_token_reference(ctx, module);
            let punctuation = fmt_symbol!(ctx, punctuation, ".");
            let type_info = Box::new(format_indexed_type_info(ctx, type_info));
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
            let base = Box::new(format_type_info(ctx, base));
            let question_mark = fmt_symbol!(ctx, question_mark, "?");
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

            if is_multiline {
                ctx.add_indent_range(braces_range);
            }

            let additional_indent_level =
                ctx.get_range_indent_increase(token_range(end_brace.token()));
            let braces = create_table_braces(
                ctx,
                start_brace,
                end_brace,
                table_type,
                additional_indent_level,
            );

            let mut fields = Punctuated::new();

            while let Some(pair) = current_fields.next() {
                let (field, punctuation) = pair.into_tuple();

                let leading_trivia = match is_multiline {
                    true => {
                        let range = token_range(field.colon_token().token());
                        let additional_indent_level = ctx.get_range_indent_increase(range);
                        FormatTriviaType::Append(vec![create_indent_trivia(
                            ctx,
                            additional_indent_level,
                        )])
                    }
                    false => FormatTriviaType::NoChange,
                };

                let formatted_field = format_type_field(ctx, &field, leading_trivia);
                let mut formatted_punctuation = None;

                match is_multiline {
                    true => {
                        // Continue adding a comma and a new line for multiline tables
                        // Add newline trivia to the end of the symbol
                        let symbol = match punctuation {
                            Some(punctuation) => fmt_symbol!(ctx, &punctuation, ","),
                            None => TokenReference::symbol(",").unwrap(),
                        }
                        .update_trailing_trivia(FormatTriviaType::Append(vec![
                            create_newline_trivia(ctx),
                        ]));
                        formatted_punctuation = Some(symbol)
                    }

                    false => {
                        if current_fields.peek().is_some() {
                            // Have more elements still to go
                            formatted_punctuation = match punctuation {
                                Some(punctuation) => Some(fmt_symbol!(ctx, &punctuation, ", ")),
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
            );
            let parentheses = format_contained_span(ctx, parentheses);
            let inner = Box::new(format_expression(ctx, inner, Shape::from_context(ctx)));
            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            }
        }

        TypeInfo::Tuple { parentheses, types } => {
            let parentheses = format_contained_span(ctx, parentheses);
            let types =
                try_format_punctuated(ctx, types, Shape::from_context(ctx), format_type_info_shape);

            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let left = Box::new(format_type_info(ctx, left));
            let pipe = fmt_symbol!(ctx, pipe, " | ");
            let right = Box::new(format_type_info(ctx, right));

            TypeInfo::Union { left, pipe, right }
        }

        TypeInfo::Variadic { ellipse, type_info } => {
            let ellipse = fmt_symbol!(ctx, ellipse, "...");
            let type_info = Box::new(format_type_info(ctx, type_info));

            TypeInfo::Variadic { ellipse, type_info }
        }

        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_indexed_type_info<'ast>(
    ctx: &mut Context,
    indexed_type_info: &IndexedTypeInfo<'ast>,
) -> IndexedTypeInfo<'ast> {
    match indexed_type_info {
        IndexedTypeInfo::Basic(token_reference) => {
            IndexedTypeInfo::Basic(format_token_reference(ctx, token_reference))
        }

        IndexedTypeInfo::Generic {
            base,
            arrows,
            generics,
        } => {
            let base = format_token_reference(ctx, base);
            let arrows = format_contained_span(ctx, arrows);
            let generics = try_format_punctuated(
                ctx,
                generics,
                Shape::from_context(ctx),
                format_type_info_shape,
            );
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
    ctx: &mut Context,
    type_field: &TypeField<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> TypeField<'ast> {
    let key = format_type_field_key(ctx, type_field.key(), leading_trivia);
    let colon_token = fmt_symbol!(ctx, type_field.colon_token(), ": ");
    let value = format_type_info(ctx, type_field.value());

    type_field
        .to_owned()
        .with_key(key)
        .with_colon_token(colon_token)
        .with_value(value)
}

pub fn format_type_field_key<'ast>(
    ctx: &mut Context,
    type_field_key: &TypeFieldKey<'ast>,
    leading_trivia: FormatTriviaType<'ast>,
) -> TypeFieldKey<'ast> {
    match type_field_key {
        TypeFieldKey::Name(token) => TypeFieldKey::Name(
            format_token_reference(ctx, token).update_leading_trivia(leading_trivia),
        ),
        TypeFieldKey::IndexSignature { brackets, inner } => TypeFieldKey::IndexSignature {
            brackets: format_contained_span(ctx, brackets).update_leading_trivia(leading_trivia),
            inner: format_type_info(ctx, inner),
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_type_assertion<'ast>(
    ctx: &mut Context,
    type_assertion: &TypeAssertion<'ast>,
) -> TypeAssertion<'ast> {
    let assertion_op = fmt_symbol!(ctx, type_assertion.assertion_op(), " :: ");
    let cast_to = format_type_info(ctx, type_assertion.cast_to());

    TypeAssertion::new(cast_to).with_assertion_op(assertion_op)
}

fn format_type_declaration<'ast>(
    ctx: &mut Context,
    type_declaration: &TypeDeclaration<'ast>,
    add_leading_trivia: bool,
) -> TypeDeclaration<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(type_declaration.type_token()));
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
    );

    if add_leading_trivia {
        let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
        type_token = type_token.update_leading_trivia(FormatTriviaType::Append(leading_trivia))
    }

    let type_name = format_token_reference(ctx, type_declaration.type_name());
    let generics = match type_declaration.generics() {
        Some(generics) => Some(format_generic_declaration(ctx, generics)),
        None => None,
    };
    let equal_token = fmt_symbol!(ctx, type_declaration.equal_token(), " = ");
    let type_definition = format_type_info(ctx, type_declaration.type_definition())
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
    ctx: &mut Context,
    type_declaration: &TypeDeclaration<'ast>,
    _shape: Shape,
) -> TypeDeclaration<'ast> {
    format_type_declaration(ctx, type_declaration, true)
}

pub fn format_generic_declaration<'ast>(
    ctx: &mut Context,
    generic_declaration: &GenericDeclaration<'ast>,
) -> GenericDeclaration<'ast> {
    let arrows = format_contained_span(ctx, generic_declaration.arrows());
    let generics = try_format_punctuated(
        ctx,
        generic_declaration.generics(),
        Shape::from_context(ctx),
        format_token_reference_mut,
    );

    generic_declaration
        .to_owned()
        .with_arrows(arrows)
        .with_generics(generics)
}

pub fn format_type_specifier<'ast>(
    ctx: &mut Context,
    type_specifier: &TypeSpecifier<'ast>,
) -> TypeSpecifier<'ast> {
    let punctuation = fmt_symbol!(ctx, type_specifier.punctuation(), ": ");
    let type_info = format_type_info(ctx, type_specifier.type_info());

    type_specifier
        .to_owned()
        .with_punctuation(punctuation)
        .with_type_info(type_info)
}

pub fn format_exported_type_declaration<'ast>(
    ctx: &mut Context,
    exported_type_declaration: &ExportedTypeDeclaration<'ast>,
    _shape: Shape,
) -> ExportedTypeDeclaration<'ast> {
    // Calculate trivia
    let additional_indent_level =
        ctx.get_range_indent_increase(token_range(exported_type_declaration.export_token()));
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];

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
    )
    .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let type_declaration =
        format_type_declaration(ctx, exported_type_declaration.type_declaration(), false);

    exported_type_declaration
        .to_owned()
        .with_export_token(export_token)
        .with_type_declaration(type_declaration)
}
