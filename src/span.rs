use crate::ctx::Context;
use crate::ctx::Match;
use crate::err::Error;
use crate::iter::IndexBySpan;
use crate::iter::IteratorByOptionSpan;
use crate::iter::OptionSpanIterator;
use crate::regex::Regex;

use core::fmt::Display;

/// Represents a span of data with a starting position and length.
///
/// A [`Span`] identifies a contiguous region within a larger data source,
/// such as a string, byte array, or file. It's commonly used in parsers,
/// tokenizers, and other text processing systems to track the location
/// and extent of matched content.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

impl Span {
    /// Creates a new span with the specified beginning position and length.
    pub fn new(beg: usize, len: usize) -> Self {
        Self { beg, len }
    }

    /// Returns the starting position of the span.
    pub fn beg(&self) -> usize {
        self.beg
    }

    /// Returns the length of the span.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the span has zero length.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Extends the current span to include another span.
    ///
    /// This method modifies the current span to cover the range from its beginning
    /// to the end of the other span. The spans must be contiguous or overlapping;
    /// the other span must starts after the current span.
    pub fn add_assign(&mut self, other: Self) -> &mut Self {
        debug_assert!(other.beg >= self.beg, "other beg must bigger than current");
        self.len += other.len + other.beg - (self.beg + self.len);
        self
    }

    /// Retrieves the original content covered by this span from the context.
    ///
    /// This method extracts the actual data (rather than just positions) that this
    /// span refers to, using the provided context. The context must implement the
    /// [`Context`] trait to provide access to the original data source.
    pub fn orig<'a, C>(&self, ctx: &C) -> Result<<C as Context<'a>>::Orig<'a>, crate::err::Error>
    where
        C: Context<'a>,
    {
        ctx.orig_sub(self.beg, self.len)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{beg: {}, len: {}}}", self.beg, self.len)
    }
}

#[cfg(feature = "alloc")]
mod alloc_vec_storer {

    use crate::alloc::Vec;
    use crate::alloc::vec;
    use crate::ctx::Match;
    use crate::err::Error;
    use crate::iter::IndexBySpan;
    use crate::iter::IteratorBySpan;
    use crate::iter::SpanIterator;
    use crate::regex::Regex;

    use super::Span;

    /// A storage container for managing multiple groups of spans.
    ///
    /// [`VecStorer`] is designed to capture and store spans during parsing or pattern matching
    /// operations. It organizes spans into multiple groups (identified by numeric IDs), where each
    /// group can contain multiple spans. This is particularly useful for tracking multiple matches
    /// of different patterns within the same input data.
    ///

    #[derive(Debug, Clone, Default)]
    pub struct VecStorer {
        spans: Vec<Vec<Span>>,
    }

    impl VecStorer {
        /// Creates a new span storer with the specified number of groups.
        /// The storer will be initialized with `capacity` empty groups, ready to store spans.
        pub fn new(capacity: usize) -> Self {
            Self {
                spans: vec![vec![]; capacity],
            }
        }

        /// Creates a new span storer initialized with the provided span groups.
        pub fn new_with(spans: Vec<Vec<Span>>) -> Self {
            Self { spans }
        }

        /// Reinitializes the storer with a new capacity, discarding all existing spans.
        ///
        /// This method resets the internal storage to contain exactly `capacity` empty groups.
        /// Any previously stored spans are lost.
        pub fn with_capacity(mut self, capacity: usize) -> Self {
            self.spans = vec![vec![]; capacity];
            self
        }

        /// Returns the number of span groups in the storer.
        pub fn len(&self) -> usize {
            self.spans.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Clears all spans from all groups, resetting the storer to its initial empty state.
        ///
        /// The number of groups remains unchanged - only the spans within each group are removed.
        pub fn reset(&mut self) -> &mut Self {
            self.spans.iter_mut().for_each(|v| v.clear());
            self
        }
    }

