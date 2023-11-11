use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_op, fmt_symbol,
    formatters::{
        assignment::hang_equal_token,
        expression::{format_expression, format_var},
        general::{
            format_contained_punctuated_multiline, format_contained_span, format_punctuated,
            format_symbol, format_token_reference,
        },
        table::{create_table_braces, format_multiline_table, format_singleline_table, TableType},
        trivia::{
            strip_leading_trivia, strip_trailing_trivia, strip_trivia, FormatTriviaType,
            UpdateLeadingTrivia, UpdateTrailingTrivia, UpdateTrivia,
        },
        trivia_util::{
            contains_comments, contains_singleline_comments, spans_multiple_lines,
            token_contains_comments, trivia_is_comment, trivia_is_newline, CommentSearch,
            GetLeadingTrivia, GetTrailingTrivia,
        },
    },
    shape::Shape,
};
use full_moon::ast::types::{
    CompoundAssignment, CompoundOp, ExportedTypeDeclaration, GenericDeclaration,
    GenericDeclarationParameter, GenericParameterInfo, IndexedTypeInfo, TypeArgument,
    TypeAssertion, TypeDeclaration, TypeField, TypeFieldKey, TypeInfo, TypeSpecifier,
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
    }, |other| panic!("unknown node {:?}", other))
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

// If we have a type like
// A | B | {
//    ...
// }
// we should try and hug them together if possible
fn should_hug_type(type_info: &TypeInfo) -> bool {
    match type_info {
        TypeInfo::Union { left, right, .. } | TypeInfo::Intersection { left, right, .. } => {
            should_hug_type(left) || should_hug_type(right)
        }
        TypeInfo::Table { .. } => true,
        _ => false,
    }
}

fn format_hangable_type_info_internal(
    ctx: &Context,
    type_info: &TypeInfo,
    context: TypeInfoContext,
    shape: Shape,
    hang_level: usize,
) -> TypeInfo {
    let singleline_type_info = format_type_info(ctx, type_info, shape.with_infinite_width());

    // If we can hang the type definition, and its over width, then lets try doing so
    if can_hang_type(type_info)
        && (should_hang_type(type_info, CommentSearch::Single)
            || shape.test_over_budget(&strip_trailing_trivia(&singleline_type_info)))
    {
        hang_type_info(ctx, type_info, context, shape, hang_level)
    } else {
        // Use the proper formatting
        format_type_info_internal(ctx, type_info, context, shape)
    }
}

// Formats a type info, then determines whether it is still over width. If so, it tries to hang it.
fn format_hangable_type_info(
    ctx: &Context,
    type_info: &TypeInfo,
    shape: Shape,
    hang_level: usize,
) -> TypeInfo {
    format_hangable_type_info_internal(ctx, type_info, TypeInfoContext::new(), shape, hang_level)
}

fn format_type_info_generics(
    ctx: &Context,
    arrows: &ContainedSpan,
    generics: &Punctuated<TypeInfo>,
    shape: Shape,
) -> (ContainedSpan, Punctuated<TypeInfo>) {
    const ARROW_LEN: usize = 1; // 1 = "<"

    let context = TypeInfoContext::new().mark_within_generic();

    let singleline_arrows = format_contained_span(ctx, arrows, shape);
    let singleline_generics = format_punctuated(
        ctx,
        generics,
        shape.with_infinite_width(),
        |ctx, type_info, shape| format_type_info_internal(ctx, type_info, context, shape),
    );

    let (start_arrow, end_arrow) = arrows.tokens();
    let contains_comments = start_arrow.has_trailing_comments(CommentSearch::Single)
        || end_arrow.has_leading_comments(CommentSearch::Single)
        || generics.pairs().any(|generic_pair| {
            contains_singleline_comments(generic_pair.value())
                    || generic_pair.value().has_leading_comments(CommentSearch::All) // Look for leading multiline comments - these suggest expansion
                    || generic_pair
                        .punctuation()
                        .map_or(false, contains_singleline_comments)
        });

    let should_expand = contains_comments
        || shape
            .add_width(ARROW_LEN * 2)
            .test_over_budget(&singleline_generics);

    // If the generics is just a single type table, then we can hug it
    let can_hug_table = should_expand
        && generics.len() == 1
        && match generics.iter().next().unwrap() {
            TypeInfo::Table { braces, .. } => {
                // Check there is not leading or trailing comments on the brace
                let (start_brace, end_brace) = braces.tokens();
                !start_brace.has_leading_comments(CommentSearch::Single)
                    || !end_brace.has_trailing_comments(CommentSearch::Single)
            }
            _ => false,
        };

    if should_expand && !can_hug_table {
        format_contained_punctuated_multiline(
            ctx,
            arrows,
            generics,
            |ctx, type_info, shape| {
                format_hangable_type_info_internal(ctx, type_info, context, shape, 0)
            },
            shape,
        )
    } else {
        (singleline_arrows, singleline_generics)
    }
}

