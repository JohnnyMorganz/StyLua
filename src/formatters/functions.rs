use full_moon::{
    ast::{
        Call, FunctionArgs, FunctionBody, FunctionCall, FunctionDeclaration, FunctionName,
        LocalFunction, MethodCall, Parameter,
    },
    tokenizer::TokenReference,
};
use pretty::{docs, DocAllocator, DocBuilder};

use crate::context::Context;

use super::{base::contained_span, Formatter};

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
        docs![
            allocator,
            self.prefix().to_doc(ctx, allocator),
            allocator.intersperse(
                self.suffixes().map(|suffix| suffix.to_doc(ctx, allocator)),
                allocator.line_()
            ),
        ]
    }
}

impl Formatter for Call {
    fn to_doc<'a, D, A>(&'a self, ctx: &Context, allocator: &'a D) -> DocBuilder<'a, D, A>
    where
        D: DocAllocator<'a, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Call::AnonymousCall(function_args) => function_args.to_doc(ctx, allocator),
            Call::MethodCall(method_call) => method_call.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for MethodCall {
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
        docs![
            allocator,
            self.colon_token().to_doc(ctx, allocator),
            self.name().to_doc(ctx, allocator),
            self.args().to_doc(ctx, allocator)
        ]
    }
}

impl Formatter for FunctionArgs {
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
        match self {
            FunctionArgs::Parentheses {
                parentheses,
                arguments,
            } => contained_span(ctx, allocator, parentheses, arguments),
            FunctionArgs::String(string) => {
                if ctx.should_omit_string_parens() {
                    string.to_doc(ctx, allocator)
                } else {
                    allocator
                        .text("(")
                        .append(string.to_doc(ctx, allocator))
                        .append(allocator.text(")"))
                }
            }
            FunctionArgs::TableConstructor(table) => {
                if ctx.should_omit_table_parens() {
                    table.to_doc(ctx, allocator)
                } else {
                    allocator
                        .text("(")
                        .append(table.to_doc(ctx, allocator))
                        .append(allocator.text(")"))
                }
            }
            other => unreachable!("unknown node: {:?}", other),
        }
    }
}

impl Formatter for FunctionName {
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
        let mut docs =
            allocator.concat(self.names().pairs().map(|pair| pair.to_doc(ctx, allocator)));

        if let (Some(method_colon), Some(method_name)) = (self.method_colon(), self.method_name()) {
            docs = docs
                .append(method_colon.to_doc(ctx, allocator))
                .append(method_name.to_doc(ctx, allocator))
        }

        docs
    }
}

impl Formatter for FunctionBody {
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
        contained_span(
            ctx,
            allocator,
            self.parameters_parentheses(),
            self.parameters(),
        )
        .append(
            allocator
                .hardline()
                .append(self.block().to_doc(ctx, allocator))
                .nest(ctx.config().indent_width_signed()),
        )
        .append(allocator.hardline())
        .append(self.end_token().to_doc(ctx, allocator))
        .group()
    }
}

impl Formatter for Parameter {
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
        match self {
            Parameter::Name(token) | Parameter::Ellipse(token) => token.to_doc(ctx, allocator),
            other => unreachable!("unknown node: {:?}", other),
        }
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
        docs![
            allocator,
            self.function_token().to_doc(ctx, allocator),
            allocator.space(),
            self.name().to_doc(ctx, allocator),
            self.body().to_doc(ctx, allocator),
        ]
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
        docs![
            allocator,
            self.local_token().to_doc(ctx, allocator),
            allocator.space(),
            self.function_token().to_doc(ctx, allocator),
            allocator.space(),
            self.name().to_doc(ctx, allocator),
            self.body().to_doc(ctx, allocator),
        ]
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
    let (token, body) = function;
    docs![
        allocator,
        token.to_doc(ctx, allocator),
        body.to_doc(ctx, allocator),
    ]
}

#[cfg(test)]
mod tests {
    use pretty::RcDoc;

    #[test]
    fn test() {
        let params = [
            "laaaaaaaaaaaarge",
            "laaaaaaaaaaaarge",
            "laaaaaaaaaaaarge",
            "laaaaaaaaaaaarge",
            "laaaaaaaaaaaarge",
        ];

        let params = RcDoc::text("(")
            .append(
                RcDoc::line_()
                    .append(RcDoc::intersperse(
                        params,
                        RcDoc::text(",").append(RcDoc::line()),
                    ))
                    .nest(4),
            )
            .append(RcDoc::line_())
            .append(RcDoc::text(")"))
            .group();

        let body = RcDoc::hardline()
            .append(RcDoc::text("x"))
            .append(RcDoc::space())
            .append(RcDoc::text("="))
            .append(RcDoc::space())
            .append(RcDoc::text("1"))
            .nest(4);

        println!(
            "{}",
            RcDoc::<()>::text("function")
                .append(RcDoc::space())
                .append(RcDoc::text("name"))
                .append(params)
                .append(body)
                .append(RcDoc::hardline())
                .append(RcDoc::text("end"))
                .pretty(80)
        )
    }
}
