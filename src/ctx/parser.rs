use std::ops::Deref;
use std::ops::DerefMut;
use std::str::CharIndices;

use super::Context;
use super::Parse;
use super::Span;

use crate::ctx::Policy;
use crate::err::Error;
use crate::ext::Extract;
use crate::ext::Handler;
use crate::ext::Invoke;
use crate::iter::BytesIndices;
use crate::span::SimpleStorer;

pub struct LazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    pub(crate) parser: &'b mut Parser<'a, T>,
}

impl<'a, 'b, T> Deref for LazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    type Target = &'b mut Parser<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}

impl<'a, 'b, T> DerefMut for LazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

pub struct NonLazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    pub(crate) parser: &'b mut Parser<'a, T>,
}

impl<'a, 'b, T> Deref for NonLazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    type Target = &'b mut Parser<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}

impl<'a, 'b, T> DerefMut for NonLazyContext<'a, 'b, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

#[derive(Debug)]
pub struct Parser<'a, T>
where
    T: ?Sized,
{
    dat: &'a T,
    offset: usize,
}

impl<'a, T> Clone for Parser<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Parser<'a, T> where T: ?Sized {}

impl<'a, T> Parser<'a, T>
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

    pub fn lazy(&mut self) -> LazyContext<'a, '_, T> {
        LazyContext { parser: self }
    }

    pub fn non_lazy(&mut self) -> NonLazyContext<'a, '_, T> {
        NonLazyContext { parser: self }
    }

    pub fn span_storer(&self, capacity: usize) -> SimpleStorer {
        SimpleStorer::new(capacity)
    }
}

impl<'a> Context<'a> for Parser<'a, [u8]> {
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
        self.dat.get(offset..).ok_or(Error::ReachEnd)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..(offset + len)).ok_or(Error::ReachEnd)
    }
}

impl<'a> Context<'a> for Parser<'a, str> {
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
        self.dat.get(offset..).ok_or(Error::ReachEnd)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..(offset + len)).ok_or(Error::ReachEnd)
    }
}

impl<'a, T> Policy<Parser<'a, T>> for Parser<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat<Pat: Parse<Parser<'a, T>>>(&mut self, pat: &Pat) -> Result<Pat::Ret, Error> {
        self.try_mat_policy(pat, |_| Ok(()), |_, ret| ret)
    }

    fn try_mat_policy<Pat: Parse<Parser<'a, T>>>(
        &mut self,
        pat: &Pat,
        mut pre: impl FnMut(&mut Parser<'a, T>) -> Result<(), Error>,
        mut post: impl FnMut(&mut Parser<'a, T>, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error> {
        pre(self)?;
        let ret = pat.try_parse(self);
        post(self, ret)
    }
}

impl<'a, T, R> Extract<'a, Self, R> for Parser<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    type Out<'b> = Parser<'a, T>;

    type Error = Error;

    fn extract(ctx: &Self, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, T> Parser<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    pub fn map_with<H, A, P, M, O>(&mut self, pat: &P, func: &mut H) -> Result<O, Error>
    where
        P: Invoke<'a, Self, M, O>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Span, Out<'a> = A, Error = Error>,
    {
        pat.invoke(self, func)
    }

    pub fn map<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Invoke<'a, Self, &'a <Self as Context<'a>>::Orig, O>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Span, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.map_with(pat, &mut |orig: &'a <Self as Context<'a>>::Orig| Ok(orig))
    }
}
