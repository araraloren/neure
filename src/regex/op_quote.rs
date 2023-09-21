use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::err::Error;
use crate::parser::Context;
use crate::parser::Policy;
use crate::parser::Span;
use crate::regex::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Quote<P, L, R> {
    pat: P,
    left: L,
    right: R,
}

impl<P, L, R> Quote<P, L, R> {
    pub fn new(pat: P, left: L, right: R) -> Self {
        Self { pat, left, right }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<'a, C, L, R, P, M, O> Invoke<'a, C, M, O> for Quote<P, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    P: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = self.pat.invoke(g.ctx(), func);
        let ret = g.process_ret(ret)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}

impl<'a, C, L, R, P> Regex<C> for Quote<P, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = g.try_mat(&self.pat)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}