    impl VecStorer {
        /// Checks if the specified group contains any spans.
        ///
        /// Returns true if group `id` exists and contains at least one span.
        /// Returns false if the group doesn't exist or is empty.
        pub fn contain(&self, id: usize) -> bool {
            self.spans.get(id).map(|v| !v.is_empty()).unwrap_or(false)
        }

        /// Adds a span to the specified group.
        ///
        /// The span is appended to the end of the group's span list.
        /// If the group ID is out of bounds, this will panic.
        pub fn add_span(&mut self, id: usize, span: Span) -> &mut Self {
            self.spans[id].push(span);
            self
        }

        /// Clears all spans from the specified group.
        ///
        /// If the group ID is out of bounds, this does nothing.
        pub fn clr_span(&mut self, id: usize) -> &mut Self {
            if let Some(v) = self.spans.get_mut(id) {
                v.clear()
            };
            self
        }

        /// Retrieves a specific span from a group by index.
        ///
        /// Returns `None` if either:
        /// - The group ID is out of bounds
        /// - The index is out of bounds for that group
        pub fn span(&self, id: usize, index: usize) -> Option<&Span> {
            self.spans.get(id).and_then(|v| v.get(index))
        }

        /// Retrieves all spans for a specific group.
        ///
        /// Returns `None` if either:
        /// - The group ID is out of bounds
        /// - The group exists but contains no spans
        pub fn spans(&self, id: usize) -> Option<&Vec<Span>> {
            self.spans
                .get(id)
                .and_then(|v| if v.is_empty() { None } else { Some(v) })
        }

        /// Creates an iterator over the spans in the specified group.
        ///
        /// Returns `None` if the group doesn't exist or contains no spans.
        pub fn spans_iter(&self, id: usize) -> Option<SpanIterator<'_>> {
            self.spans(id).map(|v| SpanIterator::new(v.as_slice()))
        }
    }

    impl VecStorer {
        /// Extracts a slice from a value using a span from the specified group and index.
        ///
        /// This method uses the [`IndexBySpan`] trait to extract content from any type that
        /// implements it, based on the span's position and length.
        ///
        /// Returns `None` if either:
        /// - The group ID is out of bounds
        /// - The index is out of bounds for that group
        /// - The span doesn't correspond to a valid range in the value
        pub fn slice<'a, T>(
            &self,
            value: &'a T,
            id: usize,
            index: usize,
        ) -> Option<&'a <T as IndexBySpan>::Output>
        where
            T: IndexBySpan + ?Sized,
        {
            let span = self.span(id, index)?;

            value.get_by_span(span)
        }

        /// Creates an iterator over slices extracted from a value using all spans in a group.
        ///
        /// Returns `None` if the group doesn't exist or contains no spans.
        pub fn slice_iter<'a, T>(&self, str: &'a T, id: usize) -> Option<IteratorBySpan<'a, '_, T>>
        where
            T: IndexBySpan + ?Sized,
        {
            Some(IteratorBySpan::new(str, self.spans(id)?))
        }
    }

    impl VecStorer {
        /// Attempts to capture a match from the context using the provided pattern.
        ///
        /// If matching succeeds, the resulting span is stored in the specified group
        /// and the span is returned. If matching fails, an error is returned and no
        /// span is stored.
        ///
        /// This method is designed to be used in loops to capture all matches of a
        /// pattern within an input.
        ///
        /// # Example
        ///
        /// ```
        /// # use neure::prelude::*;
        /// #
        /// # fn main() -> Result<(), Box<dyn core::error::Error>> {
        ///     let bc = &mut CharsCtx::new("11helloworld!");
        ///     let regexs = [
        ///         neu::digit(10).once().into_dyn_regex(),
        ///         neu::ascii_alphabetic().once().into_dyn_regex(),
        ///         "!".into_dyn_regex(),
        ///     ];
        ///     let mut storer = VecStorer::new(regexs.len());
        ///
        ///     for (id, regex) in regexs.iter().enumerate() {
        ///         while storer.try_cap(id, bc, regex).is_ok() {}
        ///     }
        ///
        ///     let tests = [
        ///         ["1", "1"].as_slice(),
        ///         &["h", "e", "l", "l", "o", "w", "o", "r", "l", "d"],
        ///         &["!"],
        ///     ];
        ///
        ///     for (i, test) in tests.iter().enumerate() {
        ///         if let Some(origs) = storer.slice_iter(bc, i) {
        ///             for (orig, test) in origs.zip(test.iter()) {
        ///                 assert_eq!(orig, *test);
        ///             }
        ///         }
        ///     }
        ///
        /// #   Ok(())
        /// # }
        /// ```
        pub fn try_cap<'a, C, P>(&mut self, id: usize, ctx: &mut C, pat: &P) -> Result<Span, Error>
        where
            P: Regex<C>,
            C: Match<'a>,
        {
            let ret = ctx.try_mat(pat)?;

            self.add_span(id, ret);
            Ok(ret)
        }
    }
}

