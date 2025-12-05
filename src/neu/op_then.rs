use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

use super::length_of;
use super::Condition;
use super::Neu;
use super::NeuCond;

///
/// Construct a regex that matches unit `L` and then unit `R`.
///
/// # Ctor
///
/// Return [`Orig`](crate::ctx::Context::Orig) with the [`Span`] as the index if the match is found.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let re = b'+'
///         .or(b'-')
///         .then(u8::is_ascii_hexdigit)
///         .then(u8::is_ascii_hexdigit.repeat_times::<3>())
///         .pat();
///
///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, b"+AE00");
///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, b"-83FD");
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct NeureThen<C, L, R, T, I>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    cond: I,
    marker: PhantomData<(C, T)>,
}

impl<C, L, R, T, I> std::ops::Not for NeureThen<C, L, R, T, I>
where
    L: Neu<T>,
    R: Neu<T>,
{
    type Output = crate::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, L, R, T, I> Debug for NeureThen<C, L, R, T, I>
where
    I: Debug,
    L: Neu<T> + Debug,
    R: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NeureThen")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("cond", &self.cond)
            .finish()
    }
}

impl<C, L, R, T, I> Clone for NeureThen<C, L, R, T, I>
where
    I: Clone,
    L: Neu<T> + Clone,
    R: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            cond: self.cond.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R, T, I> NeureThen<C, L, R, T, I>
where
    L: Neu<T>,
    R: Neu<T>,
{
    pub fn new(left: L, right: R, r#if: I) -> Self {
        Self {
            left,
            cond: r#if,
            right,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn set_left(&mut self, unit: L) -> &mut Self {
        self.left = unit;
        self
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_right(&mut self, unit: R) -> &mut Self {
        self.right = unit;
        self
    }
}

impl<'a, C, L, R, I> Condition<'a, C> for NeureThen<C, L, R, C::Item, I>
where
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    C: Context<'a> + 'a,
{
    type Out<F> = NeureThen<C, L, R, C::Item, F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>,
    {
        NeureThen::new(self.left, self.right, r#if)
    }
}

impl<'a, L, R, C, O, I, H, A> Ctor<'a, C, O, O, H, A> for NeureThen<C, L, R, C::Item, I>
where
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    I: NeuCond<'a, C>,
    C: Context<'a> + Match<C> + 'a,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, L, R, C, I> Regex<C> for NeureThen<C, L, R, C::Item, I>
where
    C: Context<'a> + 'a,
    L: Neu<C::Item>,
    R: Neu<C::Item>,
    I: NeuCond<'a, C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::NeureThen);

        crate::debug_regex_beg!("NeureThen", ctx.beg());
        loop {
            let mut iter = ctx.peek()?;

            if let (Some(fst), Some(snd)) = (iter.next(), iter.next()) {
                if self.left.is_match(&fst.1)
                    && self.cond.check(ctx.ctx(), &(fst.0, fst.1))?
                    && self.right.is_match(&snd.1)
                    && self.cond.check(ctx.ctx(), &(snd.0, snd.1))?
                {
                    let len = length_of(fst.0, ctx.ctx(), iter.next().map(|v| v.0));

                    ret = Ok(ctx.inc(len));
                }
            } else if ctx.req_data()? {
                continue;
            }
            break;
        }
        crate::debug_regex_reval!("NeureThen", ctx.process_ret(ret))
    }
}
