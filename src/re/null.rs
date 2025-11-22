use std::fmt::Debug;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

use super::def_not;
use super::trace;
use super::Ctor;
use super::Extract;
use super::Handler;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullRegex;

def_not!(NullRegex);

impl NullRegex {
    pub fn new() -> Self {
        Self
    }
}

impl<'a, C> Regex<C> for NullRegex
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let beg = ctx.offset();
        let ret = Ok(Span::new(beg, 0));

        trace!("null", beg => ctx.offset(), ret)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for NullRegex
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let beg = ctx.offset();
        let ret = ctx.try_mat(self);

        trace!("null", beg -> ctx.offset(), ret.is_ok());
        handler.invoke(A::extract(ctx, &ret?)?)
    }
}
