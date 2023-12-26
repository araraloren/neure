use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::trace;
use crate::re::Regex;

/// First try to match `L`, if it fails, then try to match `R`.
///
/// # Regex
///
/// Return the result of either `L` or `R`.
#[derive(Debug, Default, Copy)]
pub struct RegexOr<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Clone for RegexOr<C, L, R>
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

impl<C, L, R> RegexOr<C, L, R> {
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

impl<'a, C, L, R, O> Regex<C> for RegexOr<C, L, R>
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
        let ret = trace!("or", beg @ "left", g.try_mat(&self.left).or_else(|_| {
            trace!("or", beg @ "right", g.try_mat(&self.right))
        }));

        trace!("or", beg => g.end(), ret.is_ok());
        ret
    }
}
