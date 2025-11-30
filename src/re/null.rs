use std::fmt::Debug;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Regex;

use super::def_not;
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
        crate::debug_regex_reval!("NullRegex", Ok(Span::new(ctx.offset(), 0)))
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
        let ret = ctx.try_mat(self);

        handler.invoke(A::extract(ctx, &ret?)?)
    }
}
