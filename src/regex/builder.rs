use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::Wrap;

///
/// [`into_regex_builder`] can dynamically construct a new regex based on the [`Span`]
/// result of given `P`, then use this newly regex to continue matching forward. When
/// successful, it will return [`Span`] of newly regex.
///
/// # Example
/// ```
/// # use neure::{prelude::*, regex::DynamicRegexBuilderHelper};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let year = regex::string("rust")
///         .into_regex_builder(|_, s| Ok(neu::ascii_alphanumeric().repeat_range(s.len()..=s.len())));
///     let year = year.try_map(map::from_str::<u64>());
///     let mut ctx = CharsCtx::new("rust2028");
///
///     assert_eq!(ctx.ctor(&year)?, 2028);
///
///     let what = regex::string("rust").into_regex_builder(|ctx: &mut CharsCtx, s| {
///         // reset to begin of previous regex
///         ctx.set_offset(s.beg());
///         Ok(regex::consume(s.len()).then(neu::ascii_alphanumeric().repeat_one_more()))
///     });
///     let mut ctx = CharsCtx::new("rust10086");
///
///     assert_eq!(ctx.ctor(&what)?, "rust10086");
///     Ok(())
/// # }
/// ```
pub fn into_regex_builder<'a, C, P, T, F>(pat: P, func: F) -> Wrap<impl Regex<C>, C>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Match<'a>,
    F: Fn(&mut C, &Span) -> Result<T, Error>,
{
    let regex = move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);

        crate::debug_regex_beg!("into_regex_builder", g.beg());

        // match first regex
        let ret = g.try_mat(&pat)?;

        // build new regex base on result, let the user control ctx
        // continue match from end of previous span
        let pat = (func)(g.ctx(), &ret)?;
        let ret = g.try_mat(&pat);

        crate::debug_regex_reval!("into_regex_builder", ret)
    };

    Wrap::new(regex)
}

pub trait DynamicRegexBuilderHelper<'a, C>
where
    Self: Sized,
    C: Match<'a>,
{
    fn into_regex_builder<F, R>(self, func: F) -> Wrap<impl Regex<C>, C>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>;
}

impl<'a, C, T> DynamicRegexBuilderHelper<'a, C> for T
where
    Self: Regex<C> + Sized,
    C: Match<'a>,
{
    fn into_regex_builder<F, R>(self, func: F) -> Wrap<impl Regex<C>, C>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>,
    {
        into_regex_builder(self, func)
    }
}
