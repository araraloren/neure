use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Conditional zero-width assertion combinator that validates pattern match outcomes.
///
/// [`Assert<T>`] provides a generalized zero-width assertion that succeeds only when an inner pattern's
/// match result (success/failure) exactly matches an expected boolean condition. It never consumes
/// input regardless of outcome, and serves as a unified implementation for both positive (`Peek`) and
/// negative (`Not`) lookahead assertions. This combinator is essential for implementing contextual
/// constraints where specific patterns must either appear or be absent at particular positions.
///
/// # Regex
/// - **Success condition**: When `(inner_pattern.matches() == expected_value)` evaluates to `true`
///   - Returns zero-length span at current position (`Span::new(ctx.beg(), 0)`)
///   - Parser position remains unchanged
/// - **Failure condition**: When actual match result contradicts expected value
///   - Returns `Error::Assert` without consuming input
///   - Context position is reset to pre-match state
/// - **Zero-width guarantee**: Never advances parser position in any scenario
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Assert<T> {
    pat: T,
    value: bool,
}

impl<T> Assert<T> {
    pub const fn new(pat: T, value: bool) -> Self {
        Self { pat, value }
    }

    pub const fn pat(&self) -> &T {
        &self.pat
    }

    pub const fn value(&self) -> bool {
        self.value
    }

    pub const fn pat_mut(&mut self) -> &mut T {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: T) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = value;
        self
    }

    pub fn with_value(mut self, value: bool) -> Self {
        self.value = value;
        self
    }
}

impl_not_for_regex!(Assert<T>);

impl<'a, C, O, T, H> Ctor<'a, C, O, H> for Assert<T>
where
    T: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, T> Regex<C> for Assert<T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let offset = ctx.offset();

        crate::debug_regex_beg!("Assert", offset);
        let ret = if ctx.try_mat(&self.pat).is_ok() == self.value {
            Ok(Span::new(offset, 0))
        } else if self.value {
            Err(Error::AssertTrue)
        } else {
            Err(Error::AssertFalse)
        };

        ctx.set_offset(offset); // force reset the offset
        crate::debug_regex_reval!("Assert", ret)
    }
}

///
/// Conditional zero-width assertion combinator that validates pattern match outcomes.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let langs = regex::array([b"rust".as_ref(), b"jawa", b"golang"]);
///     let mut ctx = BytesCtx::new(b"javascript is not awesome!");
///
///     assert_eq!(ctx.try_mat(&regex::assert(langs, false))?, Span::new(0, 0));
/// #   Ok(())
/// # }
/// ```
pub const fn assert<T>(pat: T, value: bool) -> Assert<T> {
    Assert::new(pat, value)
}

///
/// Negative lookahead assertion combinator that succeeds when its pattern fails to match.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let re = regex::not("]]]");
///     let mut ctx = CharsCtx::new("[123,456,789]");
///
///     assert_eq!(ctx.try_mat(&re)?, Span::new(0, 0));
///     Ok(())
/// # }
/// ```
pub const fn not<T>(pat: T) -> crate::regex::Assert<T> {
    crate::regex::Assert::new(pat, false)
}

///
/// Negative lookahead assertion combinator that succeeds when its pattern fails to match.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let parser = regex::peek(b"rust");
///     let mut ctx = BytesCtx::new(b"rust is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 0));
/// #   Ok(())
/// # }
/// ```
pub const fn peek<T>(pat: T) -> crate::regex::Assert<T> {
    crate::regex::Assert::new(pat, true)
}
