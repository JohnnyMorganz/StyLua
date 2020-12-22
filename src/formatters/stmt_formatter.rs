use crate::formatters::CodeFormatter;
use full_moon::ast::{Do, ElseIf, GenericFor, If, NumericFor, Repeat, Stmt, While};
use full_moon::tokenizer::TokenReference;
use std::borrow::Cow;

impl CodeFormatter {
    /// Format a Do node
    pub fn format_do_block<'ast>(&self, do_block: Do<'ast>) -> Do<'ast> {
        let do_token = self.format_symbol(
            do_block.do_token().to_owned(),
            TokenReference::symbol("do").unwrap(),
        );
        let end_token = self.format_symbol(
            do_block.end_token().to_owned(),
            TokenReference::symbol("end").unwrap(),
        );

        do_block.with_do_token(do_token).with_end_token(end_token)
    }

    /// Format a GenericFor node
    pub fn format_generic_for<'ast>(&mut self, generic_for: GenericFor<'ast>) -> GenericFor<'ast> {
        let for_token = self.format_symbol(
            generic_for.for_token().to_owned(),
            TokenReference::symbol("for ").unwrap(),
        );
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

        let in_token = self.format_symbol(
            generic_for.in_token().to_owned(),
            TokenReference::symbol(" in ").unwrap(),
        );
        let formatted_expr_list = self.format_punctuated(
            generic_for.expr_list().to_owned(),
            &CodeFormatter::format_expression,
        );
        let do_token = self.format_symbol(
            generic_for.do_token().to_owned(),
            TokenReference::symbol(" do").unwrap(),
        );
        let end_token = self.format_symbol(
            generic_for.end_token().to_owned(),
            TokenReference::symbol("end").unwrap(),
        );

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
        let formatted_else_if_token = self.format_symbol(
            else_if_node.else_if_token().to_owned(),
            TokenReference::symbol("elseif ").unwrap(),
        );
        let formatted_condition = self.format_expression(else_if_node.condition().to_owned());
        let formatted_then_token = self.format_symbol(
            else_if_node.then_token().to_owned(),
            TokenReference::symbol(" then").unwrap(),
        );

        else_if_node
            .with_else_if_token(formatted_else_if_token)
            .with_condition(formatted_condition)
            .with_then_token(formatted_then_token)
    }

    /// Format an If node
    pub fn format_if<'ast>(&mut self, if_node: If<'ast>) -> If<'ast> {
        let formatted_if_token = self.format_symbol(
            if_node.if_token().to_owned(),
            TokenReference::symbol("if ").unwrap(),
        );
        let formatted_condition = self.format_expression(if_node.condition().to_owned());
        let formatted_then_token = self.format_symbol(
            if_node.then_token().to_owned(),
            TokenReference::symbol(" then").unwrap(),
        );
        let formatted_end_token = self.format_symbol(
            if_node.end_token().to_owned(),
            TokenReference::symbol("end").unwrap(),
        );

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
            Some(token) => {
                Some(self.format_symbol(token.to_owned(), TokenReference::symbol("else").unwrap()))
            }
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
        let for_token = self.format_symbol(
            numeric_for.for_token().to_owned(),
            TokenReference::symbol("for ").unwrap(),
        );
        let formatted_index_variable =
            Cow::Owned(self.format_plain_token_reference(numeric_for.index_variable().to_owned()));
        #[cfg(feature = "luau")]
        let type_specifier = match numeric_for.type_specifier() {
            Some(type_specifier) => Some(self.format_type_specifier(type_specifier.to_owned())),
            None => None,
        };

        #[cfg(feature = "luau")]
        let numeric_for = numeric_for.with_type_specifier(type_specifier);

        let equal_token = self.format_symbol(
            numeric_for.equal_token().to_owned(),
            TokenReference::symbol(" = ").unwrap(),
        );
        let formatted_start_expression = self.format_expression(numeric_for.start().to_owned());
        let start_end_comma = self.format_symbol(
            numeric_for.start_end_comma().to_owned(),
            TokenReference::symbol(", ").unwrap(),
        );
        let formatted_end_expression = self.format_expression(numeric_for.end().to_owned());

        let (end_step_comma, formatted_step_expression) = match numeric_for.step() {
            Some(step) => (
                Some(self.format_symbol(
                    numeric_for.end_step_comma().unwrap().to_owned(),
                    TokenReference::symbol(", ").unwrap(),
                )),
                Some(self.format_expression(step.to_owned())),
            ),
            None => (None, None),
        };

        let do_token = self.format_symbol(
            numeric_for.do_token().to_owned(),
            TokenReference::symbol(" do").unwrap(),
        );
        let end_token = self.format_symbol(
            numeric_for.end_token().to_owned(),
            TokenReference::symbol("end").unwrap(),
        );

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
        let repeat_token = self.format_symbol(
            repeat_block.repeat_token().to_owned(),
            TokenReference::symbol("repeat").unwrap(),
        );
        let until_token = self.format_symbol(
            repeat_block.until_token().to_owned(),
            TokenReference::symbol("until ").unwrap(),
        );
        let formatted_until = self.format_expression(repeat_block.until().to_owned());

        repeat_block
            .with_repeat_token(repeat_token)
            .with_until_token(until_token)
            .with_until(formatted_until)
    }

    /// Format a While node
    pub fn format_while_block<'ast>(&mut self, while_block: While<'ast>) -> While<'ast> {
        let while_token = self.format_symbol(
            while_block.while_token().to_owned(),
            TokenReference::symbol("while ").unwrap(),
        );
        let formatted_condition = self.format_expression(while_block.condition().to_owned());
        let do_token = self.format_symbol(
            while_block.do_token().to_owned(),
            TokenReference::symbol(" do").unwrap(),
        );
        let end_token = self.format_symbol(
            while_block.end_token().to_owned(),
            TokenReference::symbol("end").unwrap(),
        );

        while_block
            .with_while_token(while_token)
            .with_condition(formatted_condition)
            .with_do_token(do_token)
            .with_end_token(end_token)
    }

    pub fn format_stmt<'ast>(&mut self, stmt: Stmt<'ast>) -> Stmt<'ast> {
        match stmt {
            Stmt::Assignment(assignment) => Stmt::Assignment(self.format_assignment(assignment)),
            Stmt::Do(do_block) => Stmt::Do(self.format_do_block(do_block)),
            Stmt::FunctionCall(function_call) => {
                Stmt::FunctionCall(self.format_function_call(function_call))
            }
            Stmt::FunctionDeclaration(function_declaration) => {
                Stmt::FunctionDeclaration(self.format_function_declaration(function_declaration))
            }
            Stmt::GenericFor(generic_for) => Stmt::GenericFor(self.format_generic_for(generic_for)),
            Stmt::If(if_node) => Stmt::If(self.format_if(if_node)),
            Stmt::LocalAssignment(local_assignment) => {
                Stmt::LocalAssignment(self.format_local_assignment(local_assignment))
            }
            Stmt::LocalFunction(local_function) => {
                Stmt::LocalFunction(self.format_local_function(local_function))
            }
            Stmt::NumericFor(numeric_for) => Stmt::NumericFor(self.format_numeric_for(numeric_for)),
            Stmt::Repeat(repeat) => Stmt::Repeat(self.format_repeat_block(repeat)),
            Stmt::While(while_block) => Stmt::While(self.format_while_block(while_block)),
            #[cfg(feature = "luau")]
            Stmt::CompoundAssignment(compound_assignment) => {
                Stmt::CompoundAssignment(self.format_compound_assignment(compound_assignment))
            }
            #[cfg(feature = "luau")]
            Stmt::ExportedTypeDeclaration(exported_type_declaration) => {
                Stmt::ExportedTypeDeclaration(
                    self.format_exported_type_declaration(exported_type_declaration),
                )
            }
            #[cfg(feature = "luau")]
            Stmt::TypeDeclaration(type_declaration) => {
                Stmt::TypeDeclaration(self.format_type_declaration(type_declaration))
            }
        }
    }
}
