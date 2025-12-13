use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Consumes a fixed number of elements without content validation.
///
/// `Consume` is a low-level combinator that advances the parser by a specified number of elements
/// regardless of their content. It succeeds if sufficient input remains, failing otherwise. This
/// combinator is useful for skipping fixed-size fields or reserved space where content validation
/// is unnecessary, deferred, or handled separately.
///
/// # Regex
///
/// - **Success**: When remaining input length >= specified count
///   - Returns span covering exactly `count` elements
///   - Consumes exactly `count` elements from input stream
/// - **Failure**: When insufficient input remains
///   - Returns [`Error::Consume`]
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let consume = regex::consume(8);
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&consume)?, Span::new(0, 8));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Consume(usize);

impl_not_for_regex!(Consume);

impl Consume {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for Consume
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

impl<'a, C> Regex<C> for Consume
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("Consume", ctx.beg());

        let ret = if ctx.remaining_len() >= self.0 {
            Ok(ctx.inc(self.0))
        } else {
            Err(Error::Consume)
        };

        debug_regex_reval!("Consume", ret)
    }
}

///
/// Consumes a fixed number of elements without content validation.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let parser = regex::consume(6);
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
pub fn consume(len: usize) -> Consume {
    Consume::new(len)
}

///
/// Consumes all remaining elements from current position to end of input.
///
/// `ConsumeAll` is a terminal combinator that matches the entire remaining input stream from the current
/// position to the end. It always succeeds (even with zero remaining elements) and consumes all available
/// data. This combinator is useful for capturing trailing content, implementing greedy captures, or
/// terminating parsers that should process all remaining input.
///
/// # Regex
///
/// - **Always succeeds**:
///   - When input remains: Returns span covering all remaining elements
///   - At end of input: Returns zero-length span at current position
/// - **Consumption**:
///   - Advances parser to end of input (position = input length)
///   - Consumes exactly `remaining_length` elements
///   - Never fails (unlike `Consume` which fails on insufficient input)
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let parser = regex::consume_all();
///     let mut ctx = BytesCtx::new(b"rust is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 19));
/// #  Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumeAll;

impl_not_for_regex!(ConsumeAll);

impl ConsumeAll {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, O, H> for ConsumeAll
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

impl<'a, C> Regex<C> for ConsumeAll
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("ConsumeAll", ctx.beg());

        let len = ctx.len().saturating_sub(ctx.beg());
        let ret = Ok(ctx.inc(len));

        debug_regex_reval!("ConsumeAll", ret)
    }
}

///
/// Consumes all remaining elements from current position to end of input.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let parser = regex::consume_all();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 8));
/// #   Ok(())
/// # }
/// ```
pub fn consume_all() -> ConsumeAll {
    ConsumeAll::new()
}
