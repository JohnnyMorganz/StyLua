#[cfg(feature = "luau")]
use full_moon::ast::types::TypeSpecifier;
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    Assignment, LocalAssignment,
};
use full_moon::node::Node;
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    trivia_util, CodeFormatter,
};

impl CodeFormatter {
    /// Returns an Assignment with leading and trailing trivia removed
    fn strip_assignment_trivia<'ast>(assignment: &Assignment<'ast>) -> Assignment<'ast> {
        let mut var_list = Punctuated::new();
        let mut added_first = false;

        for pair in assignment.variables().pairs() {
            if added_first {
                var_list.push(pair.to_owned());
            } else {
                var_list.push(pair.to_owned().map(|value| {
                    trivia_formatter::var_add_leading_trivia(
                        value,
                        FormatTriviaType::Replace(vec![]),
                    )
                }));
                added_first = true;
            }
        }

        let mut expr_list = assignment.expressions().to_owned();
        if let Some(last_pair) = expr_list.pop() {
            expr_list.push(last_pair.map(|value| {
                trivia_formatter::expression_add_trailing_trivia(
                    value,
                    FormatTriviaType::Replace(vec![]),
                )
            }));
        }

        Assignment::new(var_list, expr_list)
            .with_equal_token(Cow::Owned(assignment.equal_token().to_owned()))
    }

    pub fn format_assignment<'ast>(&mut self, assignment: &Assignment<'ast>) -> Assignment<'ast> {
        // Calculate trivia - pick an arbitrary range within the whole assignment expression to see if
        // indentation is required
        // Leading trivia added to before the var_list, trailing trivia added to the end of the expr_list
        let additional_indent_level = self.get_range_indent_increase(
            CodeFormatter::get_token_range(assignment.equal_token().token()),
        );
        let leading_trivia = vec![self.create_indent_trivia(additional_indent_level)];
        let mut trailing_trivia = vec![self.create_newline_trivia()];

        let (var_list, mut var_comments_buf) =
            self.format_punctuated(assignment.variables(), &CodeFormatter::format_var);

        let (mut expr_list, mut expr_comments_buf) =
            self.format_punctuated(assignment.expressions(), &CodeFormatter::format_expression);

        // Create preliminary assignment
        let formatted_assignment = Assignment::new(var_list.to_owned(), expr_list.to_owned())
            .with_equal_token(Cow::Owned(TokenReference::symbol(" = ").unwrap()));

        // Test whether we need to hang the expression, using the updated assignment
        // We have to format normally before this, since we may be expanding the expression onto multiple lines
        // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
        let indent_spacing =
            (self.indent_level + additional_indent_level.unwrap_or(0)) * self.config.indent_width;
        let require_multiline_expression = indent_spacing
            + CodeFormatter::strip_assignment_trivia(&formatted_assignment)
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
            // Add the expression list into the indent range, as it will be indented by one
            let expr_range = assignment
                .expressions()
                .range()
                .expect("no range for assignment expr");
            self.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

            expr_list = Punctuated::new();
            // Format each expression and hang them
            // We need to format again because we will now take into account the indent increase
            for pair in assignment.expressions().pairs() {
                let expr = self.format_expression(pair.value());
                let value =
                    self.hang_expression_no_trailing_newline(expr, additional_indent_level, None);
                expr_list.push(Pair::new(
                    value,
                    pair.punctuation()
                        .map(|x| crate::fmt_symbol!(self, x, ", ")),
                ))
            }
        }

        // Add any trailing trivia to the lasts expression
        match expr_list.pop() {
            Some(pair) => {
                var_comments_buf.append(&mut expr_comments_buf);
                // Add on trailing trivia
                var_comments_buf.append(&mut trailing_trivia);
                let pair = pair.map(|expr| {
                    trivia_formatter::expression_add_trailing_trivia(
                        expr,
                        FormatTriviaType::Append(var_comments_buf),
                    )
                });
                expr_list.push(pair);
            }
            None => panic!("assignment with no expression"),
        }

        // Add on leading trivia
        let mut formatted_var_list = Punctuated::new();
        let mut iterator = var_list.pairs();

        // Retrieve first item and add indent to trailing trivia
        if let Some(first_pair) = iterator.next() {
            let updated_pair = first_pair.to_owned().map(|value| {
                trivia_formatter::var_add_leading_trivia(
                    value,
                    FormatTriviaType::Append(leading_trivia),
                )
            });
            formatted_var_list.push(updated_pair);
        }
        for pair in iterator {
            formatted_var_list.push(pair.to_owned())
        }

        formatted_assignment
            .with_variables(formatted_var_list)
            .with_expressions(expr_list)
    }

    /// Returns a LocalAssignment with leading and trailing trivia removed
    fn strip_local_assignment_trivia<'ast>(
        local_assignment: &LocalAssignment<'ast>,
    ) -> LocalAssignment<'ast> {
        let local_token = trivia_formatter::token_reference_add_trivia(
            local_assignment.local_token().to_owned(),
            FormatTriviaType::Replace(vec![]),
            FormatTriviaType::NoChange,
        );

        if local_assignment.expressions().is_empty() {
            let mut name_list = local_assignment.names().to_owned();
            if let Some(last_pair) = name_list.pop() {
                name_list.push(last_pair.map(|value| {
                    Cow::Owned(trivia_formatter::token_reference_add_trivia(
                        value.into_owned(),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Replace(vec![]),
                    ))
                }));
            }

            LocalAssignment::new(name_list).with_local_token(Cow::Owned(local_token))
        } else {
            let mut expr_list = local_assignment.expressions().to_owned();
            if let Some(last_pair) = expr_list.pop() {
                expr_list.push(last_pair.map(|value| {
                    trivia_formatter::expression_add_trailing_trivia(
                        value,
                        FormatTriviaType::Replace(vec![]),
                    )
                }));
            }
            LocalAssignment::new(local_assignment.names().to_owned())
                .with_local_token(Cow::Owned(local_token))
                .with_equal_token(
                    local_assignment
                        .equal_token()
                        .map(|x| Cow::Owned(x.to_owned())),
                )
                .with_expressions(expr_list)
        }
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
        let mut trailing_trivia = vec![self.create_newline_trivia()];

        let local_token = Cow::Owned(trivia_formatter::token_reference_add_trivia(
            crate::fmt_symbol!(self, assignment.local_token(), "local ").into_owned(),
            FormatTriviaType::Append(leading_trivia),
            FormatTriviaType::NoChange,
        ));

        let (mut name_list, mut name_list_comments_buf) = self.format_punctuated(
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
            if let Some(type_specifier) = type_specifiers.pop() {
                if let Some(specifier) = type_specifier {
                    let specifier = trivia_formatter::type_specifier_add_trailing_trivia(
                        specifier,
                        FormatTriviaType::Append(trailing_trivia.to_owned()),
                    );
                    type_specifiers.push(Some(specifier));
                    new_line_added = true;
                }
            }

            if let Some(pair) = name_list.pop() {
                // Add the trailing trivia to the end of the name_list if not already added
                if !new_line_added {
                    name_list_comments_buf.append(&mut trailing_trivia);
                }

                let pair = pair.map(|name| {
                    Cow::Owned(trivia_formatter::token_reference_add_trivia(
                        name.to_owned().into_owned(),
                        FormatTriviaType::NoChange,
                        FormatTriviaType::Append(name_list_comments_buf),
                    ))
                });
                name_list.push(pair);
            }

            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(None)
                .with_expressions(Punctuated::new());

            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
            local_assignment
        } else {
            let equal_token = crate::fmt_symbol!(self, assignment.equal_token().unwrap(), " = ");
            // Format the expression normally
            let (mut expr_list, mut expr_comments_buf) =
                self.format_punctuated(assignment.expressions(), &CodeFormatter::format_expression);
            // Create our preliminary new assignment
            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(Some(equal_token))
                .with_expressions(expr_list.to_owned());
            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);

            // Test whether we need to hang the expression, using the updated assignment
            // We have to format normally before this, since we may be expanding the expression onto multiple lines
            // (e.g. if it was a table). We only want to use the first line to determine if we need to hang the expression
            let indent_spacing = (self.indent_level + additional_indent_level.unwrap_or(0))
                * self.config.indent_width;
            let require_multiline_expression = indent_spacing
                + CodeFormatter::strip_local_assignment_trivia(&local_assignment)
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
                // Add the expression list into the indent range, as it will be indented by one
                let expr_range = assignment
                    .expressions()
                    .range()
                    .expect("no range for local assignment expr");
                self.add_indent_range((expr_range.0.bytes(), expr_range.1.bytes()));

                expr_list = Punctuated::new();

                // Format each expression and hang them
                // We need to format again because we will now take into account the indent increase
                for pair in assignment.expressions().pairs() {
                    let expr = self.format_expression(pair.value());
                    let value = self.hang_expression_no_trailing_newline(
                        expr,
                        additional_indent_level,
                        None,
                    );
                    expr_list.push(Pair::new(
                        value,
                        pair.punctuation()
                            .map(|x| crate::fmt_symbol!(self, x, ", ")),
                    ))
                }
            }

            // Add any trailing trivia to the end of the expression list
            if let Some(pair) = expr_list.pop() {
                // Append any comments to the end of the pair
                name_list_comments_buf.append(&mut expr_comments_buf);
                // Append any trailing trivia, if we aren't hanging the expression
                name_list_comments_buf.append(&mut trailing_trivia);

                let pair = pair.map(|expr| {
                    trivia_formatter::expression_add_trailing_trivia(
                        expr,
                        FormatTriviaType::Append(name_list_comments_buf),
                    )
                });
                expr_list.push(pair);
            }

            // Update our local assignment
            local_assignment.with_expressions(expr_list)
        }
    }
}