#[derive(Clone, Copy)]
struct TypeInfoContext {
    // A TypeInfo within an optional type
    // we should NOT remove parentheses in a type (A | B)?
    within_optional: bool,
    // A TypeInfo within a variadic type
    // we should NOT remove parentheses in a type ...(A | B)
    within_variadic: bool,
    // A TypeInfo as a generic parameter
    // Foo<(string), (number)>
    // we should NOT remove these parentheses are they may correspond to single-type type packs
    within_generic: bool,

    /// A TypeInfo part of a union/intersection operation
    /// If its a mixed composite type, then we should not remove excess parentheses. e.g.
    /// A & (B | C)
    /// A & (B?)
    /// A | (B & C)
    /// Note, we should remove parentheses in these cases:
    /// A | (B | C)
    /// A & (B & C)
    contains_union: bool,
    contains_intersect: bool,
}

impl TypeInfoContext {
    fn new() -> Self {
        Self {
            within_optional: false,
            within_variadic: false,
            within_generic: false,
            contains_union: false,
            contains_intersect: false,
        }
    }

    fn mark_within_optional(self) -> TypeInfoContext {
        Self {
            within_optional: true,
            ..self
        }
    }

    fn mark_within_variadic(self) -> TypeInfoContext {
        Self {
            within_variadic: true,
            ..self
        }
    }

    fn mark_within_generic(self) -> TypeInfoContext {
        Self {
            within_generic: true,
            ..self
        }
    }

    fn mark_contains_union(self) -> TypeInfoContext {
        Self {
            contains_union: true,
            ..self
        }
    }

    fn mark_contains_intersect(self) -> TypeInfoContext {
        Self {
            contains_intersect: true,
            ..self
        }
    }
}

fn keep_parentheses(internal_type: &TypeInfo, context: TypeInfoContext) -> bool {
    match internal_type {
        TypeInfo::Callback { .. }
            if context.within_optional
                || context.within_variadic
                || context.contains_intersect
                || context.contains_union =>
        {
            true
        }
        TypeInfo::Union { .. } | TypeInfo::Optional { .. }
            if context.within_optional || context.within_variadic || context.contains_intersect =>
        {
            true
        }
        TypeInfo::Intersection { .. }
            if context.within_optional || context.within_variadic || context.contains_union =>
        {
            true
        }
        _ if context.within_generic => true,
        _ => false,
    }
}

pub fn format_type_info(ctx: &Context, type_info: &TypeInfo, shape: Shape) -> TypeInfo {
    format_type_info_internal(ctx, type_info, TypeInfoContext::new(), shape)
}

