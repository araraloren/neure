use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub struct Or<L, R> {
    left: L,
    right: R,
}

impl<L, R> Or<L, R> {
    pub fn new(pat1: L, pat2: R) -> Self {
        Self {
            left: pat1,
            right: pat2,
        }
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

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<'a, C, L, R, M, O> Invoke<'a, C, M, O> for Or<L, R>
where
    L: Invoke<'a, C, M, O>,
    R: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        match self.left.invoke(g.ctx(), func) {
            Ok(ret) => Ok(ret),
            Err(_) => {
                let ret = self.right.invoke(g.reset().ctx(), func);

                g.process_ret(ret)
            }
        }
    }
}

impl<'a, C, L, R> Parse<C> for Or<L, R>
where
    L: Parse<C, Ret = Span>,
    R: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = L::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)
            .or_else(|_| g.reset().try_mat(&self.right))
    }
}
