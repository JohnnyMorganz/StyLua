use full_moon::ast::{punctuated::Punctuated, Assignment, LocalAssignment};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

use crate::formatters::CodeFormatter;

impl CodeFormatter {
    pub fn format_assignment<'ast>(&mut self, assignment: &Assignment<'ast>) -> Assignment<'ast> {
        let var_list = self.format_punctuated(assignment.var_list(), &CodeFormatter::format_var);
        let expr_list =
            self.format_punctuated(assignment.expr_list(), &CodeFormatter::format_expression);

        Assignment::new(var_list, expr_list)
            .with_equal_token(Cow::Owned(TokenReference::symbol(" = ").unwrap()))
    }

    pub fn format_local_assignment<'ast>(
        &mut self,
        assignment: &LocalAssignment<'ast>,
    ) -> LocalAssignment<'ast> {
        let local_token = crate::fmt_symbol!(self, assignment.local_token(), "local ");
        let name_list = self.format_punctuated(
            assignment.name_list(),
            &CodeFormatter::format_token_reference_mut,
        );

        #[cfg(feature = "luau")]
        let type_specifiers = assignment
            .type_specifiers()
            .map(|x| match x {
                Some(type_specifier) => Some(self.format_type_specifier(type_specifier)),
                None => None,
            })
            .collect();

        if assignment.expr_list().is_empty() {
            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(None)
                .with_expr_list(Punctuated::new());
            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);
            local_assignment
        } else {
            let equal_token = crate::fmt_symbol!(self, assignment.equal_token().unwrap(), " = ");
            let expr_list =
                self.format_punctuated(assignment.expr_list(), &CodeFormatter::format_expression);

            let local_assignment = LocalAssignment::new(name_list)
                .with_local_token(local_token)
                .with_equal_token(Some(equal_token))
                .with_expr_list(expr_list);
            #[cfg(feature = "luau")]
            let local_assignment = local_assignment.with_type_specifiers(type_specifiers);

            local_assignment
        }
    }
}
