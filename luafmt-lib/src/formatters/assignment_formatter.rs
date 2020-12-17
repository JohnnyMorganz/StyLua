use full_moon::ast::{punctuated::Punctuated, Assignment, LocalAssignment};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

use crate::formatters::{expression_formatter, format_punctuated, format_token_reference};

pub fn format_assignment<'ast>(assignment: Assignment<'ast>) -> Assignment<'ast> {
    let var_list = format_punctuated(
        assignment.var_list().to_owned(),
        &expression_formatter::format_var,
    );
    let expr_list = format_punctuated(
        assignment.expr_list().to_owned(),
        &expression_formatter::format_expression,
    );

    assignment
        .with_var_list(var_list)
        .with_equal_token(Cow::Owned(TokenReference::symbol(" = ").unwrap()))
        .with_expr_list(expr_list)
}

pub fn format_local_assignment<'ast>(assignment: LocalAssignment<'ast>) -> LocalAssignment<'ast> {
    if assignment.expr_list().is_empty() {
        let name_list =
            format_punctuated(assignment.name_list().to_owned(), &format_token_reference);
        assignment
            .with_local_token(Cow::Owned(TokenReference::symbol("local ").unwrap()))
            .with_name_list(name_list)
            .with_equal_token(None)
            .with_expr_list(Punctuated::new())
    } else {
        let name_list =
            format_punctuated(assignment.name_list().to_owned(), &format_token_reference);
        let expr_list = format_punctuated(
            assignment.expr_list().to_owned(),
            &expression_formatter::format_expression,
        );

        assignment
            .with_local_token(Cow::Owned(TokenReference::symbol("local ").unwrap()))
            .with_name_list(name_list)
            .with_equal_token(Some(Cow::Owned(TokenReference::symbol(" = ").unwrap())))
            .with_expr_list(expr_list)
        // LocalAssignment {
        //     local_token: Cow::Owned(TokenReference::symbol("local ").unwrap()),
        //     name_list,
        //     equal_token: Some(Cow::Owned(TokenReference::symbol("= ").unwrap())),
        //     expr_list: Punctuated::new(),
        // }
    }
}
