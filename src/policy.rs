use crate::ctx::{CharCtx, StrCtx};
use crate::err::Error;
use crate::parser::Parser;
use crate::span::{Span, SpanStore};

/// first is count of char, second is count of byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ret {
    pub char: usize,

    pub byte: usize,
}

impl Ret {
    pub fn is_zero(&self) -> bool {
        self.char == 0 && self.byte == 0
    }
}

impl From<(usize, usize)> for Ret {
    fn from(value: (usize, usize)) -> Self {
        Self {
            char: value.0,
            byte: value.1,
        }
    }
}

pub trait MatchPolicy {
    fn try_mat_policy(
        &mut self,
        mut parser: impl Parser<Self>,
        mut policy: impl FnMut(&mut Self, &Result<Ret, Error>),
    ) -> Result<Ret, Error>
    where
        Self: Sized,
    {
        let ret = parser.try_parse(self);

        policy(self, &ret);
        ret
    }

    fn try_mat(&mut self, parser: impl Parser<Self>) -> Result<Ret, Error>
    where
        Self: Sized;

    fn try_cap(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self>,
    ) -> Result<Ret, Error>
    where
        Self: Sized;

    fn mat(&mut self, parser: impl Parser<Self>) -> bool
    where
        Self: Sized,
    {
        self.try_mat(parser).is_ok()
    }

    fn cap(&mut self, id: usize, storer: &mut impl SpanStore, parser: impl Parser<Self>) -> bool
    where
        Self: Sized,
    {
        self.try_cap(id, storer, parser).is_ok()
    }
}

impl<T> MatchPolicy for T
where
    T: CharCtx + StrCtx,
{
    fn try_mat(&mut self, parser: impl Parser<Self>) -> Result<Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(parser, |ctx, ret| {
            if let Ok(ret) = ret {
                CharCtx::inc(ctx, ret.char);
                StrCtx::inc(ctx, ret.byte);
            }
        })
    }

    fn try_cap(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self>,
    ) -> Result<Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(parser, |ctx, ret| {
            if let Ok(ret) = ret {
                storer.add_span(
                    id,
                    Span {
                        beg: StrCtx::offset(ctx),
                        len: ret.byte,
                    },
                );
                CharCtx::inc(ctx, ret.char);
                StrCtx::inc(ctx, ret.byte);
            }
        })
    }
}
