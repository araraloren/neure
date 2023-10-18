use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Invoke;
use crate::re::Regex;

use super::ret_and_inc;
use super::length_of;
use super::CRange;
use super::Neu;
use super::NeuCond;

#[derive(Debug, Copy)]
pub struct NeureRepeat<'a, const M: usize, const N: usize, C, U, I>
where
    C: Context<'a>,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(&'a (), C)>,
}

impl<'a, const M: usize, const N: usize, C, U, I> Clone for NeureRepeat<'a, M, N, C, U, I>
where
    I: Clone,
    C: Context<'a>,
    I: NeuCond<'a, C>,
    U: Neu<C::Item> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<'a, const M: usize, const N: usize, C, U, I> NeureRepeat<'a, M, N, C, U, I>
where
    C: Context<'a>,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
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

impl<'a, const M: usize, const N: usize, C, U, I> NeureRepeat<'a, M, N, C, U, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    pub fn with_if<F>(self, r#if: F) -> NeureRepeat<'a, M, N, C, U, F>
    where
        F: NeuCond<'a, C>,
    {
        NeureRepeat::<M, N, C, U, F>::new(self.unit, r#if)
    }
}

impl<'a, const M: usize, const N: usize, U, C, O, I> Invoke<'a, C, O, O>
    for NeureRepeat<'a, M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C> + 'a,
{
    #[inline(always)]
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, const M: usize, const N: usize, U, C, I> Regex<C> for NeureRepeat<'a, M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        if let Ok(mut iter) = iter {
            while cnt < N {
                if let Some(pair) = iter.next() {
                    if self.unit.is_match(&pair.1) && self.cond.check(ctx, &pair)? {
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

                return Ok(ret_and_inc(
                    ctx,
                    cnt,
                    beg.map(|v| length_of(v, ctx, end)).unwrap_or(0),
                ));
            }
        }
        Err(crate::err::Error::UnitRepeat)
    }
}

#[derive(Debug, Copy)]
pub struct NeureRepeatRange<'a, C, U, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
    I: NeuCond<'a, C>,
{
    unit: U,
    cond: I,
    range: CRange<usize>,
    marker: PhantomData<(&'a (), C)>,
}

impl<'a, C, U, I> Clone for NeureRepeatRange<'a, C, U, I>
where
    C: Context<'a>,
    U: Neu<C::Item> + Clone,
    I: NeuCond<'a, C> + Clone,
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

impl<'a, C, U, I> NeureRepeatRange<'a, C, U, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
    I: NeuCond<'a, C>,
{
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

impl<'a, C, U, I> NeureRepeatRange<'a, C, U, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    pub fn with_if<F>(self, r#if: F) -> NeureRepeatRange<'a, C, U, F>
    where
        F: NeuCond<'a, C>,
    {
        NeureRepeatRange::<C, U, F>::new(self.unit, self.range, r#if)
    }
}

impl<'a, U, C, M, I> Invoke<'a, C, M, M> for NeureRepeatRange<'a, C, U, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<M, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, C, I> Regex<C> for NeureRepeatRange<'a, C, U, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

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
                    if self.unit.is_match(&pair.1) && self.cond.check(ctx, &pair)? {
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

                return Ok(ret_and_inc(
                    ctx,
                    cnt,
                    beg.map(|v| length_of(v, ctx, end)).unwrap_or(0),
                ));
            }
        }
        Err(crate::err::Error::Repeat)
    }
}
