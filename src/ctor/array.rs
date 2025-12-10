use std::ops::Deref;
use std::ops::DerefMut;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

use super::Ctor;

///
/// Iterate over the array and match the [`regex`](crate::regex::Regex) against the [`Context`].
///
/// Attempts to match each element in the array sequentially until one succeeds.
/// If a match succeeds, returns immediately with that result. If all elements
/// fail to match, the context position is restored to its original state.
///
/// # Regex
///
/// Attempts to match each pattern in the array sequentially. Returns the span
/// of the first successful match. If no patterns match, returns an error and
/// resets the context position.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let array = regex::array(["a", "b", "c"]);
///
///     // First match succeeds, returns immediately
///     CharsCtx::with("abc", |mut ctx| {
///         assert!(ctx.try_mat(&array).is_ok());
///         assert_eq!(ctx.offset(), 1);
///     });
///
///     // No match possible
///     CharsCtx::with("xyz", move |mut ctx| {
///         assert!(ctx.try_mat(&array).is_err());
///         assert_eq!(ctx.offset(), 0);
///     });
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Returns the result of the first regex that matches. Constructed values are
/// produced from the matching element only.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let array = regex::array(["a", "b", "c"]);
///
///     assert_eq!(CharsCtx::new("abc").ctor_span(&array)?, Span::new(0, 1));
///     assert_eq!(CharsCtx::new("bcd").ctor(&array)?, "b");
///     assert_eq!(CharsCtx::new("cde").ctor_span(&array)?, Span::new(0, 1));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Performance
///
/// This combinator attempts patterns in array order. For optimal performance,
/// place more frequently occurring patterns earlier in the array.
#[derive(Debug, Clone, Copy)]
pub struct Array<const N: usize, T>([T; N]);

impl<const N: usize, T> std::ops::Not for Array<N, T> {
    type Output = crate::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
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

impl<'a, const N: usize, C, T, M, O, H> Ctor<'a, C, M, O, H> for Array<N, T>
where
    C: Match<'a>,
    T: Ctor<'a, C, M, O, H>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Array);

        crate::debug_ctor_beg!("Array", g.beg());
        for regex in self.0.iter() {
            let res = regex.construct(g.ctx(), func);

            if res.is_ok() {
                ret = Ok(res?);
                break;
            } else {
                g.reset();
            }
        }
        crate::debug_ctor_reval!("Array", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, const N: usize, C, T> Regex<C> for Array<N, T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Array);

        crate::debug_regex_beg!("Array", g.beg());
        for regex in self.0.iter() {
            let span = g.try_mat(regex);

            match span {
                Ok(span) => {
                    ret = Ok(span);
                    break;
                }
                Err(_) => {
                    g.reset();
                }
            }
        }
        crate::debug_regex_reval!("Array", ret)
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
///     let vec = regex::pair_array([("a", Kind::A), ("b", Kind::B), ("c", Kind::C)]);
///
///     assert_eq!(CharsCtx::new("cab").ctor(&vec)?, ("c", Kind::C));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PairArray<const N: usize, K, V>([(K, V); N]);

impl<const N: usize, K, V> std::ops::Not for PairArray<N, K, V> {
    type Output = crate::regex::RegexNot<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
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

impl<'a, const N: usize, C, K, M, O, V, H> Ctor<'a, C, M, (O, V), H> for PairArray<N, K, V>
where
    V: Clone,
    K: Ctor<'a, C, M, O, H>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairArray);

        crate::debug_ctor_beg!("PairArray", g.beg());
        for (regex, value) in self.0.iter() {
            let res = regex.construct(g.ctx(), func);

            if res.is_ok() {
                ret = Ok((res?, value.clone()));
                break;
            } else {
                g.reset();
            }
        }
        crate::debug_ctor_reval!("PairArray", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, const N: usize, C, K, V> Regex<C> for PairArray<N, K, V>
where
    K: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairArray);

        crate::debug_regex_beg!("PairArray", g.beg());
        for (regex, _) in self.0.iter() {
            let span = g.try_mat(regex);

            match span {
                Ok(span) => {
                    ret = Ok(span);
                    break;
                }
                Err(_) => {
                    g.reset();
                }
            }
        }
        crate::debug_regex_reval!("PairArray", ret)
    }
}
