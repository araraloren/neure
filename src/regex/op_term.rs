use std::marker::PhantomData;

use super::Collect;
use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

#[derive(Debug, Default, Copy)]
pub struct Terminated<C, P, S> {
    pat: P,
    sep: S,
    marker: PhantomData<C>,
}

impl<C, P, S> Clone for Terminated<C, P, S>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, S> Terminated<C, P, S> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            marker: PhantomData,
        }
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

impl<C, P, S> Terminated<C, P, S> {
    pub fn collect<O>(self) -> Collect<C, Self, O> {
        Collect::new(self)
    }
}

impl<'a, C, S, P, M, O> Invoke<'a, C, M, O> for Terminated<C, P, S>
where
    S: Regex<C, Ret = Span>,
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

impl<'a, C, S, P> Regex<C> for Terminated<C, P, S>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
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
