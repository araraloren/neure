use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::trace_log;

///
/// Match `L` and `R`, return the longest match result.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return the result of the longest match of either `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let dec = neu::digit(10).repeat_one_more();
///     let hex = neu::digit(16).repeat_one_more();
///     let dec = dec.map(re::map::from_str_radix::<i32>(10));
///     let hex = hex.map(re::map::from_str_radix(16));
///     let num = dec.ltm(hex);
///     let val = num.sep(",".ws()).quote("{", "}");
///     let mut ctx = CharsCtx::new(r#"{12, E1, A8, 88}"#);
///
///     assert_eq!(ctx.ctor(&val)?, [12, 0xe1, 0xa8, 88]);
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct LongestTokenMatch<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
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

impl<'a, C, L, R, M, O> Ctor<'a, C, M, O> for LongestTokenMatch<C, L, R>
where
    L: Ctor<'a, C, M, O>,
    R: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let r_l = trace!("ltm", beg @ "left", self.left.constrct(g.ctx(), func));
        let offset_l = g.end();
        let r_r = trace!("ltm", beg @ "right", self.right.constrct(g.reset().ctx(), func));
        let offset_r = g.end();
        let (offset, ret) = if offset_l >= offset_r {
            (offset_l, r_l)
        } else {
            (offset_r, r_r)
        };

        trace_log!(
            "r`ltm`@{} -> {{l: offset = {}, r: offset = {}}}",
            beg,
            offset_l,
            offset_r
        );
        g.ctx().set_offset(offset);
        trace!("ltm", beg -> g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, L, R> Regex<C> for LongestTokenMatch<C, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = L::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let r_l = trace!("ltm", beg @ "left", g.try_mat(&self.left));
        let offset_l = g.end();
        let r_r = trace!("ltm", beg @ "right", g.reset().try_mat(&self.right));
        let offset_r = g.end();
        let (off, ret) = if offset_l >= offset_r {
            (offset_l, r_l)
        } else {
            (offset_r, r_r)
        };

        trace_log!(
            "r`ltm`@{} -> {{l: offset = {}, ret = {:?}; r: offset = {}, ret = {:?}}}",
            beg,
            offset_l,
            r_l,
            offset_r,
            r_r
        );
        g.ctx().set_offset(off);
        trace!("ltm", beg => g.end(), g.process_ret(ret))
    }
}
