use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::length_of;
use super::ret_and_inc;
use super::Condition;
use super::Neu;
use super::NeuCond;

#[derive(Debug, Copy)]
pub struct NeureThen<C, L, R, T, I>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, L, R, T, I> Clone for NeureThen<C, L, R, T, I>
where
    I: Clone,
    L: Neu<T> + Clone,
    R: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            cond: self.cond.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R, T, I> NeureThen<C, L, R, T, I>
where
    L: Neu<T>,
    R: Neu<T>,
{
    pub fn new(left: L, right: R, r#if: I) -> Self {
        Self {
            left,
            cond: r#if,
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

    pub fn set_left(&mut self, unit: L) -> &mut Self {
        self.left = unit;
        self
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_right(&mut self, unit: R) -> &mut Self {
        self.right = unit;
        self
    }
}

impl<'a, C, L, R, I> Condition<'a, C> for NeureThen<C, L, R, C::Item, I>
where
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureThen<C, L, R, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureThen::new(self.left, self.right, r#if)
    }
}

impl<'a, L, R, C, O, I> Ctor<'a, C, O, O> for NeureThen<C, L, R, C::Item, I>
where
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C> + 'a,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("neure_one", beg, g.try_mat(self));

        trace!("neure_one", beg -> g.end(), ret.is_ok());
        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, L, R, C, I> Regex<C> for NeureThen<C, L, R, C::Item, I>
where
    C: Context<'a> + 'a,
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut iter = g.ctx().peek()?;
        let mut ret = Err(Error::NeuOne);
        let beg = g.beg();

        trace!("neure_one", beg, ());
        if let Some((offset, item)) = iter.next() {
            if self.left.is_match(&item) && self.cond.check(g.ctx(), &(offset, item))? {
                if let Some((offset, item)) = iter.next() {
                    if self.right.is_match(&item) && self.cond.check(g.ctx(), &(offset, item))? {
                        let len = length_of(offset, g.ctx(), iter.next().map(|v| v.0));
                        ret = Ok(ret_and_inc(g.ctx(), 1, len));
                    }
                }
            }
        }
        trace!("neure_one", beg => g.end(), g.process_ret(ret))
    }
}