#[cfg(feature = "alloc")]
pub use alloc_vec_storer::*;

/// A storage container for managing multiple groups of spans.
///
/// [`ArrayStorer`] is designed to capture and store spans during parsing or pattern matching
/// operations. It organizes spans into multiple groups (identified by numeric IDs), where each
/// group can contain multiple spans. This is particularly useful for tracking multiple matches
/// of different patterns within the same input data.
///

#[derive(Debug, Clone)]
pub struct ArrayStorer<const M: usize, const N: usize> {
    spans: [[Option<Span>; N]; M],
}

impl<const M: usize, const N: usize> Default for ArrayStorer<M, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const M: usize, const N: usize> ArrayStorer<M, N> {
    /// Creates a new span storer with the specified number of groups.
    /// The storer will be initialized with `capacity` empty groups, ready to store spans.
    pub fn new() -> Self {
        Self {
            spans: [[const { None }; N]; M],
        }
    }

    /// Creates a new span storer initialized with the provided span groups.
    pub fn new_with(spans: [[Option<Span>; N]; M]) -> Self {
        Self { spans }
    }

    /// Returns the number of span groups in the storer.
    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears all spans from all groups, resetting the storer to its initial empty state.
    ///
    /// The number of groups remains unchanged - only the spans within each group are removed.
    pub fn reset(&mut self) -> &mut Self {
        self.spans.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|v| {
                *v = None;
            })
        });
        self
    }
}

impl<const M: usize, const N: usize> ArrayStorer<M, N> {
    /// Checks if the specified group contains any spans.
    ///
    /// Returns true if group `id` exists and contains at least one span.
    /// Returns false if the group doesn't exist or is empty.
    pub fn contain(&self, id: usize) -> bool {
        self.spans
            .get(id)
            .map(|v| v.iter().any(|v| v.is_some()))
            .unwrap_or(false)
    }

    /// Adds a span to the specified group.
    ///
    /// The span is appended to the end of the group's span list.
    /// If the group ID or index is out of bounds, this will panic.
    pub fn set_span(&mut self, id: usize, index: usize, span: Span) -> &mut Self {
        self.spans[id][index] = Some(span);
        self
    }

    /// Clears all spans from the specified group.
    ///
    /// If the group ID is out of bounds, this does nothing.
    pub fn clr_span(&mut self, id: usize) -> &mut Self {
        if let Some(v) = self.spans.get_mut(id) {
            v.iter_mut().for_each(|v| {
                *v = None;
            })
        };
        self
    }

    /// Retrieves a specific span from a group by index.
    ///
    /// Returns `None` if either:
    /// - The group ID is out of bounds
    /// - The index is out of bounds for that group
    pub fn span(&self, id: usize, index: usize) -> Option<&Span> {
        self.spans
            .get(id)
            .and_then(|v| v.get(index).and_then(|v| v.as_ref()))
    }

