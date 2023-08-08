use crate::{
    err::Error,
    iter::{Char, CharIter},
    peek::{CharPeek, Span, StrPeek},
};

#[derive(Debug, Default)]
pub struct CharsCtx<'a> {
    str: &'a str,
    byte: usize,
    char: usize,
    chars: Vec<Char>,
    spans: Vec<Vec<Span>>,
}

impl<'a> CharsCtx<'a> {
    pub fn new(str: &'a str, capacity: usize) -> Self {
        let mut chars = Vec::with_capacity(str.len());

        Self::chars_from_str(&str, &mut chars);
        Self {
            str,
            byte: 0,
            char: 0,
            chars,
            spans: vec![vec![]; capacity],
        }
    }

    fn chars_from_str(str: &str, chars: &mut Vec<Char>) {
        chars.clear();
        for (offset, char) in str.char_indices() {
            chars.push(Char {
                offset,
                len: 0,
                char,
            });
        }
        for idx in 0..chars.len() {
            let next_offset = chars.get(idx + 1).map(|v| v.offset).unwrap_or(str.len());
            chars[idx].len = next_offset - chars[idx].offset;
        }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.byte = offset;
        self
    }

    pub fn with_str(mut self, str: &'a str) -> Self {
        self.str = str;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.spans = vec![vec![]; capacity];
        self
    }

    pub fn reset_with(&mut self, str: &'a str) -> &mut Self {
        self.str = str;
        Self::chars_from_str(&self.str, &mut self.chars);
        self.byte = 0;
        self.char = 0;
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.byte = 0;
        self.char = 0;
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }
}

impl<'a> StrPeek for CharsCtx<'a> {
    fn len(&self) -> usize {
        self.str.len()
    }

    fn offset(&self) -> usize {
        self.byte
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.byte += offset;
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.byte -= offset;
        self
    }

    fn add_span(&mut self, id: usize, span: Span) -> &mut Self {
        self.spans[id].push(span);
        self
    }

    fn contain(&self, id: usize) -> bool {
        self.spans.get(id).map(|v| !v.is_empty()).unwrap_or(false)
    }

    fn spans(&self, id: usize) -> Result<&Vec<Span>, Error> {
        if let Some(span) = self.spans.get(id) {
            if !span.is_empty() {
                return Ok(span);
            }
        }
        Err(Error::Null)
    }

    fn spans_mut(&mut self, id: usize) -> Result<&mut Vec<Span>, crate::err::Error> {
        if let Some(span) = self.spans.get_mut(id) {
            if !span.is_empty() {
                return Ok(span);
            }
        }
        Err(Error::Null)
    }

    fn peek_at(&self, offset: usize) -> Result<&str, Error> {
        self.str.get(offset..).ok_or(Error::ReachEnd)
    }
}

impl<'a> CharPeek for CharsCtx<'a> {
    fn len(&self) -> usize {
        self.chars.len()
    }

    fn offset(&self) -> usize {
        self.char
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.char += offset;
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.char -= offset;
        self
    }

    fn peek_at(&self, offset: usize) -> Result<CharIter<'_>, Error> {
        Ok(CharIter::new(
            self.chars.get(offset..).ok_or_else(|| Error::Null)?,
        ))
    }
}
