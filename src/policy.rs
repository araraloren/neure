use std::ops::AddAssign;

use crate::err::Error;
use crate::parser::Parser;
use crate::span::SpanStore;

pub trait Ret: AddAssign<Self>
where
    Self: Sized,
{
    fn count(&self) -> usize;

    fn length(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn new_from(ret: (usize, usize)) -> Self;
}

/// first is count of char, second is count of byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(usize);

impl Ret for Length {
    fn count(&self) -> usize {
        0
    }

    fn length(&self) -> usize {
        self.0
    }

    fn is_zero(&self) -> bool {
        self.length() == 0
    }

    fn new_from(ret: (usize, usize)) -> Self {
        Self(ret.1)
    }
}

impl AddAssign<Length> for Length {
    fn add_assign(&mut self, rhs: Length) {
        self.0 += rhs.length();
    }
}

pub trait Context {
    type Orig: ?Sized;

    type Item;

    type Iter<'a>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'a;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'_>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'_>, Error>;

    fn orig(&self) -> Result<&Self::Orig, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<&Self::Orig, Error>;

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&Self::Orig, Error>;
}

pub trait MatchPolicy {
    type Ret: Ret;

    fn try_mat_policy(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        mut pre_policy: impl FnMut(&mut Self) -> Result<(), Error>,
        mut post_policy: impl FnMut(&mut Self, Result<Self::Ret, Error>) -> Result<Self::Ret, Error>,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        pre_policy(self)?;
        let ret = parser.try_parse(self);

        post_policy(self, ret)
    }

    fn is_mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> bool
    where
        Self: Sized,
    {
        self.try_mat(parser).is_ok()
    }

    fn is_cap<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> bool
    where
        Self: Sized,
        S: SpanStore,
    {
        self.try_cap(id, storer, parser).is_ok()
    }

    fn try_mat(&mut self, parser: impl Parser<Self, Ret = Self::Ret>) -> Result<Self::Ret, Error>
    where
        Self: Sized,
    {
        self.try_mat_reset(parser, false)
    }

    fn try_cap<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore,
    {
        self.try_cap_reset(id, storer, parser, false)
    }

    fn try_mat_reset(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized;

    fn try_cap_reset<S>(
        &mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
        reset: bool,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        S: SpanStore;
}
