use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    CodeFormatter,
};
use full_moon::ast::lua52::{Goto, Label};
use full_moon::tokenizer::TokenReference;

impl CodeFormatter {
    pub fn format_goto<'ast>(&self, goto: &Goto<'ast>) -> Goto<'ast> {
        // Calculate trivia
        let additional_indent_level =
            self.get_range_indent_increase(CodeFormatter::get_token_range(goto.goto_token()));
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let goto_token = trivia_formatter::token_reference_add_trivia(
            crate::fmt_symbol!(self, goto.goto_token(), "goto "),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::NoChange,
        );

        let label_name = trivia_formatter::token_reference_add_trivia(
            self.format_token_reference(goto.label_name()),
            FormatTriviaType::NoChange,
            FormatTriviaType::Append(trailing_trivia),
        );

        Goto::new(label_name).with_goto_token(goto_token)
    }

    pub fn format_label<'ast>(&self, label: &Label<'ast>) -> Label<'ast> {
        // Calculate trivia
        let additional_indent_level =
            self.get_range_indent_increase(CodeFormatter::get_token_range(label.left_colons()));
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let left_colons = trivia_formatter::token_reference_add_trivia(
            crate::fmt_symbol!(self, label.left_colons(), "::"),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::NoChange,
        );
        let name = self.format_token_reference(label.body());

        let right_colons = trivia_formatter::token_reference_add_trivia(
            crate::fmt_symbol!(self, label.right_colons(), "::"),
            FormatTriviaType::NoChange,
            FormatTriviaType::Append(trailing_trivia),
        );

        Label::new(name)
            .with_left_colons(left_colons)
            .with_right_colons(right_colons)
    }
}
