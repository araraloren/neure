use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::iter::IndexBySpan;
use crate::iter::IteratorBySpan;
use crate::iter::SpanIterator;
use crate::regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct SimpleStorer {
    spans: Vec<Vec<Span>>,
}

impl SimpleStorer {
    pub fn new(capacity: usize) -> Self {
        Self {
            spans: vec![vec![]; capacity],
        }
    }

    pub fn new_with(spans: Vec<Vec<Span>>) -> Self {
        Self { spans }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.spans = vec![vec![]; capacity];
        self
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reset(&mut self) -> &mut Self {
        self.spans.iter_mut().for_each(|v| v.clear());
        self
    }
}

impl SimpleStorer {
    pub fn contain(&self, id: usize) -> bool {
        self.spans.get(id).map(|v| !v.is_empty()).unwrap_or(false)
    }

    pub fn add_span(&mut self, id: usize, span: Span) -> &mut Self {
        self.spans[id].push(span);
        self
    }

    pub fn clr_span(&mut self, id: usize) -> &mut Self {
        if let Some(v) = self.spans.get_mut(id) {
            v.clear()
        };
        self
    }

    pub fn span(&self, id: usize, index: usize) -> Option<&Span> {
        self.spans.get(id).and_then(|v| v.get(index))
    }

    pub fn spans(&self, id: usize) -> Option<&Vec<Span>> {
        self.spans
            .get(id)
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
    }

    pub fn spans_iter(&self, id: usize) -> Option<SpanIterator<'_>> {
        self.spans(id).map(SpanIterator::new)
    }
}

impl SimpleStorer {
    pub fn slice<'a, T>(
        &self,
        value: &'a T,
        id: usize,
        index: usize,
    ) -> Option<&'a <T as IndexBySpan>::Output>
    where
        T: IndexBySpan + ?Sized,
    {
        let span = self.span(id, index)?;

        value.get_by_span(span)
    }

    pub fn slice_iter<'a, T>(&self, str: &'a T, id: usize) -> Option<IteratorBySpan<'a, '_, T>>
    where
        T: IndexBySpan + ?Sized,
    {
        Some(IteratorBySpan::new(str, self.spans(id)?))
    }
}

impl SimpleStorer {
    pub fn try_cap<'a, C, P>(&mut self, id: usize, ctx: &mut C, pat: &P) -> Result<Span, Error>
    where
        P: Regex<C>,
        C: Match<'a>,
    {
        let ret = ctx.try_mat(pat)?;

        self.add_span(id, ret);
        Ok(ret)
    }
}