    /// Retrieves all spans for a specific group.
    ///
    /// Returns `None` if either:
    /// - The group ID is out of bounds
    /// - The group exists but contains no spans
    pub fn spans(&self, id: usize) -> Option<&[Option<Span>]> {
        self.spans.get(id).and_then(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.as_slice())
            }
        })
    }

    /// Creates an iterator over the spans in the specified group.
    ///
    /// Returns `None` if the group doesn't exist or contains no spans.
    pub fn spans_iter(&self, id: usize) -> Option<OptionSpanIterator<'_>> {
        self.spans(id).map(OptionSpanIterator::new)
    }
}

impl<const M: usize, const N: usize> ArrayStorer<M, N> {
    /// Extracts a slice from a value using a span from the specified group and index.
    ///
    /// This method uses the [`IndexBySpan`] trait to extract content from any type that
    /// implements it, based on the span's position and length.
    ///
    /// Returns `None` if either:
    /// - The group ID is out of bounds
    /// - The index is out of bounds for that group
    /// - The span doesn't correspond to a valid range in the value
    pub fn slice<'a, T>(
        &self,
        value: &'a T,
        id: usize,
        index: usize,
    ) -> Option<&'a <T as IndexBySpan>::Output>
    where
        T: IndexBySpan + ?Sized,
    {
        let span = self.span(id, index)?;

        value.get_by_span(span)
    }

    /// Creates an iterator over slices extracted from a value using all spans in a group.
    ///
    /// Returns `None` if the group doesn't exist or contains no spans.
    pub fn slice_iter<'a, T>(
        &self,
        str: &'a T,
        id: usize,
    ) -> Option<IteratorByOptionSpan<'a, '_, T>>
    where
        T: IndexBySpan + ?Sized,
    {
        Some(IteratorByOptionSpan::new(str, self.spans(id)?))
    }
}

impl<const M: usize, const N: usize> ArrayStorer<M, N> {
    /// Attempts to capture a match from the context using the provided pattern.
    ///
    /// If matching succeeds, the resulting span is stored in the specified group
    /// and the span is returned. If matching fails, an error is returned and no
    /// span is stored.
    ///
    /// This method is designed to be used in loops to capture all matches of a
    /// pattern within an input.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn core::error::Error>> {
    ///     let bc = &mut CharsCtx::new("11helloworld!");
    ///     let regexs = [
    ///         neu::digit(10).once().into_dyn_regex(),
    ///         neu::ascii_alphabetic().once().into_dyn_regex(),
    ///         "!".into_dyn_regex(),
    ///     ];
    ///     let mut storer = ArrayStorer::<3, 20>::new();
    ///
    ///     for (id, regex) in regexs.iter().enumerate() {
    ///         let mut index = 0;
    ///
    ///         while storer.try_cap(id, index, bc, regex).is_ok() {
    ///             index += 1;
    ///         }
    ///     }
    ///
    ///     let tests = [
    ///         ["1", "1"].as_slice(),
    ///         &["h", "e", "l", "l", "o", "w", "o", "r", "l", "d"],
    ///         &["!"],
    ///     ];
    ///
    ///     for (i, test) in tests.iter().enumerate() {
    ///         if let Some(origs) = storer.slice_iter(bc, i) {
    ///             for (orig, test) in origs.zip(test.iter()) {
    ///                 assert_eq!(orig, *test);
    ///             }
    ///         }
    ///     }
    ///
    /// #   Ok(())
    /// # }
    /// ```
    pub fn try_cap<'a, C, P>(
        &mut self,
        id: usize,
        index: usize,
        ctx: &mut C,
        pat: &P,
    ) -> Result<Span, Error>
    where
        P: Regex<C>,
        C: Match<'a>,
    {
        let ret = ctx.try_mat(pat)?;

        self.set_span(id, index, ret);
        Ok(ret)
    }
}
