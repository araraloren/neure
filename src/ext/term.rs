use super::Collect;
use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Terminated<P, S> {
    pat: P,
    sep: S,
}

impl<P, S> Terminated<P, S> {
    pub fn new(pat: P, sep: S) -> Self {
        Self { pat, sep }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }
}

impl<P, S> Terminated<P, S> {
    pub fn collect<O>(self) -> Collect<Self, O> {
        Collect::new(self)
    }
}

impl<'a, C, S, P, M, O> Invoke<'a, C, M, O> for Terminated<P, S>
where
    S: Parse<C, Ret = Span>,
    P: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let ret = self.pat.invoke(g.ctx(), func);
        let ret = g.process_ret(ret)?;

        g.try_mat(&self.sep)?;
        Ok(ret)
    }
}

impl<'a, C, S, P> Parse<C> for Terminated<P, S>
where
    S: Parse<C, Ret = Span>,
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(&self.pat)?;

        g.try_mat(&self.sep)?;
        Ok(ret)
    }
}
