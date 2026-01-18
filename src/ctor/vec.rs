use core::ops::Deref;
use core::ops::DerefMut;

use crate::alloc::Vec;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

use super::Ctor;

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
pub struct Vector<T>(Vec<T>);

impl_not_for_regex!(Vector<T>);

impl<T> Vector<T> {
    pub fn new(val: Vec<T>) -> Self {
        Self(val)
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let offset = ctx.offset();
        let mut ret = Err(Error::Vector);

        debug_ctor_beg!("Vector", offset);
        for regex in self.0.iter() {
            if let Ok(span) = regex.construct(ctx, func).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(span);
                break;
            }
        }

        debug_ctor_reval!("Vector", offset, ctx.offset(), ret.is_ok());
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
        let offset = ctx.offset();
        let mut ret = Err(Error::Vector);

        debug_regex_beg!("Vector", offset);
        for regex in self.0.iter() {
            if let Ok(span) = ctx.try_mat(regex).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(span);
                break;
            }
        }
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
pub struct PairVector<T, V>(Vec<(T, V)>);

impl_not_for_regex!(PairVector<T, V>);

impl<T, V> PairVector<T, V> {
    pub fn new(val: Vec<(T, V)>) -> Self {
        Self(val)
    }
}

impl<T, V> Deref for PairVector<T, V> {
    type Target = Vec<(T, V)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, V> DerefMut for PairVector<T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let offset = ctx.offset();
        let mut ret = Err(Error::PairVector);

        debug_ctor_beg!("PairVector", offset);
        for (regex, value) in self.0.iter() {
            if let Ok(span) = regex.construct(ctx, func).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok((span, value.clone()));
                break;
            }
        }

        debug_ctor_reval!("PairVector", offset, ctx.offset(), ret.is_ok());
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
        let offset = ctx.offset();
        let mut ret = Err(Error::PairVector);

        debug_regex_beg!("PairVector", offset);
        for (regex, _) in self.0.iter() {
            if let Ok(span) = ctx.try_mat(regex).inspect_err(|_| {
                ctx.set_offset(offset);
            }) {
                ret = Ok(span);
                break;
            }
        }
        debug_regex_reval!("PairVector", ret)
    }
}
