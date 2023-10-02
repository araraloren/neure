use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::neure::CRange;
use crate::prelude::Ret;
use crate::regex::Regex;

#[derive(Debug, Copy)]
pub struct Repeat<C, P> {
    pat: P,
    range: CRange<usize>,
    capacity: usize,
    marker: PhantomData<C>,
}

impl<C, P> Clone for Repeat<C, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            range: self.range,
            capacity: self.capacity,
            marker: self.marker,
        }
    }
}

impl<C, P> Repeat<C, P> {
    pub fn new(pat: P, range: impl Into<CRange<usize>>) -> Self {
        Self {
            pat,
            range: range.into(),
            capacity: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn range(&self) -> &CRange<usize> {
        &self.range
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_range(&mut self, range: impl Into<CRange<usize>>) -> &mut Self {
        self.range = range.into();
        self
    }

    pub fn set_capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn with_pat(mut self, pat: P) -> Self {
        self.pat = pat;
        self
    }

    pub fn with_range(mut self, range: impl Into<CRange<usize>>) -> Self {
        self.range = range.into();
        self
    }

    pub fn with_capacity(mut self, cap: usize) -> Self {
        self.capacity = cap;
        self
    }

    fn is_contain(&self, count: usize) -> bool {
        match std::ops::RangeBounds::end_bound(&self.range) {
            std::ops::Bound::Included(max) => count < *max,
            std::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, C, P, M, O> Invoke<'a, C, M, O> for Repeat<C, P>
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
        let mut cnt = 0;
        let mut res = Vec::with_capacity(self.capacity);
        let mut g = CtxGuard::new(ctx);

        while self.is_contain(cnt) {
            let ret = self.pat.invoke(g.ctx(), handler);

            res.push(g.process_ret(ret)?);
            cnt += 1;
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(O::from_iter(res))
        } else {
            Err(Error::Repeat)
        }
    }
}

impl<'a, C, P> Regex<C> for Repeat<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut cnt = 0;
        let mut g = CtxGuard::new(ctx);
        let mut ret = Span::new(0, 0);

        while self.is_contain(cnt) {
            ret.add_assign(g.try_mat(&self.pat)?);
            cnt += 1;
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(ret)
        } else {
            Err(Error::Repeat)
        }
    }
}

#[derive(Debug, Copy)]
pub struct TryRepeat<C, P> {
    pat: P,
    range: CRange<usize>,
    capacity: usize,
    marker: PhantomData<C>,
}

impl<C, P> Clone for TryRepeat<C, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            range: self.range,
            capacity: self.capacity,
            marker: self.marker,
        }
    }
}

impl<C, P> TryRepeat<C, P> {
    pub fn new(pat: P, range: impl Into<CRange<usize>>) -> Self {
        Self {
            pat,
            range: range.into(),
            capacity: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn range(&self) -> &CRange<usize> {
        &self.range
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_range(&mut self, range: impl Into<CRange<usize>>) -> &mut Self {
        self.range = range.into();
        self
    }

    pub fn set_capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn with_pat(mut self, pat: P) -> Self {
        self.pat = pat;
        self
    }

    pub fn with_range(mut self, range: impl Into<CRange<usize>>) -> Self {
        self.range = range.into();
        self
    }

    pub fn with_capacity(mut self, cap: usize) -> Self {
        self.capacity = cap;
        self
    }

    fn is_contain(&self, count: usize) -> bool {
        match std::ops::RangeBounds::end_bound(&self.range) {
            std::ops::Bound::Included(max) => count < *max,
            std::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, C, P, M, O> Invoke<'a, C, M, O> for TryRepeat<C, P>
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
        let mut cnt = 0;
        let mut res = Vec::with_capacity(self.capacity);
        let mut g = CtxGuard::new(ctx);

        while self.is_contain(cnt) {
            let ret = self.pat.invoke(g.ctx(), handler);
            let ret = g.process_ret(ret);

            match ret {
                Ok(ret) => {
                    res.push(ret);
                    cnt += 1;
                }
                Err(_) => break,
            }
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(O::from_iter(res))
        } else {
            Err(Error::TryRepeat)
        }
    }
}

impl<'a, C, P> Regex<C> for TryRepeat<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut cnt = 0;
        let mut g = CtxGuard::new(ctx);
        let mut ret = Span::new(0, 0);

        while self.is_contain(cnt) {
            match g.try_mat(&self.pat) {
                Ok(span) => {
                    ret.add_assign(span);
                    cnt += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(ret)
        } else {
            Err(Error::TryRepeat)
        }
    }
}
