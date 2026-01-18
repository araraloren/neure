use core::fmt::Debug;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::span::Span;
use crate::err::Error;
use crate::regex::Regex;

use super::impl_not_for_regex;

///
/// A zero-width regex combinator that always succeeds without consuming any input.
///
/// [`EmptyRegex`] is a fundamental building block that matches the empty string at any position
/// in the input stream. It always succeeds (never fails) and consumes zero elements, making it
/// ideal for building optional patterns, zero-width assertions, and serving as a neutral element
/// in parser composition. Unlike position-constrained anchors (e.g., [`AnchorEnd`](crate::regex::AnchorEnd)), [`EmptyRegex`]
/// succeeds at every position regardless of context.
///
/// # Regex
///
/// - **Always succeeds** at any position in the input stream
/// - **Zero-width match**: Returns span with length 0 at current position
/// - **No consumption**: Parser position remains unchanged after match
/// - **No failure cases**: Never returns an error (not even at input end)
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let parser = regex::empty();
///     let mut ctx = BytesCtx::new(b"rust is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 0));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmptyRegex;

impl_not_for_regex!(EmptyRegex);

impl EmptyRegex {
    pub fn new() -> Self {
        Self
    }
}

impl<'a, C> Regex<C> for EmptyRegex
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        crate::debug_regex_reval!("EmptyRegex", Ok(Span::new(ctx.offset(), 0)))
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for EmptyRegex
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self);

        handler.invoke(ctx, &ret?).map_err(Into::into)
    }
}

///
/// A zero-width regex combinator that always succeeds without consuming any input.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let empty = regex::empty();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&empty)?, Span::new(0, 0));
///
/// #   Ok(())
/// # }
/// ```
pub fn empty() -> EmptyRegex {
    EmptyRegex::new()
}
