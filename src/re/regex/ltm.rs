use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::trace;
use crate::re::Regex;
use crate::trace_log;

/// Match `L` and `R`, return the longest match result.
///
/// # Regex
///
/// It will return the result of the longest match of either `L` or `R`.
#[derive(Default, Copy)]
pub struct RegexLongestTokenMatch<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Debug for RegexLongestTokenMatch<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegexLongestTokenMatch")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Clone for RegexLongestTokenMatch<C, L, R>
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

impl<C, L, R> RegexLongestTokenMatch<C, L, R> {
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

impl<'a, C, L, R, O> Regex<C> for RegexLongestTokenMatch<C, L, R>
where
    L: Regex<C, Ret = O>,
    R: Regex<C, Ret = O>,
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
            "r`ltm`@{} -> {{l: offset = {}; r: offset = {}}}",
            beg,
            offset_l,
            offset_r,
        );
        g.ctx().set_offset(off);
        trace!("ltm", beg => g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}
