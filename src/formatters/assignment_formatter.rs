#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, Expression, LocalAssignment,
};
use full_moon::node::Node;
use full_moon::tokenizer::{TokenKind, TokenReference};

use crate::formatters::{
    trivia_formatter::{FormatTriviaType, UpdateLeadingTrivia, UpdateTrailingTrivia},
    trivia_util, CodeFormatter,
};

/// Returns an Assignment with leading and trailing trivia removed
fn strip_assignment_trivia<'ast>(assignment: &Assignment<'ast>) -> Assignment<'ast> {
    let var_list = assignment
        .variables()
        .update_leading_trivia(FormatTriviaType::Replace(vec![]));
    let expr_list = assignment
        .expressions()
        .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

    Assignment::new(var_list, expr_list).with_equal_token(assignment.equal_token().to_owned())
}

/// Returns a LocalAssignment with leading and trailing trivia removed
fn strip_local_assignment_trivia<'ast>(
    local_assignment: &LocalAssignment<'ast>,
) -> LocalAssignment<'ast> {
    let local_token = local_assignment
        .local_token()
        .update_leading_trivia(FormatTriviaType::Replace(vec![]));

    if local_assignment.expressions().is_empty() {
        let name_list = local_assignment
            .names()
            .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

        LocalAssignment::new(name_list).with_local_token(local_token)
    } else {
        let expr_list = local_assignment
            .expressions()
            .update_trailing_trivia(FormatTriviaType::Replace(vec![]));

        LocalAssignment::new(local_assignment.names().to_owned())
            .with_local_token(local_token)
            .with_equal_token(local_assignment.equal_token().map(|x| x.to_owned()))
            .with_expressions(expr_list)
    }
}

