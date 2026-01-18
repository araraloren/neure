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

pub trait Context<'a> {
    type Orig<'b>;

    type Item: MayDebug;

    type Iter<'b>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'b;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    fn orig(&self) -> Result<Self::Orig<'a>, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        self.orig_sub(offset, self.len() - offset)
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error>;

    fn clone_at(&self, offset: usize) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Match<'a>: Context<'a>
where
    Self: Sized,
{
    fn is_mat<Pat>(&mut self, pat: &Pat) -> bool
    where
        Pat: Regex<Self> + ?Sized,
    {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

pub trait PolicyMatch<'a>
where
    Self: Sized,
{
    fn try_mat_before<P, B>(&mut self, pat: &P, before: &B) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, before, &|_: &mut Self| Ok(Span::default()))
    }

    fn try_mat_after<P, A>(&mut self, pat: &P, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, &|_: &mut Self| Ok(Span::default()), after)
    }

    fn try_mat_policy<P, B, A>(&mut self, pat: &P, before: &B, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized;
}

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

pub trait MatchExt<'a>
where
    Self: Context<'a> + Sized,
{
    fn ctor_handler<H, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, H>,
        H: Handler<Self>,
    {
        pat.construct(self, &mut handler)
    }

    fn ctor_with<H, P, O, R>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, H>,
        H: FnMut(&Self, &Span) -> Result<R, Error>,
    {
        self.ctor_handler(pat, handler)
    }

    fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, Extract<Span>>,
    {
        self.ctor_handler(pat, extract())
    }

    fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, Extract<Self::Orig<'a>>>,
        Extract<Self::Orig<'a>>: Handler<Self>,
    {
        self.ctor_handler(pat, extract())
    }

    fn map_handler<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<Self, Out = O>;

    fn map_with<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: FnMut(&Self, &Span) -> Result<O, Error>,
    {
        self.map_handler(pat, handler)
    }

    fn map_span<P, O, M>(&mut self, pat: &P, mapper: M) -> Result<O, Error>
    where
        P: Regex<Self>,
        M: FallibleMap<Span, O>,
    {
        mapper.try_map(self.map_handler(pat, extract::<Span>())?)
    }

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
