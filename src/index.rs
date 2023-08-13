use crate::span::Span;

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

impl<'a> IndexBySpan for &'a str {
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

impl<'a> IndexBySpan for &'a [u8] {
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

impl<'a> IndexBySpan for &'a Vec<u8> {
    type Output = [u8];

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        self.get(span.beg..(span.beg + span.len))
    }
}
