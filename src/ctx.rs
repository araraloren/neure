use std::str::CharIndices;

use crate::{err::Error, Parser};

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

pub trait Context {
    type Error: Into<Error>;

    fn len(&self) -> usize;

    fn offset(&self) -> usize;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn add_span(&mut self, id: usize, span: Span) -> &mut Self;

    fn get_span(&self, id: usize) -> Option<&Vec<Span>>;

    fn peek_chars(&self) -> Result<CharIndices<'_>, Error> {
        Ok(self.peek().map_err(Into::into)?.char_indices())
    }

    fn peek_chars_at(&self, offset: usize) -> Result<CharIndices<'_>, Error> {
        Ok(self.peek_at(offset).map_err(Into::into)?.char_indices())
    }

    fn peek_char(&self) -> Result<(usize, char), Error> {
        self.peek()
            .map_err(Into::into)?
            .char_indices()
            .next()
            .ok_or_else(|| Error::Chars)
    }

    fn peek_char_at(&self, offset: usize) -> Result<(usize, char), Error> {
        self.peek_at(offset)
            .map_err(Into::into)?
            .char_indices()
            .next()
            .ok_or_else(|| Error::Chars)
    }

    fn peek(&self) -> Result<&str, Self::Error> {
        self.peek_at(0)
    }

    fn peek_at(&self, offset: usize) -> Result<&str, Self::Error>;

    fn try_match_policy(
        &mut self,
        parser: impl Parser<Self>,
        mut policy: impl FnMut(&mut Self, &Result<usize, Error>),
    ) -> Result<usize, Error>
    where
        Self: Sized,
    {
        let ret = parser.parse(self).map_err(Into::into);

        policy(self, &ret);
        ret
    }

    fn try_match(&mut self, parser: impl Parser<Self>) -> Result<usize, Error>
    where
        Self: Sized,
    {
        self.try_match_policy(parser, |ctx, ret| {
            if let Ok(len) = ret {
                ctx.inc(*len);
            }
        })
    }

    fn try_capture(&mut self, id: usize, parser: impl Parser<Self>) -> Result<usize, Error>
    where
        Self: Sized,
    {
        self.try_match_policy(parser, |ctx, ret| {
            if let Ok(len) = ret {
                ctx.add_span(
                    id,
                    Span {
                        beg: ctx.offset(),
                        len: *len,
                    },
                )
                .inc(*len);
            }
        })
    }

    fn r#match(&mut self, parser: impl Parser<Self>) -> bool
    where
        Self: Sized,
    {
        self.try_match(parser).is_ok()
    }

    fn capture(&mut self, key: usize, parser: impl Parser<Self>) -> bool
    where
        Self: Sized,
    {
        self.try_capture(key, parser).is_ok()
    }
}

#[derive(Debug, Default)]
pub struct CharsCtx {
    str: String,
    offset: usize,
    spans: Vec<Vec<Span>>,
}

impl CharsCtx {
    pub fn new(str: impl Into<String>, capacity: usize) -> Self {
        Self {
            str: str.into(),
            offset: 0,
            spans: vec![vec![]; capacity],
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.spans = vec![vec![]; capacity];
        self
    }

    pub fn substr(&self, span: &Span) -> Result<&str, Error> {
        self.str
            .get(span.beg..(span.beg + span.len))
            .ok_or_else(|| Error::SubStr)
    }

    pub fn has_span(&self, index: usize) -> bool {
        self.spans
            .get(index)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    pub fn reset_with(&mut self, string: impl Into<String>) -> &mut Self {
        self.str = string.into();
        self.offset = 0;
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }
}

impl Context for CharsCtx {
    type Error = Error;

    fn len(&self) -> usize {
        self.str.len()
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

    fn add_span(&mut self, id: usize, span: Span) -> &mut Self {
        self.spans[id].push(span);
        self
    }

    fn get_span(&self, id: usize) -> Option<&Vec<Span>> {
        self.spans.get(id)
    }

    fn peek_at(&self, offset: usize) -> Result<&str, Self::Error> {
        self.str.get(offset..).ok_or_else(|| Error::ReachEnd)
    }
}
