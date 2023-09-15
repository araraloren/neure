mod parser;
mod r#return;
mod span;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::err::Error;

pub use self::parser::Parser;
pub use self::r#return::Return;
pub use self::span::Span;

pub type BytesCtx<'a> = Parser<'a, [u8]>;
pub type CharsCtx<'a> = Parser<'a, str>;

pub trait Parse<C> {
    type Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error>;

    fn parse(&self, ctx: &mut C) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

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
    fn is_mat<Pat: Parse<C> + ?Sized>(&mut self, pat: &Pat) -> bool {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat: Parse<C> + ?Sized>(&mut self, pat: &Pat) -> Result<Pat::Ret, Error>;

    fn try_mat_policy<Pat: Parse<C> + ?Sized>(
        &mut self,
        pat: &Pat,
        pre: impl FnMut(&mut C) -> Result<(), Error>,
        post: impl FnMut(&mut C, Result<Pat::Ret, Error>) -> Result<Pat::Ret, Error>,
    ) -> Result<Pat::Ret, Error>;
}

impl<C, F, R> Parse<C> for F
where
    F: Fn(&mut C) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

impl<'a, C> Parse<C> for char
where
    C: Context<'a, Item = char> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::parser::one(crate::regex::char(*self));
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, C> Parse<C> for &'b str
where
    C: Context<'a, Orig = str> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::parser::string(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, C> Parse<C> for u8
where
    C: Context<'a, Item = u8> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::parser::one(crate::regex::equal(*self));
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, C> Parse<C> for &'b [u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::parser::bytes(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, Ret, C> Parse<C> for Box<dyn Parse<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, 'b, P, C> Parse<C> for RefCell<P>
where
    P: Parse<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&*self.borrow())
    }
}

impl<'a, 'b, P, C> Parse<C> for Cell<P>
where
    P: Parse<C> + Copy,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.get())
    }
}

impl<'a, 'b, P, C> Parse<C> for Mutex<P>
where
    P: Parse<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret = self.lock().expect("Oops ?! Can not unwrap mutex ...");
        ctx.try_mat(&*ret)
    }
}

impl<'a, 'b, P, C> Parse<C> for Arc<P>
where
    P: Parse<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, 'b, Ret, C> Parse<C> for Arc<dyn Parse<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, 'b, P, C> Parse<C> for Rc<P>
where
    P: Parse<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, 'b, Ret, C> Parse<C> for Rc<dyn Parse<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}
