use super::Extract;
use super::Handler;
use super::Mapper;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub trait OpExtension<'a, C>
where
    Self: Sized,
    C: Context<'a>,
{
    fn pattern(self) -> Pattern<Self>;

    fn map<F>(self, f: F) -> Map<Self, F>;
}

impl<'a, C, P> OpExtension<'a, C> for P
where
    P: Parse<C>,
    C: Context<'a>,
{
    fn pattern(self) -> Pattern<Self> {
        Pattern { pat: self }
    }

    fn map<F>(self, f: F) -> Map<Self, F> {
        Map { pat: self, func: f }
    }
}

pub struct Pattern<P> {
    pat: P,
}

impl<'a, C, O, P> Mapper<'a, C, O, O> for Pattern<P>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.pat)?;

        Handler::invoke(&mut func, A::extract(ctx, &ret)?)
    }
}

pub struct Map<P, F> {
    pat: P,
    func: F,
}

impl<'a, C, M, O, P, F> Mapper<'a, C, M, O> for Map<P, F>
where
    F: Fn(M) -> O,
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.pat)?;

        Handler::invoke(&mut func, A::extract(ctx, &ret)?).map(&self.func)
    }
}
