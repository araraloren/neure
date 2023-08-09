use crate::iter::SubStrIter;
use crate::span::SpanStorer;
use crate::{err::Error, iter::CharIter, span::Span};

pub trait CharCtx {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<CharIter<'_>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<CharIter<'_>, Error>;
}

pub trait StrCtx {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<&str, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<&str, Error>;

    fn substr(&self, span: &Span) -> Result<&str, Error> {
        self.peek_at(0)?
            .get(span.beg..(span.beg + span.len))
            .ok_or(Error::SubStr)
    }

    fn substrs<'a>(&self, spans: &'a Vec<Span>) -> Result<SubStrIter<'_, 'a>, Error>
    where
        Self: Sized,
    {
        Ok(SubStrIter::new(self.peek_at(0)?, spans))
    }
}

#[derive(Debug, Default)]
pub struct CharsCtx<'a> {
    str: &'a str,
    offset: usize,
}

impl<'a> CharsCtx<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { str, offset: 0 }
    }

    pub fn with_str(mut self, str: &'a str) -> Self {
        self.str = str;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn reset_with(&mut self, str: &'a str) -> &mut Self {
        self.str = str;
        self.offset = 0;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.offset = 0;
        self
    }

    pub fn len(&self) -> usize {
        self.str.len()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn span_storer(&self, capacity: usize) -> SpanStorer {
        SpanStorer::new(capacity)
    }
}

impl<'a> StrCtx for CharsCtx<'a> {
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

    fn peek_at(&self, offset: usize) -> Result<&str, Error> {
        self.str.get(offset..).ok_or(Error::ReachEnd)
    }
}

impl<'a> CharCtx for CharsCtx<'a> {
    fn len(&self) -> usize {
        self.str.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    /// do nothing
    fn inc(&mut self, _offset: usize) -> &mut Self {
        self
    }

    /// do nothing
    fn dec(&mut self, _offset: usize) -> &mut Self {
        self
    }

    fn peek_at(&self, offset: usize) -> Result<CharIter<'_>, Error> {
        Ok(CharIter::new(StrCtx::peek_at(self, offset)?))
    }
}
