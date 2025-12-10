use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

/// Implement the [`Regex`] and [`Ctor`] traits any type implements [`Regex`].
#[derive(Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrap<I, C> {
    inner: I,
    marker: PhantomData<C>,
}

def_not!(Wrap<I, C>);

impl<I: Debug, C> Debug for Wrap<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wrap").field("inner", &self.inner).finish()
    }
}

impl<I: Clone, C> Clone for Wrap<I, C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<I, C> From<I> for Wrap<I, C> {
    fn from(value: I) -> Self {
        Self {
            inner: value,
            marker: PhantomData,
        }
    }
}

impl<I, C> Wrap<I, C> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: I) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &I {
        &self.inner
    }

    pub fn set_inner(&mut self, inner: I) -> &mut Self {
        self.inner = inner;
        self
    }

    pub fn into_inner(self) -> I {
        self.inner
    }
}

impl<T, C> Wrap<T, C>
where
    T: Regex<C>,
{
    pub fn regex(regex: T) -> Self {
        Self::new(regex)
    }
}

#[derive(Debug, Clone)]
pub struct BoxedRegex<T> {
    inner: Box<T>,
}

impl<C, T> Regex<C> for BoxedRegex<T>
where
    T: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, T, H> Ctor<'a, C, O, O, H> for BoxedRegex<T>
where
    T: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        handler.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<T, C> Wrap<BoxedRegex<T>, C>
where
    T: Regex<C>,
{
    pub fn r#box(regex: T) -> Self {
        Self::new(BoxedRegex {
            inner: Box::new(regex),
        })
    }
}

impl<T, C> Wrap<std::rc::Rc<T>, C>
where
    T: Regex<C>,
{
    pub fn rc(regex: T) -> Self {
        Self::new(std::rc::Rc::new(regex))
    }
}

impl<T, C> Wrap<std::sync::Arc<T>, C>
where
    T: Regex<C>,
{
    pub fn arc(regex: T) -> Self {
        Self::new(std::sync::Arc::new(regex))
    }
}

impl<T, C> Wrap<std::cell::Cell<T>, C>
where
    T: Regex<C>,
{
    pub fn cell(regex: T) -> Self {
        Self::new(std::cell::Cell::new(regex))
    }
}

impl<T, C> Wrap<std::sync::Mutex<T>, C>
where
    T: Regex<C>,
{
    pub fn mutex(regex: T) -> Self {
        Self::new(std::sync::Mutex::new(regex))
    }
}

impl<T, C> Wrap<std::cell::RefCell<T>, C>
where
    T: Regex<C>,
{
    pub fn refcell(regex: T) -> Self {
        Self::new(std::cell::RefCell::new(regex))
    }
}

impl<'a, C> Wrap<std::sync::Arc<dyn Regex<C> + 'a>, C> {
    pub fn dyn_arc(regex: impl Regex<C> + 'a) -> Self {
        Self::new(std::sync::Arc::new(regex))
    }
}

impl<'a, C> Wrap<Box<dyn Regex<C> + 'a>, C> {
    pub fn dyn_box(regex: impl Regex<C> + 'a) -> Self {
        Self::new(Box::new(regex))
    }
}

impl<'a, C> Wrap<std::rc::Rc<dyn Regex<C> + 'a>, C> {
    pub fn dyn_rc(regex: impl Regex<C> + 'a) -> Self {
        Self::new(std::rc::Rc::new(regex))
    }
}

impl<C, I> Regex<C> for Wrap<I, C>
where
    I: Regex<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner().try_parse(ctx)
    }
}

impl<'a, C, O, H, I> Ctor<'a, C, O, O, H> for Wrap<I, C>
where
    I: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}
