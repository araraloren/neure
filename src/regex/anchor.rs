use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

/// Success if the [`offset`](crate::ctx::Context#tymethod.offset) of [`Context`] is equal to 0.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorStart;

def_not!(AnchorStart);

impl AnchorStart {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for AnchorStart
where
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for AnchorStart
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("AnchorStart", ctx.beg());

        let ret = if ctx.beg() == 0 {
            Ok(ctx.inc(0))
        } else {
            Err(Error::AnchorStart)
        };

        debug_regex_reval!("AnchorStart", ret)
    }
}

/// Success if the [`offset`](crate::ctx::Context#tymethod.offset) of [`Context`] is equal to [`len`](crate::ctx::Context#tymethod.len).
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorEnd;

def_not!(AnchorEnd);

impl AnchorEnd {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for AnchorEnd
where
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for AnchorEnd
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("AnchorEnd", ctx.beg());

        let ret = if ctx.beg() == ctx.len() {
            Ok(ctx.inc(0))
        } else {
            Err(Error::AnchorEnd)
        };

        debug_regex_reval!("AnchorEnd", ret)
    }
}
