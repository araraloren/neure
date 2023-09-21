use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::err::Error;
use crate::parser::Context;
use crate::parser::Policy;
use crate::parser::Span;
use crate::prelude::Ret;
use crate::regex::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Repeat<P> {
    pat: P,
    times: usize,
}

impl<P> Repeat<P> {
    pub fn new(pat: P, times: usize) -> Self {
        Self { pat, times }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn times(&self) -> usize {
        self.times
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_times(&mut self, times: usize) -> &mut Self {
        self.times = times;
        self
    }
}

impl<'a, C, P, M, O> Invoke<'a, C, M, O> for Repeat<P>
where
    O: FromIterator<M>,
    P: Invoke<'a, C, M, M>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ok(O::from_iter(
            std::iter::repeat(self.times)
                .map(|_| self.pat.invoke(ctx, handler))
                .collect::<Result<Vec<M>, _>>()?,
        ))
    }
}

impl<'a, C, P> Regex<C> for Repeat<P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Span::new(0, 0);

        for _ in 0..self.times {
            ret.add_assign(g.try_mat(&self.pat)?);
        }
        Ok(ret)
    }
}

pub struct TryRepeat<P> {
    pat: P,
    times: usize,
}

impl<P> TryRepeat<P> {
    pub fn new(pat: P, times: usize) -> Self {
        Self { pat, times }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn times(&self) -> usize {
        self.times
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_times(&mut self, times: usize) -> &mut Self {
        self.times = times;
        self
    }
}

impl<'a, C, P, M, O> Invoke<'a, C, M, O> for TryRepeat<P>
where
    O: FromIterator<M>,
    P: Invoke<'a, C, M, M>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut count = 0;

        Ok(O::from_iter(std::iter::from_fn(|| {
            count += 1;
            if count > self.times {
                None
            } else {
                self.pat.invoke(ctx, handler).ok()
            }
        })))
    }
}

impl<'a, C, P> Regex<C> for TryRepeat<P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Span::new(0, 0);

        for _ in 0..self.times {
            if let Ok(span) = g.try_mat(&self.pat) {
                ret.add_assign(span);
            } else {
                break;
            }
        }
        Ok(ret)
    }
}
