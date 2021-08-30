use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_op, fmt_symbol,
    formatters::{
        expression::{format_expression, format_var},
        general::{
            format_contained_span, format_end_token, format_punctuated,
            format_punctuated_multiline, format_symbol, format_token_reference,
            try_format_punctuated, EndTokenType,
        },
        table::{create_table_braces, format_multiline_table, format_singleline_table, TableType},
        trivia::{
            strip_leading_trivia, strip_trailing_trivia, strip_trivia, FormatTriviaType,
            UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
        trivia_util::{
            contains_comments, token_trivia_contains_comments, trivia_is_comment,
            trivia_is_newline, type_info_trailing_trivia,
        },
    },
    shape::Shape,
};
use full_moon::ast::types::{
    CompoundAssignment, CompoundOp, ExportedTypeDeclaration, GenericDeclaration, IndexedTypeInfo,
    TypeArgument, TypeAssertion, TypeDeclaration, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
};
use full_moon::ast::{punctuated::Punctuated, span::ContainedSpan};
use full_moon::tokenizer::{Token, TokenReference, TokenType};
use std::boxed::Box;

pub fn format_compound_op(ctx: &Context, compound_op: &CompoundOp, shape: Shape) -> CompoundOp {
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

pub fn format_compound_assignment(
    ctx: &Context,
    compound_assignment: &CompoundAssignment,
    shape: Shape,
) -> CompoundAssignment {
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

pub fn format_type_info(ctx: &Context, type_info: &TypeInfo, shape: Shape) -> TypeInfo {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            let (start_brace, end_brace) = braces.tokens().to_owned();
            let braces = ContainedSpan::new(
                fmt_symbol!(ctx, start_brace, "{ ", shape),
                fmt_symbol!(ctx, end_brace, " }", shape),
            );
            let type_info = Box::new(format_type_info(ctx, type_info, shape + 2)); // 2 = "{ "

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
            let (start_parens, end_parens) = parentheses.tokens();

            let force_multiline = token_trivia_contains_comments(start_parens.trailing_trivia())
                || token_trivia_contains_comments(end_parens.leading_trivia())
                || contains_comments(arguments)
                || shape
                    .add_width(
                        2 + 4
                            + arguments.to_string().len()
                            + strip_trailing_trivia(&**return_type).to_string().len(),
                    )
                    .over_budget(); // 2 = opening/closing parens, 4 = " -> "

            let (parentheses, arguments, shape) = if force_multiline {
                let start_parens = fmt_symbol!(ctx, start_parens, "(", shape)
                    .update_trailing_trivia(FormatTriviaType::Append(vec![
                        create_newline_trivia(ctx),
                        create_indent_trivia(ctx, shape.increment_additional_indent()),
                    ]));
                let end_parens =
                    format_end_token(ctx, end_parens, EndTokenType::ClosingParens, shape)
                        .update_leading_trivia(FormatTriviaType::Append(vec![
                            create_newline_trivia(ctx),
                            create_indent_trivia(ctx, shape),
                        ]));

                let parentheses = ContainedSpan::new(start_parens, end_parens);

                let arguments = format_punctuated_multiline(
                    ctx,
                    arguments,
                    shape.reset(),
                    format_type_argument,
                    Some(1),
                );

                let shape = shape.reset() + 1; // 1 = ")"

                (parentheses, arguments, shape)
            } else {
                let parentheses = format_contained_span(ctx, parentheses, shape);
                let arguments = format_punctuated(ctx, arguments, shape + 1, format_type_argument);
                let shape = shape + (2 + arguments.to_string().len()); // 2 = opening and closing parens

                (parentheses, arguments, shape)
            };

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
            let generics = try_format_punctuated(
                ctx,
                generics,
                shape + (strip_trivia(&base).to_string().len() + 1), // 1 = "<"
                format_type_info,
                None,
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
            let left = Box::new(format_type_info(ctx, left, shape));
            let ampersand = fmt_symbol!(ctx, ampersand, " & ", shape);
            let right = Box::new(format_type_info(ctx, right, shape + 3)); // 3 = " & "
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
            let type_info = Box::new(format_indexed_type_info(
                ctx,
                type_info,
                shape + (strip_trivia(&module).to_string().len() + 1), // 1 = "."
            ));
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
            let contains_comments = start_brace.trailing_trivia().any(trivia_is_comment)
                || end_brace.leading_trivia().any(trivia_is_comment)
                || fields.pairs().any(|field| {
                    contains_comments(field.punctuation()) || contains_comments(field.value())
                });

            let table_type = match (contains_comments, fields.iter().next()) {
                // Table contains comments, so force multiline
                (true, _) => TableType::MultiLine,

                (false, Some(_)) => {
                    let braces_range = (
                        start_brace.token().end_position().bytes(),
                        end_brace.token().start_position().bytes(),
                    );

                    let singleline_shape = shape + (braces_range.1 - braces_range.0);

                    match singleline_shape.over_budget() {
                        true => TableType::MultiLine,
                        false => {
                            // Determine if there was a new line at the end of the start brace
                            // If so, then we should always be multiline
                            if start_brace.trailing_trivia().any(trivia_is_newline) {
                                TableType::MultiLine
                            } else {
                                TableType::SingleLine
                            }
                        }
                    }
                }

                (false, None) => TableType::Empty,
            };

            let (braces, fields) = match table_type {
                TableType::Empty => {
                    let braces =
                        create_table_braces(ctx, start_brace, end_brace, table_type, shape);
                    (braces, Punctuated::new())
                }
                TableType::SingleLine => {
                    format_singleline_table(ctx, braces, fields, format_type_field, shape)
                }
                TableType::MultiLine => {
                    format_multiline_table(ctx, braces, fields, format_type_field, shape)
                }
            };

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
                        identifier: "typeof".into(),
                    }),
                    vec![],
                ),
                shape,
            );
            let shape = shape + 6; // 6 = "typeof"
            let parentheses = format_contained_span(ctx, parentheses, shape);
            let inner = Box::new(format_expression(ctx, inner, shape + 1)); // 1 = "("
            TypeInfo::Typeof {
                typeof_token,
                parentheses,
                inner,
            }
        }

        TypeInfo::Tuple { parentheses, types } => {
            let parentheses = format_contained_span(ctx, parentheses, shape);
            let types = try_format_punctuated(ctx, types, shape + 1, format_type_info, None); // 1 = "("

            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let left = Box::new(format_type_info(ctx, left, shape));
            let pipe = fmt_symbol!(ctx, pipe, " | ", shape);
            let right = Box::new(format_type_info(ctx, right, shape + 3)); // 3 = " | "

            TypeInfo::Union { left, pipe, right }
        }

        TypeInfo::Variadic { ellipse, type_info } => {
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);
            let type_info = Box::new(format_type_info(ctx, type_info, shape + 3)); // 3 = "..."

            TypeInfo::Variadic { ellipse, type_info }
        }

        other => panic!("unknown node {:?}", other),
    }
}

