use core::ops::Deref;
use core::ops::DerefMut;

use crate::alloc::Vec;
use crate::ctor;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

use super::Ctor;
use super::r2r;
use super::r2r_kv;
use super::sel;
use super::sel_kv;

///
/// Matches the **first successful expression** from a dynamic sequence of alternatives.
///
/// [`Vector`] provides ordered choice behavior over a runtime-defined collection of expressions:
/// 1. Tries expressions **in sequence** from the vector
/// 2. Returns immediately on the **first successful match**
/// 3. Fails only if **all expressions fail**
///
/// This is the dynamic counterpart to static alternatives like nested [`Or`](crate::ctor::Or) combinators.
///
/// # Regex
///
/// Returns the [`Span`] of the first successfully matched expression.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let keywords = regex::vector(["for", "while", "repeat"]);
///
///     assert_eq!(CharsCtx::new("while").try_mat(&keywords)?, Span::new(0, 5));
///     assert_eq!(CharsCtx::new("repeat").try_mat(&keywords)?, Span::new(0, 6));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Returns the constructed value from the first successfully matched expression.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let year = neu::digit(10).count::<4>();
///     let month = neu::digit(10).count::<2>();
///     let day = month;
///
///     let style1 = year.sep_once("-", month.sep_once("-", day));
///     let style1 = style1.map(|(y, (m, d))| (y, m, d));
///
///     let style2 = day.sep_once("/", month.sep_once("/", year));
///     let style2 = style2.map(|(d, (m, y))| (y, m, d));
///
///     let style3 = year.then(month.then(day));
///     let style3 = style3.map(|(y, (m, d))| (y, m, d));
///
///     let date = regex::vector([style1.into_dyn(), style2.into_dyn(), style3.into_dyn()]);
///
///     assert_eq!(CharsCtx::new("20251112").ctor(&date)?, ("2025", "11", "12"));
///     assert_eq!(
///         CharsCtx::new("12/10/2025").ctor(&date)?,
///         ("2025", "10", "12")
///     );
///     assert_eq!(
///         CharsCtx::new("2025-10-08").ctor(&date)?,
///         ("2025", "10", "08")
///     );
///
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Vector<T> {
    inner: Vec<T>,
    longest: bool,
}

impl_not_for_regex!(Vector<T>);

impl<T> Vector<T> {
    pub const fn new(inner: Vec<T>, longest: bool) -> Self {
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

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, C, T, O, H> Ctor<'a, C, O, H> for Vector<T>
where
    T: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let beg = ctx.offset();

        debug_ctor_beg!("Vector", beg);

        let ret = if self.longest {
            let handler = ctor::handler_ltm(Error::Vector, sel, r2r);

            handler(&self.inner, ctx, func)
        } else {
            let handler = ctor::handler(Error::Vector, sel, r2r);

            handler(&self.inner, ctx, func)
        };

        debug_ctor_reval!("Vector", beg, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, T> Regex<C> for Vector<T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        debug_regex_beg!("Vector", ctx.offset());

        let ret = if self.longest {
            let handler = regex::handler_ltm(Error::Vector, sel);

            handler(&self.inner, ctx)
        } else {
            let handler = regex::handler(Error::Vector, sel);

            handler(&self.inner, ctx)
        };
        debug_regex_reval!("Vector", ret)
    }
}

///
/// Matches the first successful expression from a dynamic sequence while carrying associated data.
///
/// [`PairVector`] extends ordered choice behavior by associating a value with each expression:
/// 1. Tries expressions **in sequence** from the vector of `(expression, value)` pairs
/// 2. Returns immediately on the **first successful match**
/// 3. Carries the associated value alongside the match result when successful
///
/// This combinator is particularly useful for mapping patterns to semantic values at runtime.
///
/// # Regex
///
/// Returns the [`Span`] of the first successfully matched expression, **ignoring associated values**.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     #[derive(Debug, Clone, Copy)]
///     enum Keyword {
///         For,
///         While,
///         Repeat,
///     }
///
///     let keywords = regex::pair_vector([
///         ("for", Keyword::For),
///         ("while", Keyword::While),
///         ("repeat", Keyword::Repeat),
///     ]);
///
///     assert_eq!(CharsCtx::new("while").try_mat(&keywords)?, Span::new(0, 5));
///     assert_eq!(CharsCtx::new("repeat").try_mat(&keywords)?, Span::new(0, 6));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
/// Returns a tuple `(O, V)` where:
/// - `O` is the constructed value from the matched expression
/// - `V` is the **cloned associated value** from the successful pair
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     #[derive(Debug, Clone, Copy, PartialEq)]
///     enum Keyword {
///         For,
///         While,
///         Repeat,
///     }
///
///     let keywords = regex::pair_vector([
///         ("for", Keyword::For),
///         ("while", Keyword::While),
///         ("repeat", Keyword::Repeat),
///     ]);
///
///     assert_eq!(
///         CharsCtx::new("while").ctor(&keywords)?,
///         ("while", Keyword::While)
///     );
///     assert_eq!(
///         CharsCtx::new("repeat").ctor(&keywords)?,
///         ("repeat", Keyword::Repeat)
///     );
///
/// #   Ok(())
/// # }
/// ```
///
/// # Key Requirements
///
/// - `V` must implement [`Clone`] for Ctor mode (to return owned values)
/// - All expressions must produce values of the same type in [`Ctor`] mode
#[derive(Debug, Clone)]
pub struct PairVector<T, V> {
    inner: Vec<(T, V)>,
    longest: bool,
}

impl_not_for_regex!(PairVector<T, V>);

impl<T, V> PairVector<T, V> {
    pub const fn new(inner: Vec<(T, V)>, longest: bool) -> Self {
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

impl<T, V> Deref for PairVector<T, V> {
    type Target = Vec<(T, V)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, V> DerefMut for PairVector<T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, C, T, O, V, H> Ctor<'a, C, (O, V), H> for PairVector<T, V>
where
    V: Clone,
    T: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let beg = ctx.offset();

        debug_ctor_beg!("PairVector", beg);

        let ret = if self.longest {
            let handler = ctor::handler_ltm(Error::PairVector, sel_kv, r2r_kv);

            handler(&self.inner, ctx, func)
        } else {
            let handler = ctor::handler(Error::PairVector, sel_kv, r2r_kv);

            handler(&self.inner, ctx, func)
        };

        debug_ctor_reval!("PairVector", beg, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, T, V> Regex<C> for PairVector<T, V>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        debug_regex_beg!("PairVector", ctx.offset());

        let ret = if self.longest {
            let handler = regex::handler_ltm(Error::PairVector, sel_kv);

            handler(&self.inner, ctx)
        } else {
            let handler = regex::handler(Error::PairVector, sel_kv);

            handler(&self.inner, ctx)
        };
        debug_regex_reval!("PairVector", ret)
    }
}
