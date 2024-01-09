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
use std::marker::PhantomData;

pub struct BoxedRegex<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T: Debug> Debug for BoxedRegex<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedRegex")
            .field("inner", &self.inner)
            .field("marker", &self.marker)
            .finish()
    }
}

impl<C, T: Clone> Clone for BoxedRegex<C, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker,
        }
    }
}

impl<C, T> Regex<C> for BoxedRegex<C, T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, T, H, A> Ctor<'a, C, O, O, H, A> for BoxedRegex<C, T>
where
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<C, T> Wrapped for BoxedRegex<C, T> {
    type Inner = T;

    fn wrap(inner: Self::Inner) -> Self {
        Self {
            inner: Box::new(inner),
            marker: PhantomData,
        }
    }

    fn inner(&self) -> &Self::Inner {
        self.inner.as_ref()
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self.inner.as_mut()
    }
}
