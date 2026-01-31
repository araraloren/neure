use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::new_span_inc;
use crate::err::Error;
use crate::regex::Regex;
use crate::span::Span;

///
/// Matches an exact literal with zero-copy efficiency.
///
/// [`Literal`] provides exact byte/element-wise matching of a predefined sequence, succeeding only when
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
/// # Example1
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let parser = regex::literal(b"magic");
///     let mut ctx = BytesCtx::new(b"magic 0xff");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 5));
/// #   Ok(())
/// # }
/// ```
///
/// # Example2
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let parser = regex::literal("hello");
///     let mut ctx = CharsCtx::new("hello world");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 5));
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Literal<'a, T: ?Sized> {
    literal: &'a T,
}

impl<'a, T: ?Sized> Clone for Literal<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Copy for Literal<'a, T> {}

impl<'a, T: ?Sized> core::ops::Not for Literal<'a, T> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<'a, T: ?Sized> Literal<'a, T> {
    pub const fn new(literal: &'a T) -> Self {
        Self { literal }
    }
}

impl<'a, C, O, T, H> Ctor<'a, C, O, H> for Literal<'_, T>
where
    T: ?Sized + LiteralTy + 'a,
    H: Handler<C, Out = O>,
    C: Match<'a, Orig<'a> = <T as LiteralTy>::Orig<'a>>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, T> Regex<C> for Literal<'_, T>
where
    T: ?Sized + LiteralTy + 'a,
    C: Context<'a, Orig<'a> = <T as LiteralTy>::Orig<'a>>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ret = Err(Error::Literal);
        let slice_len = self.literal.length();
        let remaining_len = ctx.len() - ctx.offset();

        crate::debug_regex_beg!("Literal", ctx.offset());
        if remaining_len >= slice_len && self.literal.prefix_of(&ctx.orig()?) {
            ret = Ok(new_span_inc(ctx, slice_len));
        }
        crate::debug_regex_reval!("Literal", ret)
    }
}

pub trait LiteralTy {
    type Orig<'a>;

    fn length(&self) -> usize;

    fn prefix_of<'a>(&self, other: &Self::Orig<'a>) -> bool;
}

impl LiteralTy for str {
    type Orig<'a> = &'a str;

    fn length(&self) -> usize {
        str::len(self)
    }

    fn prefix_of<'a>(&self, other: &Self::Orig<'a>) -> bool {
        str::starts_with(other, self)
    }
}

#[cfg(feature = "alloc")]
impl LiteralTy for crate::alloc::String {
    type Orig<'a> = &'a str;

    fn length(&self) -> usize {
        str::len(self)
    }

    fn prefix_of<'a>(&self, other: &Self::Orig<'a>) -> bool {
        str::starts_with(other, self.as_str())
    }
}

impl LiteralTy for [u8] {
    type Orig<'a> = &'a [u8];

    fn length(&self) -> usize {
        <[u8]>::len(self)
    }

    fn prefix_of<'a>(&self, other: &Self::Orig<'a>) -> bool {
        <[u8]>::starts_with(other, self)
    }
}

impl<const N: usize> LiteralTy for [u8; N] {
    type Orig<'a> = &'a [u8];

    fn length(&self) -> usize {
        <[u8]>::len(self)
    }

    fn prefix_of<'a>(&self, other: &Self::Orig<'a>) -> bool {
        <[u8]>::starts_with(other, self)
    }
}

///
/// Matches an exact literal with zero-copy efficiency.
///
/// # Example
///
/// Matches an exact literal string with Unicode-aware correctness.
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///      let rust = regex::literal("rust");
///      let mut ctx = CharsCtx::new("rust2023");
///
///      assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
///
/// # Example
///
/// Matches an exact literal slice of elements with zero-copy efficiency.
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let parser = regex::literal(&[0xff, 0xff]);
///     let mut ctx = BytesCtx::new(&[0xff, 0xff, 0x12]);
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 2));
/// #   Ok(())
/// # }
/// ```
pub const fn literal<T: LiteralTy + ?Sized>(lit: &T) -> Literal<'_, T> {
    Literal::new(lit)
}
