use std::marker::PhantomData;

use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Map<P, F, M, O, V> {
    pat: P,
    func: F,
    marker: PhantomData<(M, O, V)>,
}

impl<P, F, M, O, V> Map<P, F, M, O, V> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
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

impl<'a, C, M, O, V, P, F> Invoke<'a, C, M, V> for Map<P, F, M, O, V>
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

impl<'a, C, P, F, M, O, V> Parse<C> for Map<P, F, M, O, V>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
