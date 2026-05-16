mod guard;
mod policy;
#[allow(clippy::module_inception)]
mod regex;

use crate::MayDebug;
use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctor::extract;
use crate::err::Error;
use crate::map::FallibleMap;
use crate::regex::Regex;
use crate::span::Span;

pub use self::guard::CtxGuard;
pub use self::policy::PolicyCtx;
pub use self::regex::RegexCtx;

pub type BytesCtx<'a> = RegexCtx<'a, [u8]>;
pub type CharsCtx<'a> = RegexCtx<'a, str>;

/// A trait representing a context that can be matched against during parsing or pattern matching.
///
/// The `Context` trait abstracts over different types of input data (e.g., byte slices, strings)
/// and provides a uniform interface for:
/// - Querying the current position (offset) within the input
/// - Advancing or rewinding the position
/// - Peeking at data without consuming it
/// - Extracting original data slices at specific positions
pub trait Context<'a> {
    /// The type of the original data slice at a given position.
    type Orig<'b>;

    /// The type of individual elements in this context.
    type Item: MayDebug;

    /// An iterator type that yields `(offset, Item)` pairs.
    type Iter<'b>: Iterator<Item = (usize, Self::Item)>;

    /// Returns the total length of the underlying data in bytes.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the current offset (position) within the context.
    fn offset(&self) -> usize;

    /// Sets the offset to a specific position.
    fn set_offset(&mut self, offset: usize) -> &mut Self;

    /// Increments the offset by a specified amount.
    fn inc(&mut self, offset: usize) -> &mut Self;

    /// Decrements the offset by a specified amount.
    fn dec(&mut self, offset: usize) -> &mut Self;

    /// Peeks at the data starting from the current offset without consuming it.
    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    /// Peeks at the data starting from a specific offset without consuming it.
    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    /// Returns a slice of the original data starting from the current offset.
    fn orig(&self) -> Result<Self::Orig<'a>, Error> {
        self.orig_at(self.offset())
    }

    /// Returns a slice of the original data starting from a specific offset.
    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        self.orig_sub(offset, self.len() - offset)
    }

    /// Returns a subslice of the original data with specified offset and length.
    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error>;

    /// Creates a new context instance at a specific offset.
    fn clone_at(&self, offset: usize) -> Result<Self, Error>
    where
        Self: Sized;
}

