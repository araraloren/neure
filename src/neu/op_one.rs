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
/// Matches a single context-sensitive element using combined pattern and condition checks.
///
/// [`NeureOne`] provides atomic element matching with two-stage validation:
/// 1. **Base pattern match**: Verifies the element satisfies a core pattern ([`Neu`])
/// 2. **Context condition**: Validates additional runtime constraints ([`NeuCond`])
///
/// # Regex
///
/// Attempts to match exactly one element at the current position:
/// - **Success**: Returns a span covering the single matched element
///   - Requires BOTH conditions to pass:
///     a. `unit.is_match(item)` returns true for the element
///     b. `cond.check()` returns true for the context
/// - **Failure**: Returns error immediately if either condition fails
/// - **Zero-width**: Consumes no input on failure
///
/// # Ctor
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// The specific constructed value depends on the active handler implementation.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.repeat_one();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
/// #   Ok(())
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
    type Output = crate::regex::Assert<Self>;

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
    pub fn new(unit: U, cond: I) -> Self {
        Self {
            unit,
            cond,
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

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOne::new(self.unit, cond)
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
/// Matches one or more consecutive context-sensitive elements using combined pattern and condition checks.
///
/// [`NeureOneMore`] extends [`NeureOne`]'s validation model to sequences, requiring every element to satisfy:
/// 1. **Base pattern match**: Core element validation ([`Neu`])
/// 2. **Context condition**: Runtime constraints ([`NeuCond`])
///
/// This combinator enables context-aware repetition with strict per-element validation, forming the basis
/// for complex token recognition where every character's validity depends on dynamic parser state.
///
/// # Core Behavior
///
/// # Regex
///
/// Matches one or more consecutive elements:
/// - **Success**: Returns span covering all matched elements
///   - Requires **at least one element** to match
///   - **Every element** must satisfy BOTH conditions:
///     a. `unit.is_match(item)` returns true
///     b. `cond.check()` returns true for the context
///   - Stops at first failing element (greedy match)
/// - **Failure**: Returns error if:
///   - No elements match (fails minimum length requirement)
///   - First element fails either condition
/// - **Zero-width**: Consumes no input on failure
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// The specific constructed value depends on the active handler implementation.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.repeat_one_more();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
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
    type Output = crate::regex::Assert<Self>;

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
    pub fn new(unit: U, cond: I) -> Self {
        Self {
            unit,
            cond,
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

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureOneMore::new(self.unit, cond)
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
