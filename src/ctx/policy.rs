use super::BPolicy;
use super::Context;
use super::PolicyMatch;
use super::Regex;
use super::RegexCtx;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::span::SimpleStorer;

#[derive(Debug)]
pub struct PolicyCtx<'a, T, B>
where
    T: ?Sized,
{
    pub(crate) inner: RegexCtx<'a, T>,
    pub(crate) b_policy: Option<B>,
}

impl<'a, T, B> Clone for PolicyCtx<'a, T, B>
where
    T: ?Sized,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            b_policy: self.b_policy.clone(),
        }
    }
}

impl<'a, T, B> Copy for PolicyCtx<'a, T, B>
where
    T: ?Sized,
    B: Copy,
{
}

impl<'a, T, B> PolicyCtx<'a, T, B>
where
    T: ?Sized,
{
    pub fn new(dat: &'a T, before_policy: B) -> Self {
        Self {
            inner: RegexCtx::new(dat),
            b_policy: Some(before_policy),
        }
    }

    pub fn with_b_policy<O>(self, before_policy: O) -> PolicyCtx<'a, T, O> {
        PolicyCtx {
            inner: self.inner,
            b_policy: Some(before_policy),
        }
    }

    pub fn dat(&self) -> &'a T {
        self.inner.dat()
    }

    pub fn offset(&self) -> usize {
        self.inner.offset()
    }

    pub fn with_dat(mut self, dat: &'a T) -> Self {
        self.inner = self.inner.with_dat(dat);
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.inner = self.inner.with_offset(offset);
        self
    }

    pub fn reset_with(&mut self, dat: &'a T) -> &mut Self {
        self.inner.reset_with(dat);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.inner.reset();
        self
    }

    pub fn span_storer(&self, capacity: usize) -> SimpleStorer {
        SimpleStorer::new(capacity)
    }
}

impl<'a, B> Context<'a> for PolicyCtx<'a, [u8], B>
where
    B: Clone + 'a,
{
    type Orig = <RegexCtx<'a, [u8]> as Context<'a>>::Orig;

    type Item = <RegexCtx<'a, [u8]> as Context<'a>>::Item;

    type Iter<'b> = <RegexCtx<'a, [u8]> as Context<'a>>::Iter<'b> where Self: 'b;

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
        PolicyCtx::new(orig, self.b_policy.as_ref().unwrap().clone())
    }
}

impl<'a, B> Context<'a> for PolicyCtx<'a, str, B>
where
    B: Clone + 'a,
{
    type Orig = <RegexCtx<'a, str> as Context<'a>>::Orig;

    type Item = <RegexCtx<'a, str> as Context<'a>>::Item;

    type Iter<'b> = <RegexCtx<'a, str> as Context<'a>>::Iter<'b> where Self: 'b;

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
        PolicyCtx::new(orig, self.b_policy.as_ref().unwrap().clone())
    }
}

impl<'a, T, B> Match<PolicyCtx<'a, T, B>> for PolicyCtx<'a, T, B>
where
    T: ?Sized,
    Self: Context<'a>,
    B: BPolicy<PolicyCtx<'a, T, B>>,
{
    fn try_mat_t<Pat: Regex<PolicyCtx<'a, T, B>> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        let b_policy = self.b_policy.take().unwrap();
        let ret = self.try_mat_policy(pat, &b_policy);

        self.b_policy = Some(b_policy);
        ret
    }
}

impl<'a, T, B> PolicyMatch<PolicyCtx<'a, T, B>, B> for PolicyCtx<'a, T, B>
where
    T: ?Sized,
    Self: Context<'a>,
    B: BPolicy<PolicyCtx<'a, T, B>>,
{
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Pat::Ret, Error>
    where
        Pat: Regex<PolicyCtx<'a, T, B>> + ?Sized,
    {
        b_policy.inv_before_match(self)?;
        pat.try_parse(self)
    }
}

impl<'a, T, R, B> Extract<'a, Self, R> for PolicyCtx<'a, T, B>
where
    T: ?Sized,
    Self: Context<'a>,
    B: Clone,
{
    type Out<'b> = PolicyCtx<'a, T, B>;

    type Error = Error;

    fn extract(ctx: &Self, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, T, B> PolicyCtx<'a, T, B>
where
    T: ?Sized,
    Self: Context<'a>,
    B: BPolicy<PolicyCtx<'a, T, B>> + 'a,
{
    pub fn ctor_with<H, A, P, M, O>(&mut self, pat: &P, handler: &mut H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Span, Out<'a> = A, Error = Error>,
    {
        pat.constrct(self, handler)
    }

    pub fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, &'a <Self as Context<'a>>::Orig, O>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Span, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Ok)
    }

    pub fn map<H, A, O, P>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, P::Ret, Out<'a> = A, Error = Error>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(A::extract(self, &ret)?)
    }

    pub fn map_orig<O, P>(
        &mut self,
        pat: &P,
        mut func: impl FnMut(&'a <Self as Context<'a>>::Orig) -> Result<O, Error>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, P::Ret, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error>,
    {
        (func)(self.map(pat, Ok)?)
    }

    pub fn map_span<O, P>(
        &mut self,
        pat: &P,
        mut func: impl FnMut(Span) -> Result<O, Error>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        Span: Extract<'a, Self, P::Ret, Out<'a> = Span, Error = Error>,
    {
        (func)(self.map(pat, Ok)?)
    }
}