/// A trait for types that can perform pattern matching operations on a parsing context.
///
/// The `Match` trait provides methods for attempting to match patterns (regular expressions)
/// against a context. It extends the [`Context`] trait, requiring implementations to also
/// be contexts. This trait is implemented by context types that support matching operations.
pub trait Match<'a>: Context<'a>
where
    Self: Sized,
{
    /// Checks if a pattern can be matched at the current position in the context.
    /// It doesn't provide detailed error information; use [`try_mat`](Match::try_mat) for error details.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     assert!(CharsCtx::with("98", |mut ctx| ctx.is_mat(&neu::digit(10).many1())));
    /// #   Ok(())
    /// # }
    /// ```
    fn is_mat<Pat>(&mut self, pat: &Pat) -> bool
    where
        Pat: Regex<Self> + ?Sized,
    {
        self.try_mat(pat).is_ok()
    }

    /// Attempts to match a pattern at the current position in the context.
    ///
    /// This is the core matching method that attempts to match the provided pattern
    /// against the context starting at the current offset.
    /// On success, it returns a [`Span`] describing the matched range and advances the context offset
    /// to the end of the match.
    /// On failure, it returns an [`Error`] and the offset is unchanged.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     assert_eq!(
    ///         CharsCtx::with("98", |mut ctx| ctx.try_mat(&neu::digit(10).many1()))?,
    ///         Span::new(0, 2)
    ///     );
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

/// A trait for types that support matching with policy constraints.
///
/// The `PolicyMatch` trait extends matching capabilities by allowing constraints
/// (policies) to be applied before and/or after the main pattern match. This is
/// useful for patterns that must be surrounded by specific context or for
/// implementing lookahead/lookbehind assertions.
///
/// Unlike [`Match`] which only matches a single pattern, `PolicyMatch` allows
/// specifying additional patterns that must match before (lookbehind) and/or
/// after (lookahead) the main pattern. All three patterns must succeed for the
/// overall match to succeed.
pub trait PolicyMatch<'a>
where
    Self: Sized,
{
    /// Attempts to match a pattern that must be preceded by a specific pattern.
    ///
    /// This method matches the main pattern only if it is immediately preceded
    /// by the `before` pattern. The `before` pattern is matched but not included
    /// in the returned span. This is similar to a positive lookbehind assertion
    /// in traditional regex engines.
    fn try_mat_before<P, B>(&mut self, pat: &P, before: &B) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, before, &|_: &mut Self| Ok(Span::default()))
    }

    /// Attempts to match a pattern that must be followed by a specific pattern.
    ///
    /// This method matches the main pattern only if it is immediately followed
    /// by the `after` pattern. The `after` pattern is matched but not included
    /// in the returned span. This is similar to a positive lookahead assertion
    /// in traditional regex engines.
    fn try_mat_after<P, A>(&mut self, pat: &P, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, &|_: &mut Self| Ok(Span::default()), after)
    }

    /// Attempts to match a pattern with both before and after policy constraints.
    ///
    /// This is the most general policy matching method. It matches the main pattern
    /// only if it is both preceded by the `before` pattern and followed by the
    /// `after` pattern. This is equivalent to a combination of lookbehind and
    /// lookahead assertions.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{ctx::PolicyMatch, prelude::*};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     assert_eq!(
    ///         CharsCtx::with("Give me 98 cents.", |mut ctx| {
    ///             let before = "Give me".skip_ascii_ws();
    ///             let after = "cents".skip_ascii_ws_leading();
    ///
    ///             ctx.try_mat_policy(&neu::digit(10).many1(), &before, &after)
    ///         })?,
    ///         Span::new(8, 2)
    ///     );
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    fn try_mat_policy<P, B, A>(&mut self, pat: &P, before: &B, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized;
}

