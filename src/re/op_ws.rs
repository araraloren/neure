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
pub struct PaddingWS<C, P, S> {
    pat: P,
    ws: S,
    marker: PhantomData<C>,
}

impl<C, P, S> Clone for PaddingWS<C, P, S>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            ws: self.ws.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, S> PaddingWS<C, P, S> {
    pub fn new(pat: P, ws: S) -> Self {
        Self {
            pat,
            ws,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn ws(&self) -> &S {
        &self.ws
    }

    pub fn ws_mut(&mut self) -> &mut S {
        &mut self.ws
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_ws(&mut self, ws: S) -> &mut Self {
        self.ws = ws;
        self
    }
}

impl<'a, C, P, S, M, O> Invoke<'a, C, M, O> for PaddingWS<C, P, S>
where
    P: Invoke<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a, Item = char> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.invoke(g.ctx(), func) {
            Ok(ret1) => {
                let ret = g.try_mat(&self.ws);
                Ok(ret1)
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, S> Regex<C> for PaddingWS<C, P, S>
where
    P: Regex<C, Ret = Span>,
    S: Regex<C, Ret = Span>,
    C: Context<'a, Item = char> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.pat)?;

        if let Ok(ws_ret) = g.try_mat(&self.ws) {
            ret.add_assign(ws_ret);
        }
        Ok(ret)
    }
}
