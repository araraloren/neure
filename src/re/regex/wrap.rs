use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::re::Wrappable;

/// Implement the [`Regex`] and [`Ctor`] traits for any type that implements [`Wrappable`]
/// and whose [`Inner`](crate::re::Wrappable#Inner) type implements [`Regex`].
///
/// # Example
/// ```
/// ```
#[derive(Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrap<I, C> {
    pub(crate) value: I,
    marker: PhantomData<C>,
}

def_not!(Wrap<I, C>);

impl<I: Debug, C> Debug for Wrap<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wrap").field("value", &self.value).finish()
    }
}

impl<I: Clone, C> Clone for Wrap<I, C> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

impl<I, C> From<I> for Wrap<I, C>
where
    I: Wrappable,
{
    fn from(value: I) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<I, C> Wrap<I, C>
where
    I: Wrappable,
{
    pub fn new(inner: I::Inner) -> Self {
        Self {
            value: I::wrap(inner),
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: I::Inner) -> Self {
        self.value = I::wrap(inner);
        self
    }

    pub fn inner(&self) -> &I::Inner {
        self.value.inner()
    }

    pub fn set_inner(&mut self, inner: I::Inner) -> &mut Self {
        self.value = I::wrap(inner);
        self
    }
}

impl<T, C> Wrap<Wrap<T, C>, C>
where
    T: Regex<C>,
{
    pub fn regex(regex: T) -> Self {
        Self::new(regex)
    }
}

impl<T, C> Wrap<Box<T>, C>
where
    T: Regex<C>,
{
    pub fn r#box(regex: T) -> Self {
        Self::new(regex)
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
    I: Wrappable,
    I::Inner: Regex<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner().try_parse(ctx)
    }
}

impl<'a, C, O, H, A, I> Ctor<'a, C, O, O, H, A> for Wrap<I, C>
where
    I: Wrappable,
    I::Inner: Regex<C>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<I, C> Wrappable for Wrap<I, C> {
    type Inner = I;

    fn wrap(inner: Self::Inner) -> Self {
        Self {
            value: inner,
            marker: PhantomData,
        }
    }

    fn inner(&self) -> &Self::Inner {
        &self.value
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.value
    }
}
