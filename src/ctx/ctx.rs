use std::str::CharIndices;

use super::Context;
use super::Regex;
use super::Span;

use crate::ctx::Policy;
use crate::err::Error;
use crate::iter::BytesIndices;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
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
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OutOfBound)
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
        trace_log!("set offset <- {offset}");
        self.offset = offset;
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        trace_log!("offset + {offset}");
        self.offset += offset;
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        trace_log!("offset - {offset}");
        self.offset -= offset;
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OutOfBound)
    }
}

impl<'a, T> Policy<RegexCtx<'a, T>> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat_t<Pat: Regex<RegexCtx<'a, T>> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        self.try_mat_policy(pat, |_| Ok(()), |_, ret| ret)
    }

    fn try_mat_policy<Pat: Regex<RegexCtx<'a, T>> + ?Sized>(
        &mut self,
        pat: &Pat,
        mut pre: impl FnMut(&mut RegexCtx<'a, T>) -> Result<(), Error>,
        mut post: impl FnMut(&mut RegexCtx<'a, T>, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error> {
        pre(self)?;
        let ret = pat.try_parse(self);

        post(self, ret)
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

        trace_log!("try mat return span in map: {:?}", ret);
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
