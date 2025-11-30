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

impl<'a, C> Regex<C> for AnchorStart
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("AnchorStart", g.beg());

        let ret = if g.beg() == 0 {
            Ok(g.inc(0))
        } else {
            Err(Error::Start)
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

impl<'a, C> Regex<C> for AnchorEnd
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("AnchorEnd", g.beg());

        let ret = if g.beg() == g.ctx().len() {
            Ok(g.inc(0))
        } else {
            Err(Error::End)
        };

        debug_regex_reval!("AnchorEnd", ret)
    }
}
