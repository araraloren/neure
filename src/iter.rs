use crate::span::Span;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    pub offset: usize,

    pub len: usize,

    pub char: char,
}

#[derive(Debug, Clone)]
pub struct CharIter<'a> {
    len: usize,

    cache: Option<(usize, char)>,

    chars: CharIndices<'a>,
}

impl<'a> CharIter<'a> {
    pub fn new(str: &'a str) -> Self {
        let len = str.len();
        let mut chars = str.char_indices();
        let cache = chars.next();

        Self { len, chars, cache }
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((offset, char)) = self.cache.take() {
            let next = self.chars.next();
            let next_offset = next.map(|v| v.0).unwrap_or(self.len);

            self.cache = next;
            Some(Char {
                char,
                offset,
                len: next_offset - offset,
            })
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for CharIter<'a> {}

#[derive(Debug, Clone, Copy)]
pub struct SubStrIter<'a, 'b> {
    cur: usize,

    str: &'a str,

    spans: &'b Vec<Span>,
}

impl<'a, 'b> SubStrIter<'a, 'b> {
    pub fn new(str: &'a str, spans: &'b Vec<Span>) -> Self {
        Self { str, spans, cur: 0 }
    }
}

impl<'a, 'b> Iterator for SubStrIter<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        self.cur += 1;
        self.spans
            .get(cur)
            .and_then(|v| self.str.get(v.beg..(v.beg + v.len)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.spans.len(), Some(self.spans.len()))
    }
}

impl<'a, 'b> ExactSizeIterator for SubStrIter<'a, 'b> {}
