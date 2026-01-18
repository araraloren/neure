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
/// A zero-width regex combinator that always fails without consuming any input.
///
/// [`FailRegex`] acts as an atomic failure primitive in the regex combinator system—like a gate
/// that never opens. It immediately rejects any input stream while preserving the parser position,
/// ensuring no characters are consumed during its execution. This zero-width failure combinator
/// compiles down to a single error-returning instruction under compiler optimizations,
/// yet plays a pivotal role in parser logic composition.
///
/// # Regex
///
/// - **Always fail** at any position in the input stream
/// - **Zero-width match**: Returns Err([`Error::Fail`])
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
pub struct FailRegex;

impl_not_for_regex!(FailRegex);

impl FailRegex {
    pub fn new() -> Self {
        Self
    }
}

impl<'a, C> Regex<C> for FailRegex
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        crate::debug_regex_reval!("FailRegex", Err(Error::Fail))
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for FailRegex
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
/// A zero-width regex combinator that always fails without consuming any input.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let fail = regex::fail();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert!(ctx.try_mat(&fail).is_err());
///
/// #   Ok(())
/// # }
/// ```
pub fn fail() -> FailRegex {
    FailRegex::new()
}
