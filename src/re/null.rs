use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

use super::def_not;
use super::trace;
use super::Ctor;
use super::Extract;
use super::Handler;

#[derive(Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullRegex<R>(PhantomData<R>);

def_not!(NullRegex<R>);

impl<R> Debug for NullRegex<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NullRegex").field(&self.0).finish()
    }
}

impl<R> Clone for NullRegex<R> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<R> NullRegex<R> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, C, R> Regex<C> for NullRegex<R>
where
    R: Ret,
    C: Context<'a>,
{
    type Ret = R;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let beg = ctx.offset();
        let ret = Ok(<R as Ret>::from_ctx(ctx, (0, 0)));

        trace!("null", beg => ctx.offset(), ret)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for NullRegex<Span>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let beg = ctx.offset();
        let ret = ctx.try_mat(self);

        trace!("null", beg -> ctx.offset(), ret.is_ok());
        handler.invoke(A::extract(ctx, &ret?)?)
    }
}
