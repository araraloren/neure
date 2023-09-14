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

pub struct Then<P, T, M, O> {
    pat: P,
    then: T,
    marker: PhantomData<(M, O)>,
}

impl<P, T, M, O> Then<P, T, M, O> {
    pub fn new(pat1: P, then: T) -> Self {
        Self {
            pat: pat1,
            then,
            marker: PhantomData,
        }
    }
}

impl<'a, C, P, T, M, O> Invoke<'a, C, M, O> for Then<P, T, M, O>
where
    P: Invoke<'a, C, M, O>,
    T: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.invoke(g.ctx(), func) {
            Ok(_) => {
                let ret = self.then.invoke(g.reset().ctx(), func);

                g.process_ret(ret)
            }
            Err(e) => Err(e),
        }
    }
}

impl<'a, C, P, T, M, O> Parse<C> for Then<P, T, M, O>
where
    P: Parse<C, Ret = Span>,
    T: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.pat).or(g.reset().try_mat(&self.then))
    }
}
