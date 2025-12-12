use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

/// Success if the [`offset`](crate::ctx::Context#tymethod.offset) of [`Context`] is equal to 0.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorStart;

impl_not_for_regex!(AnchorStart);

impl AnchorStart {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for AnchorStart
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
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

impl_not_for_regex!(AnchorEnd);

impl AnchorEnd {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for AnchorEnd
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
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
