use super::IndexBySpan;

use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct IteratorBySpan<'a, 'b, T: ?Sized> {
    cur: usize,

    value: &'a T,

    spans: &'b [Span],
}

impl<'a, 'b, T: ?Sized> IteratorBySpan<'a, 'b, T> {
    pub fn new(str: &'a T, spans: &'b [Span]) -> Self {
        Self {
            value: str,
            spans,
            cur: 0,
        }
    }
}

impl<'a, T> Iterator for IteratorBySpan<'a, '_, T>
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
        let size = self.spans.len() - self.cur;

        (size, Some(size))
    }
}

impl<T: ?Sized + IndexBySpan> ExactSizeIterator for IteratorBySpan<'_, '_, T> {}

#[derive(Debug, Clone)]
pub struct SpanIterator<'a> {
    offset: usize,

    spans: &'a [Span],
}

impl<'a> SpanIterator<'a> {
    pub fn new(spans: &'a [Span]) -> Self {
        Self { offset: 0, spans }
    }
}

impl Iterator for SpanIterator<'_> {
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

#[derive(Debug, Clone, Copy)]
pub struct IteratorByOptionSpan<'a, 'b, T: ?Sized> {
    cur: usize,

    value: &'a T,

    spans: &'b [Option<Span>],
}

impl<'a, 'b, T: ?Sized> IteratorByOptionSpan<'a, 'b, T> {
    pub fn new(str: &'a T, spans: &'b [Option<Span>]) -> Self {
        Self {
            value: str,
            spans,
            cur: 0,
        }
    }
}

impl<'a, T> Iterator for IteratorByOptionSpan<'a, '_, T>
where
    T: ?Sized + IndexBySpan,
{
    type Item = &'a <T as IndexBySpan>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        self.cur += 1;
        self.spans
            .get(cur)
            .and_then(|v| v.as_ref())
            .and_then(|v| self.value.get_by_span(v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.spans.len() - self.cur;

        (size, Some(size))
    }
}

impl<T: ?Sized + IndexBySpan> ExactSizeIterator for IteratorByOptionSpan<'_, '_, T> {}

#[derive(Debug, Clone)]
pub struct OptionSpanIterator<'a> {
    offset: usize,

    spans: &'a [Option<Span>],
}

impl<'a> OptionSpanIterator<'a> {
    pub fn new(spans: &'a [Option<Span>]) -> Self {
        Self { offset: 0, spans }
    }
}

impl Iterator for OptionSpanIterator<'_> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;

        if offset < self.spans.len() {
            self.offset += 1;
            self.spans.get(offset).and_then(|v| v.as_ref()).copied()
        } else {
            None
        }
    }
}
