use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::new_span_inc;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Matches the absolute start of input (position zero).
///
/// [`AnchorStart`] is a zero-width assertion that succeeds only at the very beginning of the input stream,
/// analogous to the `^` anchor in regular expressions but without multi-line support. It consumes no input
/// on success and serves as a positional constraint rather than a content matcher.
///
/// # Regex
///
/// - **Success**: At position zero (offset 0)
///   - Returns zero-length span at start position
///   - Does not consume any input
/// - **Failure**: At any position greater than zero
///   - Returns [`Error::AnchorStart`]
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// The constructed value typically represents a positional marker rather than content.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let anchor = regex::start();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&anchor)?, Span::new(0, 0));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorStart;

impl_not_for_regex!(AnchorStart);

impl AnchorStart {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for AnchorStart
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C> Regex<C> for AnchorStart
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        debug_regex_beg!("AnchorStart", ctx.offset());

        let ret = if ctx.offset() == 0 {
            Ok(new_span_inc(ctx, 0))
        } else {
            Err(Error::AnchorStart)
        };

        debug_regex_reval!("AnchorStart", ret)
    }
}

///
/// Matches the absolute start of input (position zero).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let pos = regex::start();
///     let rust = regex::literal("rust");
///     let year = neu::digit(10).count::<4>();
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&pos)?, Span::new(0, 0));
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&year)?, Span::new(4, 4));
///
///     Ok(())
/// # }
/// ```
pub const fn start() -> AnchorStart {
    AnchorStart::new()
}

///
/// Matches the absolute end of input (position equal to input length).
///
/// [`AnchorEnd`] is a zero-width assertion that succeeds only at the very end of the input stream,
/// analogous to the `$` anchor in regular expressions but without multi-line support. It consumes no input
/// on success and serves as a positional constraint rather than a content matcher.
///
/// # Regex
///
/// - **Success**: At position equal to input length
///   - Returns zero-length span at end position
///   - Does not consume any input
/// - **Failure**: At any position before the end
///   - Returns [`Error::AnchorEnd`]
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
/// The constructed value typically represents a positional marker rather than content.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let anchor = regex::end();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&regex::consume(8))?, Span::new(0, 8));
///     assert_eq!(ctx.try_mat(&anchor)?, Span::new(8, 0));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnchorEnd;

impl_not_for_regex!(AnchorEnd);

impl AnchorEnd {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for AnchorEnd
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C> Regex<C> for AnchorEnd
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        debug_regex_beg!("AnchorEnd", ctx.offset());

        let ret = if ctx.offset() == ctx.len() {
            Ok(new_span_inc(ctx, 0))
        } else {
            Err(Error::AnchorEnd)
        };

        debug_regex_reval!("AnchorEnd", ret)
    }
}

///
/// Matches the absolute end of input (position equal to input length).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let rust = regex::literal("rust");
///     let year = neu::digit(10).count::<4>();
///     let end = regex::end();
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&year)?, Span::new(4, 4));
///     assert_eq!(ctx.try_mat(&end)?, Span::new(8, 0));
///
///     Ok(())
/// # }
/// ```
pub const fn end() -> AnchorEnd {
    AnchorEnd::new()
}
