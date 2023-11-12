use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

use super::Ctor;
use super::Extract;
use super::Handler;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullRegex<R>(PhantomData<R>);

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

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        Ok(<R as Ret>::from(ctx, (0, 0)))
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for NullRegex<Span>
where
    C: Context<'a, Orig = str> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}