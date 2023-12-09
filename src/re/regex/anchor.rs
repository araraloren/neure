use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexStart;

impl RegexStart {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for RegexStart
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for RegexStart
where
    C: Context<'a>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut ret = Err(Error::Start);
        let beg = ctx.offset();

        if ctx.offset() == 0 {
            ret = Ok(<Span as Ret>::from_ctx(ctx, (0, 0)))
        }
        trace!("start", beg => ctx.offset(), ret)
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexEnd;

impl RegexEnd {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for RegexEnd
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for RegexEnd
where
    C: Context<'a>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut ret = Err(Error::End);
        let beg = ctx.offset();

        if ctx.len() == ctx.offset() {
            ret = Ok(<Span as Ret>::from_ctx(ctx, (0, 0)));
        }
        trace!("start", beg => ctx.offset(), ret)
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexConsume(usize);

impl RegexConsume {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for RegexConsume
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for RegexConsume
where
    C: Context<'a>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut ret = Err(Error::Consume);
        let beg = ctx.offset();

        if ctx.len() - beg >= self.0 {
            ctx.inc(self.0);
            ret = Ok(Span::new(beg, self.0));
        }
        trace!("consume", beg => ctx.offset(), ret)
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexConsumeAll;

impl RegexConsumeAll {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for RegexConsumeAll
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Regex<C> for RegexConsumeAll
where
    C: Context<'a>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let beg = ctx.offset();
        let len = ctx.len().saturating_sub(ctx.offset());

        ctx.inc(len);
        trace!("consume_all", beg => ctx.offset(), Ok(Span::new(beg, len)))
    }
}
