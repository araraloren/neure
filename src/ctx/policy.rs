use super::Context;
use super::PolicyMatch;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::iter::IndexBySpan;

/// A policy-based context wrapper that applies a regex policy before each match operation.
///
/// This struct wraps an inner context and a regex policy. When performing match operations,
/// it first applies the policy regex to the inner context, then applies the target pattern
/// to itself. This is useful for enforcing match policies or preprocessing constraints.
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
    /// Creates a new `PolicyCtx` with the given inner context and policy regex.
    pub const fn new(inner: I, regex: R) -> Self {
        Self { inner, regex }
    }

    /// Replaces the current policy regex with a new one, returning a new `PolicyCtx`.
    pub fn with_regex<O>(self, regex: O) -> PolicyCtx<I, O> {
        PolicyCtx {
            inner: self.inner,
            regex,
        }
    }

    /// Returns an immutable reference to the inner context.
    pub const fn inner(&self) -> &I {
        &self.inner
    }

    /// Returns an mutable reference to the inner context.
    pub const fn inner_mut(&mut self) -> &mut I {
        &mut self.inner
    }

    /// Replaces the inner context with a new one, modifying the current context in place.
    pub fn set_inner(&mut self, dat: I) -> &mut Self {
        self.inner = dat;
        self
    }

    /// Consumes `self` and returns a new `PolicyCtx` with a different inner context.
    pub fn with_inner(mut self, dat: I) -> Self {
        self.inner = dat;
        self
    }

    /// Replaces the inner context and returns a mutable reference to self.
    pub fn reset_with(&mut self, inner: I) -> &mut Self {
        self.inner = inner;
        self
    }

    /// Creates a span storer with the specified capacity(with feature `alloc` enabled).
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
    /// Attempts to match a pattern against the context.
    ///
    /// This method first applies the policy regex to the inner context, then attempts
    /// to match the provided pattern against the policy context.
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
    /// Attempts to match a pattern with before and after policy constraints.
    ///
    /// This method provides more granular control by allowing separate regexes
    /// to be applied before and after the main pattern match.
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
