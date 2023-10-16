#[allow(clippy::module_inception)]
mod ctx;
mod ret;
mod span;

use crate::err::Error;
use crate::re::Regex;

pub use self::ctx::RegexCtx;
pub use self::ret::Return;
pub use self::span::Span;

pub type BytesCtx<'a> = RegexCtx<'a, [u8]>;
pub type CharsCtx<'a> = RegexCtx<'a, str>;

pub trait Context<'a> {
    type Orig: ?Sized;

    type Item;

    type Iter<'b>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'b;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    fn orig(&self) -> Result<&'a Self::Orig, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error>;

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error>;
}

pub trait Ret
where
    Self: Sized,
{
    fn fst(&self) -> usize;

    fn snd(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn add_assign(&mut self, other: Self) -> &mut Self;

    fn from<'a, C>(ctx: &mut C, info: (usize, usize)) -> Self
    where
        C: Context<'a>;
}

pub trait Policy<C> {
    fn is_mat<Pat: Regex<C> + ?Sized>(&mut self, pat: &Pat) -> bool {
        self.try_mat_t(pat).is_ok()
    }

    fn try_mat_t<Pat: Regex<C> + ?Sized>(&mut self, pat: &Pat) -> Result<Pat::Ret, Error>;

    fn try_mat<Pat: Regex<C, Ret = Span> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        self.try_mat_t(pat)
    }

    fn try_mat_policy<Pat: Regex<C> + ?Sized>(
        &mut self,
        pat: &Pat,
        pre: impl FnMut(&mut C) -> Result<(), Error>,
        post: impl FnMut(&mut C, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error>;
}

impl Ret for () {
    fn fst(&self) -> usize {
        0
    }

    fn snd(&self) -> usize {
        0
    }

    fn is_zero(&self) -> bool {
        true
    }

    fn add_assign(&mut self, _: Self) -> &mut Self {
        self
    }

    fn from<'a, C>(_: &mut C, _: (usize, usize)) -> Self
    where
        C: Context<'a>,
    {
    }
}
