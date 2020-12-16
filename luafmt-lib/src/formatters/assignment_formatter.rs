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

// #[cfg(test)]
// mod tests {
//     use crate::formatters::assignment_formatter::AssignmentFormatter;
//     use full_moon::visitors::VisitorMut;
//     use full_moon::{parse, print};
//     #[test]
//     fn test_assignment_formatter() {
//         let mut visitor = AssignmentFormatter::default();
//         let ast = parse("    x =   1").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "x = 1\n");
//     }

//     #[test]
//     fn test_multiple_var_assignment_formatter() {
//         let mut visitor = AssignmentFormatter::default();
//         let ast = parse("x    ,   y =   1,    'foo'").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "x, y = 1, 'foo'\n");
//     }

//     #[test]
//     fn test_local_assignment_formatter() {
//         let mut visitor = AssignmentFormatter::default();
//         let ast = parse("local      x       =     'test'  ").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local x = 'test'\n");
//     }

//     #[test]
//     fn test_local_assignment_no_expr_list_formatter() {
//         let mut visitor = AssignmentFormatter::default();
//         let ast = parse("local      x       ").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local x\n");
//     }

//     #[test]
//     fn test_local_assignment_multiple_vars_formatter() {
//         let mut visitor = AssignmentFormatter::default();
//         let ast = parse("local      x,      y       =     'test'  ").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local x, y = 'test'\n");
//     }
// }
