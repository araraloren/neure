mod parser;
mod pattern;
mod r#return;

use crate::err::Error;

pub use self::parser::LazyContext;
pub use self::parser::NonLazyContext;
pub use self::parser::Parser;
pub use self::pattern::Pattern;
pub use self::pattern::True;
pub use self::r#return::Return;

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
    fn count(&self) -> usize;

    fn length(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn new_from(ret: (usize, usize)) -> Self;

    fn add_assign(&mut self, other: Self) -> &mut Self;
}

pub trait Policy<C> {
    type Ret: Ret;

    fn is_mat<Pat>(&mut self, pat: Pat) -> bool
    where
        Self: Sized,
        Pat: Pattern<C, Ret = Self::Ret>,
    {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat>(&mut self, pat: Pat) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        Pat: Pattern<C, Ret = Self::Ret>;

    fn try_mat_policy<Pat, Pre, Post>(
        &mut self,
        pat: Pat,
        pre: Pre,
        post: Post,
    ) -> Result<Self::Ret, Error>
    where
        Self: Sized,
        Pat: Pattern<C, Ret = Self::Ret>,
        Pre: FnMut(&mut C) -> Result<(), Error>,
        Post: FnMut(&mut C, Result<Self::Ret, Error>) -> Result<Self::Ret, Error>;
}
