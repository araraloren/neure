use core::ops::Deref;
use core::ops::DerefMut;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;
use crate::span::Span;

use super::Ctor;

///
/// Iterate over the array and match the [`regex`](crate::regex::Regex) against the [`Context`](crate::ctx::Context).
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

impl<const N: usize, T> core::ops::Not for Array<N, T> {
    type Output = crate::regex::Assert<Self>;

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

impl<'a, const N: usize, C, T, O, H> Ctor<'a, C, O, H> for Array<N, T>
where
    C: Match<'a>,
    T: Ctor<'a, C, O, H>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let offset = ctx.offset();
        let mut ret = Err(Error::Array);

        crate::debug_ctor_beg!("Array", offset);
        for regex in self.0.iter() {
            if let Ok(val) = regex.construct(ctx, func).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(val);
                break;
            }
        }
        crate::debug_ctor_reval!("Array", offset, ctx.offset(), ret.is_ok());
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
        let offset = ctx.offset();
        let mut ret = Err(Error::Array);

        crate::debug_regex_beg!("Array", offset);
        for regex in self.0.iter() {
            if let Ok(span) = ctx.try_mat(regex).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(span);
                break;
            }
        }
        crate::debug_regex_reval!("Array", ret)
    }
}

///
/// Iterate over the array and match the regex against the [`Context`](crate::ctx::Context).
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

impl<const N: usize, K, V> core::ops::Not for PairArray<N, K, V> {
    type Output = crate::regex::Assert<Self>;

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

impl<'a, const N: usize, C, K, O, V, H> Ctor<'a, C, (O, V), H> for PairArray<N, K, V>
where
    V: Clone,
    K: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let offset = ctx.offset();
        let mut ret = Err(Error::PairArray);

        crate::debug_ctor_beg!("PairArray", offset);
        for (regex, value) in self.0.iter() {
            if let Ok(out) = regex.construct(ctx, func).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok((out, value.clone()));
                break;
            }
        }
        crate::debug_ctor_reval!("PairArray", offset, ctx.offset(), ret.is_ok());
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
        let offset = ctx.offset();
        let mut ret = Err(Error::PairArray);

        crate::debug_regex_beg!("PairArray", offset);
        for (regex, _) in self.0.iter() {
            if let Ok(span) = ctx.try_mat(regex).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(span);
                break;
            }
        }
        crate::debug_regex_reval!("PairArray", ret)
    }
}
