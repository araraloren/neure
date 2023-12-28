mod anchor;
mod collect;
mod dthen;
mod dynamic;
mod ltm;
mod map;
mod not;
mod or;
mod quote;
mod repeat;
mod sep;
mod slice;
mod then;

pub use self::anchor::RegexConsume;
pub use self::anchor::RegexConsumeAll;
pub use self::anchor::RegexEnd;
pub use self::anchor::RegexStart;
pub use self::collect::RegexCollect;
pub use self::dthen::DynamicCreateRegexThen;
pub use self::dthen::DynamicCreateRegexThenHelper;
pub use self::dynamic::into_dyn_regex;
pub use self::dynamic::DynamicRegex;
pub use self::dynamic::DynamicRegexHandler;
pub use self::dynamic::DynamicRegexHelper;
pub use self::ltm::RegexLongestTokenMatch;
pub use self::map::RegexMap;
pub use self::not::RegexNot;
pub use self::or::RegexOr;
pub use self::quote::RegexQuote;
pub use self::repeat::RegexRepeat;
pub use self::sep::RegexSepCollect;
pub use self::sep::RegexSepOnce;
pub use self::sep::RegexSeparate;
pub use self::slice::RegexSlice;
pub use self::slice::RegexString;
pub use self::then::RegexThen;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::neu::CRange;
use crate::re::Regex;

use super::map::MapSingle;

/// First try to match `L`. If it succeeds, then try to match `R`.
///
/// # Return
///
/// Return a tuple of result of `L` and result of `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let val = neu::ascii_alphabetic().repeat_one_more();
///     let num = neu::ascii_alphanumeric().repeat_one_more();
///     let tuple = re::regex::then(val, num);
///
///     assert_eq!(
///         CharsCtx::new("abc42").try_mat_t(&tuple)?,
///         (Span::new(0, 3), Span::new(3, 2))
///     );
///     Ok(())
/// # }
/// ```
pub fn then<'a, C, L, R>(left: L, right: R) -> RegexThen<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    RegexThen::new(left, right)
}

///
/// First try to match `L`, if it fails, then try to match `R`.
/// Return the result of either `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let name = re::string("localhost");
///     let ip = re::string("127.0.0.1");
///     let local = re::or(name, ip);
///     let mut ctx = CharsCtx::new("127.0.0.1");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 9));
///     let mut ctx = CharsCtx::new("localhost");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 9));
///
///     Ok(())
/// # }
/// ```
pub fn or<'a, C, O, L, R>(left: L, right: R) -> RegexOr<C, L, R>
where
    O: Ret,
    L: Regex<C, Ret = O>,
    R: Regex<C, Ret = O>,
    C: Context<'a> + Match<C>,
{
    RegexOr::new(left, right)
}

/// Match `L` and `R`, return the longest match result.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let name = re::string("localhost");
///     let ip = re::string("8080");
///     let withip = name.sep_once(":", ip);
///     let local = re::or(name, withip);
///     let mut ctx = CharsCtx::new("localhost:8080");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 9));
///     let local = re::ltm(name, withip);
///     let mut ctx = CharsCtx::new("localhost");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 9));
///     let mut ctx = CharsCtx::new("localhost:8080");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 14));
///
///     Ok(())
/// # }
/// ```
pub fn ltm<'a, C, O, L, R>(left: L, right: R) -> RegexLongestTokenMatch<C, L, R>
where
    O: Ret,
    L: Regex<C, Ret = O>,
    R: Regex<C, Ret = O>,
    C: Context<'a> + Match<C>,
{
    RegexLongestTokenMatch::new(left, right)
}

///
/// First try to match `L`. If it is succeeds, then try to match `P`.
/// If it is succeeds, then try to match `R`.
/// It will return the result of `P`, ignoring the result of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let comma = re::one(',');
///     let digit = re::one_more('0'..='9');
///     let digit = re::sep(digit, comma, true);
///     let array = re::quote(digit, re::one('['), re::one(']'));
///     let mut ctx = CharsCtx::new("[123,456,789]");
///
///     assert_eq!(
///         ctx.try_mat_t(&array)?,
///         vec![Span::new(1, 3), Span::new(5, 3), Span::new(9, 3)]
///     );
///     Ok(())
/// # }
/// ```
pub fn quote<'a, C, L, R, P>(pat: P, left: L, right: R) -> RegexQuote<C, P, L, R>
where
    P: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    RegexQuote::new(pat, left, right)
}

