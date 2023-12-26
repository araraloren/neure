use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::trace;
use crate::re::Regex;

/// First try to match `L`. If it succeeds, then try to match `R`.
///
/// # Regex
///
/// Return a tuple of result of `L` and result of `R`.
#[derive(Debug, Default, Copy)]
pub struct RegexThen<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Clone for RegexThen<C, L, R>
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

impl<C, P, T> RegexThen<C, P, T> {
    pub fn new(pat: P, then: T) -> Self {
        Self {
            left: pat,
            right: then,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &P {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut P {
        &mut self.left
    }

    pub fn right(&self) -> &T {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut T {
        &mut self.right
    }

    pub fn set_left(&mut self, left: P) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: T) -> &mut Self {
        self.right = right;
        self
    }
}

impl<'a, C, L, R> Regex<C> for RegexThen<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = (L::Ret, R::Ret);

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let r_p = trace!("then", beg @ "left", g.try_mat(&self.left)?);
        let r_t = trace!("then", beg @ "right", g.try_mat(&self.right)?);

        trace!("then", beg => g.end(), true);
        Ok((r_p, r_t))
    }
}
