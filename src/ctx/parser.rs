use std::str::CharIndices;

use super::Context;
use super::Pattern;

use crate::err::Error;
use crate::iter::BytesIndices;
use crate::policy::Length;
use crate::policy::Policy;
use crate::policy::Ret;
use crate::span::SpanStorer;

#[derive(Debug)]
pub struct Parser<'a, T>
where
    T: ?Sized,
{
    dat: &'a T,
    offset: usize,
}

impl<'a, T> Parser<'a, T> {
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

    pub fn span_storer(&self, capacity: usize) -> SpanStorer {
        SpanStorer::new(capacity)
    }
}

impl<'a> Context<'a> for Parser<'a, [u8]>
where
    u8: Copy,
{
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
    Self: Context<'a>,
{
    type Ret = Length;

    fn try_mat<Pat>(&mut self, pat: Pat) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        Pat: Pattern<Self, Ret = Self::Ret>,
    {
        self.try_mat_policy(
            pat,
            |_| Ok(()),
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    Context::inc(ctx, ret.length());
                }
                ret
            },
        )
    }

    fn try_mat_policy<Pat, Pre, Post>(
        &mut self,
        pat: Pat,
        mut pre: Pre,
        mut post: Post,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        Pat: Pattern<Self, Ret = Self::Ret>,
        Pre: FnMut(&mut Self) -> Result<(), Error>,
        Post: FnMut(&mut Self, Result<Self::Ret, Error>) -> Result<Self::Ret, Error>,
    {
        pre(self)?;
        let ret = pat.try_parse(self);
        post(self, ret)
    }
}
