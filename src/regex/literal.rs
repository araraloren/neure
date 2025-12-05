use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::def_not;
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

def_not!(LitSlice<'a, T>);

impl<'a, T> LitSlice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self { val }
    }
}

impl<'a, C, O, T, H, A> Ctor<'a, C, O, O, H, A> for LitSlice<'_, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig<'a> = &'a [T]> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
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
        ctx.req_data_less_than(slice_len)?;
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

def_not!(LitString<'a>);

impl<'a> LitString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for LitString<'_>
where
    C: Context<'a, Orig<'a> = &'a str> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
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
        ctx.req_data_less_than(literal_len)?;
        if ctx.remaining_len() >= literal_len && ctx.ctx().orig()?.starts_with(self.val) {
            ret = Ok(ctx.inc(literal_len));
        }
        crate::debug_regex_reval!("LitString", self.val, ret)
    }
}
