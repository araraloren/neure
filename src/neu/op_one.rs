use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
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
/// Repeat the unit `U` one time.
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
///     let hex = hex.repeat_one();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct NeureOne<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for NeureOne<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for NeureOne<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeureOne")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
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
    C: Match<'a> + 'a,
{
    type Out<F> = NeureOne<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOne::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, O, H> for NeureOne<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a> + 'a,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(g.ctx(), &ret?).map_err(Into::into)
    }
}

impl<'a, U, C, I> Regex<C> for NeureOne<C, U, C::Item, I>
where
    C: Match<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::NeureOne);
        let mut iter = ctx.peek()?;

        crate::debug_regex_beg!("NeureOne", ctx.beg());
        if let Some((offset, item)) = iter.next() {
            if self.unit.is_match(&item) && self.cond.check(ctx.ctx(), &(offset, item))? {
                let len = length_of(offset, ctx.ctx(), iter.next().map(|v| v.0));

                ret = Ok(ctx.inc(len));
            }
        }

        crate::debug_regex_reval!("NeureOne", ctx.process_ret(ret))
    }
}

///
/// Repeat the unit `U` one or more times.
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
///     let hex = hex.repeat_one_more();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct NeureOneMore<C, U, T, I>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for NeureOneMore<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for NeureOneMore<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeureOneMore")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
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
    C: Match<'a> + 'a,
{
    type Out<F> = NeureOneMore<C, U, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOneMore::new(self.unit, r#if)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, O, H> for NeureOneMore<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a> + 'a,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(g.ctx(), &ret?).map_err(Into::into)
    }
}

impl<'a, U, C, I> Regex<C> for NeureOneMore<C, U, C::Item, I>
where
    C: Match<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::NeureOneMore);
        let mut iter = ctx.peek()?;

        crate::debug_regex_beg!("NeureOneMore", ctx.beg());
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

            ret = Ok(ctx.inc(len))
        }

        crate::debug_regex_reval!("NeureOneMore", ctx.process_ret(ret))
    }
}
