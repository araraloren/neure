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
        let mut ret = Err(Error::Consume);
        let beg = ctx.offset();

        if ctx.len() - beg >= self.0 {
            ctx.inc(self.0);
            ret = Ok(Span::new(beg, self.0));
        }
        trace!("consume", beg => ctx.offset(), ret)
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
        let beg = ctx.offset();
        let len = ctx.len().saturating_sub(ctx.offset());

        ctx.inc(len);
        trace!("consume_all", beg => ctx.offset(), Ok(Span::new(beg, len)))
    }
}
