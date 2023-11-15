use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::trace_log;

use super::length_of;
use super::ret_and_inc;
use super::Neu;
use super::NeuCond;

#[derive(Debug, Copy)]
pub struct NeureZeroOne<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for NeureZeroOne<C, U, T, I>
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

impl<C, U, T, I> NeureZeroOne<C, U, T, I>
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

impl<'a, C, U, T, I> NeureZeroOne<C, U, T, I>
where
    U: Neu<T>,
    C: Context<'a>,
{
    pub fn set_cond<F>(self, r#if: F) -> NeureZeroOne<C, U, T, F>
    where
        F: NeuCond<'a, C>,
    {
        NeureZeroOne::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Ctor<'a, C, O, O> for NeureZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, C, I> Regex<C> for NeureZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        trace_log!("match data in zero_one(0..2)");
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if self.unit.is_match(&item) && self.cond.check(ctx, &(offset, item))? {
                    return Ok(ret_and_inc(
                        ctx,
                        1,
                        length_of(offset, ctx, iter.next().map(|v| v.0)),
                    ));
                }
            }
        }
        Ok(<Self::Ret as Ret>::from(ctx, (0, 0)))
    }
}

#[derive(Debug, Copy)]
pub struct NeureZeroMore<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for NeureZeroMore<C, U, T, I>
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

impl<C, U, T, I> NeureZeroMore<C, U, T, I>
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

impl<'a, C, U, T, I> NeureZeroMore<C, U, T, I>
where
    U: Neu<T>,
    C: Context<'a>,
{
    pub fn set_cond<F>(self, r#if: F) -> NeureZeroMore<C, U, T, F>
    where
        F: NeuCond<'a, C>,
    {
        NeureZeroMore::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Ctor<'a, C, O, O> for NeureZeroMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, C, I> Regex<C> for NeureZeroMore<C, U, C::Item, I>
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

        trace_log!("match data in zero_more(0..)");
        if let Ok(mut iter) = ctx.peek() {
            for pair in iter.by_ref() {
                if !self.unit.is_match(&pair.1) || !self.cond.check(ctx, &pair)? {
                    end = Some(pair);
                    break;
                }
                cnt += 1;
                if beg.is_none() {
                    beg = Some(pair.0);
                }
            }
        }
        if let Some(start) = beg {
            Ok(ret_and_inc(
                ctx,
                cnt,
                length_of(start, ctx, end.map(|v| v.0)),
            ))
        } else {
            Ok(<Self::Ret as Ret>::from(ctx, (0, 0)))
        }
    }
}