pub fn hang_type_info(ctx: &Context, type_info: TypeInfo, shape: Shape) -> TypeInfo {
    match type_info {
        TypeInfo::Union { left, pipe, right } => TypeInfo::Union {
            left,
            pipe: pipe.update_leading_trivia(FormatTriviaType::Replace(vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
            ])),
            right: Box::new(hang_type_info(ctx, *right, shape)),
        },
        _ => type_info,
    }
}

pub fn format_indexed_type_info(
    ctx: &Context,
    indexed_type_info: &IndexedTypeInfo,
    shape: Shape,
) -> IndexedTypeInfo {
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
            let generics = try_format_punctuated(
                ctx,
                generics,
                shape + (strip_trivia(&base).to_string().len() + 1), // 1 = "<"
                format_type_info,
                None,
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

fn format_type_argument(ctx: &Context, type_argument: &TypeArgument, shape: Shape) -> TypeArgument {
    let name = match type_argument.name() {
        Some((name, colon_token)) => {
            let name = format_token_reference(ctx, name, shape);
            let colon_token = fmt_symbol!(ctx, colon_token, ": ", shape);

            Some((name, colon_token))
        }
        None => None,
    };

    let type_info = format_type_info(
        ctx,
        type_argument.type_info(),
        shape
            + name
                .as_ref()
                .map_or(0, |(name, _)| strip_trivia(name).to_string().len() + 2), // 2 = ": "
    );

    type_argument
        .to_owned()
        .with_name(name)
        .with_type_info(type_info)
}

/// Formats a [`TypeField`] present inside of a [`TypeInfo::Table`]
/// Returns the new [`TypeField`] and any trailing trivia associated with its value (as this may need to later be moved).
/// If the [`TableType`] provided is [`TableType::MultiLine`] then the trailing trivia from the value will be removed.
pub fn format_type_field(
    ctx: &Context,
    type_field: &TypeField,
    table_type: TableType,
    shape: Shape,
) -> (TypeField, Vec<Token>) {
    let leading_trivia = match table_type {
        TableType::MultiLine => FormatTriviaType::Append(vec![create_indent_trivia(ctx, shape)]),
        _ => FormatTriviaType::NoChange,
    };

    let key = format_type_field_key(ctx, type_field.key(), leading_trivia, shape);
    let colon_token = fmt_symbol!(ctx, type_field.colon_token(), ": ", shape);
    let shape = shape + (strip_leading_trivia(&key).to_string().len() + 2);
    let mut value = format_type_info(ctx, type_field.value(), shape);

    let trailing_trivia = type_info_trailing_trivia(&value);

    if let TableType::MultiLine = table_type {
        value = value.update_trailing_trivia(FormatTriviaType::Replace(vec![]))
    }

    (
        type_field
            .to_owned()
            .with_key(key)
            .with_colon_token(colon_token)
            .with_value(value),
        trailing_trivia,
    )
}

pub fn format_type_field_key(
    ctx: &Context,
    type_field_key: &TypeFieldKey,
    leading_trivia: FormatTriviaType,
    shape: Shape,
) -> TypeFieldKey {
    match type_field_key {
        TypeFieldKey::Name(token) => TypeFieldKey::Name(
            format_token_reference(ctx, token, shape).update_leading_trivia(leading_trivia),
        ),
        TypeFieldKey::IndexSignature { brackets, inner } => TypeFieldKey::IndexSignature {
            brackets: format_contained_span(ctx, brackets, shape)
                .update_leading_trivia(leading_trivia),
            inner: format_type_info(ctx, inner, shape + 1), // 1 = "["
        },
        other => panic!("unknown node {:?}", other),
    }
}

pub fn format_type_assertion(
    ctx: &Context,
    type_assertion: &TypeAssertion,
    shape: Shape,
) -> TypeAssertion {
    let assertion_op = fmt_symbol!(ctx, type_assertion.assertion_op(), " :: ", shape);
    let cast_to = format_type_info(ctx, type_assertion.cast_to(), shape + 4); // 4 = " :: "

    TypeAssertion::new(cast_to).with_assertion_op(assertion_op)
}

fn format_type_declaration(
    ctx: &Context,
    type_declaration: &TypeDeclaration,
    add_leading_trivia: bool,
    shape: Shape,
) -> TypeDeclaration {
    // Calculate trivia
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let mut type_token = format_symbol(
        ctx,
        type_declaration.type_token(),
        &TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: "type".into(),
            }),
            vec![Token::new(TokenType::spaces(1))],
        ),
        shape,
    );

    if add_leading_trivia {
        let leading_trivia = vec![create_indent_trivia(ctx, shape)];
        type_token = type_token.update_leading_trivia(FormatTriviaType::Append(leading_trivia))
    }

    let shape = shape + 5; // 5 = "type "
    let type_name = format_token_reference(ctx, type_declaration.type_name(), shape);
    let shape = shape + type_name.to_string().len();

    let generics = type_declaration
        .generics()
        .map(|generics| format_generic_declaration(ctx, generics, shape));

    let shape = match generics {
        Some(ref generics) => shape.take_last_line(&generics),
        None => shape,
    };

    let mut equal_token = fmt_symbol!(ctx, type_declaration.equal_token(), " = ", shape);
    let mut type_definition =
        format_type_info(ctx, type_declaration.type_definition(), shape + 3) // 3 = " = "
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    let shape = shape.take_last_line(&strip_trailing_trivia(&type_definition));

    if shape.over_budget() {
        let shape = shape.increment_additional_indent();
        equal_token = equal_token.update_trailing_trivia(FormatTriviaType::Replace(vec![
            create_newline_trivia(ctx),
            create_indent_trivia(ctx, shape),
        ]));
        type_definition = hang_type_info(ctx, type_definition, shape);
    }

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
pub fn format_type_declaration_stmt(
    ctx: &Context,
    type_declaration: &TypeDeclaration,
    shape: Shape,
) -> TypeDeclaration {
    format_type_declaration(ctx, type_declaration, true, shape)
}

pub fn format_generic_declaration(
    ctx: &Context,
    generic_declaration: &GenericDeclaration,
    shape: Shape,
) -> GenericDeclaration {
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

pub fn format_type_specifier(
    ctx: &Context,
    type_specifier: &TypeSpecifier,
    shape: Shape,
) -> TypeSpecifier {
    let punctuation = fmt_symbol!(ctx, type_specifier.punctuation(), ": ", shape);
    let type_info = format_type_info(ctx, type_specifier.type_info(), shape + 2); // 2 = ": "

    type_specifier
        .to_owned()
        .with_punctuation(punctuation)
        .with_type_info(type_info)
}

pub fn format_exported_type_declaration(
    ctx: &Context,
    exported_type_declaration: &ExportedTypeDeclaration,
    shape: Shape,
) -> ExportedTypeDeclaration {
    // Calculate trivia
    let shape = shape.reset();
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];

    let export_token = format_symbol(
        ctx,
        exported_type_declaration.export_token(),
        &TokenReference::new(
            vec![],
            Token::new(TokenType::Identifier {
                identifier: "export".into(),
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
        shape + 7, // 7 = "export "
    );

    exported_type_declaration
        .to_owned()
        .with_export_token(export_token)
        .with_type_declaration(type_declaration)
}
