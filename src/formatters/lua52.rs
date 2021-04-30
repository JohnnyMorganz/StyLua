use crate::{
    context::{create_indent_trivia, create_newline_trivia, Context},
    fmt_symbol,
    formatters::{
        general::format_token_reference,
        trivia::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
        util::token_range,
    },
    shape::Shape,
};
use full_moon::ast::lua52::{Goto, Label};
use full_moon::tokenizer::TokenReference;

pub fn format_goto<'ast>(ctx: &Context, goto: &Goto<'ast>, _shape: Shape) -> Goto<'ast> {
    // Calculate trivia
    let additional_indent_level = ctx.get_range_indent_increase(token_range(goto.goto_token()));
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let goto_token = fmt_symbol!(ctx, goto.goto_token(), "goto ")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

    let label_name = format_token_reference(ctx, goto.label_name())
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Goto::new(label_name).with_goto_token(goto_token)
}

pub fn format_label<'ast>(ctx: &Context, label: &Label<'ast>, _shape: Shape) -> Label<'ast> {
    // Calculate trivia
    let additional_indent_level = ctx.get_range_indent_increase(token_range(label.left_colons()));
    let leading_trivia = vec![create_indent_trivia(ctx, additional_indent_level)];
    let trailing_trivia = vec![create_newline_trivia(ctx)];

    let left_colons = fmt_symbol!(ctx, label.left_colons(), "::")
        .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
    let name = format_token_reference(ctx, label.name());

    let right_colons = fmt_symbol!(ctx, label.right_colons(), "::")
        .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

    Label::new(name)
        .with_left_colons(left_colons)
        .with_right_colons(right_colons)
}
