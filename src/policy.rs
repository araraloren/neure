use crate::bytes::BytesCtx;
use crate::ctx::Context;
use crate::err::Error;
use crate::parser::Parser;
use crate::span::{Span, SpanStore};
use crate::CharsCtx;

/// first is count of char, second is count of byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ret(usize);

impl Ret {
    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<(usize, usize)> for Ret {
    fn from(value: (usize, usize)) -> Self {
        Self(value.1)
    }
}

pub trait MatchPolicy {
    type Ret: From<(usize, usize)>;

    fn try_mat_policy(
        &mut self,
        mut parser: impl Parser<Self, Ret = Self::Ret>,
        mut pre_policy: impl FnMut(&mut Self) -> Result<(), Error>,
        mut post_policy: impl FnMut(&mut Self, Result<Self::Ret, Error>) -> Result<Self::Ret, Error>,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        pre_policy(self)?;
        let ret = parser.try_parse(self);

        post_policy(self, ret)
    }

    fn mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> bool
    where
        Self: Sized,
    {
        self.try_mat(parser).is_ok()
    }

    fn cap(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> bool
    where
        Self: Sized,
    {
        self.try_cap(id, storer, parser).is_ok()
    }

    fn try_mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_reset(parser, false)
    }

    fn try_cap(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_cap_reset(id, storer, parser, false)
    }

    fn try_mat_reset(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized;

    fn try_cap_reset(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized;
}

impl MatchPolicy for CharsCtx<'_> {
    type Ret = Ret;

    fn try_mat_reset(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(
            parser,
            |ctx| {
                if reset {
                    ctx.reset();
                }
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    ctx.inc(ret.offset());
                }
                ret
            },
        )
    }

    fn try_cap_reset(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(
            parser,
            |ctx| {
                if reset {
                    ctx.reset();
                }
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    storer.add_span(
                        id,
                        Span {
                            beg: ctx.offset(),
                            len: ret.offset(),
                        },
                    );
                    ctx.inc(ret.offset());
                }
                ret
            },
        )
    }
}

impl MatchPolicy for BytesCtx<'_> {
    type Ret = Ret;

    fn try_mat_reset(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(
            parser,
            |ctx| {
                if reset {
                    ctx.reset();
                }
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    ctx.inc(ret.offset());
                }
                ret
            },
        )
    }

    fn try_cap_reset(
        &mut self,
        id: usize,
        storer: &mut impl SpanStore,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_policy(
            parser,
            |ctx| {
                if reset {
                    ctx.reset();
                }
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    storer.add_span(
                        id,
                        Span {
                            beg: ctx.offset(),
                            len: ret.offset(),
                        },
                    );
                    ctx.inc(ret.offset());
                }
                ret
            },
        )
    }
}
