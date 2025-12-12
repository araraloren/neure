use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

/// Match given slice in the [`Context`].
///
/// # Regex
///
/// Return a [`Span`] as match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitSlice<'a, T> {
    val: &'a [T],
}

impl_not_for_regex!(LitSlice<'a, T>);

impl<'a, T> LitSlice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self { val }
    }
}

impl<'a, C, O, T, H> Ctor<'a, C, O, O, H> for LitSlice<'_, T>
where
    T: PartialOrd + 'a,
    C: Match<'a, Orig<'a> = &'a [T]>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, T> Regex<C> for LitSlice<'_, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig<'a> = &'a [T]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::LitSlice);
        let slice_len = self.val.len();

        crate::debug_regex_beg!("LitSlice", ctx.beg());
        if ctx.remaining_len() >= slice_len && ctx.ctx().orig()?.starts_with(self.val) {
            ret = Ok(ctx.inc(slice_len));
        }
        crate::debug_regex_reval!("LitSlice", ret)
    }
}

/// Match given string in the [`Context`].
///
/// # Regex
///
/// Return a [`Span`] as match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitString<'a> {
    val: &'a str,
}

impl_not_for_regex!(LitString<'a>);

impl<'a> LitString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for LitString<'_>
where
    C: Match<'a, Orig<'a> = &'a str>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C> Regex<C> for LitString<'_>
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::LitString);
        let literal_len = self.val.len();

        crate::debug_regex_beg!("LitString", ctx.beg());
        if ctx.remaining_len() >= literal_len && ctx.ctx().orig()?.starts_with(self.val) {
            ret = Ok(ctx.inc(literal_len));
        }
        crate::debug_regex_reval!("LitString", self.val, ret)
    }
}
