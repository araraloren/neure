use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

///
/// [`into_regex_builder`] can dynamically construct a new regex based on the [`Span`]
/// result of given `P`, then use this newly regex to continue matching forward. When
/// successful, it will return [`Span`] of newly regex.
///
/// # Example
/// ```
/// # use neure::{prelude::*, re::DynamicRegexBuilderHelper};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let year = re::string("rust")
///         .into_regex_builder(|_, s| Ok(neu::ascii_alphanumeric().repeat_range(s.len()..=s.len())));
///     let year = year.map(map::from_str::<u64>());
///     let mut ctx = CharsCtx::new("rust2028");
///
///     assert_eq!(ctx.ctor(&year)?, 2028);
///
///     let what = re::string("rust").into_regex_builder(|ctx: &mut CharsCtx, s| {
///         // reset to begin of previous regex
///         ctx.set_offset(s.beg());
///         Ok(re::consume(s.len()).then(neu::ascii_alphanumeric().repeat_one_more()))
///     });
///     let mut ctx = CharsCtx::new("rust10086");
///
///     assert_eq!(ctx.ctor(&what)?, "rust10086");
///     Ok(())
/// # }
/// ```
pub fn into_regex_builder<'a, C, P, T, F>(pat: P, func: F) -> impl Regex<C>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Context<'a> + Match<C>,
    F: Fn(&mut C, &Span) -> Result<T, Error>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);

        crate::debug_regex_beg!("into_regex_builder", g.beg());

        // match first regex
        let ret = g.try_mat(&pat)?;

        // build new regex base on result, let the user control ctx
        // continue match from end of previous span
        let pat = (func)(g.ctx(), &ret)?;
        let ret = g.try_mat(&pat);

        crate::debug_regex_reval!("into_regex_builder", ret)
    }
}

pub trait DynamicRegexBuilderHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Match<C>,
{
    fn into_regex_builder<F, R>(self, func: F) -> impl Regex<C>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>;
}

impl<'a, C, T> DynamicRegexBuilderHelper<'a, C> for T
where
    Self: Regex<C> + Sized,
    C: Context<'a> + Match<C>,
{
    fn into_regex_builder<F, R>(self, func: F) -> impl Regex<C>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>,
    {
        into_regex_builder(self, func)
    }
}
