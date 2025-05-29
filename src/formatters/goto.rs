use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        general::format_token_reference,
        trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
    },
    shape::Shape,
};
use full_moon::ast::lua52::{Goto, Label};
use full_moon::tokenizer::TokenReference;

pub fn format_goto(ctx: &Context, goto: &Goto, shape: Shape) -> Goto {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let goto_token = fmt_symbol!(ctx, goto.goto_token(), "goto ", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

    let label_name = format_token_reference(ctx, goto.label_name(), shape)
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Goto::new(label_name).with_goto_token(goto_token)
}

pub fn format_goto_no_trivia(ctx: &Context, goto: &Goto, shape: Shape) -> Goto {
    let goto_token = fmt_symbol!(ctx, goto.goto_token(), "goto ", shape);
    let label_name = format_token_reference(ctx, goto.label_name(), shape);

    Goto::new(label_name).with_goto_token(goto_token)
}

pub fn format_label(ctx: &Context, label: &Label, shape: Shape) -> Label {
    // Calculate trivia
    let leading_trivia = vec![create_indent_trivia(ctx, shape)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let left_colons = fmt_symbol!(ctx, label.left_colons(), "::", shape)
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let name = format_token_reference(ctx, label.name(), shape);

    let right_colons = fmt_symbol!(ctx, label.right_colons(), "::", shape)
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Label::new(name)
        .with_left_colons(left_colons)
        .with_right_colons(right_colons)
}
