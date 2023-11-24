use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::prelude::Ret;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct Collect<C, P, O> {
    pat: P,
    min: usize,
    marker: PhantomData<(O, C)>,
}

impl<C, P, O> Clone for Collect<C, P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, O> Collect<C, P, O> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            min: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, P, M, O, V> Ctor<'a, C, M, V> for Collect<C, P, O>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let ret = V::from_iter(std::iter::from_fn(|| {
            match self.pat.constrct(g.ctx(), func) {
                Ok(ret) => {
                    cnt += 1;
                    Some(ret)
                }
                Err(_) => None,
            }
        }));

        g.process_ret(if cnt >= self.min {
            Ok(ret)
        } else {
            Err(Error::Collect)
        })
    }
}

impl<'a, C, P, O> Regex<C> for Collect<C, P, O>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = <Span as Ret>::from(g.ctx(), (0, 0));

        // don't use g.try_mat
        while let Ok(ret) = g.ctx().try_mat(&self.pat) {
            span.add_assign(ret);
        }
        g.process_ret(if span.len >= self.min {
            Ok(span)
        } else {
            Err(Error::Collect)
        })
    }
}