/// Repeatedly match regex `P`, and the number of matches must meet the given range.
/// It will return a [`Vec`] of `P`'s match results.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let name = re::string("foo");
///     let names = re::repeat(name, 1..5);
///     let mut ctx = CharsCtx::new("foofoofoofoo");
///
///     assert_eq!(
///         ctx.try_mat_t(&names)?,
///         vec![
///             Span::new(0, 3),
///             Span::new(3, 3),
///             Span::new(6, 3),
///             Span::new(9, 3)
///         ]
///     );
///
///     Ok(())
/// # }
/// ```
pub fn repeat<'a, C, P, R>(pat: P, range: R) -> RegexRepeat<C, P>
where
    P: Regex<C>,
    R: Into<CRange<usize>>,
    C: Context<'a> + Match<C>,
{
    RegexRepeat::new(pat, range)
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
/// It will return a [`Vec`] of `P`'s match results.
/// Set `skip` to `true` if the last delimiter is optional.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let comma = re::one(',');
///     let digit = re::one_more('0'..='9');
///     let digit = re::sep(digit, comma, true);
///     let mut ctx = CharsCtx::new("123,456,789");
///
///     assert_eq!(ctx.try_mat_t(&digit)?, vec![Span::new(0, 3), Span::new(4, 3), Span::new(8, 3)]);
///     Ok(())
/// # }
/// ```
pub fn sep<'a, C, S, P>(pat: P, sep: S, skip: bool) -> RegexSeparate<C, P, S>
where
    P: Regex<C>,
    S: Regex<C>,
    C: Context<'a> + Match<C>,
{
    RegexSeparate::new(pat, sep).with_skip(skip)
}

/// Match regex `P` as many times as possible, with S as the delimiter.
/// It will return a `T` that can constructed from `P`'s match results
/// using [`from_iter`](std::iter::FromIterator::from_iter).
///
/// # Notice
///
/// `SepCollect` will always succeed if the minimum size is 0, be careful to use it with other `.sep` faimly APIs.
/// The default size is 1.
///
/// # Example
///
/// ```
/// ```
pub fn sep_collect<'a, C, P, T>(
    pat: P,
    sep: impl Regex<C>,
    skip: bool,
) -> RegexSepCollect<C, P, impl Regex<C>, T>
where
    P: Regex<C>,
    T: FromIterator<P::Ret>,
    C: Context<'a> + Match<C>,
{
    RegexSepCollect::new(pat, sep).with_skip(skip)
}

/// Match `L` and `R` separated by `S`.
/// It will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// ```
pub fn sep_once<'a, C, L, S, R>(left: L, sep: S, right: R) -> RegexSepOnce<C, L, S, R>
where
    L: Regex<C>,
    S: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<C>,
{
    RegexSepOnce::new(left, sep, right)
}

///
/// Match the regex `P` repeatedly, and collect the result into given type `T`.
///
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let val = neu::ascii_alphabetic().repeat_one();
///     let vec = re::collect::<_, _, Vec<_>>(val, 1);
///
///     assert_eq!(
///         CharsCtx::new("abcdf").try_mat_t(&vec)?,
///         vec![
///             Span::new(0, 1),
///             Span::new(1, 1),
///             Span::new(2, 1),
///             Span::new(3, 1),
///             Span::new(4, 1),
///         ]
///     );
///     Ok(())
/// # }
/// ```
pub fn collect<'a, C, P, T>(pat: P, min: usize) -> RegexCollect<C, P, T>
where
    P: Regex<C>,
    T: FromIterator<P::Ret>,
    C: Context<'a> + Match<C>,
{
    RegexCollect::new(pat).at_least(min)
}

pub fn re_map<'a, C, P, F, O>(pat: P, func: F) -> RegexMap<C, P, F, O>
where
    P: Regex<C>,
    F: MapSingle<P::Ret, O>,
    C: Context<'a> + Match<C>,
{
    RegexMap::new(pat, func)
}
