use crate::formatters::{
    trivia_formatter::{self, FormatTriviaType},
    CodeFormatter,
};
use full_moon::ast::{Do, ElseIf, GenericFor, If, NumericFor, Repeat, Stmt, While};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

macro_rules! fmt_stmt {
    ($fmter:expr, $value:ident, { $($(#[$inner:meta])* $operator:ident = $output:ident,)+ }) => {
        match $value {
            $(
                $(#[$inner])*
                Stmt::$operator(stmt) => Stmt::$operator($fmter.$output(stmt)),
            )+
        }
    };
}

impl CodeFormatter {
    /// Format a Do node
    pub fn format_do_block<'ast>(&self, do_block: Do<'ast>) -> Do<'ast> {
        let do_token = crate::fmt_symbol!(self, do_block.do_token().to_owned(), "do");
        let end_token = self.format_end_token(do_block.end_token().to_owned());

        do_block.with_do_token(do_token).with_end_token(end_token)
    }

    /// Format a GenericFor node
    pub fn format_generic_for<'ast>(&mut self, generic_for: GenericFor<'ast>) -> GenericFor<'ast> {
        let for_token = crate::fmt_symbol!(self, generic_for.for_token().to_owned(), "for ");
        let formatted_names = self.format_punctuated(
            generic_for.names().to_owned(),
            &CodeFormatter::format_token_reference_mut,
        );

        #[cfg(feature = "luau")]
        let type_specifiers = generic_for
            .type_specifiers()
            .map(|x| match x {
                Some(type_specifier) => Some(self.format_type_specifier(type_specifier.to_owned())),
                None => None,
            })
            .collect();

        #[cfg(feature = "luau")]
        let generic_for = generic_for.with_type_specifiers(type_specifiers);

        let in_token = crate::fmt_symbol!(self, generic_for.in_token().to_owned(), " in ");
        let formatted_expr_list = self.format_punctuated(
            generic_for.expr_list().to_owned(),
            &CodeFormatter::format_expression,
        );
        let do_token = crate::fmt_symbol!(self, generic_for.do_token().to_owned(), " do");
        let end_token = self.format_end_token(generic_for.end_token().to_owned());

        generic_for
            .with_for_token(for_token)
            .with_names(formatted_names)
            .with_in_token(in_token)
            .with_expr_list(formatted_expr_list)
            .with_do_token(do_token)
            .with_end_token(end_token)
    }

    /// Formats an ElseIf node - This must always reside within format_if
    fn format_else_if<'ast>(&mut self, else_if_node: ElseIf<'ast>) -> ElseIf<'ast> {
        let formatted_else_if_token =
            crate::fmt_symbol!(self, else_if_node.else_if_token().to_owned(), "elseif ");
        let formatted_condition = self.format_expression(else_if_node.condition().to_owned());
        let formatted_then_token =
            crate::fmt_symbol!(self, else_if_node.then_token().to_owned(), " then");

        else_if_node
            .with_else_if_token(formatted_else_if_token)
            .with_condition(formatted_condition)
            .with_then_token(formatted_then_token)
    }

    /// Format an If node
    pub fn format_if<'ast>(&mut self, if_node: If<'ast>) -> If<'ast> {
        let formatted_if_token = crate::fmt_symbol!(self, if_node.if_token().to_owned(), "if ");
        let formatted_condition = self.format_expression(if_node.condition().to_owned());
        let formatted_then_token =
            crate::fmt_symbol!(self, if_node.then_token().to_owned(), " then");
        let formatted_end_token = self.format_end_token(if_node.end_token().to_owned());

        let formatted_else_if = match if_node.else_if() {
            Some(else_if) => Some(
                else_if
                    .iter()
                    .map(|else_if| self.format_else_if(else_if.to_owned()))
                    .collect(),
            ),
            None => None,
        };

        let formatted_else_token = match if_node.else_token() {
            Some(token) => Some(crate::fmt_symbol!(self, token.to_owned(), "else")),
            None => None,
        };

        if_node
            .with_if_token(formatted_if_token)
            .with_condition(formatted_condition)
            .with_then_token(formatted_then_token)
            .with_else_if(formatted_else_if)
            .with_else_token(formatted_else_token)
            .with_end_token(formatted_end_token)
    }

    /// Format a NumericFor node
    pub fn format_numeric_for<'ast>(&mut self, numeric_for: NumericFor<'ast>) -> NumericFor<'ast> {
        let for_token = crate::fmt_symbol!(self, numeric_for.for_token().to_owned(), "for ");
        let formatted_index_variable =
            Cow::Owned(self.format_plain_token_reference(numeric_for.index_variable().to_owned()));

        #[cfg(feature = "luau")]
        let type_specifier = match numeric_for.type_specifier() {
            Some(type_specifier) => Some(self.format_type_specifier(type_specifier.to_owned())),
            None => None,
        };
        #[cfg(feature = "luau")]
        let numeric_for = numeric_for.with_type_specifier(type_specifier);

        let equal_token = crate::fmt_symbol!(self, numeric_for.equal_token().to_owned(), " = ");
        let formatted_start_expression = self.format_expression(numeric_for.start().to_owned());
        let start_end_comma =
            crate::fmt_symbol!(self, numeric_for.start_end_comma().to_owned(), ", ");
        let formatted_end_expression = self.format_expression(numeric_for.end().to_owned());

        let (end_step_comma, formatted_step_expression) = match numeric_for.step() {
            Some(step) => (
                Some(crate::fmt_symbol!(
                    self,
                    numeric_for.end_step_comma().unwrap().to_owned(),
                    ", "
                )),
                Some(self.format_expression(step.to_owned())),
            ),
            None => (None, None),
        };

        let do_token = crate::fmt_symbol!(self, numeric_for.do_token().to_owned(), " do");
        let end_token = self.format_end_token(numeric_for.end_token().to_owned());

        numeric_for
            .with_for_token(for_token)
            .with_index_variable(formatted_index_variable)
            .with_equal_token(equal_token)
            .with_start(formatted_start_expression)
            .with_start_end_comma(start_end_comma)
            .with_end(formatted_end_expression)
            .with_end_step_comma(end_step_comma)
            .with_step(formatted_step_expression)
            .with_do_token(do_token)
            .with_end_token(end_token)
    }

    /// Format a Repeat node
    pub fn format_repeat_block<'ast>(&mut self, repeat_block: Repeat<'ast>) -> Repeat<'ast> {
        let repeat_token =
            crate::fmt_symbol!(self, repeat_block.repeat_token().to_owned(), "repeat");
        let until_token = crate::fmt_symbol!(self, repeat_block.until_token().to_owned(), "until ");
        let formatted_until = self.format_expression(repeat_block.until().to_owned());

        repeat_block
            .with_repeat_token(repeat_token)
            .with_until_token(until_token)
            .with_until(formatted_until)
    }

    /// Format a While node
    pub fn format_while_block<'ast>(&mut self, while_block: While<'ast>) -> While<'ast> {
        let while_token = crate::fmt_symbol!(self, while_block.while_token().to_owned(), "while ");
        let formatted_condition = self.format_expression(while_block.condition().to_owned());
        let do_token = crate::fmt_symbol!(self, while_block.do_token().to_owned(), " do");
        let end_token = self.format_end_token(while_block.end_token().to_owned());

        while_block
            .with_while_token(while_token)
            .with_condition(formatted_condition)
            .with_do_token(do_token)
            .with_end_token(end_token)
    }

    pub fn format_stmt<'ast>(&mut self, stmt: Stmt<'ast>) -> Stmt<'ast> {
        fmt_stmt!(self, stmt, {
            Assignment = format_assignment,
            Do = format_do_block,
            FunctionCall = format_function_call,
            FunctionDeclaration = format_function_declaration,
            GenericFor = format_generic_for,
            If = format_if,
            LocalAssignment = format_local_assignment,
            LocalFunction = format_local_function,
            NumericFor = format_numeric_for,
            Repeat = format_repeat_block,
            While = format_while_block,
            #[cfg(feature = "luau")] CompoundAssignment = format_compound_assignment,
            #[cfg(feature = "luau")] ExportedTypeDeclaration = format_exported_type_declaration,
            #[cfg(feature = "luau")] TypeDeclaration = format_type_declaration,
        })
    }

    pub fn stmt_add_trivia<'ast>(
        &self,
        stmt: Stmt<'ast>,
        additional_indent_level: Option<usize>,
    ) -> Stmt<'ast> {
        let leading_trivia =
            FormatTriviaType::Append(vec![self.create_indent_trivia(additional_indent_level)]);
        let trailing_trivia = FormatTriviaType::Append(vec![self.create_newline_trivia()]);

        match stmt {
            Stmt::Assignment(assignment) => {
                Stmt::Assignment(trivia_formatter::assignment_add_trivia(
                    assignment,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::Do(do_block) => Stmt::Do(trivia_formatter::do_block_add_trivia(
                do_block,
                leading_trivia,
                trailing_trivia,
            )),
            Stmt::FunctionCall(function_call) => {
                Stmt::FunctionCall(trivia_formatter::function_call_add_trivia(
                    function_call,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::FunctionDeclaration(function_declaration) => {
                Stmt::FunctionDeclaration(trivia_formatter::function_declaration_add_trivia(
                    function_declaration,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::GenericFor(generic_for) => {
                Stmt::GenericFor(trivia_formatter::generic_for_add_trivia(
                    generic_for,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::If(if_block) => Stmt::If(trivia_formatter::if_block_add_trivia(
                if_block,
                leading_trivia,
                trailing_trivia,
            )),
            Stmt::LocalAssignment(local_assignment) => {
                Stmt::LocalAssignment(trivia_formatter::local_assignment_add_trivia(
                    local_assignment,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::LocalFunction(local_function) => {
                Stmt::LocalFunction(trivia_formatter::local_function_add_trivia(
                    local_function,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::NumericFor(numeric_for) => {
                Stmt::NumericFor(trivia_formatter::numeric_for_add_trivia(
                    numeric_for,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
            Stmt::Repeat(repeat_block) => Stmt::Repeat(trivia_formatter::repeat_block_add_trivia(
                repeat_block,
                leading_trivia,
                trailing_trivia,
            )),
            Stmt::While(while_block) => Stmt::While(trivia_formatter::while_block_add_trivia(
                while_block,
                leading_trivia,
                trailing_trivia,
            )),

            #[cfg(feature = "luau")]
            Stmt::CompoundAssignment(compound_assignment) => {
                Stmt::CompoundAssignment(trivia_formatter::compound_assignment_add_trivia(
                    compound_assignment,
                    leading_trivia,
                    trailing_trivia,
                ))
            }

            #[cfg(feature = "luau")]
            Stmt::ExportedTypeDeclaration(exported_type_declaration) => {
                Stmt::ExportedTypeDeclaration(
                    trivia_formatter::exported_type_declaration_add_trivia(
                        exported_type_declaration,
                        leading_trivia,
                        trailing_trivia,
                    ),
                )
            }

            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(type_declaration) => {
                Stmt::TypeDeclaration(trivia_formatter::type_declaration_add_trivia(
                    type_declaration,
                    leading_trivia,
                    trailing_trivia,
                ))
            }
        }
    }
}
