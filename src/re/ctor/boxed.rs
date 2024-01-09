use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::re::Wrapped;

// into_box
pub struct BoxedCtor<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T: Debug> Debug for BoxedCtor<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedCtor")
            .field("inner", &self.inner)
            .field("marker", &self.marker)
            .finish()
    }
}

impl<C, T: Clone> Clone for BoxedCtor<C, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker,
        }
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

impl<'a, C, M, O, T, H, A> Ctor<'a, C, M, O, H, A> for BoxedCtor<C, T>
where
    T: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.constrct(ctx, handler)
    }
}

impl<C, T> Wrapped for BoxedCtor<C, T> {
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