impl CodeFormatter {
    fn hang_punctuated_list<'ast>(
        &mut self,
        punctuated: &Punctuated<'ast, Expression<'ast>>,
        additional_indent_level: Option<usize>,
    ) -> Punctuated<'ast, Expression<'ast>> {
        // Add the expression list into the indent range, as it will be indented by one
        let expr_range = punctuated
            .range()
            .expect("no range for assignment punctuated list");
        self.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));
        let mut output = Punctuated::new();

        // Format each expression and hang them
        // We need to format again because we will now take into account the indent increase
        for pair in punctuated.pairs() {
            let expr = self.format_expression(pair.value());
            let value =
                self.hang_expression_no_trailing_newline(expr, additional_indent_level, None);
            output.push(Pair::new(
                value,
                pair.punctuation()
                    .map(|x| crate::fmt_symbol!(self, x, ", ")),
            ))
        }

        output
    }

    /// Checks the list of assigned expressions to see if any were hangable.
    /// If not, then we still have a long list of assigned expressions - we split it onto a newline at the equal token.
    /// Returns the new equal token [`TokenReference`]
    fn check_long_expression<'ast>(
        &mut self,
        expressions: &Punctuated<'ast, Expression<'ast>>,
        equal_token: TokenReference<'ast>,
        additional_indent_level: Option<usize>,
    ) -> TokenReference<'ast> {
        // See if any of our expressions were hangable.
        // If not, then its still a big long line - we should newline at the end of the equals token,
        // then indent the first item
        if !expressions
            .iter()
            .any(|x| trivia_util::can_hang_expression(x))
        {
            let equal_token_trailing_trivia = vec![
                self.create_newline_trivia(),
                self.create_indent_trivia(additional_indent_level.or(Some(0)).map(|x| x + 1)),
            ]
            .iter()
            .chain(
                // Remove the space that was present after the equal token
                equal_token
                    .trailing_trivia()
                    .skip_while(|x| x.token_kind() == TokenKind::Whitespace),
            )
            .map(|x| x.to_owned())
            .collect();

            equal_token
                .update_trailing_trivia(FormatTriviaType::Replace(equal_token_trailing_trivia))
        } else {
            equal_token
        }
    }

    pub fn format_assignment<'ast>(&mut self, assignment: &Assignment<'ast>) -> Assignment<'ast> {
        // Calculate trivia - pick an arbitrary range within the whole assignment expression to see if
        // indentation is required
        // Leading trivia added to before the var_list, trailing trivia added to the end of the expr_list
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(assignment.equal_token().token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let (var_list, var_comments_buf) =
            self.format_punctuated(assignment.variables(), &CodeFormatter::format_var);

        let (mut expr_list, expr_comments_buf) =
            self.format_punctuated(assignment.expressions(), &CodeFormatter::format_expression);

        let mut equal_token = crate::fmt_symbol!(self, assignment.equal_token(), " = ");

        // Create preliminary assignment
        let formatted_assignment = Assignment::new(var_list.to_owned(), expr_list.to_owned())
            .with_equal_token(equal_token.to_owned());

        // Test whether we need to hang the expression, using the updated assignment
        // We have to format normally before this, since we may be expanding the expression onto multiple lines
        // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
        let indent_spacing =
            (self.indent_level + additional_indent_level.unwrap_or(0)) * self.config.indent_width;
        let require_multiline_expression = indent_spacing
            + strip_assignment_trivia(&formatted_assignment)
                .to_string()
                .lines()
                .next()
                .expect("no lines")
                .len()
            > self.config.column_width
            || assignment.expressions().pairs().any(|pair| {
                pair.punctuation()
                    .map_or(false, |punc| trivia_util::token_contains_comments(punc))
                    || trivia_util::expression_contains_inline_comments(pair.value())
            });

        if require_multiline_expression {
            expr_list =
                self.hang_punctuated_list(assignment.expressions(), additional_indent_level);

            equal_token = self.check_long_expression(
                assignment.expressions(),
                equal_token,
                additional_indent_level,
            );
        }

        // Add any trailing trivia to the end of the expression list
        let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(
            var_comments_buf
                .iter()
                .chain(expr_comments_buf.iter())
                .chain(trailing_trivia.iter())
                .map(|x| x.to_owned())
                .collect(),
        ));

        // Add on leading trivia
        let formatted_var_list =
            var_list.update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        formatted_assignment
            .with_variables(formatted_var_list)
            .with_equal_token(equal_token)
            .with_expressions(expr_list)
    }

    pub fn format_local_assignment<'ast>(
        &mut self,
        assignment: &LocalAssignment<'ast>,
    ) -> LocalAssignment<'ast> {
        // Calculate trivia - pick an arbitrary range within the whole local assignment expression to see if
        // indentation is required
        // Leading trivia added to before the local token, and trailing trivia added to the end of the expr_list, or name_list if no expr_list provided
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(assignment.local_token().token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let trailing_trivia = vec![self.create_newline_trivia()];

        let local_token = crate::fmt_symbol!(self, assignment.local_token(), "local ")
            .update_leading_trivia(FormatTriviaType::Append(leading_trivia));

        let (mut name_list, name_list_comments_buf) = self.format_punctuated(
            assignment.names(),
            &CodeFormatter::format_token_reference_mut,
        );

        #[cfg(feature = "luau")]
        let mut type_specifiers: Vec<Option<TypeSpecifier<'ast>>> = assignment
            .type_specifiers()
            .map(|x| match x {
                Some(type_specifier) => Some(self.format_type_specifier(type_specifier)),
                None => None,
            })
            .collect();

        if assignment.expressions().is_empty() {
            // See if the last variable assigned has a type specifier, and add a new line to that
            #[allow(unused_mut)]
            let mut new_line_added = false;

            #[cfg(feature = "luau")]
            if let Some(Some(specifier)) = type_specifiers.pop() {
                type_specifiers.push(Some(specifier.update_trailing_trivia(
                    FormatTriviaType::Append(trailing_trivia.to_owned()),
                )));
                new_line_added = true;
            }

            // Add any trailing trivia to the end of the expression list
            name_list =
                name_list.update_trailing_trivia(FormatTriviaType::Append(match new_line_added {
                    true => name_list_comments_buf,
                    false => name_list_comments_buf
                        .iter()
                        .chain(trailing_trivia.iter())
                        .map(|x| x.to_owned())
                        .collect(),
                }));

            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(None)
                .with_expressions(Punctuated::new());

            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
            local_assignment
        } else {
            let mut equal_token =
                crate::fmt_symbol!(self, assignment.equal_token().unwrap(), " = ");
            // Format the expression normally
            let (mut expr_list, expr_comments_buf) =
                self.format_punctuated(assignment.expressions(), &CodeFormatter::format_expression);
            // Create our preliminary new assignment
            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(Some(equal_token.to_owned()))
                .with_expressions(expr_list.to_owned());
            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);

            // Test whether we need to hang the expression, using the updated assignment
            // We have to format normally before this, since we may be expanding the expression onto multiple lines
            // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
            let indent_spacing = (self.indent_level + additional_indent_level.unwrap_or(0))
                * self.config.indent_width;
            let require_multiline_expression = indent_spacing
                + strip_local_assignment_trivia(&local_assignment)
                    .to_string()
                    .lines()
                    .next()
                    .expect("no lines")
                    .len()
                > self.config.column_width
                || assignment.expressions().pairs().any(|pair| {
                    pair.punctuation()
                        .map_or(false, |punc| trivia_util::token_contains_comments(punc))
                        || trivia_util::expression_contains_inline_comments(pair.value())
                })
                || !name_list_comments_buf.is_empty();

            // Format the expression depending on whether we are multline or not
            if require_multiline_expression {
                expr_list =
                    self.hang_punctuated_list(assignment.expressions(), additional_indent_level);

                equal_token = self.check_long_expression(
                    assignment.expressions(),
                    equal_token,
                    additional_indent_level,
                );
            }

            // Add any trailing trivia to the end of the expression list
            let expr_list = expr_list.update_trailing_trivia(FormatTriviaType::Append(
                name_list_comments_buf
                    .iter()
                    .chain(expr_comments_buf.iter())
                    .chain(trailing_trivia.iter())
                    .map(|x| x.to_owned())
                    .collect(),
            ));

            // Update our local assignment
            local_assignment
                .with_equal_token(Some(equal_token))
                .with_expressions(expr_list)
        }
    }
}
