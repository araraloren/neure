use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::re::ctor::Map;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// First try to match `P`. If it succeeds, then try to match `T`.
///
/// # Ctor
///
/// It will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let str = neu::ascii_alphabetic().repeat_one_more();
///     let str = str.quote("\"", "\"").map(Ok);
///     let int = neu::digit(10).repeat_one_more();
///     let int = int.map(map::from_str_radix::<i32>(10));
///     let tuple = str.ws().then(",".ws())._0().then(int.ws());
///     let tuple = tuple.quote("(", ")");
///     let mut ctx = CharsCtx::new(r#"("Galaxy", 42)"#);
///
///     assert_eq!(ctx.ctor(&tuple)?, ("Galaxy", 42));
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Then<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

def_not!(Then<C, L, R>);

impl<C, L, R> Debug for Then<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Then")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Clone for Then<C, L, R>
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

impl<C, L, R> Then<C, L, R> {
    pub fn new(pat: L, then: R) -> Self {
        Self {
            left: pat,
            right: then,
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

    pub fn set_left(&mut self, pat: L) -> &mut Self {
        self.left = pat;
        self
    }

    pub fn set_right(&mut self, then: R) -> &mut Self {
        self.right = then;
        self
    }

    pub fn _0<O>(self) -> Map<C, Self, Select0, O> {
        Map::new(self, Select0)
    }

    pub fn _1<O>(self) -> Map<C, Self, Select1, O> {
        Map::new(self, Select1)
    }

    pub fn _eq<I1, I2>(self) -> Map<C, Self, SelectEq, (I1, I2)> {
        Map::new(self, SelectEq)
    }
}

impl<'a, C, L, R, M, O1, O2, H, A> Ctor<'a, C, M, (O1, O2), H, A> for Then<C, L, R>
where
    L: Ctor<'a, C, M, O1, H, A>,
    R: Ctor<'a, C, M, O2, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("then", beg @ "left", self.left.construct(g.ctx(), func).map(|ret1| {
            trace!("then", beg @ "right", self.right.construct(g.ctx(), func).map(|ret2| (ret1, ret2)))   
        }) );
        let ret = g.process_ret(ret)?;

        trace!("then", beg => g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, L, R> Regex<C> for Then<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("then", beg @ "left", g.try_mat(&self.left)?);

        ret.add_assign(trace!("then", beg @ "right", g.try_mat(&self.right)?));
        trace!("then", beg => g.end(), Ok(ret))
    }
}

///
/// First try to match `P`. If it succeeds, then try to match `I`.
/// If it succeeds, then try to match `T`.
///
/// # Ctor
///
/// If `I` match succeeds, return a tuple of result of `L` and Some(`T`)(result of `R`).
/// Otherwise return a tulpe of result of `L` and None.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let val = neu::ascii_alphabetic().repeat_one_more().ws();
///     let tuple = val.if_then(",".ws(), val).quote("(", ")");
///
///     assert_eq!(CharsCtx::new("(abc)").ctor(&tuple)?, ("abc", None));
///     assert_eq!(
///         CharsCtx::new("(abc, cde)").ctor(&tuple)?,
///         ("abc", Some("cde"))
///     );
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct IfThen<C, L, I, R> {
    r#if: I,
    left: L,
    right: R,
    marker: PhantomData<C>,
}

def_not!(IfThen<C, L, I, R>);

impl<C, L, I, R> Debug for IfThen<C, L, I, R>
where
    L: Debug,
    R: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfThen")
            .field("r#if", &self.r#if)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, I, R> Clone for IfThen<C, L, I, R>
where
    L: Clone,
    R: Clone,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            r#if: self.r#if.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, I, R> IfThen<C, L, I, R> {
    pub fn new(left: L, r#if: I, right: R) -> Self {
        Self {
            r#if,
            left,
            right,
            marker: PhantomData,
        }
    }

    pub fn r#if(&self) -> &I {
        &self.r#if
    }

    pub fn r#if_mut(&mut self) -> &mut I {
        &mut self.r#if
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

    pub fn set_if(&mut self, r#if: I) -> &mut Self {
        self.r#if = r#if;
        self
    }

    pub fn set_left(&mut self, pat: L) -> &mut Self {
        self.left = pat;
        self
    }

    pub fn set_right(&mut self, then: R) -> &mut Self {
        self.right = then;
        self
    }

    pub fn _0<O>(self) -> Map<C, Self, Select0, O> {
        Map::new(self, Select0)
    }

    pub fn _1<O>(self) -> Map<C, Self, Select1, O> {
        Map::new(self, Select1)
    }

    pub fn _eq<I1, I2>(self) -> Map<C, Self, SelectEq, (I1, I2)> {
        Map::new(self, SelectEq)
    }
}

impl<'a, C, L, I, R, M, O1, O2, H, A> Ctor<'a, C, M, (O1, Option<O2>), H, A> for IfThen<C, L, I, R>
where
    L: Ctor<'a, C, M, O1, H, A>,
    R: Ctor<'a, C, M, O2, H, A>,
    I: Regex<C>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, Option<O2>), Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let r_l = trace!("if_then", beg @ "left", self.left.construct(g.ctx(), func));
        let r_l = g.process_ret(r_l)?;
        let r_i = trace!("if_then", beg @ "if", g.try_mat(&self.r#if));
        let ret = if r_i.is_ok() {
            let r_r = trace!("if_then", beg @ "right", self.right.construct(g.ctx(), func));
            let r_r = g.process_ret(r_r)?;

            // if matched, return (01, Some(O2))
            (r_l, Some(r_r))
        } else {
            // not matched, return None
            (r_l, None)
        };

        trace!("if_then", beg => g.end(), true);
        Ok(ret)
    }
}

impl<'a, C, L, I, R> Regex<C> for IfThen<C, L, I, R>
where
    I: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("if_then", beg @ "left", g.try_mat(&self.left)?);

        if trace!("if_then", beg @ "if", g.try_mat(&self.r#if)).is_ok() {
            ret.add_assign(trace!("if_then", beg @ "right", g.try_mat(&self.right)?));
        }
        trace!("if_then", beg => g.end(), Ok(ret))
    }
}
