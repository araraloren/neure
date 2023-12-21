use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub type DynamicCtorHandler<'a, C, O> = Box<dyn Fn(&mut C) -> Result<O, Error> + 'a>;

pub struct DynamicCtor<'a, C, O> {
    inner: DynamicCtorHandler<'a, C, O>,
}

impl<'a, C, O> DynamicCtor<'a, C, O> {
    pub fn new(inner: DynamicCtorHandler<'a, C, O>) -> Self {
        Self { inner }
    }

    pub fn with_inner(mut self, inner: DynamicCtorHandler<'a, C, O>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &DynamicCtorHandler<'a, C, O> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DynamicCtorHandler<'a, C, O> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: DynamicCtorHandler<'a, C, O>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, C, R> Regex<C> for DynamicCtor<'a, C, R> {
    type Ret = Span;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unreachable!("Dynamic invoke not support `Regex` trait")
    }
}

impl<'a, 'b, C, M, O> Ctor<'a, C, M, O> for DynamicCtor<'b, C, O>
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, _: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        (self.inner)(ctx)
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
///     color_eyre::install()?;
///
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
///     color_eyre::install()?;
///     let num = u8::is_ascii_digit
///         .repeat_one()
///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
///         .map(re::map::from_str::<usize>());
///     let num = num.clone().sep_once(b",", num);
///     let re = re::ctor::into_dyn_ctor(|ctx: &mut BytesCtx| ctx.ctor(&num));
///
///     assert_eq!(BytesCtx::new(b"3,0").ctor(&re)?, (3, 0));
///     assert_eq!(BytesCtx::new(b"2,1").ctor(&re)?, (2, 1));
///     assert_eq!(BytesCtx::new(b"0,3").ctor(&re)?, (0, 3));
///     Ok(())
/// # }
/// ```
pub fn into_dyn_ctor<'a, 'b, C, O>(
    invoke: impl Fn(&mut C) -> Result<O, Error> + 'b,
) -> DynamicCtor<'b, C, O>
where
    C: Context<'a>,
{
    DynamicCtor::new(Box::new(invoke))
}

pub trait DynamicCtorHelper<'a, 'b, C, O>
where
    C: Context<'a>,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'b, C, O>;
}

impl<'a, 'b, C, O, T> DynamicCtorHelper<'a, 'b, C, O> for T
where
    C: Context<'a>,
    T: Fn(&mut C) -> Result<O, Error> + 'b,
{
    fn into_dyn_ctor(self) -> DynamicCtor<'b, C, O> {
        DynamicCtor::new(Box::new(self))
    }
}
