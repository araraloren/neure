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
    mini_size: usize,
    marker: PhantomData<(O, C)>,
}

impl<C, P, O> Clone for Collect<C, P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            mini_size: self.mini_size,
            marker: self.marker,
        }
    }
}

impl<C, P, O> Collect<C, P, O> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            mini_size: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn mini_size(&self) -> usize {
        self.mini_size
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_mini_size(&mut self, mini_size: usize) -> &mut Self {
        self.mini_size = mini_size;
        self
    }

    pub fn at_least(mut self, mini_size: usize) -> Self {
        self.mini_size = mini_size;
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
        let mut cnt = 0;
        let ret = V::from_iter(std::iter::from_fn(|| {
            cnt += 1;
            self.pat.constrct(ctx, func).ok()
        }));

        if cnt >= self.mini_size {
            Ok(ret)
        } else {
            Err(Error::Collect)
        }
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
        let mut span = g.try_mat(&self.pat)?;

        while let Ok(ret) = g.try_mat(&self.pat) {
            span.add_assign(ret);
        }
        if span.len >= self.mini_size {
            Ok(span)
        } else {
            Err(Error::Collect)
        }
    }
}
