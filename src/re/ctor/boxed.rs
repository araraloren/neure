use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

// into_box
#[derive(Debug, Clone)]
pub struct BoxedCtor<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T> BoxedCtor<C, T> {
    pub fn new(inner: Box<T>) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: Box<T>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Box<T> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: Box<T>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<C, T> Regex<C> for BoxedCtor<C, T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Boxed invoke not support `Regex` trait")
    }
}

impl<'a, C, M, O, T> Ctor<'a, C, M, O> for BoxedCtor<C, T>
where
    T: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.inner.constrct(ctx, handler)
    }
}

pub fn into_boxed_ctor<'a, C, M, O, I>(invoke: I) -> BoxedCtor<C, I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    BoxedCtor::new(Box::new(invoke))
}

pub trait BoxedCtorHelper<'a, C, M, O>
where
    C: Context<'a> + Policy<C>,
{
    fn into_boxed_ctor(self) -> BoxedCtor<C, Self>
    where
        Self: Sized;
}

impl<'a, C, M, O, I> BoxedCtorHelper<'a, C, M, O> for I
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn into_boxed_ctor(self) -> BoxedCtor<C, Self>
    where
        Self: Sized,
    {
        BoxedCtor::new(Box::new(self))
    }
}
