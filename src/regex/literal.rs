use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Matches an exact literal slice of elements with zero-copy efficiency.
///
/// [`LitSlice`] provides exact byte/element-wise matching of a predefined sequence, succeeding only when
/// the input contains the precise sequence at the current position. It functions as a low-level building block
/// for matching fixed patterns in binary data, text tokens, or structured element sequences with minimal overhead.
///
/// # Regex
///
/// - **Success**: When remaining input starts with exact sequence in `val`
///   - Returns span covering the entire matched sequence
///   - Consumes exactly `val.len()` elements
/// - **Failure**: When input is shorter or sequence mismatch
///   - Returns `Error::LitSlice`
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
///     let parser = regex::lit_slice(b"magic");
///     let mut ctx = BytesCtx::new(b"magic 0xff");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 5));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitSlice<'a, T> {
    val: &'a [T],
}

impl_not_for_regex!(LitSlice<'a, T>);

impl<'a, T> LitSlice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self { val }
    }
}

impl<'a, C, O, T, H> Ctor<'a, C, O, H> for LitSlice<'_, T>
where
    T: PartialOrd + 'a,
    C: Match<'a, Orig<'a> = &'a [T]>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, T> Regex<C> for LitSlice<'_, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig<'a> = &'a [T]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::LitSlice);
        let slice_len = self.val.len();

        crate::debug_regex_beg!("LitSlice", ctx.beg());
        if ctx.remaining_len() >= slice_len && ctx.ctx().orig()?.starts_with(self.val) {
            ret = Ok(ctx.inc(slice_len));
        }
        crate::debug_regex_reval!("LitSlice", ret)
    }
}

///
/// Matches an exact literal slice of elements with zero-copy efficiency.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let parser = regex::lit_slice(&[0xff, 0xff]);
///     let mut ctx = BytesCtx::new(&[0xff, 0xff, 0x12]);
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 2));
/// #   Ok(())
/// # }
/// ```
pub fn lit_slice<T>(lit: &[T]) -> LitSlice<'_, T> {
    LitSlice::new(lit)
}

///
/// Matches an exact literal string with Unicode-aware correctness.
///
/// [`LitString`] provides zero-copy exact matching of a predefined string literal, succeeding only when
/// the input contains the precise sequence of characters at the current position. It respects UTF-8
/// boundaries and performs efficient byte-wise comparison while maintaining Unicode correctness.
///
/// # Regex
///
/// - **Success**: When remaining input starts with exact string in `val`
///   - Returns span covering the entire matched string
///   - Consumes exactly `val.len()` **bytes** (not character count!)
///   - Requires match to start at valid UTF-8 boundary
/// - **Failure**: When any condition fails returns [`Error::LitString`]
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
///     let parser = regex::string("hello");
///     let mut ctx = CharsCtx::new("hello world");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 5));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitString<'a> {
    val: &'a str,
}

impl_not_for_regex!(LitString<'a>);

impl<'a> LitString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for LitString<'_>
where
    C: Match<'a, Orig<'a> = &'a str>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C> Regex<C> for LitString<'_>
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut ret = Err(Error::LitString);
        let literal_len = self.val.len();

        crate::debug_regex_beg!("LitString", ctx.beg());
        if ctx.remaining_len() >= literal_len && ctx.ctx().orig()?.starts_with(self.val) {
            ret = Ok(ctx.inc(literal_len));
        }
        crate::debug_regex_reval!("LitString", self.val, ret)
    }
}

///
/// Matches an exact literal string with Unicode-aware correctness.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///      let rust = regex::string("rust");
///      let mut ctx = CharsCtx::new("rust2023");
///
///      assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
pub fn string(lit: &str) -> LitString<'_> {
    LitString::new(lit)
}
