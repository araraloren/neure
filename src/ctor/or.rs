use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Provides alternation between two patterns, trying the second only if the first fails.
///
/// This combinator implements logical "or" behavior for patterns. It first attempts the `left`
/// pattern, and only if that fails (returns an error), it attempts the `right` pattern from the
/// original starting position. This is equivalent to the `|` operator in regular expressions.
///
/// # Regex
///
/// Attempts to match the `left` pattern first. If successful, returns its span immediately.
/// If the `left` pattern fails to match, resets the context position and attempts the `right`
/// pattern. Returns the span of whichever pattern succeeds first. If both patterns fail to
/// match, returns an error containing the error from the `right` pattern.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let digits = neu::digit(16).many1();
///     let strs = ('a'..='z').or('A'..='Z');
///     let strs = strs.many1();
///     let parser = digits.or(strs);
///
///     assert_eq!(CharsCtx::new(r#"8848"#).try_mat(&parser)?, Span::new(0, 4));
///     assert_eq!(
///         CharsCtx::new(r#"hello world"#).try_mat(&parser)?,
///         Span::new(0, 5)
///     );
///     Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Attempts to construct a value using the `left` pattern first. If successful, returns its
/// value immediately. If the `left` pattern fails, resets the context position and attempts the
/// `right` pattern. Returns the value from whichever pattern succeeds first. If both patterns
/// fail to construct a value, returns an error containing the error from the `right` pattern.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     macro_rules! digit {
///         ($p:expr, $r:literal) => {
///             $p.then(
///                 neu::digit($r)
///                     .many1()
///                     .try_map(map::from_str_radix::<i64>($r)),
///             )
///             ._1()
///         };
///     }
///
///     let dec = digit!("0d", 10);
///     let oct = digit!("0o",  8);
///     let hex = digit!("0x", 16);
///     let bin = digit!("0b",  2);
///     let pos = "+".map(|_| 1);
///     let neg = "-".map(|_| -1);
///     let sign = pos.or(neg.or(regex::empty().map(|_| 1)));
///     let num = bin.or(oct.or(dec.or(hex)));
///     let num = sign.then(num).map(|(s, v)| s * v);
///     let parser = num.sep(",".skip_ws());
///
///     assert_eq!(
///         CharsCtx::new(r#"0d18, +0o17, -0x18, -0b1010"#).ctor(&parser)?,
///         [18, 15, -24, -10]
///     );
/// #   Ok(())
/// # }
/// ```
///
/// # Behavior Notes
///
/// - Patterns are evaluated in order: `left` first, then `right` only if `left` fails
/// - The context position is reset to the starting position before attempting the `right` pattern
/// - Only the successful pattern's side effects (if any) will be visible after matching/construction
/// - If both patterns fail, the error from the `right` pattern is returned (or the `left` pattern's
///   error if the `right` pattern wasn't attempted due to context constraints)
/// - This combinator does not attempt to find the "longest match" - it returns the first successful
///   match in declaration order
///
/// # Performance
///
/// The evaluation is short-circuited: if the `left` pattern succeeds, the `right` pattern is never
/// evaluated. For optimal performance:
/// - Place more frequently occurring patterns first
/// - Place cheaper-to-evaluate patterns first
/// - Avoid expensive patterns on the right side when possible
#[derive(Copy)]
pub struct Or<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Or<C, L, R>);

impl<C, L, R> Debug for Or<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Or")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Default for Or<C, L, R>
where
    L: Default,
    R: Default,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, L, R> Clone for Or<C, L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R> Or<C, L, R> {
    pub const fn new(pat1: L, pat2: R) -> Self {
        Self {
            left: pat1,
            right: pat2,
            marker: PhantomData,
        }
    }

    pub const fn left(&self) -> &L {
        &self.left
    }

    pub const fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub const fn right(&self) -> &R {
        &self.right
    }

    pub const fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<'a, C, L, R, O, H> Ctor<'a, C, O, H> for Or<C, L, R>
where
    L: Ctor<'a, C, O, H>,
    R: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let offset = ctx.offset();

        debug_ctor_beg!("Or", offset);
        let ret = debug_ctor_stage!("Or", "l", self.left.construct(ctx, func))
            .or_else(|_| {
                ctx.set_offset(offset);
                debug_ctor_stage!("Or", "r", self.right.construct(ctx, func))
            })
            .inspect_err(|_| {
                ctx.set_offset(offset);
            });

        debug_ctor_reval!("Or", offset, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, L, R> Regex<C> for Or<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let offset = ctx.offset();

        debug_regex_beg!("Or", offset);
        let ret = debug_regex_stage!("Or", "l", ctx.try_mat(&self.left))
            .or_else(|_| {
                ctx.set_offset(offset);
                debug_regex_stage!("Or", "r", ctx.try_mat(&self.right))
            })
            .inspect_err(|_| {
                ctx.set_offset(offset);
            });

        debug_regex_reval!("Or", ret)
    }
}