/// A trait for performing zero-width assertions on a parsing context.
///
/// The `Assert` trait provides methods to check if a pattern can be matched
/// at the current position in the context without advancing the offset or
/// consuming any input. This is useful for lookahead/lookbehind assertions,
/// validation checks, and conditional parsing logic.
///
/// Unlike the [`Match`] trait which consumes input and advances the offset,
/// `Assert` performs zero-width checks that leave the context position unchanged
/// regardless of success or failure.
///
/// This trait is automatically implemented for all types that implement [`Match<'a>`],
/// allowing any matching context to also support assertions.
pub trait Assert<'a>
where
    Self: Sized,
{
    /// Silent assertion that swallows all errors and returns `false` on failure
    /// without altering context position.
    fn assert<Pat>(&mut self, pat: &Pat) -> bool
    where
        Pat: Regex<Self> + ?Sized,
    {
        self.try_assert(pat).unwrap_or_default()
    }

    /// Precise assertion preserving error details without altering context position.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut ctx = CharsCtx::new("42");
    ///
    ///     assert!(ctx.try_assert(&"42")?);
    ///     assert_eq!(ctx.offset(), 0);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    fn try_assert<Pat>(&mut self, pat: &Pat) -> Result<bool, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

impl<'a, T> Assert<'a> for T
where
    T: Match<'a>,
{
    fn try_assert<Pat>(&mut self, pat: &Pat) -> Result<bool, Error>
    where
        Pat: Regex<Self> + ?Sized,
    {
        let offset = self.offset();
        let ret = self.try_mat(pat);

        self.set_offset(offset);
        Ok(ret.is_ok())
    }
}

/// Extension trait for matching contexts that adds value construction methods.
///
/// The `MatchExt` trait provides a rich set of methods for constructing values
/// from pattern matches. These methods build on the basic [`Match::try_mat`]
/// functionality to enable extracting, transforming, and building typed data
/// from parsed input.
pub trait MatchExt<'a>
where
    Self: Context<'a> + Sized,
{
    /// Constructs a value using a constructor pattern and a custom handler.
    ///
    /// This is the most general construction method, allowing full control
    /// over both the matching logic (via the constructor) and the value
    /// extraction/transformation (via the handler).
    fn ctor_handler<H, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, H>,
        H: Handler<Self>,
    {
        pat.construct(self, &mut handler)
    }

    /// Constructs a value using a constructor pattern and a closure handler.
    ///
    /// This is a convenience method that allows using a closure instead of
    /// implementing the [`Handler`] trait. The closure receives the context
    /// and matched span and returns a [`Result`].
    fn ctor_with<H, P, O, R>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, H>,
        H: FnMut(&Self, &Span) -> Result<R, Error>,
    {
        self.ctor_handler(pat, handler)
    }

    /// Constructs a value using a constructor pattern, returning the matched span.
    ///
    /// This method uses the [`Extract<Span>`] handler, which simply returns
    /// the matched span without any processing. This is useful when you need
    /// the span information for further processing or when the constructor
    /// itself handles all the transformation logic.
    fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, Extract<Span>>,
    {
        self.ctor_handler(pat, extract())
    }

    /// Constructs a value by extracting the matched substring.
    ///
    /// This is the most commonly used construction method. It matches the
    /// pattern and extracts the corresponding substring from the original input.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     assert_eq!(
    ///         RegexCtx::with("2026", |mut ctx| {
    ///             ctx.ctor(&neu::digit(10).many1().try_map(map::from_str::<i32>()))
    ///         })?,
    ///         2026,
    ///         "rust in 2026!"
    ///     );
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, Extract<Self::Orig<'a>>>,
        Extract<Self::Orig<'a>>: Handler<Self>,
    {
        self.ctor_handler(pat, extract())
    }

    /// Matches a pattern and processes the result with a handler.
    ///
    /// This method combines matching and processing into a single operation.
    /// It's similar to [`ctor_handler`](MatchExt::ctor_handler) but works with regular patterns
    /// ([`Regex`]) rather than constructors ([`Ctor`]).
    fn map_handler<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<Self, Out = O>;

    /// Matches a pattern and processes the result with a closure.
    ///
    /// This is a convenience version of [`map_handler`](MatchExt::map_handler) that accepts a closure
    /// instead of requiring a full [`Handler`] implementation.
    fn map_with<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: FnMut(&Self, &Span) -> Result<O, Error>,
    {
        self.map_handler(pat, handler)
    }

    /// Matches a pattern, extracts the span, and maps it using a mapper.
    ///
    /// This method performs a three-step operation:
    /// 1. Matches the pattern
    /// 2. Extracts the matched span
    /// 3. Applies a mapper function to transform the span
    ///
    /// This is useful when you need to perform additional transformation
    /// on the span itself (e.g., converting to a different representation).
    fn map_span<P, O, M>(&mut self, pat: &P, mapper: M) -> Result<O, Error>
    where
        P: Regex<Self>,
        M: FallibleMap<Span, O>,
    {
        mapper.try_map(self.map_handler(pat, extract::<Span>())?)
    }

    /// Matches a pattern, extracts the matched substring, and maps it.
    ///
    /// This method performs a three-step operation:
    /// 1. Matches the pattern
    /// 2. Extracts the matched substring from the original input
    /// 3. Applies a mapper function to transform the substring
    ///
    /// This is the most common pattern for parsing: match text, extract it,
    /// and then parse/transform it into the desired type.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     assert_eq!(
    ///         RegexCtx::with("2026", |mut ctx| {
    ///             ctx.map(&neu::digit(10).many1(), map::from_str::<i32>())
    ///         })?,
    ///         2026,
    ///         "rust in 2026!"
    ///     );
    /// #
    /// #    Ok(())
    /// # }
    /// ```
    fn map<P, O, M>(&mut self, pat: &P, mapper: M) -> Result<O, Error>
    where
        P: Regex<Self>,
        M: FallibleMap<Self::Orig<'a>, O>,
    {
        mapper.try_map(self.map_handler(pat, |ctx: &Self, span: &Span| {
            ctx.orig_sub(span.beg, span.len)
        })?)
    }
}

impl<'a, C> MatchExt<'a> for C
where
    C: Sized + Match<'a>,
{
    fn map_handler<H, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<Self, Out = O>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(self, &ret).map_err(Into::into)
    }
}

