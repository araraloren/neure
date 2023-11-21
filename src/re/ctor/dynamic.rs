use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub type DynamicCtorHandler<'a, C, O> = Box<dyn Fn(&mut C) -> Result<O, Error> + 'a>;

pub struct DynamicCtor<'a, C, O> {
    inner: DynamicCtorHandler<'a, C, O>,
}

impl<'a, C, O> DynamicCtor<'a, C, O> {
    pub fn new(inner: DynamicCtorHandler<'a, C, O>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicCtorHandler<'a, C, O>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicCtorHandler<'a, C, O> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicCtorHandler<'a, C, O> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicCtorHandler<'a, C, O>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, C, R> Regex<C> for DynamicCtor<'a, C, R> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O> Ctor<'a, C, M, O> for DynamicCtor<'b, C, O>
where
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, _: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        (self.inner)(ctx)
    }
}

pub fn into_dyn_ctor<'a, 'b, C, O>(
    invoke: impl Fn(&mut C) -> Result<O, Error> + 'b,
) -> DynamicCtor<'b, C, O>
where
    C: Context<'a>,
{
    DynamicCtor::new(Box::new(invoke))
}

pub trait DynamicCtorHelper<'a, 'b, C, O>
where
    C: Context<'a>,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'b, C, O>;
}

impl<'a, 'b, C, O, T> DynamicCtorHelper<'a, 'b, C, O> for T
where
    C: Context<'a>,
    T: Fn(&mut C) -> Result<O, Error> + 'b,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'b, C, O> {
        DynamicCtor::new(Box::new(self))
    }
}
