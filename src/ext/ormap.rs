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

pub struct OrMap<P1, P2, F, M, O, V> {
    pat1: P1,
    pat2: P2,
    func: F,
    marker: PhantomData<(M, O, V)>,
}

impl<P1, P2, F, M, O, V> OrMap<P1, P2, F, M, O, V> {
    pub fn new(pat1: P1, pat2: P2, func: F) -> Self {
        Self {
            pat1,
            pat2,
            func,
            marker: PhantomData,
        }
    }
}

impl<'a, C, P1, P2, F, M, O, V> Invoke<'a, C, M, V> for OrMap<P1, P2, F, M, O, V>
where
    P1: Invoke<'a, C, M, V>,
    P2: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
    F: Fn(O) -> Result<V, Error>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat1.invoke(g.ctx(), func) {
            Ok(ret) => Ok(ret),
            Err(_) => {
                let ret = self.pat2.invoke(g.reset().ctx(), func);

                (self.func)(g.process_ret(ret)?)
            }
        }
    }
}

impl<'a, C, P1, P2, F, M, O, V> Parse<C> for OrMap<P1, P2, F, M, O, V>
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