fn format_type_info_internal(
    ctx: &Context,
    type_info: &TypeInfo,
    context: TypeInfoContext,
    shape: Shape,
) -> TypeInfo {
    match type_info {
        TypeInfo::Array { braces, type_info } => {
            const BRACKET_LEN: usize = "{ ".len();

            let (start_brace, end_brace) = braces.tokens().to_owned();

            let contains_comments = start_brace.trailing_trivia().any(trivia_is_comment)
                || end_brace.leading_trivia().any(trivia_is_comment)
                || contains_comments(type_info);

            let (table_type, new_type_info) = if contains_comments {
                (TableType::MultiLine, None)
            } else {
                let new_type_info =
                    format_hangable_type_info(ctx, type_info, shape + BRACKET_LEN, 0);

                (
                    if spans_multiple_lines(&new_type_info) {
                        TableType::MultiLine
                    } else {
                        TableType::SingleLine
                    },
                    Some(new_type_info),
                )
            };
            let braces = create_table_braces(ctx, start_brace, end_brace, table_type, shape);

            let (new_type_info, leading_trivia, trailing_trivia) = match table_type {
                TableType::MultiLine => (
                    format_hangable_type_info(
                        ctx,
                        type_info,
                        shape.increment_additional_indent(),
                        0,
                    ),
                    FormatTriviaType::Append(vec![create_indent_trivia(
                        ctx,
                        shape.increment_additional_indent(),
                    )]),
                    FormatTriviaType::Append(vec![create_newline_trivia(ctx)]),
                ),
                _ => (
                    new_type_info.unwrap_or_else(|| format_type_info(ctx, type_info, shape)),
                    FormatTriviaType::NoChange,
                    FormatTriviaType::NoChange,
                ),
            };

            TypeInfo::Array {
                braces,
                type_info: Box::new(new_type_info.update_trivia(leading_trivia, trailing_trivia)),
            }
        }

        TypeInfo::Basic(token_reference) => {
            let token_reference = format_token_reference(ctx, token_reference, shape);
            TypeInfo::Basic(token_reference)
        }

        // Special cases for singleton types
        TypeInfo::String(string) => TypeInfo::String(format_token_reference(ctx, string, shape)),
        TypeInfo::Boolean(boolean) => {
            TypeInfo::Boolean(format_token_reference(ctx, boolean, shape))
        }

        TypeInfo::Callback {
            generics,
            parentheses,
            arguments,
            arrow,
            return_type,
        } => {
            const PAREN_LEN: usize = "(".len();
            const ARROW_LEN: usize = " -> ".len();

            let (start_parens, end_parens) = parentheses.tokens();

            let generics = generics
                .as_ref()
                .map(|generics| format_generic_declaration(ctx, generics, shape));

            let shape = match generics {
                Some(ref generics) => shape.take_last_line(&generics),
                None => shape,
            };

            let force_multiline = start_parens.has_trailing_comments(CommentSearch::All)
                || end_parens.has_leading_comments(CommentSearch::All)
                || contains_comments(arguments)
                || shape
                    .add_width(
                        PAREN_LEN * 2
                            + ARROW_LEN
                            + arguments.to_string().len()
                            + strip_trailing_trivia(&**return_type).to_string().len(),
                    )
                    .over_budget();

            let (parentheses, arguments, shape) = if force_multiline {
                let (parentheses, formatted_arguments) = format_contained_punctuated_multiline(
                    ctx,
                    parentheses,
                    arguments,
                    format_type_argument,
                    shape,
                );
                let shape = shape.reset() + PAREN_LEN;

                (parentheses, formatted_arguments, shape)
            } else {
                let parentheses = format_contained_span(ctx, parentheses, shape);
                let arguments = format_punctuated(ctx, arguments, shape + 1, format_type_argument);
                let shape = shape + (PAREN_LEN * 2 + arguments.to_string().len());

                (parentheses, arguments, shape)
            };

            let arrow = fmt_symbol!(ctx, arrow, " -> ", shape);
            let shape = shape + ARROW_LEN;
            let return_type = Box::new(format_hangable_type_info(ctx, return_type, shape, 1));

            TypeInfo::Callback {
                generics,
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
            let shape = shape.take_first_line(&base);
            let (arrows, generics) = format_type_info_generics(ctx, arrows, generics, shape);

            TypeInfo::Generic {
                base,
                arrows,
                generics,
            }
        }

        TypeInfo::GenericPack { name, ellipse } => {
            let name = format_token_reference(ctx, name, shape);
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);

            TypeInfo::GenericPack { name, ellipse }
        }

        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => {
            let left = Box::new(format_type_info_internal(
                ctx,
                left,
                context.mark_contains_intersect(),
                shape,
            ));
            let ampersand = fmt_symbol!(ctx, ampersand, " & ", shape);
            let right = Box::new(format_type_info_internal(
                ctx,
                right,
                context.mark_contains_intersect(),
                shape + 3,
            )); // 3 = " & "
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
            let base = Box::new(format_type_info_internal(
                ctx,
                base,
                context.mark_within_optional().mark_contains_union(),
                shape,
            ));
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
            let (start_brace, end_brace) = parentheses.tokens();
            let should_format_multiline = start_brace.has_trailing_comments(CommentSearch::Single)
                || end_brace.has_leading_comments(CommentSearch::Single)
                || types.pairs().any(|pair| {
                    pair.punctuation().map_or_else(
                        || pair.value().has_trailing_comments(CommentSearch::All),
                        contains_comments,
                    )
                });

            let singleline_parentheses = format_contained_span(ctx, parentheses, shape);
            let singleline_types =
                format_punctuated(ctx, types, shape + 1, |ctx, type_info, shape| {
                    format_type_info_internal(ctx, type_info, context, shape)
                }); // 1 = "("

            let (parentheses, types) = if should_format_multiline
                || shape.add_width(2).test_over_budget(&singleline_types)
            {
                format_contained_punctuated_multiline(
                    ctx,
                    parentheses,
                    types,
                    |ctx, type_info, shape| format_hangable_type_info(ctx, type_info, shape, 0),
                    shape,
                )
            } else if types.len() == 1 && !keep_parentheses(types.iter().next().unwrap(), context) {
                // If its just a single type inside parentheses, and its not a function or composite type, then remove the parens
                let internal_type = singleline_types.into_iter().next().unwrap();

                // Transfer over any comments
                return internal_type.update_trailing_trivia(FormatTriviaType::Append(
                    singleline_parentheses.tokens().1.trailing_comments(),
                ));
            } else {
                (singleline_parentheses, singleline_types)
            };

            TypeInfo::Tuple { parentheses, types }
        }

        TypeInfo::Union { left, pipe, right } => {
            let left = Box::new(format_type_info_internal(
                ctx,
                left,
                context.mark_contains_union(),
                shape,
            ));
            let pipe = fmt_symbol!(ctx, pipe, " | ", shape);
            let right = Box::new(format_type_info_internal(
                ctx,
                right,
                context.mark_contains_union(),
                shape + 3,
            )); // 3 = " | "

            TypeInfo::Union { left, pipe, right }
        }

        TypeInfo::Variadic { ellipse, type_info } => {
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);
            let type_info = Box::new(format_type_info_internal(
                ctx,
                type_info,
                context.mark_within_variadic(),
                shape + 3,
            )); // 3 = "..."

            TypeInfo::Variadic { ellipse, type_info }
        }

        TypeInfo::VariadicPack { ellipse, name } => {
            let name = format_token_reference(ctx, name, shape);
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);

            TypeInfo::VariadicPack { ellipse, name }
        }

        other => panic!("unknown node {:?}", other),
    }
}

