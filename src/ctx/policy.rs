use super::BPolicy;
use super::Context;
use super::PolicyMatch;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::span::SimpleStorer;

#[derive(Debug)]
pub struct PolicyCtx<I, B> {
    pub(crate) inner: I,
    pub(crate) b_policy: B,
}

impl<I, B> Clone for PolicyCtx<I, B>
where
    I: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            b_policy: self.b_policy.clone(),
        }
    }
}

impl<I, B> Copy for PolicyCtx<I, B>
where
    B: Copy,
    I: Copy,
{
}

impl<I, B> PolicyCtx<I, B> {
    pub fn new(inner: I, before_policy: B) -> Self {
        Self {
            inner,
            b_policy: before_policy,
        }
    }

    pub fn with_policy<O>(self, before_policy: O) -> PolicyCtx<I, O> {
        PolicyCtx {
            inner: self.inner,
            b_policy: before_policy,
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

    pub fn span_storer(&self, capacity: usize) -> SimpleStorer {
        SimpleStorer::new(capacity)
    }
}

impl<'a, I, B> Context<'a> for PolicyCtx<I, B>
where
    B: Clone + 'a,
    I: Context<'a>,
{
    type Orig = <I as Context<'a>>::Orig;

    type Item = <I as Context<'a>>::Item;

    type Iter<'b> = <I as Context<'a>>::Iter<'b> where Self: 'b;

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

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        Context::orig_at(&self.inner, offset)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Context::peek_at(&self.inner, offset)
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        Context::orig_sub(&self.inner, offset, len)
    }

    fn clone_with(&self, orig: &'a Self::Orig) -> Self {
        PolicyCtx {
            inner: I::clone_with(&self.inner, orig),
            b_policy: self.b_policy.clone(),
        }
    }
}

impl<'a, I, B> Match<PolicyCtx<I, B>> for PolicyCtx<I, B>
where
    B: BPolicy<I>,
    I: Context<'a>,
    Self: Context<'a>,
{
    fn try_mat_t<Pat: Regex<PolicyCtx<I, B>> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        self.b_policy.invoke_policy(&mut self.inner)?;
        pat.try_parse(self)
    }
}

impl<'a, I, B> PolicyMatch<PolicyCtx<I, B>, B> for PolicyCtx<I, B>
where
    B: BPolicy<I>,
    I: Context<'a>,
    Self: Context<'a>,
{
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Pat::Ret, Error>
    where
        Pat: Regex<PolicyCtx<I, B>> + ?Sized,
    {
        b_policy.invoke_policy(&mut self.inner)?;
        pat.try_parse(self)
    }
}

impl<'a, I, R, B> Extract<'a, Self, R> for PolicyCtx<I, B>
where
    B: Clone,
    Self: Context<'a>,
    I: Context<'a> + Clone,
{
    type Out<'b> = PolicyCtx<I, B>;

    type Error = Error;

    fn extract(ctx: &Self, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, I, B> PolicyCtx<I, B>
where
    I: Context<'a>,
    Self: Context<'a>,
    B: BPolicy<I> + 'a,
{
    pub fn ctor_with<H, A, P, M, O>(&mut self, pat: &P, handler: &mut H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Span, Out<'a> = A, Error = Error>,
    {
        pat.constrct(self, handler)
    }

    pub fn map_with<H, A, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, P::Ret, Out<'a> = A, Error = Error>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(A::extract(self, &ret)?)
    }

    pub fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, &'a <Self as Context<'a>>::Orig, O>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Span, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Ok)
    }

    pub fn map<P, O>(
        &mut self,
        pat: &P,
        mut func: impl FnMut(&'a <Self as Context<'a>>::Orig) -> Result<O, Error>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        <Self as Context<'a>>::Orig: 'a,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, P::Ret, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error>,
    {
        (func)(self.map_with(pat, Ok)?)
    }

    pub fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O>,
        Span: Extract<'a, Self, Span, Out<'a> = Span, Error = Error>,
    {
        self.ctor_with(pat, &mut Ok)
    }

    pub fn map_span<P, O>(
        &mut self,
        pat: &P,
        mut func: impl FnMut(Span) -> Result<O, Error>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        Span: Extract<'a, Self, P::Ret, Out<'a> = Span, Error = Error>,
    {
        (func)(self.map_with(pat, Ok)?)
    }
}
