use std::marker::PhantomData;

use super::Extract;
use super::Handler;
use super::Invoke;

use crate::err::Error;
use crate::parser::Context;
use crate::parser::Policy;
use crate::parser::Span;
use crate::regex::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Map<P, F, O> {
    pat: P,
    func: F,
    marker: PhantomData<O>,
}

impl<P, F, O> Map<P, F, O> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn func(&self) -> &F {
        &self.func
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_func(&mut self, func: F) -> &mut Self {
        self.func = func;
        self
    }
}

impl<'a, C, M, O, V, P, F> Invoke<'a, C, M, V> for Map<P, F, O>
where
    P: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
    F: Fn(O) -> Result<V, Error>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        (self.func)(self.pat.invoke(ctx, func)?)
    }
}

impl<'a, C, P, F, O> Regex<C> for Map<P, F, O>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
