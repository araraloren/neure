use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct Then<C, P, T> {
    pat: P,
    then: T,
    marker: PhantomData<C>,
}

impl<C, P, T> Clone for Then<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            then: self.then.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> Then<C, P, T> {
    pub fn new(pat: P, then: T) -> Self {
        Self {
            pat,
            then,
            marker: PhantomData,
        }
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

impl<'a, C, P, T, M, O> Invoke<'a, C, M, O> for Then<C, P, T>
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
                let ret = self.then.invoke(g.ctx(), func);

                g.process_ret(ret)
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, T> Regex<C> for Then<C, P, T>
where
    P: Regex<C, Ret = Span>,
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.pat)?;

        ret.add_assign(g.try_mat(&self.then)?);
        Ok(ret)
    }
}
