use full_moon::ast::TableConstructor;

use super::Formatter;

impl Formatter for TableConstructor {
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
