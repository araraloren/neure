use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace_v;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::length_of;
use super::ret_and_inc;
use super::CRange;
use super::Condition;
use super::Neu;
use super::NeuCond;

#[derive(Debug, Copy)]
pub struct NeureRepeat<const M: usize, const N: usize, C, U, I> {
    unit: U,
    cond: I,
    marker: PhantomData<C>,
}

impl<const M: usize, const N: usize, C, U, I> Clone for NeureRepeat<M, N, C, U, I>
where
    I: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<const M: usize, const N: usize, C, U, I> NeureRepeat<M, N, C, U, I> {
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

impl<'a, const M: usize, const N: usize, C, U, I> Condition<'a, C> for NeureRepeat<M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureRepeat<M, N, C, U, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureRepeat::<M, N, C, U, F>::new(self.unit, r#if)
    }
}

impl<'a, const M: usize, const N: usize, U, C, O, I> Ctor<'a, C, O, O>
    for NeureRepeat<M, N, C, U, I>
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
        let range = M..N;
        let ret = trace_v!("neu repeat", &range, beg, g.try_mat(self));

        trace_v!( "neu repeat", range, beg -> g.end(), ret.is_ok(), 1);
        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, const M: usize, const N: usize, U, C, I> Regex<C> for NeureRepeat<M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::NeuRepeat);
        let iter = g.ctx().peek();
        let offset = g.beg();
        let range = M..N;

        trace_v!("neu repeat", &range, offset, ());
        if let Ok(mut iter) = iter {
            while cnt < N {
                if let Some(pair) = iter.next() {
                    if self.unit.is_match(&pair.1) && self.cond.check(g.ctx(), &pair)? {
                        cnt += 1;
                        if beg.is_none() {
                            beg = Some(pair.0);
                        }
                        continue;
                    } else {
                        end = Some(pair);
                    }
                }
                break;
            }
            if cnt >= M {
                let end = end.or_else(|| iter.next()).map(|v| v.0);
                let len = beg.map(|v| length_of(v, g.ctx(), end)).unwrap_or(0);
                ret = Ok(ret_and_inc(g.ctx(), cnt, len));
            }
        }
        trace_v!("neu repeat", range, offset => g.end(), ret, cnt)
    }
}

#[derive(Debug, Copy)]
pub struct NeureRepeatRange<C, U, I> {
    unit: U,
    cond: I,
    range: CRange<usize>,
    marker: PhantomData<C>,
}

impl<C, U, I> Clone for NeureRepeatRange<C, U, I>
where
    U: Clone,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            range: self.range,
            marker: self.marker,
        }
    }
}

impl<C, U, I> NeureRepeatRange<C, U, I> {
    pub fn new(unit: U, range: CRange<usize>, cond: I) -> Self {
        Self {
            unit,
            range,
            cond,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn range(&self) -> &CRange<usize> {
        &self.range
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn range_mut(&mut self) -> &mut CRange<usize> {
        &mut self.range
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn set_range(&mut self, range: CRange<usize>) -> &mut Self {
        self.range = range;
        self
    }
}

impl<'a, C, U, I> Condition<'a, C> for NeureRepeatRange<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureRepeatRange<C, U, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureRepeatRange::<C, U, F>::new(self.unit, self.range, r#if)
    }
}

impl<'a, U, C, M, I> Ctor<'a, C, M, M> for NeureRepeatRange<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C> + 'a,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<M, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace_v!("neu repeat_range", self.range, beg, g.try_mat(self));

        trace_v!( "neu repeat_range", self.range, beg -> g.end(), ret.is_ok(), 1);
        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, U, C, I> Regex<C> for NeureRepeatRange<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::NeuRepeatRange);
        let iter = g.ctx().peek();
        let offset = g.beg();

        trace_v!("neu repeat_range", self.range, offset, ());
        if let Ok(mut iter) = iter {
            fn bound_checker(max: Option<usize>) -> impl Fn(usize) -> bool {
                move |val| max.map(|max| val < max).unwrap_or(true)
            }

            let cond = bound_checker(match self.range.end_bound() {
                std::ops::Bound::Included(max) => Some(*max),
                std::ops::Bound::Excluded(max) => Some(max.saturating_sub(1)),
                std::ops::Bound::Unbounded => None,
            });

            while cond(cnt) {
                if let Some(pair) = iter.next() {
                    if self.unit.is_match(&pair.1) && self.cond.check(g.ctx(), &pair)? {
                        cnt += 1;
                        if beg.is_none() {
                            beg = Some(pair.0);
                        }
                        continue;
                    } else {
                        end = Some(pair);
                    }
                }
                break;
            }
            if self.range.contains(&cnt) {
                let end = end.or_else(|| iter.next()).map(|v| v.0);
                let len = beg.map(|v| length_of(v, g.ctx(), end)).unwrap_or(0);
                ret = Ok(ret_and_inc(g.ctx(), cnt, len));
            }
        }
        trace_v!("neu repeat_range", self.range, offset => g.end(), ret, cnt)
    }
}