// A clone of [`hang_binop`], except for TypeInfo tokens. TODO: can we merge the two?
fn hang_type_info_binop(
    ctx: &Context,
    binop: TokenReference,
    shape: Shape,
    rhs: &TypeInfo,
) -> TokenReference {
    // Get the leading comments of a binop, as we need to preserve them
    // Intersperse a newline and indent trivia between them
    // iter_intersperse is currently not available, so we need to do something different. Tracking issue: https://github.com/rust-lang/rust/issues/79524
    let leading_comments = binop
        .leading_trivia()
        .filter(|token| trivia_is_comment(token))
        .flat_map(|x| {
            vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
                x.to_owned(),
            ]
        })
        // If there are any comments trailing the BinOp, we need to move them to before the BinOp
        .chain(
            binop
                .trailing_trivia()
                .filter(|token| trivia_is_comment(token))
                // Prepend a single space beforehand
                .flat_map(|x| vec![Token::new(TokenType::spaces(1)), x.to_owned()]),
        )
        // If there are any leading comments to the RHS expression, we need to move them to before the BinOp
        .chain(rhs.leading_comments().iter().flat_map(|x| {
            vec![
                create_newline_trivia(ctx),
                create_indent_trivia(ctx, shape),
                x.to_owned(),
            ]
        }))
        // Create a newline just before the BinOp, and preserve the indentation
        .chain(std::iter::once(create_newline_trivia(ctx)))
        .chain(std::iter::once(create_indent_trivia(ctx, shape)))
        .collect();

    binop.update_trivia(
        FormatTriviaType::Replace(leading_comments),
        FormatTriviaType::Replace(vec![Token::new(TokenType::spaces(1))]),
    )
}

