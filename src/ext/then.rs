use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Then<P, T> {
    pat: P,
    then: T,
}

impl<P, T> Then<P, T> {
    pub fn new(pat1: P, then: T) -> Self {
        Self { pat: pat1, then }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn then(&self) -> &T {
        &self.then
    }

    pub fn then_mut(&mut self) -> &mut T {
        &mut self.then
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_then(&mut self, then: T) -> &mut Self {
        self.then = then;
        self
    }
}

impl<'a, C, P, T, M, O> Invoke<'a, C, M, O> for Then<P, T>
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
        let ret = self.pat.invoke(g.ctx(), func);
        #[allow(unused)]
        let ret = g.process_ret(ret);
        let ret = self.then.invoke(g.ctx(), func);

        g.process_ret(ret)
    }
}

impl<'a, C, P, T> Parse<C> for Then<P, T>
where
    P: Parse<C, Ret = Span>,
    T: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let _ = g.try_mat(&self.pat);

        g.try_mat(&self.then)
    }
}
