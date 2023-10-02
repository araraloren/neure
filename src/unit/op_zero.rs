use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Extract;
use crate::regex::Handler;
use crate::regex::Invoke;
use crate::regex::Regex;
use crate::trace_log;

use super::inc_and_ret;
use super::length_of;
use super::Unit;
use super::UnitCond;

#[derive(Debug, Copy)]
pub struct UnitZeroOne<C, U, T, I>
where
    U: Unit<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for UnitZeroOne<C, U, T, I>
where
    I: Clone,
    U: Unit<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<C, U, T, I> UnitZeroOne<C, U, T, I>
where
    U: Unit<T>,
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

impl<'a, C, U, T, I> UnitZeroOne<C, U, T, I>
where
    U: Unit<T>,
    C: Context<'a>,
{
    pub fn with_if<F>(self, r#if: F) -> UnitZeroOne<C, U, T, F>
    where
        F: UnitCond<'a, C>,
    {
        UnitZeroOne::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Invoke<'a, C, O, O> for UnitZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    I: UnitCond<'a, C>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, C, I> Regex<C> for UnitZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    I: UnitCond<'a, C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        trace_log!("match data in zero_one(0..=1)");
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if self.unit.is_match(&item) && self.cond.check(ctx, &(offset, item))? {
                    return Ok(inc_and_ret(
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
pub struct UnitZeroMore<C, U, T, I>
where
    U: Unit<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> Clone for UnitZeroMore<C, U, T, I>
where
    I: Clone,
    U: Unit<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            cond: self.cond.clone(),
            marker: self.marker,
        }
    }
}

impl<C, U, T, I> UnitZeroMore<C, U, T, I>
where
    U: Unit<T>,
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

impl<'a, C, U, T, I> UnitZeroMore<C, U, T, I>
where
    U: Unit<T>,
    C: Context<'a>,
{
    pub fn with_if<F>(self, r#if: F) -> UnitZeroMore<C, U, T, F>
    where
        F: UnitCond<'a, C>,
    {
        UnitZeroMore::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I> Invoke<'a, C, O, O> for UnitZeroMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    I: UnitCond<'a, C>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, C, I> Regex<C> for UnitZeroMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    I: UnitCond<'a, C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;

        trace_log!("match data in zero_more(0..)");
        if let Ok(mut iter) = ctx.peek() {
            for pair in iter.by_ref() {
                if !self.unit.is_match(&pair.1) && self.cond.check(ctx, &pair)? {
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
            Ok(inc_and_ret(
                ctx,
                cnt,
                length_of(start, ctx, end.map(|v| v.0)),
            ))
        } else {
            Ok(<Self::Ret as Ret>::from(ctx, (0, 0)))
        }
    }
}
