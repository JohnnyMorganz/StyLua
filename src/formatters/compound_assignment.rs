use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        expression::{format_expression, format_var},
        trivia::{
            strip_leading_trivia, FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia,
        },
    },
    shape::Shape,
};
use full_moon::{
    ast::{CompoundAssignment, CompoundOp},
    tokenizer::TokenReference,
};

pub fn format_compound_op(ctx: &Context, compound_op: &CompoundOp, shape: Shape) -> CompoundOp {
    fmt_op!(ctx, CompoundOp, compound_op, shape, {
        PlusEqual = " += ",
        MinusEqual = " -= ",
        StarEqual = " *= ",
        SlashEqual = " /= ",
        #[cfg(feature = "luau")]
        PercentEqual = " %= ",
        CaretEqual = " ^= ",
        #[cfg(feature = "luau")]
        TwoDotsEqual = " ..= ",
        #[cfg(feature = "luau")]
        DoubleSlashEqual = " //= ",
        #[cfg(feature = "cfxlua")]
        DoubleLessThanEqual = " <<= ",
        #[cfg(feature = "cfxlua")]
        DoubleGreaterThanEqual = " >>= ",
        #[cfg(feature = "cfxlua")]
        AmpersandEqual = " &= ",
        #[cfg(feature = "cfxlua")]
        PipeEqual = " |= ",
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
