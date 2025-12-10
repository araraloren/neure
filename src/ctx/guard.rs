use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

#[derive(Debug)]
pub struct CtxGuard<'a, 'b, C>
where
    C: Context<'b>,
{
    ctx: &'a mut C,

    offset: usize,

    reset: bool,

    marker: PhantomData<&'b ()>,
}

impl<'a, 'b, C> CtxGuard<'a, 'b, C>
where
    C: Context<'b>,
{
    pub fn new(ctx: &'a mut C) -> Self {
        let offset = ctx.offset();

        Self {
            ctx,
            offset,
            reset: false,
            marker: PhantomData,
        }
    }

    pub fn beg(&self) -> usize {
        self.offset
    }

    pub fn end(&self) -> usize {
        self.ctx.offset()
    }

    pub(crate) fn remaining_len(&self) -> usize {
        self.ctx.len() - self.beg()
    }

    pub fn len(&self) -> usize {
        self.ctx.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ctx.is_empty()
    }

    pub fn ctx(&mut self) -> &mut C {
        self.ctx
    }

    // Return `Span { beg: self.beg(), len }` and incrment the offset of `C`
    pub fn inc(&mut self, len: usize) -> Span {
        let span = Span::new(self.beg(), len);

        self.ctx.inc(len);
        span
    }

    pub fn peek(&self) -> Result<<C as Context<'b>>::Iter<'b>, Error> {
        self.ctx.peek()
    }

    pub fn peek_at(&self, offset: usize) -> Result<<C as Context<'b>>::Iter<'b>, Error> {
        self.ctx.peek_at(offset)
    }

    pub fn reset(&mut self) -> &mut Self {
        self.ctx.set_offset(self.offset);
        self
    }

    pub fn process_ret<R>(&mut self, ret: Result<R, Error>) -> Result<R, Error> {
        if ret.is_err() {
            self.reset = true;
        }
        ret
    }
}

impl<'a, C> CtxGuard<'_, 'a, C>
where
    C: Match<'a>,
{
    pub fn try_mat<P: Regex<C> + ?Sized>(&mut self, pattern: &P) -> Result<Span, Error> {
        self.ctx.try_mat(pattern).inspect(|_| {
            self.reset = false;
        })
    }
}

impl<'b, C> Drop for CtxGuard<'_, 'b, C>
where
    C: Context<'b>,
{
    fn drop(&mut self) {
        if self.reset {
            self.ctx.set_offset(self.offset);
        }
    }
}
