use std::fmt::Debug;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;

use super::def_not;

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

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for NullRegex
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self);

        handler.invoke(ctx, &ret?).map_err(Into::into)
    }
}
