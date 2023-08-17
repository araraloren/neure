use crate::{
    err::Error,
    index::IndexBySpan,
    iter::{IteratorBySpan, SpanIterator},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

pub trait SpanStore {
    type Id: Copy;

    type Index;

    type Iter<'a>: Iterator<Item = Span>
    where
        Self: 'a;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn reset(&mut self) -> &mut Self;

    fn contain(&self, id: Self::Id) -> bool;

    fn clr_span(&mut self, id: Self::Id) -> &mut Self;

    fn add_span(&mut self, id: Self::Id, span: Span) -> &mut Self;

    fn spans(&self, span_id: Self::Id) -> Result<Self::Iter<'_>, Error>;

    fn span(&self, span_id: Self::Id, index: Self::Index) -> Result<Span, Error>;

    fn slice<'a, T: IndexBySpan + ?Sized>(
        &self,
        value: &'a T,
        id: <Self as SpanStore>::Id,
        index: <Self as SpanStore>::Index,
    ) -> Result<&'a <T as IndexBySpan>::Output, Error>;

    fn slice_iter<'a, T: IndexBySpan + ?Sized>(
        &self,
        str: &'a T,
        id: <Self as SpanStore>::Id,
    ) -> Result<IteratorBySpan<'a, '_, T>, Error>;
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

    pub fn get_spans(&self, id: <Self as SpanStore>::Id) -> Result<&Vec<Span>, Error> {
        let span = &self.spans[id];

        if !span.is_empty() {
            Ok(span)
        } else {
            Err(Error::SpanID)
        }
    }

    pub fn substr<'a>(
        &self,
        str: &'a str,
        id: <Self as SpanStore>::Id,
        index: <Self as SpanStore>::Index,
    ) -> Result<&'a str, Error> {
        let span = self.span(id, index)?;

        str.get(span.beg..(span.beg + span.len))
            .ok_or(Error::IndexBySpan)
    }

    pub fn substrs<'a>(
        &self,
        str: &'a str,
        id: <Self as SpanStore>::Id,
    ) -> Result<IteratorBySpan<'a, '_, str>, Error>
    where
        Self: Sized,
    {
        Ok(IteratorBySpan::new(str, self.get_spans(id)?))
    }
}

impl SpanStore for SpanStorer {
    type Id = usize;

    type Index = usize;

    type Iter<'a> = SpanIterator<'a>;

    fn len(&self) -> usize {
        self.spans.len()
    }

    fn add_span(&mut self, id: usize, span: Span) -> &mut Self {
        self.spans[id].push(span);
        self
    }

    fn reset(&mut self) -> &mut Self {
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }

    fn contain(&self, id: usize) -> bool {
        self.spans.get(id).map(|v| !v.is_empty()).unwrap_or(false)
    }

    fn spans(&self, id: usize) -> Result<SpanIterator<'_>, Error> {
        let span = &self.spans[id];

        if !span.is_empty() {
            Ok(SpanIterator::new(&span))
        } else {
            Err(Error::SpanID)
        }
    }

    fn clr_span(&mut self, id: Self::Id) -> &mut Self {
        self.spans.get_mut(id).map(|v| v.clear());
        self
    }

    fn span(&self, id: Self::Id, index: Self::Index) -> Result<Span, Error> {
        let mut iter = self.spans(id)?;

        iter.nth(index).ok_or_else(|| Error::SpanIndex)
    }

    fn slice<'a, T: IndexBySpan + ?Sized>(
        &self,
        value: &'a T,
        id: <Self as SpanStore>::Id,
        index: <Self as SpanStore>::Index,
    ) -> Result<&'a <T as IndexBySpan>::Output, Error> {
        let span = self.span(id, index)?;

        value.get_by_span(&span).ok_or(Error::IndexBySpan)
    }

    fn slice_iter<'a, T: IndexBySpan + ?Sized>(
        &self,
        str: &'a T,
        id: <Self as SpanStore>::Id,
    ) -> Result<IteratorBySpan<'a, '_, T>, Error> {
        Ok(IteratorBySpan::new(str, self.get_spans(id)?))
    }
}
