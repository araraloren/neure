use super::BPolicy;
use super::Context;
use super::PolicyMatch;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::map::MapSingle;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Pass;
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
    fn try_mat_t<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<PolicyCtx<I, B>> + ?Sized,
    {
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
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Span, Error>
    where
        Pat: Regex<PolicyCtx<I, B>> + ?Sized,
    {
        b_policy.invoke_policy(&mut self.inner)?;
        pat.try_parse(self)
    }
}

impl<'a, I, B> Extract<'a, Self> for PolicyCtx<I, B>
where
    B: Clone,
    Self: Context<'a>,
    I: Context<'a> + Clone,
{
    type Out<'b> = PolicyCtx<I, B>;

    type Error = Error;

    fn extract(ctx: &Self, _: &Span) -> Result<Self::Out<'a>, Self::Error> {
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
        P: Ctor<'a, Self, M, O, H, A>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Out<'a> = A, Error = Error>,
    {
        pat.construct(self, handler)
    }

    pub fn map_with<H, A, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, Out<'a> = A, Error = Error>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(A::extract(self, &ret)?)
    }

    pub fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<
            'a,
            Self,
            &'a <Self as Context<'a>>::Orig,
            O,
            Pass,
            &'a <Self as Context<'a>>::Orig,
        >,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map<P, O>(
        &mut self,
        pat: &P,
        mapper: impl MapSingle<&'a <Self as Context<'a>>::Orig, O>,
    ) -> Result<O, Error>
    where
        P: Regex<Self>,
        <Self as Context<'a>>::Orig: 'a,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }

    pub fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O, Pass, Span>,
        Span: Extract<'a, Self, Out<'a> = Span, Error = Error>,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map_span<P, O>(&mut self, pat: &P, mapper: impl MapSingle<Span, O>) -> Result<O, Error>
    where
        P: Regex<Self>,
        Span: Extract<'a, Self, Out<'a> = Span, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }
}
