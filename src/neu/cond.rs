use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;

pub trait NeuCond<'a, C>
where
    C: Context<'a>,
{
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error>;
}

pub trait Condition<'a, C>
where
    C: Context<'a>,
{
    type Out<F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>;
}

///
/// # Check the condition when match.
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let str = neu::not(b'"')
///         .repeat_one_more()
///         // avoid match escape sequence
///         .set_cond(|ctx: &BytesCtx, (item_offset, _item): &(usize, u8)| {
///             Ok(!ctx.orig_at(ctx.offset() + item_offset)?.starts_with(b"\\\""))
///         })
///         // match the escape sequence in another regex
///         .or(b"\\\"")
///         .repeat(1..)
///         .pat();
///     let mut ctx = BytesCtx::new(br#""Hello world from \"rust\"!""#);
///
///     assert_eq!(ctx.try_mat(&str.quote(b"\"", b"\""))?, Span::new(0, 28));
///     Ok(())
/// # }
/// ```
impl<'a, C, F> NeuCond<'a, C> for F
where
    C: Context<'a>,
    F: Fn(&C, &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error> {
        let ret = (self)(ctx, item);

        crate::trace_retval!("Fn", "NeuCond", item, ret)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullCond;

impl<'a, C> NeuCond<'a, C> for NullCond
where
    C: Context<'a>,
{
    fn check(&self, _: &C, _item: &(usize, C::Item)) -> Result<bool, Error> {
        Ok(true)
    }
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegexCond<'a, C, T> {
    regex: T,
    marker: PhantomData<(&'a (), C)>,
}

impl<C, T> Debug for RegexCond<'_, C, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegexCond")
            .field("regex", &self.regex)
            .finish()
    }
}

impl<C, T> Clone for RegexCond<'_, C, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            regex: self.regex.clone(),
            marker: self.marker,
        }
    }
}

impl<C, T> RegexCond<'_, C, T> {
    pub fn new(regex: T) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<'a, C, T> NeuCond<'a, C> for RegexCond<'a, C, T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error> {
        let mut ctx = ctx.clone_with(ctx.orig_at(ctx.offset() + item.0)?);
        let ret = ctx.try_mat(&self.regex);

        crate::trace_retval!("RegexCond", "NeuCond", item, ret.is_ok());
        Ok(ret.is_ok())
    }
}

///
/// Create a condition using in [`Condition`] base on regex.
///
/// # Example
///
///```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let escape = b'\\'.then(b'"');
///     let str = neu::not(b'"')
///         .repeat_one_more()
///         // avoid match escape sequence
///         .set_cond(neu::re_cond(regex::not(escape)))
///         // match the escape sequence in another regex
///         .or(escape)
///         .repeat(1..)
///         .pat();
///     let mut ctx = BytesCtx::new(br#""Hello world from \"rust\"!""#);
///
///     assert_eq!(ctx.try_mat(&str.quote(b"\"", b"\""))?, Span::new(0, 28));
///     Ok(())
/// # }
/// ```
pub fn re_cond<'a, C, T>(regex: T) -> RegexCond<'a, C, T> {
    RegexCond::new(regex)
}
