use std::marker::PhantomData;

use super::Extract;
use super::Handler;
use super::Mapper;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct MapValue<P, F, M, O> {
    pat: P,
    func: F,
    marker: PhantomData<(M, O)>,
}

impl<P, F, M, O> MapValue<P, F, M, O> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
    }
}

impl<'a, C, M, O, P, F> Mapper<'a, C, M, O> for MapValue<P, F, M, O>
where
    F: Fn(M) -> O,
    P: Mapper<'a, C, M, M>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.pat.map(ctx, func).map(&self.func)
    }
}

impl<'a, C, P, F, M, O> Parse<C> for MapValue<P, F, M, O>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
