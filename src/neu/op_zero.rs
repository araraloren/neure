use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

use super::length_of;
use super::Condition;
use super::Neu;
use super::NeuCond;

///
/// Repeat the unit `U` zero or one time.
///
/// # Ctor
///
/// Return [`Orig`](crate::ctx::Context::Orig) with the [`Span`] as the index if the match is found.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = 'a'..'g';
///     let hex = hex.repeat_zero_one();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct NeureZeroOne<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for NeureZeroOne<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::Not<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for NeureZeroOne<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeureZeroOne")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
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

impl<'a, C, U, I> Condition<'a, C> for NeureZeroOne<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
{
    type Out<F> = NeureZeroOne<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureZeroOne::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, O, H> for NeureZeroOne<C, U, C::Item, I>
where
    C: Match<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(g.ctx(), &ret?).map_err(Into::into)
    }
}

impl<'a, U, C, I> Regex<C> for NeureZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Ok(Span::new(ctx.beg(), 0));

        crate::debug_regex_beg!("NeureZeroOne", ctx.beg());
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if self.unit.is_match(&item) && self.cond.check(ctx.ctx(), &(offset, item))? {
                    let len = length_of(offset, ctx.ctx(), iter.next().map(|v| v.0));

                    ret = Ok(ctx.inc(len));
                }
            }
        }
        crate::debug_regex_reval!("NeureZeroOne", ctx.process_ret(ret))
    }
}

///
/// Repeat the unit `U` zero or more times.
///
/// # Ctor
///
/// Return [`Orig`](crate::ctx::Context::Orig) with the [`Span`] as the index if the match is found.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = 'a'..'g';
///     let hex = hex.repeat_zero_more();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct NeureZeroMore<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for NeureZeroMore<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::Not<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for NeureZeroMore<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeureZeroMore")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
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

impl<'a, C, U, I> Condition<'a, C> for NeureZeroMore<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
{
    type Out<F> = NeureZeroMore<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureZeroMore::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, O, H> for NeureZeroMore<C, U, C::Item, I>
where
    C: Match<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(g.ctx(), &ret?).map_err(Into::into)
    }
}

impl<'a, U, C, I> Regex<C> for NeureZeroMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut beg = None;
        let mut end = None;
        let mut ret = Ok(Span::new(ctx.beg(), 0));

        crate::debug_regex_beg!("NeureZeroMore", ctx.beg());
        if let Ok(mut iter) = ctx.peek() {
            for pair in iter.by_ref() {
                if !self.unit.is_match(&pair.1) || !self.cond.check(ctx.ctx(), &pair)? {
                    end = Some(pair);
                    break;
                }
                if beg.is_none() {
                    beg = Some(pair.0);
                }
            }
            if let Some(start) = beg {
                let len = length_of(start, ctx.ctx(), end.map(|v| v.0));

                ret = Ok(ctx.inc(len));
            }
        }
        crate::debug_regex_reval!("NeureZeroMore", ctx.process_ret(ret))
    }
}
