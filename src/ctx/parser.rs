use std::ops::Deref;
use std::ops::DerefMut;
use std::str::CharIndices;

use super::Context;
use super::Pattern;
use super::Return;

use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::True;
use crate::err::Error;
use crate::ext::Extract;
use crate::ext::LazyCtxExtension;
use crate::ext::LazyPattern;
use crate::ext::LazyQuote;
use crate::ext::LazyTerm;
use crate::ext::NonLazyCtxExtension;
use crate::ext::NonLazyPattern;
use crate::ext::NonLazyQuote;
use crate::ext::NonLazyTerm;
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
    type Ret = Return;

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

impl<'a, T, R> Extract<'a, Self, R> for Parser<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    type Out<'b> = Parser<'a, T>;

    type Error = Error;

    fn extract(ctx: &Self, _: usize, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, T: ?Sized> LazyCtxExtension<'a, Parser<'a, T>> for LazyContext<'a, '_, T>
where
    Parser<'a, T>: Context<'a>,
{
    fn quote<L, R>(&mut self, left: L, right: R) -> LazyQuote<'_, Parser<'a, T>, L, R>
    where
        L: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
        R: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
    {
        LazyQuote::new(self.parser, left, right)
    }

    fn pat<P>(
        &mut self,
        pattern: P,
    ) -> LazyPattern<'_, Parser<'a, T>, P, True<Parser<'a, T>>, True<Parser<'a, T>>>
    where
        P: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
    {
        LazyPattern::new(self.parser, True::default(), True::default(), pattern)
    }

    fn term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> LazyTerm<'_, Parser<'a, T>, S, True<Parser<'a, T>>, True<Parser<'a, T>>>
    where
        S: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret> + Clone,
    {
        LazyTerm::new(
            self.parser,
            Some(True::default()),
            Some(True::default()),
            sep,
            optional,
        )
    }
}

impl<'a, T: ?Sized> NonLazyCtxExtension<'a, Parser<'a, T>> for NonLazyContext<'a, '_, T>
where
    Parser<'a, T>: Context<'a>,
{
    fn quote<L, R>(
        &mut self,
        left: L,
        right: R,
    ) -> Result<NonLazyQuote<'_, Parser<'a, T>, R>, Error>
    where
        L: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
        R: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
    {
        self.parser.try_mat(left)?;

        Ok(NonLazyQuote::new(self.parser, right))
    }

    fn pat<P>(
        &mut self,
        pattern: P,
    ) -> Result<NonLazyPattern<'_, Parser<'a, T>, True<Parser<'a, T>>>, Error>
    where
        P: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret>,
    {
        let beg = self.parser.offset();
        let ret = self.parser.try_mat(pattern);

        Ok(NonLazyPattern::new(self.parser, True::default(), beg, ret))
    }

    fn term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> Result<NonLazyTerm<'_, Parser<'a, T>, S, True<Parser<'a, T>>>, Error>
    where
        S: Pattern<Parser<'a, T>, Ret = <Parser<'a, T> as Policy<Parser<'a, T>>>::Ret> + Clone,
    {
        Ok(NonLazyTerm::new(self.parser, None, sep, optional))
    }
}
