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
use crate::re::Wrapped;

pub struct DynamicBoxedRegex<'a, C> {
    inner: Box<dyn Regex<C> + 'a>,
}

impl<'a, C> DynamicBoxedRegex<'a, C> {
    pub fn new(inner: impl Regex<C> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<C> Regex<C> for DynamicBoxedRegex<'_, C> {
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicBoxedRegex<'_, C>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Wrapped for DynamicBoxedRegex<'a, C> {
    type Inner = Box<dyn Regex<C> + 'a>;

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

pub struct DynamicArcRegex<'a, C> {
    inner: Arc<dyn Regex<C> + 'a>,
}

impl<'a, C> DynamicArcRegex<'a, C> {
    pub fn new(inner: impl Regex<C> + 'a) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<C> Regex<C> for DynamicArcRegex<'_, C> {
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicArcRegex<'_, C>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Wrapped for DynamicArcRegex<'a, C> {
    type Inner = Arc<dyn Regex<C> + 'a>;

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

pub struct DynamicRcRegex<'a, C> {
    inner: Rc<dyn Regex<C> + 'a>,
}

impl<'a, C> DynamicRcRegex<'a, C> {
    pub fn new(inner: impl Regex<C> + 'a) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<C> Regex<C> for DynamicRcRegex<'_, C> {
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicRcRegex<'_, C>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C> Wrapped for DynamicRcRegex<'a, C> {
    type Inner = Rc<dyn Regex<C> + 'a>;

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
