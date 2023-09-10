use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::err::Error;

#[derive(Debug)]
pub struct CtxGuard<'a, 'b, C>
where
    C: Context<'b> + Policy<C>,
{
    ctx: &'a mut C,

    offset: usize,

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
            marker: PhantomData,
        }
    }

    pub fn beg(&self) -> usize {
        self.offset
    }

    pub fn ctx(&mut self) -> &mut C {
        self.ctx
    }

    pub fn try_mat(&mut self, parser: impl Pattern<C, Ret = C::Ret>) -> Result<C::Ret, Error> {
        self.ctx.try_mat(parser)
    }
}

impl<'a, 'b, C> Drop for CtxGuard<'a, 'b, C>
where
    C: Context<'b> + Policy<C>,
{
    fn drop(&mut self) {
        self.ctx.set_offset(self.offset);
    }
}
