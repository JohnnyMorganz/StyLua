use full_moon::{
    ast::{FunctionBody, FunctionCall, FunctionDeclaration, LocalFunction},
    tokenizer::TokenReference,
};

use super::Formatter;

impl Formatter for FunctionCall {
    fn to_doc<'a, D, A>(
        &'a self,
        ctx: &crate::context::Context,
        allocator: &'a D,
    ) -> pretty::DocBuilder<'a, D, A>
    where
        D: pretty::DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
    }
}

impl Formatter for FunctionDeclaration {
    fn to_doc<'a, D, A>(
        &'a self,
        ctx: &crate::context::Context,
        allocator: &'a D,
    ) -> pretty::DocBuilder<'a, D, A>
    where
        D: pretty::DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
    }
}

impl Formatter for LocalFunction {
    fn to_doc<'a, D, A>(
        &'a self,
        ctx: &crate::context::Context,
        allocator: &'a D,
    ) -> pretty::DocBuilder<'a, D, A>
    where
        D: pretty::DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
    }
}

pub fn anonymous_function<'a, D, A>(
    ctx: &crate::context::Context,
    allocator: &'a D,
    function: &'a (TokenReference, FunctionBody),
) -> pretty::DocBuilder<'a, D, A>
where
    D: pretty::DocAllocator<'a, A>,
    D::Doc: Clone,
    A: Clone,
{
    todo!()
}
