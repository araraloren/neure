use std::str::CharIndices;

use super::re_policy;
use super::BPolicy;
use super::Context;
use super::PolicyCtx;
use super::PolicyMatch;
use super::RePolicy;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::iter::BytesIndices;
use crate::map::MapSingle;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Pass;
use crate::span::SimpleStorer;
use crate::trace_log;

#[derive(Debug)]
pub struct RegexCtx<'a, T>
where
    T: ?Sized,
{
    dat: &'a T,
    offset: usize,
}

impl<'a, T> Clone for RegexCtx<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for RegexCtx<'a, T> where T: ?Sized {}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
{
    pub fn new(dat: &'a T) -> Self {
        Self { dat, offset: 0 }
    }

    pub fn dat(&self) -> &'a T {
        self.dat
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn with_dat(mut self, dat: &'a T) -> Self {
        self.dat = dat;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn reset_with(&mut self, dat: &'a T) -> &mut Self {
        self.dat = dat;
        self.offset = 0;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.offset = 0;
        self
    }

    pub fn span_storer(&self, capacity: usize) -> SimpleStorer {
        SimpleStorer::new(capacity)
    }

    pub fn with_policy<O>(self, before_policy: O) -> PolicyCtx<Self, O>
    where
        O: BPolicy<Self>,
    {
        PolicyCtx {
            inner: self,
            b_policy: before_policy,
        }
    }
}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
{
    pub fn ignore<R>(self, regex: R) -> PolicyCtx<Self, RePolicy<Self, R>> {
        PolicyCtx {
            inner: self,
            b_policy: re_policy(regex),
        }
    }
}

impl<'a> Context<'a> for RegexCtx<'a, [u8]> {
    type Orig = [u8];

    type Item = u8;

    type Iter<'b> = BytesIndices<'b, u8> where Self: 'b;

    fn len(&self) -> usize {
        self.dat.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        trace_log!("set {offset} -> ctx -> {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        trace_log!("inc {offset} -> ctx -> {}", self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        trace_log!("dec {offset} -> ctx -> {}", self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OriginOutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OriginOutOfBound)
    }

    fn clone_with(&self, orig: &'a Self::Orig) -> Self {
        RegexCtx::new(orig)
    }
}

impl<'a> Context<'a> for RegexCtx<'a, str> {
    type Orig = str;

    type Item = char;

    type Iter<'b> = CharIndices<'b> where Self: 'b;

    fn len(&self) -> usize {
        self.dat.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        trace_log!("set {offset} -> ctx -> {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        trace_log!("inc {offset} -> ctx -> {}", self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        trace_log!("dec {offset} -> ctx -> {}", self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OriginOutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OriginOutOfBound)
    }

    fn clone_with(&self, orig: &'a Self::Orig) -> Self {
        RegexCtx::new(orig)
    }
}

impl<'a, T> Match<RegexCtx<'a, T>> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat_t<Pat: Regex<RegexCtx<'a, T>> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        self.try_mat_policy(pat, &|_: &mut Self| Ok(()))
    }
}

impl<'a, T, B> PolicyMatch<RegexCtx<'a, T>, B> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
    B: BPolicy<RegexCtx<'a, T>>,
{
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Pat::Ret, Error>
    where
        Pat: Regex<RegexCtx<'a, T>> + ?Sized,
    {
        b_policy.invoke_policy(self)?;
        pat.try_parse(self)
    }
}

impl<'a, T, R> Extract<'a, Self, R> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    type Out<'b> = RegexCtx<'a, T>;

    type Error = Error;

    fn extract(ctx: &Self, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    pub fn ctor_with<H, A, P, M, O>(&mut self, pat: &P, handler: &mut H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O, H, A>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Span, Out<'a> = A, Error = Error>,
    {
        pat.construct(self, handler)
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
        P: Ctor<
            'a,
            Self,
            &'a <Self as Context<'a>>::Orig,
            O,
            Pass,
            &'a <Self as Context<'a>>::Orig,
        >,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Span, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map<P, O>(
        &mut self,
        pat: &P,
        mapper: impl MapSingle<&'a <Self as Context<'a>>::Orig, O>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, P::Ret, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }

    pub fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O, Pass, Span>,
        Span: Extract<'a, Self, Span, Out<'a> = Span, Error = Error>,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map_span<P, O>(&mut self, pat: &P, mapper: impl MapSingle<Span, O>) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        Span: Extract<'a, Self, P::Ret, Out<'a> = Span, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }
}
