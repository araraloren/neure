use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::EmptyCond;
use crate::regex::Regex;

use super::Condition;
use super::Neu;
use super::NeuCond;
use super::length_of;

///
/// Matches zero or one context-sensitive element with guaranteed success.
///
/// `Opt` provides optional matching with full context validation, always succeeding in one of two ways:
/// - **One element**: When the first element satisfies both pattern and context conditions
/// - **Zero elements**: When no valid element exists at current position (returns empty span)
///
/// This combinator is the context-aware equivalent of the `?` (optional) operator in regular expressions,
/// but with runtime context validation for the potential match. Unlike standard optional patterns,
/// it performs deep validation of parser state when attempting to match an element.
///
/// # Regex
///
/// Always succeeds with one of two outcomes:
/// - **Single element match**: Returns span covering the matched element
///   - Requires the first element to satisfy BOTH:
///     a. `unit.is_match(item)` returns true
///     b. `cond.check()` returns true for the context
/// - **Empty match**: Returns zero-length span at current position when:
///   - No elements available
///   - First element fails either condition
///   - Context conditions reject the potential match
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// Handler implementations must handle both cases:
/// - Present value (when one element matched)
/// - Absent value (when zero elements matched)
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.opt();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
/// #   Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Opt<C, U, T, I = EmptyCond>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for Opt<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for Opt<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Opt")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
}

impl<C, U, T, I> Default for Opt<C, U, T, I>
where
    I: Default,
    U: Neu<T> + Default,
{
    fn default() -> Self {
        Self {
            unit: Default::default(),
            cond: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, U, T, I> Clone for Opt<C, U, T, I>
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

impl<C, U, T, I> Opt<C, U, T, I>
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

impl<'a, C, U, I> Condition<'a, C> for Opt<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
{
    type Out<F> = Opt<C, U, C::Item, F>;

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        Opt::new(self.unit, cond)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, H> for Opt<C, U, C::Item, I>
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

impl<'a, U, C, I> Regex<C> for Opt<C, U, C::Item, I>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Ok(Span::new(ctx.beg(), 0));

        crate::debug_regex_beg!("Opt", ctx.beg());
        if let Ok(mut iter) = ctx.peek()
            && let Some((offset, item)) = iter.next()
            && self.unit.is_match(&item)
            && self.cond.check(ctx.ctx(), &(offset, item))?
        {
            let len = length_of(offset, ctx.ctx(), iter.next().map(|v| v.0));

            ret = Ok(ctx.inc(len));
        }
        crate::debug_regex_reval!("Opt", ctx.process_ret(ret))
    }
}

///
/// Matches zero or more context-sensitive elements with guaranteed success.
///
/// `Many0` provides greedy repetition with full context validation, always succeeding in one of two ways:
/// - **Sequence match**: Longest possible sequence where every element satisfies both pattern and context conditions
/// - **Empty match**: Zero-length span when no valid elements exist at current position
///
/// This combinator is the context-aware equivalent of the `*` (Kleene star) operator in regular expressions,
/// but with per-element runtime context validation. It maintains parser state consistency through complex
/// context conditions during repetition, enabling sophisticated token-skipping and sequence recognition.
///
/// # Regex
///
/// Always succeeds with one of two outcomes:
/// - **Non-empty sequence**: Returns span covering all matched elements
///   - **Every element** in the sequence must satisfy BOTH:
///     a. `unit.is_match(item)` returns true
///     b. `cond.check()` returns true for the context
///   - Stops at first element that fails either condition
///   - Greedily consumes the longest valid sequence
/// - **Empty sequence**: Returns zero-length span at current position when:
///   - No elements available
///   - First element fails either condition
///   - Context conditions reject the potential match
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// Handler implementations must handle both cases:
/// - Non-empty sequence (when elements matched)
/// - Empty sequence (when zero elements matched)
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.many0();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Many0<C, U, T, I = EmptyCond>
where
    U: Neu<T>,
{
    unit: U,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, U, T, I> std::ops::Not for Many0<C, U, T, I>
where
    U: Neu<T>,
{
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, U, T, I> Debug for Many0<C, U, T, I>
where
    I: Debug,
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Many0")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
}

impl<C, U, T, I> Default for Many0<C, U, T, I>
where
    I: Default,
    U: Neu<T> + Default,
{
    fn default() -> Self {
        Self {
            unit: Default::default(),
            cond: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, U, T, I> Clone for Many0<C, U, T, I>
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

impl<C, U, T, I> Many0<C, U, T, I>
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

impl<'a, C, U, I> Condition<'a, C> for Many0<C, U, C::Item, I>
where
    U: Neu<C::Item>,
    C: Context<'a>,
{
    type Out<F> = Many0<C, U, C::Item, F>;

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        Many0::new(self.unit, cond)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, H> for Many0<C, U, C::Item, I>
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

impl<'a, U, C, I> Regex<C> for Many0<C, U, C::Item, I>
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

        crate::debug_regex_beg!("Many0", ctx.beg());
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
        crate::debug_regex_reval!("Many0", ctx.process_ret(ret))
    }
}
