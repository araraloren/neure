use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::Regex;

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

    pub fn ctx(&mut self) -> &mut C {
        self.ctx
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

impl<'b, C> CtxGuard<'_, 'b, C>
where
    C: Context<'b> + Match<C>,
{
    pub fn try_mat<P: Regex<C>>(&mut self, pattern: &P) -> Result<P::Ret, Error> {
        self.ctx.try_mat_t(pattern).inspect(|_| {
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
