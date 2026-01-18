use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::RangeBounds;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::span::Span;
use crate::ctx::new_span_inc;
use crate::err::Error;
use crate::neu::EmptyCond;
use crate::neu::calc_length;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;

use super::CRange;
use super::Condition;
use super::Neu;
use super::NeuCond;

///
/// Matches a sequence of elements with compile-time bounded repetition and context validation.
///
/// [`Between`] provides precise control over sequence length while maintaining per-element
/// context awareness, enforcing that every matched element satisfies both:
/// 1. **Base pattern match**: Core element validation ([`Neu`])
/// 2. **Context condition**: Runtime constraints ([`NeuCond`])
///
/// Matching occurs within the compile-time defined range `[M, N)`, meaning:
/// - **Minimum**: Exactly `M` elements must match
/// - **Maximum**: Matching stops after `N-1` elements (inclusive)
///
/// # Regex
///
/// Matches between `M` (inclusive) and `N` (exclusive) consecutive elements:
/// - **Success**: Returns span covering all matched elements
///   - Requires `M <= matched_count < N`
///   - **Every element** must satisfy BOTH conditions:
///     a. `unit.is_match(item)` returns true
///     b. `cond.check()` returns true for the context
///   - Stops early when:
///     - Maximum count (`N-1`) is reached
///     - An element fails either condition
/// - **Failure**: Returns error if:
///   - Total matched elements < `M`
///   - First element fails when `M > 0`
/// - **Special case**: When `M = 0`, empty matches succeed (returns zero-length span)
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
///     let hex = hex.between::<1, 6>();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
///
/// # Example 1
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
///
/// # Example 2
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.at_most::<6>();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
///
/// # Example 3
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let hex = 'a'..'g';
///     let hex = hex.at_least::<2>();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Between<const M: usize, const N: usize, C, U, I = EmptyCond> {
    unit: U,
    cond: I,
    marker: PhantomData<C>,
}

impl<const M: usize, const N: usize, C, U, I> core::ops::Not for Between<M, N, C, U, I> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<const M: usize, const N: usize, C, U, I> Debug for Between<M, N, C, U, I>
where
    I: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Between")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .finish()
    }
}

impl<const M: usize, const N: usize, C, U, I> Clone for Between<M, N, C, U, I>
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

impl<const M: usize, const N: usize, C, U, I> Between<M, N, C, U, I> {
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

impl<'a, const M: usize, const N: usize, C, U, I> Condition<'a, C> for Between<M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Out<F> = Between<M, N, C, U, F>;

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        Between::<M, N, C, U, F>::new(self.unit, cond)
    }
}

impl<'a, const M: usize, const N: usize, U, C, O, I, H> Ctor<'a, C, O, H> for Between<M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a> + 'a,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self);

        func.invoke(ctx, &ret?).map_err(Into::into)
    }
}

impl<'a, const M: usize, const N: usize, U, C, I> Regex<C> for Between<M, N, C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::Between);
        let mut iter = ctx.peek()?;
        let remaining_len = ctx.len() - ctx.offset();
        let range = M..N;

        crate::debug_regex_beg!("Between", &range, ctx.offset());
        if remaining_len >= M * self.unit.min_length() {
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
                let end = end.or_else(|| iter.next());
                let len = calc_length(beg, end.map(|v| v.0), remaining_len);

                ret = Ok(new_span_inc(ctx, len));
            }
        }
        crate::debug_regex_reval!("Between", cnt, ret)
    }
}

///
/// Matches a sequence of elements with runtime-specified repetition bounds and context validation.
///
/// [`Times`] provides dynamic control over sequence length while maintaining per-element
/// context awareness, enforcing that every matched element satisfies both:
/// 1. **Base pattern match**: Core element validation ([`Neu`])
/// 2. **Context condition**: Runtime constraints ([`NeuCond`])
///
/// Matching occurs within the runtime-defined range specified by `range`, supporting all standard
/// range types (inclusive/exclusive bounds, unbounded ranges). This combinator bridges the gap between
/// strict compile-time patterns and dynamic runtime requirements.
///
/// # Regex
///
/// Matches elements according to the specified range bounds:
/// - **Success**: Returns span covering all matched elements
///   - Requires element count `cnt` where `range.contains(&cnt)` is true
///   - **Every element** must satisfy BOTH conditions:
///     a. `unit.is_match(item)` returns true
///     b. `cond.check()` returns true for the context
///   - Stops early when:
///     - Range upper bound is reached (based on bound type)
///     - An element fails either condition
/// - **Failure**: Returns error if:
///   - Total matched elements not in specified range
///   - First element fails when minimum bound > 0
/// - **Special cases**:
///   - Empty ranges (e.g., `5..3`) always fail
///   - Unbounded ranges (e.g., `3..`) match as many as possible
///   - Zero-length ranges (e.g., `0..0`) only match empty sequences
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
///     let hex = hex.times(1..7);
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Times<C, U, I = EmptyCond> {
    unit: U,
    cond: I,
    range: CRange<usize>,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Times<C, U, I>);

impl<C, U, I> Debug for Times<C, U, I>
where
    U: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Times")
            .field("unit", &self.unit)
            .field("cond", &self.cond)
            .field("range", &self.range)
            .finish()
    }
}

impl<C, U, I> Clone for Times<C, U, I>
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

impl<C, U, I> Times<C, U, I> {
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

impl<'a, C, U, I> Condition<'a, C> for Times<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    type Out<F> = Times<C, U, F>;

    fn set_cond<F>(self, cond: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        Times::<C, U, F>::new(self.unit, self.range, cond)
    }
}

impl<'a, U, C, O, I, H> Ctor<'a, C, O, H> for Times<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Match<'a> + 'a,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self);

        func.invoke(ctx, &ret?).map_err(Into::into)
    }
}

impl<'a, U, C, I> Regex<C> for Times<C, U, I>
where
    U: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + 'a,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::Times);
        let mut iter = ctx.peek()?;
        let remaining_len = ctx.len() - ctx.offset();

        fn bound_checker(max: Option<usize>) -> impl Fn(usize) -> bool {
            move |val| max.map(|max| val < max).unwrap_or(true)
        }

        let cond = bound_checker(match self.range.end_bound() {
            core::ops::Bound::Included(max) => Some(*max),
            core::ops::Bound::Excluded(max) => Some(max.saturating_sub(1)),
            core::ops::Bound::Unbounded => None,
        });

        let min_length = || match self.range.start_bound() {
            core::ops::Bound::Included(max) => max * self.unit.min_length(),
            core::ops::Bound::Excluded(max) => max.saturating_sub(1) * self.unit.min_length(),
            core::ops::Bound::Unbounded => 0,
        };

        crate::debug_regex_beg!("Times", self.range, ctx.offset());
        if remaining_len >= min_length() {
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
                let end = end.or_else(|| iter.next());
                let len = calc_length(beg, end.map(|v| v.0), remaining_len);

                ret = Ok(new_span_inc(ctx, len));
            }
        }
        crate::debug_regex_reval!("Times", cnt, ret)
    }
}
