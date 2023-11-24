mod collect;
mod dthen;
mod dynamic;
mod or;
mod quote;
mod repeat;
mod sep;
mod then;

pub use self::collect::RegexCollect;
pub use self::dthen::DynamicCreateRegexThen;
pub use self::dthen::DynamicCreateRegexThenHelper;
pub use self::dynamic::into_dyn_regex;
pub use self::dynamic::DynamicRegex;
pub use self::dynamic::DynamicRegexHandler;
pub use self::dynamic::DynamicRegexHelper;
pub use self::or::RegexOr;
pub use self::quote::RegexQuote;
pub use self::repeat::RegexRepeat;
pub use self::sep::RegexSeparate;
pub use self::sep::RegexSeparateCollect;
pub use self::sep::RegexSeparateOnce;
pub use self::then::RegexThen;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::neu::CRange;
use crate::re::Regex;

///
/// Match `P1` then `P2`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let ip = re::string("127.0.0.1");
///     let colon = ':'.repeat_one();
///     let port = neu::digit(10).repeat_one_more();
///     let local = ip.then(colon).then(port);
///     let mut ctx = CharsCtx::new("127.0.0.1:8080");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 14));
///     Ok(())
/// # }
/// ```
pub fn then<'a, C, P, T>(pat: P, then: T) -> RegexThen<C, P, T>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    RegexThen::new(pat, then)
}

///
/// Match `P1` or `P2`.
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
///     let local = name.or(ip);
///     let local = local.then(":8080");
///     let mut ctx = CharsCtx::new("127.0.0.1:8080");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 14));
///     let mut ctx = CharsCtx::new("localhost:8080");
///
///     assert_eq!(ctx.try_mat(&local)?, Span::new(0, 14));
///     Ok(())
/// # }
/// ```
pub fn or<'a, C, O, L, R>(left: L, right: R) -> RegexOr<C, L, R>
where
    O: Ret,
    L: Regex<C, Ret = O>,
    R: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    RegexOr::new(left, right)
}

///
/// Match the `P` enclosed by `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let comma = re::zero_one(',');
///     let digit = re::one_more('0'..='9');
///     let digit = re::terminated(comma, digit);
///     let array = re::quote(re::one('['), re::one(']'), digit.repeat(3..4));
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
    C: Context<'a> + Policy<C>,
{
    RegexQuote::new(pat, left, right)
}

pub fn repeat<'a, C, P, R>(pat: P, range: R, capacity: usize) -> RegexRepeat<C, P>
where
    P: Regex<C>,
    R: Into<CRange<usize>>,
    C: Context<'a> + Policy<C>,
{
    RegexRepeat::new(pat, range).with_capacity(capacity)
}

///
/// Match the `P` terminated by `S`, return the return value of `P`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let comma = re::zero_one(',');
///     let digit = re::one_more('0'..='9');
///     let digit = re::terminated(comma, digit);
///     let mut ctx = CharsCtx::new("123,456,789");
///
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 3));
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(4, 3));
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(8, 3));
///     Ok(())
/// # }
/// ```
pub fn sep<'a, C, S, P>(
    pat: P,
    sep: S,
    min: usize,
    skip: bool,
    capacity: usize,
) -> RegexSeparate<C, P, S>
where
    P: Regex<C>,
    S: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    RegexSeparate::new(pat, sep)
        .with_capacity(capacity)
        .with_skip(skip)
        .at_least(min)
}

pub fn sep_collect<'a, C, S, P, T>(
    pat: P,
    sep: S,
    min: usize,
    skip: bool,
) -> RegexSeparateCollect<C, P, S, T>
where
    P: Regex<C>,
    S: Regex<C>,
    T: FromIterator<P::Ret>,
    C: Context<'a> + Policy<C>,
{
    RegexSeparateCollect::new(pat, sep)
        .with_skip(skip)
        .at_least(min)
}

pub fn sep_once<'a, C, L, S, R>(left: L, sep: S, right: R) -> RegexSeparateOnce<C, L, S, R>
where
    L: Regex<C>,
    S: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    RegexSeparateOnce::new(left, sep, right)
}

pub fn collect<'a, C, P, O>(pat: P, min: usize) -> RegexCollect<C, P, O>
where
    P: Regex<C>,
    O: FromIterator<P::Ret>,
    C: Context<'a> + Policy<C>,
{
    RegexCollect::new(pat).at_least(min)
}