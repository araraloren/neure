mod parser;
mod pattern;
mod r#return;
mod span;

use crate::err::Error;

pub use self::parser::LazyContext;
pub use self::parser::NonLazyContext;
pub use self::parser::Parser;
pub use self::pattern::Parse;
pub use self::r#return::Return;
pub use self::span::Span;

pub type BytesCtx<'a> = Parser<'a, [u8]>;
pub type CharsCtx<'a> = Parser<'a, str>;

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
    fn is_mat<Pat: Parse<C>>(&mut self, pat: &Pat) -> bool {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat: Parse<C>>(&mut self, pat: &Pat) -> Result<Pat::Ret, Error>;

    fn try_mat_policy<Pat: Parse<C>>(
        &mut self,
        pat: &Pat,
        pre: impl FnMut(&mut C) -> Result<(), Error>,
        post: impl FnMut(&mut C, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error>;
}
