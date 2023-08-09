use crate::{err::Error, iter::SubStrIter};

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

    pub fn substr<'a>(&self, str: &'a str, span_id: usize, index: usize) -> Result<&'a str, Error> {
        let span = self.span(span_id, index)?;

        str.get(span.beg..(span.beg + span.len))
            .ok_or(Error::SubStr)
    }

    pub fn substrs<'a>(&self, str: &'a str, span_id: usize) -> Result<SubStrIter<'a, '_>, Error>
    where
        Self: Sized,
    {
        Ok(SubStrIter::new(str, self.spans(span_id)?))
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
