use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::iter::IndexBySpan;
use crate::iter::IteratorBySpan;
use crate::iter::SpanIterator;

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

    pub fn span(&self, id: usize, index: usize) -> Result<&Span, Error> {
        self.spans[id].get(index).ok_or(Error::SpanIndex)
    }

    pub fn spans(&self, id: usize) -> Result<&Vec<Span>, Error> {
        let span = &self.spans[id];

        if !span.is_empty() {
            Ok(span)
        } else {
            Err(Error::SpanID)
        }
    }

    pub fn spans_iter(&self, id: usize) -> Result<SpanIterator<'_>, Error> {
        let span = &self.spans[id];

        if !span.is_empty() {
            Ok(SpanIterator::new(span))
        } else {
            Err(Error::SpanID)
        }
    }
}

impl SimpleStorer {
    pub fn slice<'a, T>(
        &self,
        value: &'a T,
        id: usize,
        index: usize,
    ) -> Result<&'a <T as IndexBySpan>::Output, Error>
    where
        T: IndexBySpan + ?Sized,
    {
        let span = self.span(id, index)?;

        value.get_by_span(span).ok_or(Error::IndexBySpan)
    }

    pub fn slice_iter<'a, T>(
        &self,
        str: &'a T,
        id: usize,
    ) -> Result<IteratorBySpan<'a, '_, T>, Error>
    where
        T: IndexBySpan + ?Sized,
    {
        Ok(IteratorBySpan::new(str, self.spans(id)?))
    }
}

impl SimpleStorer {
    pub fn try_cap<'a, C, P: Parse<C, Ret = Span>>(
        &mut self,
        id: usize,
        ctx: &mut C,
        pat: &P,
    ) -> Result<P::Ret, Error>
    where
        C: Context<'a> + Policy<C>,
    {
        let ret = ctx.try_mat(pat)?;

        self.add_span(id, ret);
        Ok(ret)
    }
}
