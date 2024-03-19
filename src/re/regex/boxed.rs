use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::re::Wrapped;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct BoxedRegex<T> {
    inner: Box<T>,
}

impl<T> BoxedRegex<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: Box::new(val),
        }
    }
}

impl<C, T> Regex<C> for BoxedRegex<T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, T, H, A> Ctor<'a, C, O, O, H, A> for BoxedRegex<T>
where
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<T> Wrapped for BoxedRegex<T> {
    type Inner = Box<T>;

    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
