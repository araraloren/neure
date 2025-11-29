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
    C: Context<'a, Orig = [T]> + Match<C>,
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
    C: Context<'a, Orig = [T]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Slice);

        crate::debug_regex_beg!("LitSlice", g.beg());
        if g.ctx().orig()?.starts_with(self.val) {
            ret = Ok(g.inc(self.val.len()));
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
    C: Context<'a, Orig = str> + Match<C>,
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
    C: Context<'a, Orig = str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::String);

        if g.ctx().orig()?.starts_with(self.val) {
            ret = Ok(g.inc(self.val.len()));
        }
        crate::debug_regex_reval!("LitString", self.val, ret)
    }
}
