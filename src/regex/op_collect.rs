use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::prelude::Ret;
use crate::regex::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Collect<P, O> {
    pat: P,
    marker: PhantomData<O>,
}

impl<P, O> Collect<P, O> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }
}

impl<'a, C, P, M, O, V> Invoke<'a, C, M, V> for Collect<P, O>
where
    V: FromIterator<O>,
    P: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ok(V::from_iter(std::iter::from_fn(|| {
            self.pat.invoke(ctx, func).ok()
        })))
    }
}

impl<'a, C, P, O> Regex<C> for Collect<P, O>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = g.try_mat(&self.pat)?;

        while let Ok(ret) = g.try_mat(&self.pat) {
            span.add_assign(ret);
        }
        Ok(span)
    }
}
