use crate::{
    err::Error,
    iter::{CharIter, SubStrIter},
};

pub trait CharPeek {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

pub trait StrPeek {
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

    fn add_span(&mut self, id: usize, span: Span) -> &mut Self;

    fn contain(&self, id: usize) -> bool;

    fn spans(&self, id: usize) -> Result<&Vec<Span>, Error>;

    fn spans_mut(&mut self, id: usize) -> Result<&mut Vec<Span>, Error>;

    fn span(&self, id: usize, index: usize) -> Result<&Span, Error> {
        self.spans(id)
            .and_then(|v| v.get(index).ok_or_else(|| Error::Null))
    }

    fn span_mut(&mut self, id: usize, index: usize) -> Result<&mut Span, Error> {
        self.spans_mut(id)
            .and_then(|v| v.get_mut(index).ok_or_else(|| Error::Null))
    }

    fn substr_iter(&self, id: usize) -> Result<SubStrIter<'_, Self>, Error>
    where
        Self: Sized,
    {
        self.spans(id).map(|v| SubStrIter::new(self, v))
    }

    fn substr(&self, span: &Span) -> Result<&str, Error> {
        self.peek_at(0)?
            .get(span.beg..(span.beg + span.len))
            .ok_or(Error::SubStr)
    }

    fn substr_of(&self, id: usize, index: usize) -> Result<&str, Error> {
        self.substr(self.span(id, index)?)
    }
}