/// A trait for multiple pattern matching operations on a parsing context.
///
/// The `MatchMulti` trait extends the basic matching functionality provided by [`Match`]
/// with methods for finding multiple occurrences of patterns, including searching,
/// iterative matching, and contiguous sequence matching. This trait provides the
/// foundation for parsing operations that need to locate and extract multiple
/// values from input data.
///
/// This trait is automatically implemented for all types that implement [`Match<'a>`],
/// making these multi-match methods available on all matching contexts.
pub trait MatchMulti<'a>: Sized + Match<'a> {
    ///
    /// Searches for the first occurrence of a pattern in the input.
    ///
    /// This method scans the input starting from the current position and returns the first match
    /// of the given pattern. If a match is found, it returns `Some(value)` where `value` is the
    /// extracted result of the match. If no match is found, it returns `None`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn core::error::Error>> {
    ///     let text = r#"Dec. 19   What do people love about Rust?
    /// Dec. 16     Project goals update — November 2025
    /// Dec. 11     Announcing Rust 1.92.0
    /// Dec. 8      Making it easier to sponsor Rust contributors
    /// "#;
    ///     let month = "Dec";
    ///     let day = neu::digit(10).between::<1, 2>();
    ///     let date = month.sep_once(". ", day);
    ///
    ///     let orig = CharsCtx::new(text).find::<&str>(date);
    ///
    ///     assert_eq!(orig, Some("Dec. 19"));
    ///
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// # Behavior Notes
    ///
    /// - The search starts at the current offset of the context
    /// - When a match is found, the context's position is advanced to the end of the match
    /// - Empty matches are ignored and searching continues
    /// - If matching fails with an error, the context advances by 1 character and continues searching
    /// - Returns `None` if the end of input is reached without finding a match
    fn find<O>(&mut self, pat: impl Regex<Self>) -> Option<O>
    where
        Extract<O>: Handler<Self, Out = O>,
    {
        let func = |ctx: &mut Self, val: Result<Span, Error>| match val {
            Ok(val) if !val.is_empty() => Some(val),
            _ => {
                ctx.inc(1);
                None
            }
        };

        self.find_with(pat, func, extract())
    }

    /// Searches for the first occurrence of a pattern with custom processing logic.
    fn find_with<O, F, H>(&mut self, pat: impl Regex<Self>, mut func: F, mut map: H) -> Option<O>
    where
        H: Handler<Self, Out = O>,
        F: FnMut(&mut Self, Result<Span, Error>) -> Option<Span>,
    {
        let mut next = None;

        while self.offset() < self.len() {
            let ret = self.try_mat(&pat);

            // let processer process the result
            // may modify Self
            if let Some(span) = func(self, ret) {
                // map the span to another type
                if let Ok(out) = map.invoke(self, &span) {
                    next = Some(out);
                    break;
                }
            }
        }
        next
    }

