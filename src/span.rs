use crate::{err::Error, index::IndexBySpan, iter::SpanIterator};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

pub trait SpanStore {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn reset(&mut self) -> &mut Self;

    fn contain(&self, span_id: usize) -> bool;

    fn add_span(&mut self, span_id: usize, span: Span) -> &mut Self;

    fn spans(&self, span_id: usize) -> Result<&Vec<Span>, Error>;

    fn spans_mut(&mut self, id: usize) -> Result<&mut Vec<Span>, Error>;

    fn span(&self, span_id: usize, index: usize) -> Result<&Span, Error> {
        self.spans(span_id)
            .and_then(|v| v.get(index).ok_or(Error::SpanIndex))
    }

    fn span_mut(&mut self, span_id: usize, index: usize) -> Result<&mut Span, Error> {
        self.spans_mut(span_id)
            .and_then(|v| v.get_mut(index).ok_or(Error::SpanIndex))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SpanStorer {
    spans: Vec<Vec<Span>>,
}

impl SpanStorer {
    pub fn new(capacity: usize) -> Self {
        Self {
            spans: vec![vec![]; capacity],
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.spans = vec![vec![]; capacity];
        self
    }

    pub fn substr<'a>(&self, str: &'a str, id: usize, index: usize) -> Result<&'a str, Error> {
        let span = self.span(id, index)?;

        str.get(span.beg..(span.beg + span.len))
            .ok_or(Error::IndexBySpan)
    }

    pub fn substrs<'a>(&self, str: &'a str, id: usize) -> Result<SpanIterator<'a, '_, str>, Error>
    where
        Self: Sized,
    {
        Ok(SpanIterator::new(str, self.spans(id)?))
    }

    pub fn slice<'a, T: IndexBySpan>(
        &self,
        value: &'a T,
        id: usize,
        index: usize,
    ) -> Result<&'a <T as IndexBySpan>::Output, Error> {
        let span = self.span(id, index)?;

        value.get_by_span(span).ok_or(Error::IndexBySpan)
    }

    pub fn slice_iter<'a, T: IndexBySpan>(
        &self,
        str: &'a T,
        span_id: usize,
    ) -> Result<SpanIterator<'a, '_, T>, Error> {
        Ok(SpanIterator::new(str, self.spans(span_id)?))
    }
}

impl SpanStore for SpanStorer {
    fn len(&self) -> usize {
        self.spans.len()
    }

    fn add_span(&mut self, span_id: usize, span: Span) -> &mut Self {
        self.spans[span_id].push(span);
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }

    fn contain(&self, span_id: usize) -> bool {
        self.spans
            .get(span_id)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    fn spans(&self, span_id: usize) -> Result<&Vec<Span>, Error> {
        let span = &self.spans[span_id];

        if !span.is_empty() {
            Ok(span)
        } else {
            Err(Error::SpanID)
        }
    }

    fn spans_mut(&mut self, span_id: usize) -> Result<&mut Vec<Span>, crate::err::Error> {
        let span = &mut self.spans[span_id];

        if !span.is_empty() {
            Ok(span)
        } else {
            Err(Error::SpanID)
        }
    }
}
