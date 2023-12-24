use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// First try to match `L`. If it is succeeds, then try to match `P`.
/// If it is succeeds, then try to match `R`.
///
/// # Ctor
///
/// It will return the result of `P`, ignoring the result of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
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
#[derive(Default, Copy)]
pub struct Quote<C, P, L, R> {
    pat: P,
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, P, L, R> Debug for Quote<C, P, L, R>
where
    P: Debug,
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Quote")
            .field("pat", &self.pat)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, P, L, R> Clone for Quote<C, P, L, R>
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

impl<C, P, L, R> Quote<C, P, L, R> {
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

impl<'a, C, L, R, P, M, O> Ctor<'a, C, M, O> for Quote<C, P, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    P: Ctor<'a, C, M, O>,
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
        let _ = trace!("quote", beg @ "left", g.try_mat(&self.left)?);
        let r = trace!("quote", beg @ "pat", self.pat.constrct(g.ctx(), func));
        let r = g.process_ret(r)?;
        let _ = trace!("quote", beg @ "right", g.try_mat(&self.right)?);

        trace!("quote", beg -> g.end(), true);
        Ok(r)
    }
}

impl<'a, C, L, R, P> Regex<C> for Quote<C, P, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("quote", beg @ "left", g.try_mat(&self.left)?);

        ret.add_assign(trace!("quote", beg @ "pat", g.try_mat(&self.pat)?));
        ret.add_assign(trace!("quote", beg @ "right", g.try_mat(&self.right)?));
        trace!("quote", beg => g.end(), Ok(ret))
    }
}
