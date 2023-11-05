use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Invoke;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct Terminated<C, P, S> {
    pat: P,
    sep: S,
    skip: bool,
    capacity: usize,
    marker: PhantomData<C>,
}

impl<C, P, S> Clone for Terminated<C, P, S>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            capacity: self.capacity,
            marker: self.marker,
        }
    }
}

impl<C, P, S> Terminated<C, P, S> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
            capacity: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn set_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}

impl<'a, C, S, P, M, O> Invoke<'a, C, M, Vec<O>> for Terminated<C, P, S>
where
    P: Invoke<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<Vec<O>, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut res = Vec::with_capacity(self.capacity);

        loop {
            let ret = self.pat.invoke(ctx, func);

            match ret {
                Ok(ret) => {
                    res.push(ret);

                    if let Err(e) = ctx.try_mat(&self.sep) {
                        if !self.skip {
                            return Err(e);
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(res)
    }
}

impl<'a, C, S, P> Regex<C> for Terminated<C, P, S>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut span = Span::default();

        loop {
            let ret = ctx.try_mat(&self.pat);

            match ret {
                Ok(ret) => {
                    span.add_assign(ret);
                    match ctx.try_mat(&self.sep) {
                        Ok(ret) => {
                            span.add_assign(ret);
                        }
                        Err(e) => {
                            if !self.skip {
                                return Err(e);
                            }
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(span)
    }
}
