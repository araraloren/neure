use super::IndexBySpan;

use crate::ctx::Span;

#[derive(Debug, Clone, Copy)]
pub struct IteratorBySpan<'a, 'b, T: ?Sized> {
    cur: usize,

    value: &'a T,

    spans: &'b Vec<Span>,
}

impl<'a, 'b, T: ?Sized> IteratorBySpan<'a, 'b, T> {
    pub fn new(str: &'a T, spans: &'b Vec<Span>) -> Self {
        Self {
            value: str,
            spans,
            cur: 0,
        }
    }
}

impl<'a, 'b, T> Iterator for IteratorBySpan<'a, 'b, T>
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

impl<'a, 'b, T: ?Sized + IndexBySpan> ExactSizeIterator for IteratorBySpan<'a, 'b, T> {}

pub struct SpanIterator<'a> {
    offset: usize,

    spans: &'a Vec<Span>,
}

impl<'a> SpanIterator<'a> {
    pub fn new(spans: &'a Vec<Span>) -> Self {
        Self { offset: 0, spans }
    }
}

impl<'a> Iterator for SpanIterator<'a> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;

        if offset < self.spans.len() {
            self.offset += 1;
            self.spans.get(offset).copied()
        } else {
            None
        }
    }
}
