use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Invoke;
use crate::re::Regex;

// into_box
#[derive(Debug, Clone)]
pub struct BoxedInvoke<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T> BoxedInvoke<C, T> {
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

    pub fn inner(&self) -> &Box<T> {
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

impl<'a, C, T> Regex<C> for BoxedInvoke<C, T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Boxed invoke not support `Regex` trait")
    }
}

impl<'a, C, M, O, T> Invoke<'a, C, M, O> for BoxedInvoke<C, T>
where
    T: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.inner.invoke(ctx, handler)
    }
}
