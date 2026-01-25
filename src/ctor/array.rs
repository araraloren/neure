use core::ops::Deref;
use core::ops::DerefMut;

use crate::ctor;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex;
use crate::regex::Regex;
use crate::span::Span;

use super::Ctor;
use super::r2r;
use super::r2r_kv;
use super::sel;
use super::sel_kv;

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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
pub struct Array<const N: usize, T> {
    inner: [T; N],
    longest: bool,
}

impl<const N: usize, T> core::ops::Not for Array<N, T> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<const N: usize, T> Array<N, T> {
    pub const fn new(inner: [T; N], longest: bool) -> Self {
        Self { inner, longest }
    }

    pub fn longest(&self) -> bool {
        self.longest
    }

    pub fn set_longest(&mut self, longest: bool) -> &mut Self {
        self.longest = longest;
        self
    }

    pub fn with_longest(mut self, longest: bool) -> Self {
        self.set_longest(longest);
        self
    }
}

impl<const N: usize, T> Deref for Array<N, T> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize, T> DerefMut for Array<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

        crate::debug_ctor_beg!("Array", offset);

        let ret = if self.longest {
            let handler = ctor::handler_ltm(Error::Array, sel, r2r);

            (handler)(&self.inner, ctx, func)
        } else {
            let handler = ctor::handler(Error::Array, sel, r2r);

            (handler)(&self.inner, ctx, func)
        };

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
        crate::debug_regex_beg!("Array", ctx.offset());

        let ret = if self.longest {
            let handler = regex::handler_ltm(Error::Array, sel);

            handler(&self.inner, ctx)
        } else {
            let handler = regex::handler(Error::Array, sel);

            handler(&self.inner, ctx)
        };

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
pub struct PairArray<const N: usize, K, V> {
    inner: [(K, V); N],
    longest: bool,
}

impl<const N: usize, K, V> core::ops::Not for PairArray<N, K, V> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<const N: usize, K, V> PairArray<N, K, V> {
    pub const fn new(inner: [(K, V); N], longest: bool) -> Self {
        Self { inner, longest }
    }

    pub fn longest(&self) -> bool {
        self.longest
    }

    pub fn set_longest(&mut self, longest: bool) -> &mut Self {
        self.longest = longest;
        self
    }

    pub fn with_longest(mut self, longest: bool) -> Self {
        self.set_longest(longest);
        self
    }
}

impl<const N: usize, K, V> Deref for PairArray<N, K, V> {
    type Target = [(K, V); N];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const N: usize, K, V> DerefMut for PairArray<N, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

        crate::debug_ctor_beg!("PairArray", offset);

        let ret = if self.longest {
            let handler = ctor::handler_ltm(Error::PairArray, sel_kv, r2r_kv);

            handler(&self.inner, ctx, func)
        } else {
            let handler = ctor::handler(Error::PairArray, sel_kv, r2r_kv);

            handler(&self.inner, ctx, func)
        };

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
        crate::debug_regex_beg!("PairArray", ctx.offset());

        let ret = if self.longest {
            let handler = regex::handler_ltm(Error::PairArray, sel_kv);

            handler(&self.inner, ctx)
        } else {
            let handler = regex::handler(Error::PairArray, sel_kv);

            handler(&self.inner, ctx)
        };

        crate::debug_regex_reval!("PairArray", ret)
    }
}
