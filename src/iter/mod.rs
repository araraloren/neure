mod byte;
mod span;

use crate::span::Span;

pub use self::byte::BytesIndices;
pub use self::span::IteratorBySpan;
pub use self::span::SpanIterator;

pub trait IndexBySpan {
    type Output: ?Sized;

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output>;
}

impl IndexBySpan for str {
    type Output = str;

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}

impl IndexBySpan for &'_ str {
    type Output = str;

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}

impl IndexBySpan for [u8] {
    type Output = [u8];

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}

impl IndexBySpan for &'_ [u8] {
    type Output = [u8];

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}

impl IndexBySpan for Vec<u8> {
    type Output = [u8];

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}

impl IndexBySpan for &'_ Vec<u8> {
    type Output = [u8];

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}
