use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Mapper;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Quote<P, L, R, M, O> {
    pat: P,
    left: L,
    right: R,
    marker: PhantomData<(M, O)>,
}

impl<P, L, R, M, O> Quote<P, L, R, M, O> {
    pub fn new(pat: P, left: L, right: R) -> Self {
        Self {
            pat,
            left,
            right,
            marker: PhantomData,
        }
    }
}

impl<'a, C, L, R, P, M, O> Mapper<'a, C, M, O> for Quote<P, L, R, M, O>
where
    L: Parse<C, Ret = Span>,
    R: Parse<C, Ret = Span>,
    P: Mapper<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = self.pat.map(g.ctx(), func);
        let ret = g.process_ret(ret)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}

impl<'a, C, L, R, P, M, O> Parse<C> for Quote<P, L, R, M, O>
where
    L: Parse<C, Ret = Span>,
    R: Parse<C, Ret = Span>,
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = g.try_mat(&self.pat)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}
