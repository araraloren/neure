use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
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
        let mut ret = Err(Error::Start);
        let beg = ctx.offset();

        if ctx.offset() == 0 {
            ret = Ok(Span::new(ctx.offset(), 0))
        }
        trace!("start", beg => ctx.offset(), ret)
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
        let mut ret = Err(Error::End);
        let beg = ctx.offset();

        if ctx.len() == ctx.offset() {
            ret = Ok(Span::new(ctx.offset(), 0));
        }
        trace!("start", beg => ctx.offset(), ret)
    }
}
