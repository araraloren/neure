use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

// into_box
pub struct BoxedCtor<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T> Debug for BoxedCtor<C, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxedCtor")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<C, T> Clone for BoxedCtor<C, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker,
        }
    }
}

impl<C, T> BoxedCtor<C, T> {
    pub fn new(inner: Box<T>) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: Box<T>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Box<T> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: Box<T>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<C, T> Regex<C> for BoxedCtor<C, T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Boxed invoke not support `Regex` trait")
    }
}

impl<'a, C, M, O, T> Ctor<'a, C, M, O> for BoxedCtor<C, T>
where
    T: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.inner.constrct(ctx, handler)
    }
}

///
/// Return a type that wraps `Ctor` with Box.
///
/// # Example
///
/// ```
/// # use neure::{err::Error, prelude::*, re::BoxedCtorHelper};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let re = b'+'
///         .or(b'-')
///         .then(u8::is_ascii_hexdigit)
///         .then(u8::is_ascii_hexdigit.repeat_times::<3>())
///         .pat()
///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
///         .into_boxed_ctor();
///
///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, "+AE00");
///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, "-83FD");
///     Ok(())
/// # }
/// ```
pub fn into_boxed_ctor<'a, C, M, O, I>(invoke: I) -> BoxedCtor<C, I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    BoxedCtor::new(Box::new(invoke))
}

pub trait BoxedCtorHelper<'a, C, M, O>
where
    C: Context<'a> + Match<C>,
{
    fn into_boxed_ctor(self) -> BoxedCtor<C, Self>
    where
        Self: Sized;
}

impl<'a, C, M, O, I> BoxedCtorHelper<'a, C, M, O> for I
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn into_boxed_ctor(self) -> BoxedCtor<C, Self>
    where
        Self: Sized,
    {
        BoxedCtor::new(Box::new(self))
    }
}
