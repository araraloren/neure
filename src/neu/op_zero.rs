use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::length_of;
use super::ret_and_inc;
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
    type Output = crate::re::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
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

impl<'a, U, C, O, I, H, A> Ctor<'a, C, O, O, H, A> for NeureZeroOne<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(A::extract(g.ctx(), &ret?)?)
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
        let mut g = CtxGuard::new(ctx);
        let mut ret = Ok(Span::new(g.beg(), 0));

        crate::debug_regex_beg!("NeureZeroOne", g.beg());
        if let Ok(mut iter) = g.ctx().peek() {
            if let Some((offset, item)) = iter.next() {
                if self.unit.is_match(&item) && self.cond.check(g.ctx(), &(offset, item))? {
                    let len = length_of(offset, g.ctx(), iter.next().map(|v| v.0));

                    ret = Ok(ret_and_inc(g.ctx(), len));
                }
            }
        }
        crate::debug_regex_reval!("NeureZeroOne", g.beg(), g.end(), g.process_ret(ret))
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
    type Output = crate::re::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
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

impl<'a, U, C, O, I, H, A> Ctor<'a, C, O, O, H, A> for NeureZeroMore<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(A::extract(g.ctx(), &ret?)?)
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
        let mut g = CtxGuard::new(ctx);
        let mut beg = None;
        let mut end = None;
        let mut ret = Ok(Span::new(g.beg(), 0));

        crate::debug_regex_beg!("NeureZeroMore", g.beg());
        if let Ok(mut iter) = g.ctx().peek() {
            for pair in iter.by_ref() {
                if !self.unit.is_match(&pair.1) || !self.cond.check(g.ctx(), &pair)? {
                    end = Some(pair);
                    break;
                }
                if beg.is_none() {
                    beg = Some(pair.0);
                }
            }
        }
        if let Some(start) = beg {
            let len = length_of(start, g.ctx(), end.map(|v| v.0));
            ret = Ok(ret_and_inc(g.ctx(), len));
        }
        crate::debug_regex_reval!("NeureZeroMore", g.beg(), g.end(), g.process_ret(ret))
    }
}
