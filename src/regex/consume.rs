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

/// Consume the specified number [`Item`](crate::ctx::Context::Item)s.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Consume(usize);

impl_not_for_regex!(Consume);

impl Consume {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for Consume
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

impl<'a, C> Regex<C> for Consume
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("Consume", ctx.beg());

        let ret = if ctx.remaining_len() >= self.0 {
            Ok(ctx.inc(self.0))
        } else {
            Err(Error::Consume)
        };

        debug_regex_reval!("Consume", ret)
    }
}

/// Consume all remaining [`Item`](crate::ctx::Context::Item)s of the [`Context`].
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumeAll;

impl_not_for_regex!(ConsumeAll);

impl ConsumeAll {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for ConsumeAll
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

impl<'a, C> Regex<C> for ConsumeAll
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("ConsumeAll", ctx.beg());

        let len = ctx.len().saturating_sub(ctx.beg());
        let ret = Ok(ctx.inc(len));

        debug_regex_reval!("ConsumeAll", ret)
    }
}
