use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Selects the longest matching pattern between two alternatives.
///
/// This combinator attempts both patterns from the same starting position and selects
/// the one that consumes the most input. It's particularly useful for resolving
/// ambiguities in grammars where multiple patterns could match the same input prefix.
///
/// # Regex
///
/// Attempts to match both the `left` and `right` patterns from the current context position.
/// The pattern that produces the longer span (consuming more input) is selected as the result.
/// If both patterns match the same length, the `left` pattern is preferred. If one pattern
/// fails to match but the other succeeds, the successful match is returned regardless of length.
/// Only when both patterns fail to match is an error returned.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let protocol = "http".longest("https");
///
///     assert_eq!(
///         CharsCtx::new(r#"https://docs.rs"#).try_mat(&protocol)?,
///         Span::new(0, 5)
///     );
///     assert_eq!(
///         CharsCtx::new(r#"http://docs.rs"#).try_mat(&protocol)?,
///         Span::new(0, 4)
///     );
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Similar to the `Regex` behavior but for construction. Attempts to construct values using
/// both patterns and selects the result from the pattern that consumed more input. The context
/// position is advanced to match the selected pattern's end position. If both patterns consume
/// the same amount of input, the `left` pattern's result is preferred. If one pattern succeeds
/// and the other fails, the successful result is returned. Only when both patterns fail is an
/// error returned.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let dec = regex!((neu::digit(10))+).try_map(map::from_str_radix::<i32>(10));
///     let hex = regex!((neu::digit(16))+).try_map(map::from_str_radix(16));
///     let num = dec.longest(hex);
///     let val = num.sep(",".skip_ws()).enclose("{", "}");
///     let mut ctx = CharsCtx::new(r#"{12, 1E, A8, 88, 2F}"#);
///
///     assert_eq!(ctx.ctor(&val)?, [12, 0x1e, 0xa8, 88, 0x2f]);
///
/// #   Ok(())
/// # }
/// ```
///
/// # Behavior Notes
///
/// - Both patterns are attempted regardless of success or failure of the first
/// - The context position is reset between attempts to ensure fair comparison
/// - Only the selected pattern's side effects (if any) will be visible after matching
/// - If one pattern succeeds and the other fails, the successful result is always selected
///   (even if it consumes less input than a hypothetical successful match of the other pattern)
/// - Only when both patterns fail is an error returned
///
/// # Performance
///
/// Both patterns are always fully attempted, so the performance cost is the sum of both
/// pattern evaluations. For optimal performance, place the more frequently occurring pattern
/// or the faster-to-evaluate pattern first (as the `left` parameter).
#[derive(Default, Copy)]
pub struct LongestTokenMatch<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(LongestTokenMatch<C, L, R>);

impl<C, L, R> Debug for LongestTokenMatch<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LongestTokenMatch")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Clone for LongestTokenMatch<C, L, R>
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

impl<C, L, R> LongestTokenMatch<C, L, R> {
    pub fn new(pat1: L, pat2: R) -> Self {
        Self {
            left: pat1,
            right: pat2,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
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

impl<'a, C, L, R, M, O, H> Ctor<'a, C, M, O, H> for LongestTokenMatch<C, L, R>
where
    L: Ctor<'a, C, M, O, H>,
    R: Ctor<'a, C, M, O, H>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("LongestTokenMatch", g.beg());

        let r_l = debug_ctor_stage!("LongestTokenMatch", "l", self.left.construct(g.ctx(), func));
        let offset_l = g.end();
        let r_r = debug_ctor_stage!(
            "LongestTokenMatch",
            "r",
            self.right.construct(g.reset().ctx(), func)
        );
        let offset_r = g.end();
        let (offset, ret) = if offset_l >= offset_r {
            (offset_l, r_l)
        } else {
            (offset_r, r_r)
        };

        g.ctx().set_offset(offset);
        debug_ctor_reval!("LongestTokenMatch", g.beg(), g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, L, R> Regex<C> for LongestTokenMatch<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("LongestTokenMatch", g.beg());

        let r_l = debug_regex_stage!("LongestTokenMatch", "l", g.try_mat(&self.left));
        let offset_l = g.end();
        let r_r = debug_regex_stage!("LongestTokenMatch", "r", g.reset().try_mat(&self.right));
        let offset_r = g.end();
        let (offset, ret) = if offset_l >= offset_r {
            (offset_l, r_l)
        } else {
            (offset_r, r_r)
        };

        g.ctx().set_offset(offset);
        debug_regex_reval!("LongestTokenMatch", g.process_ret(ret))
    }
}
