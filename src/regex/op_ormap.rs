use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct OrMap<L, R, F, O> {
    left: L,
    right: R,
    func: F,
    marker: PhantomData<O>,
}

impl<L, R, F, O> OrMap<L, R, F, O> {
    pub fn new(pat1: L, pat2: R, func: F) -> Self {
        Self {
            left: pat1,
            right: pat2,
            func,
            marker: PhantomData,
        }
    }

    pub fn func(&self) -> &F {
        &self.func
    }

    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
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

    pub fn set_func(&mut self, func: F) -> &mut Self {
        self.func = func;
        self
    }
}

impl<'a, C, L, R, F, M, O, V> Invoke<'a, C, M, V> for OrMap<L, R, F, O>
where
    L: Invoke<'a, C, M, V>,
    R: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
    F: Fn(O) -> Result<V, Error>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.left.invoke(g.ctx(), func) {
            Ok(ret) => Ok(ret),
            Err(_) => {
                let ret = self.right.invoke(g.reset().ctx(), func);

                (self.func)(g.process_ret(ret)?)
            }
        }
    }
}

impl<'a, C, L, R, F, O> Regex<C> for OrMap<L, R, F, O>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = L::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)
            .or_else(|_| g.reset().try_mat(&self.right))
    }
}
