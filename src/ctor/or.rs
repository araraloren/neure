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
/// First try to match `L`, if it fails, then try to match `R`.
///
/// # Ctor
///
/// Return the result of either `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, map::from_str_radix};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     macro_rules! num {
///         ($s:literal) => {
///             neu::digit($s).repeat_one_more().map(from_str_radix($s))
///         };
///     }
///
///     let (bin, oct, dec, hex) = (num!(2), num!(8), num!(10), num!(16));
///     let num = dec.ltm(hex);
///     let dec = "0d".then(dec)._1();
///     let oct = "0o".then(oct)._1();
///     let hex = "0x".then(hex)._1();
///     let bin = "0b".then(bin)._1();
///     let pos = "+".map(|_| Ok(1));
///     let neg = "-".map(|_| Ok(-1));
///     let sign = pos.or(neg.or(regex::null().map(|_| Ok(1))));
///     let num = bin.or(oct.or(dec.or(hex))).or(num);
///     let num = sign.then(num).map(|(s, v): (_, i64)| Ok(s * v));
///     let val = num.sep(",".ws()).quote("[", "]");
///     let mut ctx = CharsCtx::new(r#"[0d18, 0o17, 0x18, 0b1010, 18, 1E]"#);
///
///     assert_eq!(ctx.ctor(&val)?, [18, 15, 24, 10, 18, 30]);
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Or<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

def_not!(Or<C, L, R>);

impl<C, L, R> Debug for Or<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Or")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Clone for Or<C, L, R>
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

impl<C, L, R> Or<C, L, R> {
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

impl<'a, C, L, R, M, O, H, A> Ctor<'a, C, M, O, H, A> for Or<C, L, R>
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

        debug_ctor_beg!("Or", g.beg());

        let ret = debug_ctor_stage!("Or", "l", self.left.construct(g.ctx(), func))
            .or_else(|_| debug_ctor_stage!("Or", "r", self.right.construct(g.reset().ctx(), func)));

        debug_ctor_reval!("Or", g.beg(), g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, L, R> Regex<C> for Or<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("Or", g.beg());

        let ret = debug_regex_stage!("Or", "l", g.try_mat(&self.left))
            .or_else(|_| debug_regex_stage!("Or", "r", g.reset().try_mat(&self.right)));

        debug_regex_reval!("Or", ret)
    }
}
