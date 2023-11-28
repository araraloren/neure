use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::re::trace;
use crate::re::Regex;

///
/// Match regex `P` quoted by `L` and `R`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return result of `P` if matched.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let digit = neu::digit(10).repeat_full();
///     let digit = digit.map(|v: &str| Ok(v.parse::<i64>().unwrap()));
///     let str = re!([^ '"']+).quote("\"", "\"");
///     let tuple = digit.then(",")._0().then(str);
///     let tuple = tuple.quote("(", ")");
///
///     let mut ctx = CharsCtx::new("(42,\"rust\")");
///
///     assert_eq!(ctx.ctor(&tuple)?, (42, "rust"));
///
///     Ok(())
/// # }
/// ```
///
#[derive(Debug, Default, Copy)]
pub struct RegexQuote<C, P, L, R> {
    pat: P,
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, P, L, R> Clone for RegexQuote<C, P, L, R>
where
    P: Clone,
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, L, R> RegexQuote<C, P, L, R> {
    pub fn new(pat: P, left: L, right: R) -> Self {
        Self {
            pat,
            left,
            right,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
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

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
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

impl<'a, C, L, R, P> Regex<C> for RegexQuote<C, P, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let _ = trace!("quote", beg @ "left", g.try_mat(&self.left)?);
        let r = trace!("quote", beg @ "pat", g.try_mat(&self.pat)?);
        let _ = trace!("quote", beg @ "right", g.try_mat(&self.right)?);

        trace!("quote", beg => g.end(), true);
        Ok(r)
    }
}
