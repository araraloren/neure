#[allow(clippy::module_inception)]
mod ctx;
mod ret;
mod span;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::err::Error;
use crate::regex::Regex;

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
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat: Regex<C> + ?Sized>(&mut self, pat: &Pat) -> Result<Pat::Ret, Error>;

    fn try_mat_policy<Pat: Regex<C> + ?Sized>(
        &mut self,
        pat: &Pat,
        pre: impl FnMut(&mut C) -> Result<(), Error>,
        post: impl FnMut(&mut C, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error>;
}

impl<C, F, R> Regex<C> for F
where
    F: Fn(&mut C) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

impl<'a, 'b, C> Regex<C> for &'b str
where
    C: Context<'a, Orig = str> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::regex::string(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, C> Regex<C> for &'b [u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::regex::bytes(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, Ret, C> Regex<C> for Box<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&*self.borrow())
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.get())
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret = self.lock().expect("Oops ?! Can not unwrap mutex ...");
        ctx.try_mat(&*ret)
    }
}

impl<'a, P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Arc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Rc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
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
