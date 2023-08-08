use crate::{err::Error, peek::StrPeek, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    pub offset: usize,

    pub len: usize,

    pub char: char,
}

#[derive(Debug, Clone, Copy)]
pub struct CharIter<'a> {
    cur: usize,
    chars: &'a [Char],
}

impl<'a> CharIter<'a> {
    pub fn new(chars: &'a [Char]) -> Self {
        Self { chars, cur: 0 }
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = &'a Char;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        self.cur += 1;
        self.chars.get(cur)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.chars.len(), Some(self.chars.len()))
    }
}

impl<'a> ExactSizeIterator for CharIter<'a> {}

#[derive(Debug, Clone, Copy)]
pub struct SubStrIter<'a, T> {
    cur: usize,

    ctx: &'a T,

    spans: &'a Vec<Span>,
}

impl<'a, T> SubStrIter<'a, T> {
    pub fn new(ctx: &'a T, spans: &'a Vec<Span>) -> Self {
        Self { ctx, spans, cur: 0 }
    }
}

impl<'a, T> Iterator for SubStrIter<'a, T>
where
    T: StrPeek,
{
    type Item = Result<&'a str, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        self.cur += 1;
        self.spans.get(cur).map(|v| self.ctx.substr(v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.spans.len(), Some(self.spans.len()))
    }
}

impl<'a, T> ExactSizeIterator for SubStrIter<'a, T> where T: StrPeek {}
