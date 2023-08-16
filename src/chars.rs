use std::str::CharIndices;

use crate::ctx::Context;
use crate::err::Error;
use crate::span::SpanStorer;

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

    pub fn span_storer(&self, capacity: usize) -> SpanStorer {
        SpanStorer::new(capacity)
    }
}

impl<'a> Context for CharsCtx<'a> {
    type Orig = str;

    type Item = char;

    type Iter<'b> = CharIndices<'b> where Self: 'b;

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

    fn orig_at(&self, offset: usize) -> Result<&str, Error> {
        self.str.get(offset..).ok_or(Error::ReachEnd)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'_>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&Self::Orig, Error> {
        self.str.get(offset..(offset + len)).ok_or(Error::ReachEnd)
    }
}
