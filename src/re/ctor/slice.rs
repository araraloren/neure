use std::ops::Deref;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::Ctor;

///
/// Iterate over the array and match the regex against the [`Context`].
///
/// # Ctor
///
/// Return the result of first regex that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #   color_eyre::install()?;
///     let array = ["a", "b", "c"];
///     let tuple = re::slice(&array);
///
///     assert_eq!(CharsCtx::new("abc").ctor_span(&tuple)?, Span::new(0, 1));
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Slice<'a, const N: usize, T>(&'a [T; N]);

impl<const N: usize, T> std::ops::Not for Slice<'_, N, T> {
    type Output = crate::re::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
    }
}

impl<'a, const N: usize, T> Slice<'a, N, T> {
    pub fn new(val: &'a [T; N]) -> Self {
        Self(val)
    }
}

impl<const N: usize, T> Deref for Slice<'_, N, T> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, const N: usize, C, T, M, O, H, A> Ctor<'a, C, M, O, H, A> for Slice<'_, N, T>
where
    T: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Slice);

        debug_ctor_beg!("Slice", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }
        debug_ctor_reval!("Slice", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, const N: usize, C, T> Regex<C> for Slice<'_, N, T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Slice);

        debug_regex_beg!("Slice", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }
        debug_regex_reval!("Slice", ret)
    }
}

///
/// Iterate over the slice and match the regex against the [`Context`].
///
/// # Ctor
///
/// Return a pair of result and the value of first pair that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum Kind {
///         A,
///         B,
///         C,
///     }
///     let pairs = [("a", Kind::A), ("b", Kind::B), ("c", Kind::C)];
///     let vec = re::pair_slice(&pairs);
///
///     assert_eq!(CharsCtx::new("cab").ctor(&vec)?, ("c", Kind::C));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PairSlice<'a, const N: usize, K, V>(&'a [(K, V); N]);

impl<const N: usize, K, V> std::ops::Not for PairSlice<'_, N, K, V> {
    type Output = crate::re::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
    }
}

impl<'a, const N: usize, K, V> PairSlice<'a, N, K, V> {
    pub fn new(val: &'a [(K, V); N]) -> Self {
        Self(val)
    }
}

impl<const N: usize, K, V> Deref for PairSlice<'_, N, K, V> {
    type Target = [(K, V); N];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, const N: usize, C, K, M, O, V, H, A> Ctor<'a, C, M, (O, V), H, A>
    for PairSlice<'_, N, K, V>
where
    V: Clone,
    K: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairSlice);

        debug_ctor_beg!("PairSlice", g.beg());
        for (regex, value) in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok((res, value.clone()));
                break;
            } else {
                g.reset();
            }
        }
        debug_ctor_reval!("PairSlice", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, const N: usize, C, K, V> Regex<C> for PairSlice<'_, N, K, V>
where
    K: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairSlice);

        debug_regex_beg!("PairSlice", g.beg());
        for (regex, _) in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
            } else {
                g.reset();
            }
        }
        debug_regex_reval!("PairSlice", ret)
    }
}
