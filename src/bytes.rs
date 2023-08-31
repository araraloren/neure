use crate::err::Error;
use crate::iter::BytesIndices;
use crate::parser::Parser;
use crate::policy::Context;
use crate::policy::CtxReference;
use crate::policy::Length;
use crate::policy::MatchPolicy;
use crate::policy::Ret;
use crate::span::Span;
use crate::span::SpanStore;
use crate::span::SpanStorer;

#[derive(Debug, Default)]
pub struct BytesCtx<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> BytesCtx<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn with_str(mut self, bytes: &'a [u8]) -> Self {
        self.bytes = bytes;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn reset_with(&mut self, bytes: &'a [u8]) -> &mut Self {
        self.bytes = bytes;
        self.offset = 0;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.offset = 0;
        self
    }

    pub fn span_storer(&self, capacity: usize) -> SpanStorer {
        SpanStorer::new(capacity)
    }
}

impl<'a> Context for BytesCtx<'a> {
    type Orig = [u8];

    type Item = u8;

    type Iter<'b> = BytesIndices<'b> where Self: 'b;

    fn len(&self) -> usize {
        self.bytes.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&Self::Orig, Error> {
        self.bytes.get(offset..).ok_or(Error::ReachEnd)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'_>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&Self::Orig, Error> {
        self.bytes
            .get(offset..(offset + len))
            .ok_or(Error::ReachEnd)
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

impl<'a> CtxReference<BytesCtx<'a>> for BytesCtx<'a> {
    fn ctx(&self) -> &BytesCtx<'a> {
        self
    }

    fn ctx_mut(&mut self) -> &mut BytesCtx<'a> {
        self
    }
}
