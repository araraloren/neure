use crate::bytes::BytesCtx;
use crate::ctx::Context;
use crate::err::Error;
use crate::parser::Parser;
use crate::span::{Span, SpanStore};
use crate::CharsCtx;

pub trait Ret {
    fn count(&self) -> usize;

    fn length(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn new_from(ret: (usize, usize)) -> Self;
}

/// first is count of char, second is count of byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(usize);

impl Ret for Length {
    fn count(&self) -> usize {
        0
    }

    fn length(&self) -> usize {
        self.0
    }

    fn is_zero(&self) -> bool {
        self.length() == 0
    }

    fn new_from(ret: (usize, usize)) -> Self {
        Self(ret.1)
    }
}

pub trait MatchPolicy {
    type Ret: Ret;

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

    fn is_mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> bool
    where
        Self: Sized,
    {
        self.try_mat(parser).is_ok()
    }

    fn is_cap<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> bool
    where
        Self: Sized,
        S: SpanStore,
    {
        self.try_cap(id, storer, parser).is_ok()
    }

    fn try_mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_reset(parser, false)
    }

    fn try_cap<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore,
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

    fn try_cap_reset<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore;
}

impl MatchPolicy for CharsCtx<'_> {
    type Ret = Length;

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
                    ctx.inc(ret.length());
                }
                ret
            },
        )
    }

    fn try_cap_reset<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore,
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
                            len: ret.length(),
                        },
                    );
                    ctx.inc(ret.length());
                }
                ret
            },
        )
    }
}

impl MatchPolicy for BytesCtx<'_> {
    type Ret = Length;

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
                    ctx.inc(ret.length());
                }
                ret
            },
        )
    }

    fn try_cap_reset<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore,
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
                            len: ret.length(),
                        },
                    );
                    ctx.inc(ret.length());
                }
                ret
            },
        )
    }
}
