use std::fmt::Debug;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedTy<I> {
    pub(crate) value: I,
}

def_not!(WrappedTy<I>);

impl<I: Debug> Debug for WrappedTy<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WrappedTy")
            .field("value", &self.value)
            .finish()
    }
}

impl<I: Clone> Clone for WrappedTy<I> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<I> WrappedTy<I>
where
    I: Wrapped,
{
    pub fn new(inner: I::Inner) -> Self {
        Self {
            value: I::wrap(inner),
        }
    }

    pub fn with_inner(mut self, inner: I::Inner) -> Self {
        self.value = I::wrap(inner);
        self
    }

    pub fn inner(&self) -> &I::Inner {
        self.value.inner()
    }

    pub fn inner_mut(&mut self) -> &mut I::Inner {
        self.value.inner_mut()
    }

    pub fn set_inner(&mut self, inner: I::Inner) -> &mut Self {
        self.value = I::wrap(inner);
        self
    }
}

impl<C, I> Regex<C> for WrappedTy<I>
where
    I: Regex<C>,
{
    type Ret = <I as Regex<C>>::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.value.try_parse(ctx)
    }
}

impl<'a, C, M, O, H, A, I> Ctor<'a, C, M, O, H, A> for WrappedTy<I>
where
    C: Context<'a> + Match<C>,
    I: Ctor<'a, C, M, O, H, A>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(&self.value, ctx, handler)
    }
}

pub trait Wrapped
where
    Self: Sized,
{
    type Inner;

    fn wrap(inner: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;

    fn inner_mut(&mut self) -> &mut Self::Inner;
}

macro_rules! self_wrap {
    ($ty:path) => {
        impl<T> Wrapped for $ty {
            type Inner = $ty;

            fn wrap(inner: Self::Inner) -> Self {
                inner
            }

            fn inner(&self) -> &Self::Inner {
                self
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                self
            }
        }
    };
}

self_wrap!(std::rc::Rc<T>);

self_wrap!(std::cell::Cell<T>);

self_wrap!(std::cell::RefCell<T>);

self_wrap!(std::sync::Arc<T>);

self_wrap!(std::sync::Mutex<T>);
