use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Invoke;
use crate::re::Regex;

pub type DynamicInvokeHandler<'a, C, O> = Box<dyn Fn(&mut C) -> Result<O, Error> + 'a>;

pub struct DynamicInvoke<'a, C, O> {
    inner: DynamicInvokeHandler<'a, C, O>,
}

impl<'a, C, O> DynamicInvoke<'a, C, O> {
    pub fn new(inner: DynamicInvokeHandler<'a, C, O>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicInvokeHandler<'a, C, O>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicInvokeHandler<'a, C, O> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicInvokeHandler<'a, C, O> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicInvokeHandler<'a, C, O>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, C, R> Regex<C> for DynamicInvoke<'a, C, R> {
    type Ret = R;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O> Invoke<'a, C, M, O> for DynamicInvoke<'b, C, O>
where
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, _: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        (self.inner)(ctx)
    }
}
