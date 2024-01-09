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

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicBoxedCtor<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicBoxedCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.constrct(ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrapped for DynamicBoxedCtor<'a, 'b, C, M, O, H, A> {
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

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.constrct(ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrapped for DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A> {
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

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicArcCtor<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicArcCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.constrct(ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrapped for DynamicArcCtor<'a, 'b, C, M, O, H, A> {
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

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicRcCtor<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicRcCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.inner.constrct(ctx, handler)
    }
}

impl<'a, 'b, C, M, O, H, A> Wrapped for DynamicRcCtor<'a, 'b, C, M, O, H, A> {
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

///
/// For use in recursive parsers,
/// it will construct a type that implements `Ctor<'_, C, M, O>`
/// from a closure(`Fn(&mut C) -> Result<O, Error>`).
///
/// # Example 1
///
/// ```
/// # use neure::{err::Error, prelude::*, re::RecursiveCtor};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     pub fn parser<'a: 'b, 'b>(
///         ctor: RecursiveCtor<'b, BytesCtx<'a>, &'a [u8]>,
///     ) -> impl Fn(&mut BytesCtx<'a>) -> Result<&'a [u8], Error> + 'b {
///         move |ctx| {
///             let re = u8::is_ascii_lowercase.repeat_one();
///             let re = ctor
///                 .clone()
///                 .or(re)
///                 .quote(b"{", b"}")
///                 .or(ctor.clone().or(re).quote(b"[", b"]"));
///
///             ctx.ctor(&re)
///         }
///     }
///     
///     // into_dyn_ctor using by rec_parser
///     let parser = re::ctor::rec_parser(parser);
///
///     assert_eq!(BytesCtx::new(b"[a]").ctor(&parser)?, b"a");
///     assert_eq!(BytesCtx::new(b"{{[[{[{b}]}]]}}").ctor(&parser)?, b"b");
///     assert_eq!(BytesCtx::new(b"[{{{[c]}}}]").ctor(&parser)?, b"c");
///     Ok(())
/// # }
/// ```
///
/// # Example 2
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let num = u8::is_ascii_digit
///         .repeat_one()
///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
///         .map(map::from_str::<usize>());
///     let num = num.clone().sep_once(b",", num);
///     let re = re::ctor::into_dyn_ctor(|ctx: &mut BytesCtx| ctx.ctor(&num));
///
///     assert_eq!(BytesCtx::new(b"3,0").ctor(&re)?, (3, 0));
///     assert_eq!(BytesCtx::new(b"2,1").ctor(&re)?, (2, 1));
///     assert_eq!(BytesCtx::new(b"0,3").ctor(&re)?, (0, 3));
///     Ok(())
/// # }
/// ```
pub struct A;
// pub fn into_dyn_ctor<'a, 'b, C, M, O, H, A>(
//     invoke: impl Ctor<'a, C, M, O, H, A> + 'b,
// ) -> DynamicBoxedCtor<'a, 'b, C, M, O, H, A>
// where
//     C: Context<'a>,
// {
//     DynamicBoxedCtor::new(Rc::new(invoke))
// }

// pub fn into_dyn_ctor_sync<'a, 'b, C, M, O, H, A>(
//     invoke: impl Ctor<'a, C, M, O, H, A> + Send + 'b,
// ) -> DynamicCtorSync<'a, 'b, C, M, O, H, A>
// where
//     C: Context<'a>,
// {
//     DynamicCtorSync::new(Box::new(invoke))
// }

// pub trait DynamicCtorHelper<'a, 'b, C, M, O, H, A>
// where
//     C: Context<'a>,
// {
//     fn into_dyn_ctor(self) -> DynamicBoxedCtor<'a, 'b, C, M, O, H, A>;
// }

// impl<'a, 'b, C, O, M, T, H, A> DynamicCtorHelper<'a, 'b, C, M, O, H, A> for T
// where
//     C: Context<'a>,
//     T: Ctor<'a, C, M, O, H, A> + 'b,
// {
//     fn into_dyn_ctor(self) -> DynamicBoxedCtor<'a, 'b, C, M, O, H, A> {
//         DynamicBoxedCtor::new(Rc::new(self))
//     }
// }

// pub trait DynamicCtorHelperSync<'a, 'b, C, M, O, H, A>
// where
//     C: Context<'a>,
// {
//     fn into_dyn_ctor_sync(self) -> DynamicCtorSync<'a, 'b, C, M, O, H, A>;
// }

// impl<'a, 'b, C, O, M, T, H, A> DynamicCtorHelperSync<'a, 'b, C, M, O, H, A> for T
// where
//     C: Context<'a>,
//     T: Ctor<'a, C, M, O, H, A> + Send + 'b,
// {
//     fn into_dyn_ctor_sync(self) -> DynamicCtorSync<'a, 'b, C, M, O, H, A> {
//         DynamicCtorSync::new(Box::new(self))
//     }
// }
