use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
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
use crate::regex::def_not;
use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::regex::Regex;

///
/// Match `L` and `R`, return the longest match result.
///
/// # Ctor
///
/// It will return the result of the longest match of either `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let dec = neu::digit(10).repeat_one_more();
///     let hex = neu::digit(16).repeat_one_more();
///     let dec = dec.map(map::from_str_radix::<i32>(10));
///     let hex = hex.map(map::from_str_radix(16));
///     let num = dec.ltm(hex);
///     let val = num.sep(",".ws()).quote("{", "}");
///     let mut ctx = CharsCtx::new(r#"{12, E1, A8, 88, 2F}"#);
///
///     assert_eq!(ctx.ctor(&val)?, [12, 0xe1, 0xa8, 88, 0x2f]);
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct LongestTokenMatch<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

def_not!(LongestTokenMatch<C, L, R>);

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

impl<'a, C, L, R, M, O, H, A> Ctor<'a, C, M, O, H, A> for LongestTokenMatch<C, L, R>
where
    L: Ctor<'a, C, M, O, H, A>,
    R: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
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
    C: Context<'a> + Match<'a>,
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
