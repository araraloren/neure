use std::rc::Rc;
use std::sync::Arc;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;
use crate::re::Wrappable;

pub struct DynamicBoxedCtor<'a, 'b, C, M, O, H, A> {
    inner: Box<dyn Ctor<'a, C, M, O, H, A> + 'b>,
}

impl<'a, 'b, C, M, O, H, A> DynamicBoxedCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    pub fn new(inner: impl Ctor<'a, C, M, O, H, A> + 'b) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<C, M, O, H, A> Regex<C> for DynamicBoxedCtor<'_, '_, C, M, O, H, A> {
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicBoxedCtor not support `Regex` trait")
    }
}

impl<'a, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicBoxedCtor<'a, '_, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(self.inner.as_ref(), ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for DynamicBoxedCtor<'a, 'b, C, M, O, H, A> {
    type Inner = Box<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}

pub struct DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A> {
    inner: Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b>,
}

impl<'a, 'b, C, M, O, H, A> DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    pub fn new(inner: impl Ctor<'a, C, M, O, H, A> + Send + 'b) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<C, M, O, H, A> Regex<C> for DynamicBoxedCtorSync<'_, '_, C, M, O, H, A> {
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicBoxedCtorSync not support `Regex` trait")
    }
}

impl<'a, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicBoxedCtorSync<'a, '_, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.construct(ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A> {
    type Inner = Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}

#[derive(Clone)]
pub struct DynamicArcCtor<'a, 'b, C, M, O, H, A> {
    inner: Arc<dyn Ctor<'a, C, M, O, H, A> + 'b>,
}

impl<'a, 'b, C, M, O, H, A> DynamicArcCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    pub fn new(inner: impl Ctor<'a, C, M, O, H, A> + 'b) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<C, M, O, H, A> Regex<C> for DynamicArcCtor<'_, '_, C, M, O, H, A> {
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicArcCtor not support `Regex` trait")
    }
}

impl<'a, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicArcCtor<'a, '_, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(self.inner.as_ref(), ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for DynamicArcCtor<'a, 'b, C, M, O, H, A> {
    type Inner = Arc<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}

#[derive(Clone)]
pub struct DynamicRcCtor<'a, 'b, C, M, O, H, A> {
    inner: Rc<dyn Ctor<'a, C, M, O, H, A> + 'b>,
}

impl<'a, 'b, C, M, O, H, A> DynamicRcCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    pub fn new(inner: impl Ctor<'a, C, M, O, H, A> + 'b) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<C, M, O, H, A> Regex<C> for DynamicRcCtor<'_, '_, C, M, O, H, A> {
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicRcCtor not support `Regex` trait")
    }
}

impl<'a, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicRcCtor<'a, '_, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(self.inner.as_ref(), ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for DynamicRcCtor<'a, 'b, C, M, O, H, A> {
    type Inner = Rc<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
