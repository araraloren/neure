use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

/// Consume the specified number [`Item`](crate::ctx::Context::Item)s.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Consume(usize);

def_not!(Consume);

impl Consume {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for Consume
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for Consume
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("Consume", g.beg());

        let ret = if g.ctx().len() - g.beg() >= self.0 {
            Ok(g.inc(self.0))
        } else {
            Err(Error::Consume)
        };

        debug_regex_reval!("Consume", ret)
    }
}

/// Consume all remaining [`Item`](crate::ctx::Context::Item)s of the [`Context`].
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumeAll;

def_not!(ConsumeAll);

impl ConsumeAll {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for ConsumeAll
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for ConsumeAll
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("ConsumeAll", g.beg());

        let len = g.ctx().len().saturating_sub(g.beg());
        let ret = Ok(g.inc(len));

        debug_regex_reval!("ConsumeAll", ret)
    }
}
