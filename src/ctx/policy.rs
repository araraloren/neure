use super::Context;
use super::PolicyMatch;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::iter::IndexBySpan;

#[derive(Debug)]
pub struct PolicyCtx<I, R> {
    pub(crate) inner: I,
    pub(crate) regex: R,
}

impl<I, R> Clone for PolicyCtx<I, R>
where
    I: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            regex: self.regex.clone(),
        }
    }
}

impl<I, R> Copy for PolicyCtx<I, R>
where
    R: Copy,
    I: Copy,
{
}

impl<I, R> PolicyCtx<I, R> {
    pub fn new(inner: I, regex: R) -> Self {
        Self { inner, regex }
    }

    pub fn with_regex<O>(self, regex: O) -> PolicyCtx<I, O> {
        PolicyCtx {
            inner: self.inner,
            regex,
        }
    }

    pub fn inner(&self) -> &I {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.inner
    }

    pub fn set_inner(&mut self, dat: I) -> &mut Self {
        self.inner = dat;
        self
    }

    pub fn with_inner(mut self, dat: I) -> Self {
        self.inner = dat;
        self
    }

    pub fn reset_with(&mut self, dat: I) -> &mut Self {
        self.inner = dat;
        self
    }

    #[cfg(feature = "alloc")]
    pub fn span_storer(&self, capacity: usize) -> crate::span::VecStorer {
        crate::span::VecStorer::new(capacity)
    }
}

impl<'a, I, R> Context<'a> for PolicyCtx<I, R>
where
    R: Clone + 'a,
    I: Context<'a>,
{
    type Orig<'b> = <I as Context<'a>>::Orig<'b>;

    type Item = <I as Context<'a>>::Item;

    type Iter<'b>
        = <I as Context<'a>>::Iter<'b>
    where
        Self: 'b;

    fn len(&self) -> usize {
        Context::len(&self.inner)
    }

    fn offset(&self) -> usize {
        Context::offset(&self.inner)
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        Context::set_offset(&mut self.inner, offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        Context::inc(&mut self.inner, offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        Context::dec(&mut self.inner, offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        Context::orig_at(&self.inner, offset)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Context::peek_at(&self.inner, offset)
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error> {
        Context::orig_sub(&self.inner, offset, len)
    }

    fn clone_at(&self, offset: usize) -> Result<Self, Error> {
        Ok(PolicyCtx {
            inner: I::clone_at(&self.inner, offset)?,
            regex: self.regex.clone(),
        })
    }
}

impl<'a, I, R> Match<'a> for PolicyCtx<I, R>
where
    R: Regex<I>,
    I: Context<'a>,
    Self: Context<'a>,
{
    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<PolicyCtx<I, R>> + ?Sized,
    {
        self.regex.try_parse(&mut self.inner)?;
        pat.try_parse(self)
    }
}

impl<'a, I, R> PolicyMatch<'a> for PolicyCtx<I, R>
where
    R: Regex<I>,
    I: Context<'a>,
    Self: Context<'a>,
{
    fn try_mat_policy<P, B, A>(&mut self, pat: &P, before: &B, after: &A) -> Result<Span, Error>
    where
        P: Regex<PolicyCtx<I, R>> + ?Sized,
        B: Regex<PolicyCtx<I, R>> + ?Sized,
        A: Regex<PolicyCtx<I, R>> + ?Sized,
    {
        before.try_parse(self)?;
        let ret = pat.try_parse(self)?;

        after.try_parse(self)?;
        Ok(ret)
    }
}

impl<I, R> IndexBySpan for PolicyCtx<I, R>
where
    I: IndexBySpan,
{
    type Output = I::Output;

    fn get_by_span(&self, span: &Span) -> Option<&Self::Output> {
        IndexBySpan::get_by_span(&self.inner, span)
    }
}