/// Hangs a type info at a pipe operator, then reformats either side with the new shape
fn hang_type_info(
    ctx: &Context,
    type_info: &TypeInfo,
    context: TypeInfoContext,
    shape: Shape,
    hang_level: usize,
) -> TypeInfo {
    const PIPE_LENGTH: usize = 2; // "| "

    let hanging_shape = shape.with_indent(shape.indent().add_indent_level(hang_level));

    match type_info {
        TypeInfo::Union { left, pipe, right } => TypeInfo::Union {
            left: Box::new(format_type_info_internal(
                ctx,
                left,
                context.mark_contains_union(),
                shape,
            )),
            pipe: hang_type_info_binop(ctx, pipe.to_owned(), hanging_shape, right),
            right: Box::new(hang_type_info(
                ctx,
                &right.update_leading_trivia(FormatTriviaType::Replace(vec![])),
                context.mark_contains_union(),
                hanging_shape.reset() + PIPE_LENGTH,
                0,
            )),
        },
        TypeInfo::Intersection {
            left,
            ampersand,
            right,
        } => TypeInfo::Intersection {
            left: Box::new(format_type_info_internal(
                ctx,
                left,
                context.mark_contains_intersect(),
                shape,
            )),
            ampersand: hang_type_info_binop(ctx, ampersand.to_owned(), hanging_shape, right),
            right: Box::new(hang_type_info(
                ctx,
                &right.update_leading_trivia(FormatTriviaType::Replace(vec![])),
                context.mark_contains_intersect(),
                hanging_shape.reset() + PIPE_LENGTH,
                0,
            )),
        },
        other => format_type_info_internal(ctx, other, context, shape),
    }
}

