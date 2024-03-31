mod guard;
mod policy;
#[allow(clippy::module_inception)]
mod regex;
mod span;

use std::marker::PhantomData;

use crate::err::Error;
use crate::re::Regex;
use crate::MayDebug;

pub use self::guard::CtxGuard;
pub use self::policy::PolicyCtx;
pub use self::regex::RegexCtx;
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

    fn clone_with(&self, orig: &'a Self::Orig) -> Self;
}

pub trait Ret: MayDebug
where
    Self: Sized,
{
    fn fst(&self) -> usize;

    fn snd(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn add_assign(&mut self, other: Self) -> &mut Self;

    fn from_ctx<'a, C>(ctx: &mut C, info: (usize, usize)) -> Self
    where
        C: Context<'a>;
}

pub trait Match<C> {
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
}

pub trait PolicyMatch<C, B> {
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Pat::Ret, Error>
    where
        Pat: Regex<C> + ?Sized;
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

    fn from_ctx<'a, C>(_: &mut C, _: (usize, usize)) -> Self
    where
        C: Context<'a>,
    {
    }
}

pub trait BPolicy<C> {
    fn invoke_policy(&self, ctx: &mut C) -> Result<(), Error>;
}

impl<C, F> BPolicy<C> for F
where
    F: Fn(&mut C) -> Result<(), Error>,
{
    fn invoke_policy(&self, ctx: &mut C) -> Result<(), Error> {
        (self)(ctx)
    }
}

impl<C, B> BPolicy<C> for Option<B>
where
    B: BPolicy<C>,
{
    fn invoke_policy(&self, ctx: &mut C) -> Result<(), Error> {
        match self {
            Some(ref_) => ref_.invoke_policy(ctx),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RePolicy<C, T> {
    regex: T,
    marker: PhantomData<C>,
}

impl<C, T> Clone for RePolicy<C, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            regex: self.regex.clone(),
            marker: self.marker,
        }
    }
}

impl<C, T> RePolicy<C, T> {
    pub fn new(regex: T) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<'a, C, T> BPolicy<C> for RePolicy<C, T>
where
    C::Orig: 'a,
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn invoke_policy(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.try_mat_t(&self.regex)?;
        Ok(())
    }
}

/// Using for either [`RegexCtx::with_policy`] or [`PolicyCtx::with_policy`].
pub fn re_policy<C, T>(regex: T) -> RePolicy<C, T> {
    RePolicy::new(regex)
}
