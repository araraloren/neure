mod guard;
mod policy;
#[allow(clippy::module_inception)]
mod regex;
mod span;

use std::marker::PhantomData;

use crate::ctor::{Ctor, Extract, Handler, Pass};
use crate::err::Error;
use crate::map::MapSingle;
use crate::regex::Regex;
use crate::MayDebug;

pub use self::guard::CtxGuard;
pub use self::policy::PolicyCtx;
pub use self::regex::RegexCtx;
pub use self::span::Span;

pub type BytesCtx<'a> = RegexCtx<'a, [u8]>;
pub type CharsCtx<'a> = RegexCtx<'a, str>;

pub trait Context<'a> {
    type Orig<'b>;

    type Item: MayDebug;

    type Iter<'b>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'b;

    type Cloned: Context<'a>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn req(&mut self) -> Result<bool, Error> {
        Ok(false)
    }

    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    fn orig(&self) -> Result<Self::Orig<'a>, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error>;

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error>;

    fn clone_with(&self, orig: Self::Orig<'a>) -> Self::Cloned;
}

pub trait Match<C> {
    fn is_mat<Pat: Regex<C> + ?Sized>(&mut self, pat: &Pat) -> bool {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat: Regex<C> + ?Sized>(&mut self, pat: &Pat) -> Result<Span, Error>;
}

pub trait ContextHelper<'a>
where
    Self: Context<'a> + Sized,
{
    fn ctor_with<H, A, P, M, O>(&mut self, pat: &P, handler: &mut H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O, H, A>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Out<'a> = A, Error = Error>,
    {
        pat.construct(self, handler)
    }

    fn map_with<H, A, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, Out<'a> = A, Error = Error>;

    fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<
            'a,
            Self,
            <Self as Context<'a>>::Orig<'a>,
            O,
            Pass,
            <Self as Context<'a>>::Orig<'a>,
        >,
        <Self as Context<'a>>::Orig<'a>:
            Extract<'a, Self, Out<'a> = <Self as Context<'a>>::Orig<'a>, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Pass)
    }

    fn map<P, O>(
        &mut self,
        pat: &P,
        mapper: impl MapSingle<<Self as Context<'a>>::Orig<'a>, O>,
    ) -> Result<O, Error>
    where
        P: Regex<Self>,
        <Self as Context<'a>>::Orig<'a>:
            Extract<'a, Self, Out<'a> = <Self as Context<'a>>::Orig<'a>, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }

    fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O, Pass, Span>,
        Span: Extract<'a, Self, Out<'a> = Span, Error = Error>,
    {
        self.ctor_with(pat, &mut Pass)
    }

    fn map_span<P, O>(&mut self, pat: &P, mapper: impl MapSingle<Span, O>) -> Result<O, Error>
    where
        P: Regex<Self>,
        Span: Extract<'a, Self, Out<'a> = Span, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }
}

impl<'a, C> ContextHelper<'a> for C
where
    C: Sized + Context<'a> + Match<Self>,
{
    fn map_with<H, A, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, Out<'a> = A, Error = Error>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(A::extract(self, &ret)?)
    }
}

pub trait PolicyMatch<C, B> {
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Span, Error>
    where
        Pat: Regex<C> + ?Sized;
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
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn invoke_policy(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.try_mat(&self.regex)?;
        Ok(())
    }
}

/// Using for either [`RegexCtx::with_policy`] or [`PolicyCtx::with_policy`].
pub fn re_policy<C, T>(regex: T) -> RePolicy<C, T> {
    RePolicy::new(regex)
}
