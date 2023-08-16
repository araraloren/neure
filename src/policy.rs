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

impl From<Ret> for (usize, usize) {
    fn from(value: Ret) -> Self {
        (0, value.0)
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

pub trait PolicyExtension: MatchPolicy + Context
where
    Self: Sized,
{
    fn map<R>(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        mut map: impl FnMut(&Self, usize, Self::Ret) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let start = self.offset();
        let ret = self.try_mat(parser);

        if ret.is_ok() {
            map(self, start, ret?)
        } else {
            Err(ret.err().unwrap())
        }
    }

    fn map_orig<R>(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        mut map: impl FnMut(&<Self as Context>::Orig) -> Result<R, Error>,
    ) -> Result<R, Error>
    where
        Self::Ret: Into<(usize, usize)>,
    {
        let start = self.offset();
        let ret = self.try_mat(parser);

        if ret.is_ok() {
            let (_, len) = ret?.into();

            map(self.orig_sub(start, len)?)
        } else {
            Err(ret.err().unwrap())
        }
    }

    fn quote_cont<R>(
        &mut self,
        left: impl Parser<Self, Ret = Self::Ret>,
        right: impl Parser<Self, Ret = Self::Ret>,
        mut cont: impl FnMut(&mut Self) -> Result<R, Error>,
    ) -> Result<R, Error> {
        if self.mat(left) {
            let ret = cont(self);

            self.try_mat(right)?;
            ret
        } else {
            Err(Error::Quote)
        }
    }

    fn quote<R>(
        &mut self,
        left: impl Parser<Self, Ret = Self::Ret>,
        right: impl Parser<Self, Ret = Self::Ret>,
        cont: impl Parser<Self, Ret = Self::Ret>,
        mut map: impl FnMut(&Self, usize, Self::Ret) -> Result<R, Error>,
    ) -> Result<R, Error> {
        if self.mat(left) {
            let start = self.offset();
            let ret = self.try_mat(cont)?;
            let ret = map(self, start, ret)?;

            self.try_mat(right)?;
            Ok(ret)
        } else {
            Err(Error::Quote)
        }
    }
}

impl<T: MatchPolicy + Context> PolicyExtension for T {}

// pub fn terminated<C>(
//     cont: impl Fn(&mut C) -> Result<C::Ret, Error>,
//     sep: impl Fn(&mut C) -> Result<C::Ret, Error>,
//     min: usize,
//     sep_need: bool,
// ) -> impl Fn(&mut C) -> Result<C::Ret, Error>
// where
//     C: Context + MatchPolicy,
// {
//     move |ctx: &mut C| {
//         let ret = cont(ctx);

//         if min == 0 && ret.is_err() {
//             return Ok()
//         }

//         Ok(ret)
//     }
// }
