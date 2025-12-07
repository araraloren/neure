use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
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
use crate::regex::Regex;

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
/// #     color_eyre::install()?;
///     let digit = neu::digit(10).repeat_full();
///     let digit = digit.map(|v: &str| Ok(v.parse::<i64>().unwrap()));
///     let str = regex!([^ '"']+).quote("\"", "\"");
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

def_not!(Quote<C, P, L, R>);

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

impl<'a, C, L, R, P, M, O, H, A> Ctor<'a, C, M, O, H, A> for Quote<C, P, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("Quote", g.beg());

        let _ = debug_ctor_stage!("Quote", "l", g.try_mat(&self.left)?);
        let r = debug_ctor_stage!("Quote", "pat", self.pat.construct(g.ctx(), func));
        let r = g.process_ret(r)?;
        let _ = debug_ctor_stage!("Quote", "r", g.try_mat(&self.right)?);

        debug_ctor_reval!("Quote", g.beg(), g.end(), true);
        Ok(r)
    }
}

impl<'a, C, L, R, P> Regex<C> for Quote<C, P, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("Quote", g.beg());

        let mut ret = debug_regex_stage!("Quote", "l", g.try_mat(&self.left)?);

        ret.add_assign(debug_regex_stage!("Quote", "pat", g.try_mat(&self.pat)?));
        ret.add_assign(debug_regex_stage!("Quote", "r", g.try_mat(&self.right)?));
        debug_regex_reval!("Quote", Ok(ret))
    }
}
