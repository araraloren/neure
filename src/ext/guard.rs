use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::err::Error;

#[derive(Debug)]
pub struct CtxGuard<'a, 'b, C>
where
    C: Context<'b> + Policy<C>,
{
    ctx: &'a mut C,

    offset: usize,

    reset: bool,

    marker: PhantomData<&'b ()>,
}

impl<'a, 'b, C> CtxGuard<'a, 'b, C>
where
    C: Context<'b> + Policy<C>,
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

    pub fn ctx(&mut self) -> &mut C {
        self.ctx
    }

    pub fn process_ret<R>(&mut self, ret: Result<R, Error>) -> Result<R, Error> {
        if ret.is_err() {
            self.reset = true;
        }
        ret
    }

    pub fn try_mat<P: Parse<C>>(&mut self, pattern: &P) -> Result<P::Ret, Error> {
        self.ctx.try_mat(pattern).and_then(|r| {
            self.reset = false;
            Ok(r)
        })
    }
}

impl<'a, 'b, C> Drop for CtxGuard<'a, 'b, C>
where
    C: Context<'b> + Policy<C>,
{
    fn drop(&mut self) {
        if self.reset {
            self.ctx.set_offset(self.offset);
        }
    }
}
