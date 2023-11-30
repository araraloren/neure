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
pub struct NeureOne<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for NeureOne<C, U, T, I>
where
    I: Clone,
    U: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<C, U, T, I> NeureOne<C, U, T, I>
where
    U: Neu<T>,
{
    pub fn new(unit: U, r#if: I) -> Self {
        Self {
            unit,
            cond: r#if,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<'a, C, U, I> Condition<'a, C> for NeureOne<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureOne<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOne::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Ctor<'a, C, O, O> for NeureOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C>,
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

impl<'a, U, C, I> Regex<C> for NeureOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
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
            if self.unit.is_match(&item) && self.cond.check(g.ctx(), &(offset, item))? {
                let len = length_of(offset, g.ctx(), iter.next().map(|v| v.0));
                ret = Ok(ret_and_inc(g.ctx(), 1, len));
            }
        }
        trace!("neure_one", beg => g.end(), g.process_ret(ret))
    }
}

#[derive(Debug, Copy)]
pub struct NeureOneMore<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for NeureOneMore<C, U, T, I>
where
    I: Clone,
    U: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<C, U, T, I> NeureOneMore<C, U, T, I>
where
    U: Neu<T>,
{
    pub fn new(unit: U, r#if: I) -> Self {
        Self {
            unit,
            cond: r#if,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<'a, C, U, I> Condition<'a, C> for NeureOneMore<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureOneMore<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOneMore::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Ctor<'a, C, O, O> for NeureOneMore<C, U, C::Item, I>
where
    U: Neu<C::Item>,
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
        let ret = trace!("neure_one_more", beg, g.try_mat(self));

        trace!("neure_one_more", beg -> g.end(), ret.is_ok());
        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, U, C, I> Regex<C> for NeureOneMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::NeuOneMore);
        let mut iter = g.ctx().peek()?;
        let offset = g.beg();

        trace!("neure_one_more", offset, ());
        for pair in iter.by_ref() {
            if !self.unit.is_match(&pair.1) || !self.cond.check(g.ctx(), &pair)? {
                end = Some(pair);
                break;
            }
            cnt += 1;
            if beg.is_none() {
                beg = Some(pair.0);
            }
        }
        if let Some(start) = beg {
            let len = length_of(start, g.ctx(), end.map(|v| v.0));
            ret = Ok(ret_and_inc(g.ctx(), cnt, len))
        }
        trace!("neure_one_more", offset => g.end(), g.process_ret(ret))
    }
}
