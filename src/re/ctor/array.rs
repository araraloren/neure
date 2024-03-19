use std::ops::Deref;
use std::ops::DerefMut;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
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
///     let tuple = re::array(array);
///
///     assert_eq!(CharsCtx::new("abc").ctor_span(&tuple)?, Span::new(0, 1));
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Array<const N: usize, T>([T; N]);

impl<const N: usize, T> std::ops::Not for Array<N, T> {
    type Output = crate::re::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
    }
}

impl<const N: usize, T> Array<N, T> {
    pub fn new(val: [T; N]) -> Self {
        Self(val)
    }
}

impl<const N: usize, T> Deref for Array<N, T> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, T> DerefMut for Array<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, const N: usize, C, T, M, O, H, A> Ctor<'a, C, M, O, H, A> for Array<N, T>
where
    T: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("array", beg, regex.construct(g.ctx(), func));

            if ret.is_ok() {
                trace!("array", beg -> g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

impl<'a, const N: usize, C, T> Regex<C> for Array<N, T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = T::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("array", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("array", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

///
/// Iterate over the array and match the regex against the [`Context`].
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
///     let vec = re::pair_array([("a", Kind::A), ("b", Kind::B), ("c", Kind::C)]);
///
///     assert_eq!(CharsCtx::new("cab").ctor(&vec)?, ("c", Kind::C));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PairArray<const N: usize, K, V>([(K, V); N]);

impl<const N: usize, K, V> std::ops::Not for PairArray<N, K, V> {
    type Output = crate::re::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::re::not(self)
    }
}

impl<const N: usize, K, V> PairArray<N, K, V> {
    pub fn new(val: [(K, V); N]) -> Self {
        Self(val)
    }
}

impl<const N: usize, K, V> Deref for PairArray<N, K, V> {
    type Target = [(K, V); N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, K, V> DerefMut for PairArray<N, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, const N: usize, C, K, M, O, V, H, A> Ctor<'a, C, M, (O, V), H, A> for PairArray<N, K, V>
where
    V: Clone,
    K: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, value) in self.0.iter() {
            let ret = trace!("pair_array", beg, regex.construct(g.ctx(), func));

            if ret.is_ok() {
                trace!("pair_array", beg -> g.end(), true);
                return Ok((ret?, value.clone()));
            } else {
                g.reset();
            }
        }
        Err(Error::PairVec)
    }
}

impl<'a, const N: usize, C, K, V> Regex<C> for PairArray<N, K, V>
where
    K: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = K::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, _) in self.0.iter() {
            let ret = trace!("pair_array", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("pair_array", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::PairVec)
    }
}
