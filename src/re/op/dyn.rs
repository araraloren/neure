use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub type DynamicRegexHandler<'a, C, R> = Box<dyn Fn(&mut C) -> Result<R, Error> + 'a>;

pub struct DynamicRegex<'a, C, R> {
    inner: DynamicRegexHandler<'a, C, R>,
}

impl<'a, C, R> DynamicRegex<'a, C, R> {
    pub fn new(inner: DynamicRegexHandler<'a, C, R>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicRegexHandler<'a, C, R>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicRegexHandler<'a, C, R> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicRegexHandler<'a, C, R> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicRegexHandler<'a, C, R>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, C, R> Regex<C> for DynamicRegex<'a, C, R> {
    type Ret = R;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, 'b, C, O> Ctor<'a, C, O, O> for DynamicRegex<'b, C, Span>
where
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat_t(&self.inner)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}
