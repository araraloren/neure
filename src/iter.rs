use crate::{index::IndexBySpan, span::Span};

#[derive(Debug, Clone, Copy)]
pub struct SpanIterator<'a, 'b, T: ?Sized> {
    cur: usize,

    value: &'a T,

    spans: &'b Vec<Span>,
}

impl<'a, 'b, T: ?Sized> SpanIterator<'a, 'b, T> {
    pub fn new(str: &'a T, spans: &'b Vec<Span>) -> Self {
        Self {
            value: str,
            spans,
            cur: 0,
        }
    }
}

impl<'a, 'b, T> Iterator for SpanIterator<'a, 'b, T>
where
    T: ?Sized + IndexBySpan,
{
    type Item = &'a <T as IndexBySpan>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        self.cur += 1;
        self.spans.get(cur).and_then(|v| self.value.get_by_span(v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.spans.len(), Some(self.spans.len()))
    }
}

impl<'a, 'b, T: ?Sized + IndexBySpan> ExactSizeIterator for SpanIterator<'a, 'b, T> {}

#[derive(Debug, Clone)]
pub struct BytesIndices<'a> {
    offset: usize,

    bytes: &'a [u8],
}

impl<'a> BytesIndices<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { offset: 0, bytes }
    }
}

impl<'a> Iterator for BytesIndices<'a> {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.bytes.len() {
            let offset = self.offset;

            self.offset += 1;
            Some((offset, self.bytes[offset]))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.bytes.len(), Some(self.bytes.len()))
    }
}

impl<'a> ExactSizeIterator for BytesIndices<'a> {}
