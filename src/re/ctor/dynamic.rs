use std::rc::Rc;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub type DynamicCtorHandler<'a, 'b, C, M, O, H, A> = Rc<dyn Ctor<'a, C, M, O, H, A> + 'b>;

#[derive(Clone)]
pub struct DynamicCtor<'a, 'b, C, M, O, H, A> {
    inner: DynamicCtorHandler<'a, 'b, C, M, O, H, A>,
}

def_not!(DynamicCtor<'a, 'b, C, M, O, H, A>);

impl<'a, 'b, C, M, O, H, A> DynamicCtor<'a, 'b, C, M, O, H, A> {
    pub fn new(inner: DynamicCtorHandler<'a, 'b, C, M, O, H, A>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicCtorHandler<'a, 'b, C, M, O, H, A>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicCtorHandler<'a, 'b, C, M, O, H, A> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicCtorHandler<'a, 'b, C, M, O, H, A> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicCtorHandler<'a, 'b, C, M, O, H, A>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicCtor<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicCtor<'a, 'b, C, M, O, H, A>
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

pub type DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A> =
    Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b>;

pub struct DynamicCtorSync<'a, 'b, C, M, O, H, A> {
    inner: DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A>,
}

def_not!(DynamicCtorSync<'a, 'b, C, M, O, H, A>);

impl<'a, 'b, C, M, O, H, A> DynamicCtorSync<'a, 'b, C, M, O, H, A> {
    pub fn new(inner: DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicCtorHandlerSync<'a, 'b, C, M, O, H, A>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Regex<C> for DynamicCtorSync<'a, 'b, C, M, O, H, A> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O, H, A> Ctor<'a, C, M, O, H, A> for DynamicCtorSync<'a, 'b, C, M, O, H, A>
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
pub fn into_dyn_ctor<'a, 'b, C, M, O, H, A>(
    invoke: impl Ctor<'a, C, M, O, H, A> + 'b,
) -> DynamicCtor<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    DynamicCtor::new(Rc::new(invoke))
}

pub fn into_dyn_ctor_sync<'a, 'b, C, M, O, H, A>(
    invoke: impl Ctor<'a, C, M, O, H, A> + Send + 'b,
) -> DynamicCtorSync<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    DynamicCtorSync::new(Box::new(invoke))
}

pub trait DynamicCtorHelper<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'a, 'b, C, M, O, H, A>;
}

impl<'a, 'b, C, O, M, T, H, A> DynamicCtorHelper<'a, 'b, C, M, O, H, A> for T
where
    C: Context<'a>,
    T: Ctor<'a, C, M, O, H, A> + 'b,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'a, 'b, C, M, O, H, A> {
        DynamicCtor::new(Rc::new(self))
    }
}

pub trait DynamicCtorHelperSync<'a, 'b, C, M, O, H, A>
where
    C: Context<'a>,
{
    fn into_dyn_ctor_sync(self) -> DynamicCtorSync<'a, 'b, C, M, O, H, A>;
}

impl<'a, 'b, C, O, M, T, H, A> DynamicCtorHelperSync<'a, 'b, C, M, O, H, A> for T
where
    C: Context<'a>,
    T: Ctor<'a, C, M, O, H, A> + Send + 'b,
{
    fn into_dyn_ctor_sync(self) -> DynamicCtorSync<'a, 'b, C, M, O, H, A> {
        DynamicCtorSync::new(Box::new(self))
    }
}
