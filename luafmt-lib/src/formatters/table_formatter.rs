use crate::formatters::{expression_formatter::format_expression, format_contained_span};
use full_moon::ast::{
    punctuated::{Pair, Punctuated},
    span::ContainedSpan,
    Field, TableConstructor,
};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

pub fn format_field<'ast>(field: &Field<'ast>) -> Field<'ast> {
    match field {
        Field::ExpressionKey {
            brackets,
            key,
            equal: _,
            value,
        } => Field::ExpressionKey {
            brackets: format_contained_span(brackets.to_owned()),
            key: key.to_owned(),
            equal: Cow::Owned(TokenReference::symbol(" = ").unwrap()),
            value: value.to_owned(),
        },
        Field::NameKey {
            key,
            equal: _,
            value,
        } => Field::NameKey {
            key: Cow::Owned(TokenReference::new(
                Vec::new(),
                key.token().to_owned(),
                Vec::new(),
            )),
            equal: Cow::Owned(TokenReference::symbol(" = ").unwrap()),
            value: format_expression(value.to_owned()),
        },
        Field::NoKey(expression) => Field::NoKey(format_expression(expression.to_owned())),
    }
}

pub fn format_table_constructor<'ast>(
    table_constructor: TableConstructor<'ast>,
) -> TableConstructor<'ast> {
    let mut fields = Punctuated::new();
    let mut current_fields = table_constructor.iter_fields().peekable();

    let braces = match current_fields.peek() {
        Some(_) => ContainedSpan::new(
            Cow::Owned(TokenReference::symbol("{ ").unwrap()), // TODO: No whitespace if multiline table
            Cow::Owned(TokenReference::symbol(" }").unwrap()),
        ),

        // Table is empty, so don't add spaces in between braces
        None => ContainedSpan::new(
            Cow::Owned(TokenReference::symbol("{").unwrap()),
            Cow::Owned(TokenReference::symbol("}").unwrap()),
        ),
    };

    // TODO: Determine if to make a single or multi-line table
    // TODO: Should we sort NameKey/ExpressionKey tables?
    loop {
        match current_fields.next() {
            Some(field) => {
                let formatted_field = format_field(field);
                let mut punctuation = None;

                if let Some(_) = current_fields.peek() {
                    // Have more elements still to go
                    // TODO: No trailing whitespace on multiline table, should be replaced with a newline character
                    punctuation = Some(Cow::Owned(TokenReference::symbol(", ").unwrap()));
                }
                fields.push(Pair::new(formatted_field, punctuation))
            }
            None => break,
        }
    }

    TableConstructor::new()
        .with_braces(braces)
        .with_fields(fields)
}

// #[cfg(test)]
// mod tests {
//     use crate::formatters::table_formatter::TableFormatter;
//     use full_moon::visitors::VisitorMut;
//     use full_moon::{parse, print};

//     #[test]
//     fn test_table_no_key_small() {
//         let mut visitor = TableFormatter::default();
//         let ast = parse("local foo = {a,b,c}").unwrap();
//         assert_eq!(print(&visitor.visit_ast(ast)), "local foo = { a, b, c }");
//     }
// }
