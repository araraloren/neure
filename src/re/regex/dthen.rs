use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// [`DynamicRegexBuilder`] can dynamically construct a new regex based on the [`Span`]
/// result of given `pat`, then use this newly regex to continue matching forward. When
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
#[derive(Default, Copy)]
pub struct DynamicRegexBuilder<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

def_not!(DynamicRegexBuilder<C, P, F>);

impl<C, P, F> Debug for DynamicRegexBuilder<C, P, F>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicRegexBuilder")
            .field("pat", &self.pat)
            .field("func", &self.func)
            .finish()
    }
}

impl<C, P, F> Clone for DynamicRegexBuilder<C, P, F>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            func: self.func.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F> DynamicRegexBuilder<C, P, F> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn func(&self) -> &F {
        &self.func
    }

    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_func(&mut self, func: F) -> &mut Self {
        self.func = func;
        self
    }
}

impl<'a, C, O, H, A, P, F, T> Ctor<'a, C, O, O, H, A> for DynamicRegexBuilder<C, P, F>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Context<'a> + Match<C>,
    F: Fn(&mut C, &Span) -> Result<T, Error>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(self);

        func.invoke(A::extract(g.ctx(), &ret?)?)
    }
}

impl<'a, C, P, F, T> Regex<C> for DynamicRegexBuilder<C, P, F>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Context<'a> + Match<C>,
    F: Fn(&mut C, &Span) -> Result<T, Error>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        crate::debug_regex_beg!("DynamicRegexBuilder", g.beg());

        // match first regex
        let ret = g.try_mat(&self.pat)?;

        // build new regex base on result, let the user control ctx
        // continue match from end of previous span
        let pat = (self.func)(g.ctx(), &ret)?;
        let ret = g.try_mat(&pat);

        crate::debug_regex_reval!("DynamicRegexBuilder", g.beg(), g.end(), ret)
    }
}

pub trait DynamicRegexBuilderHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Match<C>,
{
    fn into_regex_builder<F, R>(self, func: F) -> DynamicRegexBuilder<C, Self, F>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>;
}

impl<'a, C, T> DynamicRegexBuilderHelper<'a, C> for T
where
    Self: Regex<C> + Sized,
    C: Context<'a> + Match<C>,
{
    fn into_regex_builder<F, R>(self, func: F) -> DynamicRegexBuilder<C, Self, F>
    where
        R: Regex<C>,
        F: Fn(&mut C, &Span) -> Result<R, Error>,
    {
        DynamicRegexBuilder::new(self, func)
    }
}