fn can_hang_type(type_info: &TypeInfo) -> bool {
    matches!(
        type_info,
        // Can hang a binary operation
        TypeInfo::Union { .. } | TypeInfo::Intersection { .. }
    )
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
            let shape = shape.take_first_line(&base);
            let (arrows, generics) = format_type_info_generics(ctx, arrows, generics, shape);

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
    const COLON_LEN: usize = ": ".len();

    let name = match type_argument.name() {
        Some((name, colon_token)) => {
            let name = format_token_reference(ctx, name, shape);
            let colon_token = fmt_symbol!(ctx, colon_token, ": ", shape);

            Some((name, colon_token))
        }
        None => None,
    };

    let shape = shape
        + name.as_ref().map_or(0, |(name, _)| {
            strip_trivia(name).to_string().len() + COLON_LEN
        });

    let type_info = format_hangable_type_info(ctx, type_argument.type_info(), shape, 1);

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

    // Trailing trivia consists only of single line comments - multiline comments are kept in place
    let trailing_trivia = value.trailing_comments_search(CommentSearch::Single);

    if let TableType::MultiLine = table_type {
        // If still over budget, hang the type
        if can_hang_type(type_field.value()) && shape.test_over_budget(&value) {
            value = hang_type_info(ctx, type_field.value(), TypeInfoContext::new(), shape, 1)
        };

        // Keep multiline comments in place
        let multiline_comments = value.trailing_comments_search(CommentSearch::Multiline);
        value = value.update_trailing_trivia(FormatTriviaType::Replace(multiline_comments))
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

/// Checks a type info to see if it should be hanged due to comments being present
fn should_hang_type(type_info: &TypeInfo, comment_search: CommentSearch) -> bool {
    // Only hang if its a binary type info, since it doesn't matter for unary types
    match type_info {
        TypeInfo::Union {
            left,
            pipe: binop,
            right,
        }
        | TypeInfo::Intersection {
            left,
            ampersand: binop,
            right,
        } => {
            left.has_trailing_comments(comment_search)
                || should_hang_type(left, comment_search)
                || contains_comments(binop)
                || right.has_leading_comments(comment_search)
                || should_hang_type(right, comment_search)
        }
        _ => false,
    }
}

fn attempt_assigned_type_tactics(
    ctx: &Context,
    equal_token: TokenReference,
    type_info: &TypeInfo,
    shape: Shape,
) -> (TokenReference, TypeInfo) {
    const EQUAL_TOKEN_LENGTH: usize = " = ".len();

    if token_contains_comments(&equal_token) || type_info.has_leading_comments(CommentSearch::All) {
        // We will hang at the equals token, and then format the declaration as necessary
        let equal_token = hang_equal_token(ctx, &equal_token, shape, false);

        let shape = shape.reset().increment_additional_indent();

        // Format declaration, hanging if it contains comments (ignoring leading and trailing comments, as they won't affect anything)
        let declaration = if contains_comments(strip_trivia(type_info)) {
            hang_type_info(ctx, type_info, TypeInfoContext::new(), shape, 0)
        } else {
            format_type_info(ctx, type_info, shape)
        };

        // Take the leading comments and format them nicely
        let leading_comments = type_info
            .leading_comments()
            .iter()
            .flat_map(|x| {
                vec![
                    create_indent_trivia(ctx, shape),
                    x.to_owned(),
                    create_newline_trivia(ctx),
                ]
            })
            .chain(std::iter::once(create_indent_trivia(ctx, shape)))
            .collect();

        let declaration =
            declaration.update_leading_trivia(FormatTriviaType::Replace(leading_comments));

        (equal_token, declaration)
    } else {
        let mut equal_token = equal_token;
        let type_definition;
        let singleline_type_definition =
            format_type_info(ctx, type_info, shape.with_infinite_width());
        let proper_type_definition = format_type_info(ctx, type_info, shape + EQUAL_TOKEN_LENGTH);

        // Test to see whether the type definition must be hung due to comments
        let must_hang = should_hang_type(type_info, CommentSearch::All);

        // If we can hang the type definition, and its over width, then lets try doing so
        if can_hang_type(type_info)
            && (must_hang
                || (shape.test_over_budget(&strip_trailing_trivia(&singleline_type_definition))))
        {
            // If we should hug the type, then lets check out the proper definition and see if it fits
            if !must_hang
                && should_hug_type(type_info)
                && !shape.test_over_budget(&proper_type_definition)
            {
                type_definition = proper_type_definition;
            } else {
                // Use a hanging equal token
                equal_token = hang_equal_token(ctx, &equal_token, shape, true);

                let shape = shape.reset().increment_additional_indent();
                let hanging_type_definition =
                    hang_type_info(ctx, type_info, TypeInfoContext::new(), shape, 0);
                type_definition = hanging_type_definition;
            }
        } else {
            // Test whether the proper formatting goes over the column width
            // If so, hang at the equals token and reformat
            if shape.test_over_budget(&proper_type_definition) {
                // Hang at the equal token
                equal_token = hang_equal_token(ctx, &equal_token, shape, true);

                // Add the expression list into the indent range, as it will be indented by one
                let shape = shape.reset().increment_additional_indent();
                type_definition = format_type_info(ctx, type_info, shape);
            } else {
                // Use the proper formatting
                type_definition = proper_type_definition;
            }
        }

        (equal_token, type_definition)
    }
}

fn format_type_declaration(
    ctx: &Context,
    type_declaration: &TypeDeclaration,
    add_leading_trivia: bool,
    shape: Shape,
) -> TypeDeclaration {
    const TYPE_TOKEN_LENGTH: usize = "type ".len();

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

    let shape = shape + TYPE_TOKEN_LENGTH;
    let type_name = format_token_reference(ctx, type_declaration.type_name(), shape);
    let shape = shape + type_name.to_string().len();

    let generics = type_declaration
        .generics()
        .map(|generics| format_generic_declaration(ctx, generics, shape));

    let shape = match generics {
        Some(ref generics) => shape.take_last_line(&generics),
        None => shape,
    };

    let equal_token = fmt_symbol!(ctx, type_declaration.equal_token(), " = ", shape);
    let (equal_token, type_definition) =
        attempt_assigned_type_tactics(ctx, equal_token, type_declaration.type_definition(), shape);

    // Handle comments in between the type name and generics + generics and equal token
    // (or just type name and equal token if generics not present)

    // If there are comments in between the type name and the generics, then handle them
    let (type_name, equal_token, generics) = if type_name.has_trailing_comments(CommentSearch::All)
        || generics.as_ref().map_or(false, |generics| {
            generics
                .arrows()
                .tokens()
                .0
                .has_leading_comments(CommentSearch::All)
        })
        || equal_token.has_leading_comments(CommentSearch::All)
    {
        // See if we have generics
        if let Some(generics) = generics {
            let (start_arrow, end_arrow) = generics.arrows().tokens();

            let type_name_comments = type_name
                .trailing_trivia()
                .chain(start_arrow.leading_trivia())
                .filter(|token| trivia_is_comment(token))
                .flat_map(|x| {
                    // Prepend a single space beforehand
                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                })
                .collect::<Vec<_>>();
            let type_name_comments_len = type_name_comments.len();

            let arrow_comments = end_arrow
                .trailing_trivia()
                .chain(equal_token.leading_trivia())
                .filter(|token| trivia_is_comment(token))
                .flat_map(|x| {
                    // Prepend a single space beforehand
                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                })
                .collect();

            (
                type_name.update_trailing_trivia(FormatTriviaType::Replace(type_name_comments)),
                equal_token.update_leading_trivia(FormatTriviaType::Replace(vec![Token::new(
                    TokenType::spaces(1),
                )])),
                Some(generics.to_owned().with_arrows(ContainedSpan::new(
                    start_arrow.update_leading_trivia(FormatTriviaType::Replace(
                        // If there are some comments present between the type name and generics,
                        // then lets add a single space before the arrow to make it look nicer
                        if type_name_comments_len > 0 {
                            vec![Token::new(TokenType::spaces(1))]
                        } else {
                            vec![]
                        },
                    )),
                    end_arrow.update_trailing_trivia(FormatTriviaType::Replace(arrow_comments)),
                ))),
            )
        } else {
            let comments = type_name
                .trailing_trivia()
                .chain(equal_token.leading_trivia())
                .filter(|token| trivia_is_comment(token))
                .flat_map(|x| {
                    // Prepend a single space beforehand
                    vec![Token::new(TokenType::spaces(1)), x.to_owned()]
                })
                .collect();

            (
                type_name.update_trailing_trivia(FormatTriviaType::Replace(comments)),
                equal_token.update_leading_trivia(FormatTriviaType::Replace(vec![Token::new(
                    TokenType::spaces(1),
                )])),
                generics,
            )
        }
    } else {
        (type_name, equal_token, generics)
    };

    let type_definition =
        type_definition.update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

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

fn format_generic_parameter(
    ctx: &Context,
    generic_parameter: &GenericDeclarationParameter,
    shape: Shape,
) -> GenericDeclarationParameter {
    let parameter_info = match generic_parameter.parameter() {
        GenericParameterInfo::Name(token_reference) => {
            GenericParameterInfo::Name(format_token_reference(ctx, token_reference, shape))
        }
        GenericParameterInfo::Variadic { name, ellipse } => {
            let name = format_token_reference(ctx, name, shape);
            let ellipse = fmt_symbol!(ctx, ellipse, "...", shape);

            GenericParameterInfo::Variadic { name, ellipse }
        }

        other => panic!("unknown node {:?}", other),
    };

    let default_type = match (generic_parameter.equals(), generic_parameter.default_type()) {
        (Some(equals), Some(default_type)) => {
            let equals = fmt_symbol!(ctx, equals, " = ", shape);
            let (equals, default_type) =
                attempt_assigned_type_tactics(ctx, equals, default_type, shape);
            Some((equals, default_type))
        }
        (None, None) => None,
        _ => unreachable!("have generic parameter default type with no equals or vice versa"),
    };

    generic_parameter
        .to_owned()
        .with_parameter(parameter_info)
        .with_default(default_type)
}

pub fn format_generic_declaration(
    ctx: &Context,
    generic_declaration: &GenericDeclaration,
    shape: Shape,
) -> GenericDeclaration {
    const ARROW_LEN: usize = 1; // 1 = "<"

    let singleline_arrows = format_contained_span(ctx, generic_declaration.arrows(), shape);
    let singleline_generics = format_punctuated(
        ctx,
        generic_declaration.generics(),
        shape.with_infinite_width(),
        format_generic_parameter,
    );

    let (start_arrow, end_arrow) = generic_declaration.arrows().tokens();
    let contains_comments = start_arrow.has_trailing_comments(CommentSearch::All)
        || end_arrow.has_leading_comments(CommentSearch::All)
        || contains_comments(generic_declaration.generics());

    let should_expand = contains_comments
        || shape
            .add_width(ARROW_LEN * 2)
            .test_over_budget(&singleline_generics);

    let (arrows, generics) = if should_expand {
        format_contained_punctuated_multiline(
            ctx,
            generic_declaration.arrows(),
            generic_declaration.generics(),
            format_generic_parameter, // TODO: hangable?
            shape,
        )
    } else {
        (singleline_arrows, singleline_generics)
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
