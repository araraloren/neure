use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct Separate<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, S, R> Clone for Separate<C, L, S, R>
where
    L: Clone,
    S: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            sep: self.sep.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, S, R> Separate<C, L, S, R> {
    pub fn new(left: L, sep: S, right: R) -> Self {
        Self {
            left,
            sep,
            right,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }
}

impl<'a, C, L, S, R, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for Separate<C, L, S, R>
where
    L: Ctor<'a, C, M, O1>,
    R: Ctor<'a, C, M, O2>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let ret1 = self.left.constrct(g.ctx(), func);
        let ret1 = g.process_ret(ret1)?;

        g.try_mat(&self.sep)?;

        let ret2 = self.right.constrct(g.ctx(), func);
        let ret2 = g.process_ret(ret2)?;

        Ok((ret1, ret2))
    }
}

impl<'a, C, L, S, R> Regex<C> for Separate<C, L, S, R>
where
    S: Regex<C, Ret = Span>,
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut span = Span::default();
        let mut g = CtxGuard::new(ctx);

        span.add_assign(g.try_mat(&self.left)?);
        span.add_assign(g.try_mat(&self.sep)?);
        span.add_assign(g.try_mat(&self.right)?);

        Ok(span)
    }
}
