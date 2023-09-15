use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Or<P1, P2, M, O> {
    pat1: P1,
    pat2: P2,
    marker: PhantomData<(M, O)>,
}

impl<P1, P2, M, O> Or<P1, P2, M, O> {
    pub fn new(pat1: P1, pat2: P2) -> Self {
        Self {
            pat1,
            pat2,
            marker: PhantomData,
        }
    }

    pub fn set_pat1(&mut self, pat1: P1) -> &mut Self {
        self.pat1 = pat1;
        self
    }

    pub fn set_pat2(&mut self, pat2: P2) -> &mut Self {
        self.pat2 = pat2;
        self
    }
}

impl<'a, C, P1, P2, M, O> Invoke<'a, C, M, O> for Or<P1, P2, M, O>
where
    P1: Invoke<'a, C, M, O>,
    P2: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        match self.pat1.invoke(g.ctx(), func) {
            Ok(ret) => Ok(ret),
            Err(_) => {
                let ret = self.pat2.invoke(g.reset().ctx(), func);

                g.process_ret(ret)
            }
        }
    }
}

impl<'a, C, P1, P2, M, O> Parse<C> for Or<P1, P2, M, O>
where
    P1: Parse<C, Ret = Span>,
    P2: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P1::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.pat1).or(g.reset().try_mat(&self.pat2))
    }
}
