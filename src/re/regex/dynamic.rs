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

pub struct DynamicBoxedRegex<'a, C, R> {
    inner: Box<dyn Regex<C, Ret = R> + 'a>,
}

impl<'a, C, R> DynamicBoxedRegex<'a, C, R> {
    pub fn new(inner: impl Regex<C, Ret = R> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<C, R> Regex<C> for DynamicBoxedRegex<'_, C, R> {
    type Ret = R;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicBoxedRegex<'_, C, Span>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, R> Wrapped for DynamicBoxedRegex<'a, C, R> {
    type Inner = Box<dyn Regex<C, Ret = R> + 'a>;

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

pub struct DynamicArcRegex<'a, C, R> {
    inner: Arc<dyn Regex<C, Ret = R> + 'a>,
}

impl<'a, C, R> DynamicArcRegex<'a, C, R> {
    pub fn new(inner: impl Regex<C, Ret = R> + 'a) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<C, R> Regex<C> for DynamicArcRegex<'_, C, R> {
    type Ret = R;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicArcRegex<'_, C, Span>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, R> Wrapped for DynamicArcRegex<'a, C, R> {
    type Inner = Arc<dyn Regex<C, Ret = R> + 'a>;

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

pub struct DynamicRcRegex<'a, C, R> {
    inner: Rc<dyn Regex<C, Ret = R> + 'a>,
}

impl<'a, C, R> DynamicRcRegex<'a, C, R> {
    pub fn new(inner: impl Regex<C, Ret = R> + 'a) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<C, R> Regex<C> for DynamicRcRegex<'_, C, R> {
    type Ret = R;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, H, A> Ctor<'a, C, O, O, H, A> for DynamicRcRegex<'_, C, Span>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat_t(self.inner.as_ref())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, R> Wrapped for DynamicRcRegex<'a, C, R> {
    type Inner = Rc<dyn Regex<C, Ret = R> + 'a>;

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
