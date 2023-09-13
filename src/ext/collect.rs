use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Mapper;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::prelude::Ret;

pub struct Collect<P, M, O, V> {
    pat: P,
    marker: PhantomData<(M, O, V)>,
}

impl<P, M, O, V> Collect<P, M, O, V> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            marker: PhantomData,
        }
    }
}

impl<'a, C, P, M, O, V> Mapper<'a, C, M, V> for Collect<P, M, O, V>
where
    V: FromIterator<O>,
    P: Mapper<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ok(V::from_iter(std::iter::from_fn(|| {
            self.pat.map(ctx, func).ok()
        })))
    }
}

impl<'a, C, P, M, O, V> Parse<C> for Collect<P, M, O, V>
where
    P: Parse<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = g.try_mat(&self.pat)?;

        while let Ok(ret) = g.try_mat(&self.pat) {
            span.add_assign(ret);
        }
        Ok(span)
    }
}

// pub struct CollectWith<P, O> {
//     pat: P,
//     cap: usize,
// }

// impl<'a, C, P, M, O> Mapper<'a, C, M, O> for CollectWith<P>
// where
//     O: FromIterator<M>,
//     P: Mapper<'a, C, M, M>,
//     C: Context<'a> + Policy<C>,
// {
//     fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
//     where
//         H: Handler<A, Out = M, Error = Error>,
//         A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
//     {
//         let mut g = CtxGuard::new(ctx);

//         Ok(O::from_iter(std::iter::from_fn(|| {
//             let ret = self.pat.map(g.ctx(), func);

//             g.process_ret(ret).ok()
//         })))
//     }
// }

// impl<'a, C, P> Parse<C> for Collect<P>
// where
//     P: Parse<C, Ret = Span>,
//     C: Context<'a> + Policy<C>,
// {
//     type Ret = P::Ret;

//     fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
//         let mut g = CtxGuard::new(ctx);
//         let mut span = g.try_mat(&self.pat)?;

//         while let Ok(ret) = g.try_mat(&self.pat) {
//             span.add_assign(ret);
//         }
//         Ok(span)
//     }
// }
