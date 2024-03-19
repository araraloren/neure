use std::fmt::Debug;

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
#[derive(Debug, Clone)]
pub struct BoxedCtor<I> {
    inner: Box<I>,
}

impl<I> BoxedCtor<I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<I, C> Regex<C> for BoxedCtor<I> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Boxed invoke not support `Regex` trait")
    }
}

impl<'a, C, M, O, I, H, A> Ctor<'a, C, M, O, H, A> for BoxedCtor<I>
where
    I: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(self.inner.as_ref(), ctx, handler)
    }
}

impl<I> Wrapped for BoxedCtor<I> {
    type Inner = Box<I>;

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
