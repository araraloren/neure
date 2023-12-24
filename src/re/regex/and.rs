use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::trace;
use crate::re::Regex;

///
/// Match `P` and then match `T`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let str = neu::ascii_alphabetic().repeat_one_more();
///     let str = str.quote("\"", "\"").map(Ok);
///     let int = neu::digit(10).repeat_one_more();
///     let int = int.map(re::map::from_str_radix::<i32>(10));
///     let tuple = str.ws().then(",".ws())._0().then(int.ws());
///     let tuple = tuple.quote("(", ")");
///     let mut ctx = CharsCtx::new(r#"("Galaxy", 42)"#);
///
///     assert_eq!(ctx.ctor(&tuple)?, ("Galaxy", 42));
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct RegexAnd<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Clone for RegexAnd<C, L, R>
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

impl<C, P, T> RegexAnd<C, P, T> {
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

impl<'a, C, L, R> Regex<C> for RegexAnd<C, L, R>
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
        let r_p = trace!("and", beg @ "left", g.try_mat(&self.left)?);
        let r_t = trace!("and", beg @ "right", g.try_mat(&self.right)?);

        trace!("and", beg => g.end(), true);
        Ok((r_p, r_t))
    }
}
