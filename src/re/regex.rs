mod dthen;
mod dynamic;
mod not;
mod slice;

pub use self::dthen::DynamicCreateRegexThen;
pub use self::dthen::DynamicCreateRegexThenHelper;
pub use self::dynamic::into_dyn_regex;
pub use self::dynamic::DynamicRegex;
pub use self::dynamic::DynamicRegexHandler;
pub use self::dynamic::DynamicRegexHelper;
pub use self::not::RegexNot;
pub use self::slice::RegexSlice;
pub use self::slice::RegexString;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

/// Success if the [`offset`](crate::ctx::Context#tymethod.offset) of [`Context`](crate::ctx::Context) is equal to 0.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexStart;

def_not!(RegexStart);

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

/// Success if the [`offset`](crate::ctx::Context#tymethod.offset) of [`Context`](crate::ctx::Context) is equal to [`len`](crate::ctx::Context#tymethod.len).
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexEnd;

def_not!(RegexEnd);

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

/// Consume the specified number [`Item`](crate::ctx::Context::Item)s.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexConsume(usize);

def_not!(RegexConsume);

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

/// Consume all remaining [`Item`](crate::ctx::Context::Item)s of the [`Context`].
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexConsumeAll;

def_not!(RegexConsumeAll);

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
