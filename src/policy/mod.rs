mod length;

use std::ops::AddAssign;

use crate::ctx::Pattern;
use crate::err::Error;

pub use self::length::Length;

pub trait Ret: AddAssign<Self>
where
    Self: Sized,
{
    fn count(&self) -> usize;

    fn length(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn new_from(ret: (usize, usize)) -> Self;
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