    /// Finds all non-overlapping occurrences of a pattern in the input.
    ///
    /// This method returns an iterator that yields all matches of the given pattern in the input,
    /// starting from the current position. Unlike [`find`](MatchMulti::find), which stops after the
    /// first match, this method continues searching through the entire input until no more matches
    /// can be found.
    ///
    /// Each match advances the context's position to the end of the matched text, ensuring matches
    /// don't overlap. Empty matches are skipped, and errors during matching cause the search to
    /// advance by one character before continuing.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn core::error::Error>> {
    ///     let text = r#"Stable: 1.92.0
    /// Beta: 1.93.0 (22 January, 2026, 33 days left)
    /// Nightly: 1.94.0 (5 March, 2026, 75 days left)
    /// "#;
    ///     let num = neu::digit(10).at_least::<1>();
    ///     let ver = num.sep_once(".", num).sep_once(".", num).pat();
    ///     let vers = ["1.92.0", "1.93.0", "1.94.0"];
    ///
    ///     for (i, orig) in CharsCtx::new(text).find_all::<&str>(ver).enumerate() {
    ///         assert_eq!(vers[i], orig);
    ///     }
    ///
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// # Behavior Notes
    ///
    /// - The search starts at the current offset of the context
    /// - After each successful match, the context's position is advanced to the end of that match
    /// - Empty matches are ignored and searching continues from the next position
    /// - If matching fails with an error, the context advances by 1 character and continues searching
    /// - The iterator yields `None` when the end of input is reached
    /// - The context is fully consumed after the iterator is exhausted
    fn find_all<O>(&mut self, pat: impl Regex<Self>) -> impl Iterator<Item = O>
    where
        Extract<O>: Handler<Self, Out = O>,
    {
        let func = |ctx: &mut Self, val: Result<Span, Error>| match val {
            Ok(val) if !val.is_empty() => Some(val),
            _ => {
                ctx.inc(1);
                None
            }
        };

        self.find_all_with(pat, func, extract())
    }

    /// Matches a sequence of consecutive patterns without gaps.
    ///
    /// This method returns an iterator that yields consecutive matches of the given pattern,
    /// starting from the current position. Unlike [`find_all`](MatchMulti::find_all), which
    /// continues searching the entire input even after failed matches, this method **stops
    /// immediately when a match fails**, ensuring only contiguous successful matches are returned.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn core::error::Error>> {
    ///     let text = r#"Dec. 19   What do people love about Rust?
    /// Dec. 16     Project goals update — November 2025
    /// Dec. 11     Announcing Rust 1.92.0
    /// Dec. 8      Making it easier to sponsor Rust contributors
    /// Dec. 5      crates.io: Malicious crates finch-rust and sha-rust
    /// "#;
    ///     let month = "Dec";
    ///     let day = neu::digit(10).between::<1, 2>();
    ///     let date = month.sep_once(". ", day);
    ///     let title = neu::wild().at_least::<1>();
    ///     let parser = date.sep_once("".skip_ws(), title);
    ///     let parser = parser.suffix("\n");
    ///
    ///     let mut ctx = CharsCtx::new(text);
    ///
    ///     let lines = ctx.match_seq::<&str>(parser);
    ///
    ///     for line in lines {
    ///         assert!(line.starts_with("Dec"));
    ///         assert!(line.ends_with("\n"));
    ///     }
    ///
    /// #   Ok(())
    /// # }
    /// ```
    fn match_seq<O>(&mut self, pat: impl Regex<Self>) -> impl Iterator<Item = O>
    where
        Extract<O>: Handler<Self, Out = O>,
    {
        let func = |ctx: &mut Self, val: Result<Span, Error>| match val {
            Ok(val) if !val.is_empty() => Some(val),
            _ => {
                ctx.set_offset(ctx.len());
                None
            }
        };

        self.find_all_with(pat, func, extract())
    }

    /// Returns an iterator over all occurrences of a pattern with custom processing logic.
    fn find_all_with<O, F, H>(
        &mut self,
        pat: impl Regex<Self>,
        mut func: F,
        mut map: H,
    ) -> impl Iterator<Item = O>
    where
        H: Handler<Self, Out = O>,
        F: FnMut(&mut Self, Result<Span, Error>) -> Option<Span>,
    {
        core::iter::from_fn(move || {
            let mut next = None;

            while self.offset() < self.len() {
                let ret = self.try_mat(&pat);

                // let processer process the result
                // may modify Self
                if let Some(span) = func(self, ret) {
                    // map the span to another type
                    if let Ok(out) = map.invoke(self, &span) {
                        next = Some(out);
                        break;
                    }
                }
            }
            next
        })
    }
}

impl<'a, T> MatchMulti<'a> for T where Self: Sized + Match<'a> {}

// make new span [offset, offset + len) and increment offset
pub(crate) fn new_span_inc<'a>(ctx: &mut impl Context<'a>, len: usize) -> Span {
    let span = Span::new(ctx.offset(), len);

    ctx.inc(len);
    span
}
