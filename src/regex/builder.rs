use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::span::Span;
use crate::err::Error;
use crate::regex::Adapter;
use crate::regex::Regex;

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
///         .into_regex_builder(|_, s| Ok(neu::ascii_alphanumeric().times(s.len()..=s.len())));
///     let year = year.try_map(map::from_str::<u64>());
///     let mut ctx = CharsCtx::new("rust2028");
///
///     assert_eq!(ctx.ctor(&year)?, 2028);
///
///     let what = regex::string("rust").into_regex_builder(|ctx: &mut CharsCtx, s| {
///         // reset to begin of previous regex
///         ctx.set_offset(s.beg());
///         Ok(regex::consume(s.len()).then(neu::ascii_alphanumeric().many1()))
///     });
///     let mut ctx = CharsCtx::new("rust10086");
///
///     assert_eq!(ctx.ctor(&what)?, "rust10086");
///     Ok(())
/// # }
/// ```
pub fn into_regex_builder<'a, C, P, T, F>(pat: P, func: F) -> Adapter<C, impl Regex<C>>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Match<'a>,
    F: Fn(&mut C, &Span) -> Result<T, Error>,
{
    let regex = move |ctx: &mut C| {
        let mut ctx = CtxGuard::new(ctx);

        crate::debug_regex_beg!("into_regex_builder", ctx.beg());

        // match first regex
        let span = ctx.try_mat(&pat)?;
        // build new regex base on result, let the user control ctx
        // continue match from end of previous span
        let new_pat = (func)(ctx.ctx(), &span)?;
        // match again
        let ret = ctx.try_mat(&new_pat);

        Ok(crate::debug_regex_reval!("into_regex_builder", ret)?)
    };

    Adapter::new(regex)
}

pub trait DynamicRegexBuilderHelper<'a, C>
where
    Self: Sized,
    C: Match<'a>,
{
    fn into_regex_builder<F, R>(self, func: F) -> Adapter<C, impl Regex<C>>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>;
}

impl<'a, C, T> DynamicRegexBuilderHelper<'a, C> for T
where
    Self: Regex<C> + Sized,
    C: Match<'a>,
{
    fn into_regex_builder<F, R>(self, func: F) -> Adapter<C, impl Regex<C>>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>,
    {
        into_regex_builder(self, func)
    }
}
