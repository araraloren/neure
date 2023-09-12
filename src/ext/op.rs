use super::CtxGuard;
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

    fn map_with<F>(self, f: F) -> Map<Self, F>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<L, R, Self>;
}

impl<'a, C, P> OpExtension<'a, C> for P
where
    P: Parse<C>,
    C: Context<'a>,
{
    fn pattern(self) -> Pattern<Self> {
        Pattern { pat: self }
    }

    fn map_with<F>(self, f: F) -> Map<Self, F> {
        Map { pat: self, func: f }
    }

    fn quote<L, R>(self, left: L, right: R) -> Quote<L, R, Self> {
        Quote {
            pat: self,
            left,
            right,
        }
    }
}

pub struct Quote<L, R, P> {
    pat: P,
    left: L,
    right: R,
}

impl<'a, C, L, R, P, M, O> Mapper<'a, C, M, O> for Quote<L, R, P>
where
    L: Parse<C, Ret = Span>,
    R: Parse<C, Ret = Span>,
    P: Mapper<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = self.pat.map(g.ctx(), func)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}

impl<'a, C, L, R, P> Parse<C> for Quote<L, R, P>
where
    L: Parse<C, Ret = Span>,
    R: Parse<C, Ret = Span>,
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&self.left)?;
        let ret = g.try_mat(&self.pat)?;

        g.try_mat(&self.right)?;
        Ok(ret)
    }
}

pub struct Pattern<P> {
    pat: P,
}

impl<'a, C, M, P> Mapper<'a, C, M, M> for Pattern<P>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, mut func: H) -> Result<M, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.pat)?;

        Handler::invoke(&mut func, A::extract(ctx, &ret)?)
    }
}

impl<'a, C, P> Parse<C> for Pattern<P>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}

pub struct Map<P, F> {
    pat: P,
    func: F,
}

impl<'a, C, M, O, P, F> Mapper<'a, C, M, O> for Map<P, F>
where
    F: Fn(M) -> O,
    P: Mapper<'a, C, M, M>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.pat.map(ctx, func).map(&self.func)
    }
}

impl<'a, C, P, F> Parse<C> for Map<P, F>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
