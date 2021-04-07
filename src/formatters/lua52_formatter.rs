use crate::formatters::{
    trivia_formatter::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
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

        let goto_token = crate::fmt_symbol!(self, goto.goto_token(), "goto ")
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        let label_name = self
            .format_token_reference(goto.label_name())
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        Goto::new(label_name).with_goto_token(goto_token)
    }

    pub fn format_label<'ast>(&self, label: &Label<'ast>) -> Label<'ast> {
        // Calculate trivia
        let additional_indent_level =
            self.get_range_indent_increase(CodeFormatter::get_token_range(label.left_colons()));
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let left_colons = crate::fmt_symbol!(self, label.left_colons(), "::")
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));
        let name = self.format_token_reference(label.name());

        let right_colons = crate::fmt_symbol!(self, label.right_colons(), "::")
            .update_trailing_trivia(FormatTriviaType::Append(trailing_trivia));

        Label::new(name)
            .with_left_colons(left_colons)
            .with_right_colons(right_colons)
    }
}
