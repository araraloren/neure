use std::ops::Deref;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::Regex;

use super::Ctor;

///
/// Attempts patterns in sequence, returning the first successful match from a **compile-time fixed array**.
///
/// This combinator provides **zero-cost choice semantics** for a fixed set of parsers known at compile time.
/// `Slice` uses a stack-allocated array with compile-time size `N`, enabling compiler optimizations
/// and eliminating heap allocations. It's ideal for matching finite sets of keywords, operators,
/// or mutually exclusive patterns where order matters and performance is critical.
///
/// # Regex
///
/// Iterates through the array of patterns in order:
/// 1. Attempts to match each pattern at current context position
/// 2. On first successful match:
///    - Returns the matched [`Span`]
///    - Advances context to end of match
/// 3. On failure:
///    - Resets context to initial position (no partial advancement)
///    - Tries next pattern
/// 4. If all patterns fail:
///    - Returns [`Error::Slice`]
///    - Context remains unchanged
///
/// The returned span covers exactly the matched portion of the first successful pattern.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let array = ["a", "b", "c"];
///     let parser = regex::slice(&array);
///
///     assert_eq!(CharsCtx::new("abc").try_mat(&parser)?, Span::new(0, 1));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Mirror behavior to [`Regex`], but with value construction:
/// 1. Attempts to construct each pattern's value in sequence
/// 2. On first successful construction:
///    - Returns the constructed value `O`
///    - Advances context to end of match
/// 3. On failure:
///    - Resets context to initial position
///    - Tries next pattern
/// 4. If all patterns fail:
///    - Returns [`Error::Slice`]
///    - Context remains unchanged
///
/// All patterns must produce values of identical type `O`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let array = ["abc", "def", "ghi"];
///     let parser = regex::slice(&array);
///
///     assert_eq!(CharsCtx::new("ghi").ctor(&parser)?, "ghi");
///
/// #   Ok(())
/// # }
/// ```
///
/// Optimization guidelines:
/// - **Sort by Frequency**: Place most common patterns first
/// - **Sort by Specificity**: Put longer/more specific patterns before general ones
/// - **Limit Size**: Keep N small (≤ 8) for best branch prediction
/// - **Avoid Overlap**: Minimize overlapping patterns to reduce attempts
/// - **Precompute**: Create `Slice` instances at startup, not in hot paths
#[derive(Debug, Clone, Copy)]
pub struct Slice<'a, T>(&'a [T]);

impl<T> std::ops::Not for Slice<'_, T> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<'a, T> Slice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self(val)
    }
}

impl<T> Deref for Slice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, C, T, O, H> Ctor<'a, C, O, H> for Slice<'_, T>
where
    T: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Slice);

        debug_ctor_beg!("Slice", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }
        debug_ctor_reval!("Slice", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, T> Regex<C> for Slice<'_, T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Slice);

        debug_regex_beg!("Slice", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }
        debug_regex_reval!("Slice", ret)
    }
}

///
/// Maps patterns to associated values, returning the first successful match with its paired value.
///
/// This combinator extends [`Slice`] with **value association semantics**, creating a compile-time
/// mapping between patterns and static values. When a pattern matches, its associated value is
/// cloned and returned alongside the constructed result. This is ideal for scenarios like keyword
/// tokenization, operator precedence mapping, or state machine transitions where patterns need to
/// carry semantic meaning beyond their syntactic structure.
///
/// # Regex
///
/// Behaves identically to [`Slice`] but ignores associated values:
/// - Iterates through patterns in array order
/// - Returns first successful match's `Span`
/// - Resets context on failure for each attempt
/// - Associated values `V` are completely ignored in this mode
/// - Returns [`Error::PairSlice`] if no patterns match
///
/// This mode is useful when you only need the span information and not the semantic mapping.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum Kind {
///         Str,
///         Num,
///         Other,
///     }
///
///     let num = regex::Adapter::dyn_box(neu::digit(10).many1());
///     let str = regex::Adapter::dyn_box(neu::word().many1());
///     let other = regex::consume_all().into_dyn_regex();
///
///     let pairs = [(num, Kind::Num), (str, Kind::Str), (other, Kind::Other)];
///     let parser = regex::pair_slice(&pairs);
///
///     assert_eq!(CharsCtx::new("crab").ctor(&parser)?, ("crab", Kind::Str));
///     assert_eq!(CharsCtx::new("2025").ctor(&parser)?, ("2025", Kind::Num));
///     assert_eq!(CharsCtx::new("&ptr").try_mat(&parser)?, Span::new(0, 4));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Core mapping behavior with dual return values:
/// 1. Iterates through `(pattern, value)` pairs in array order
/// 2. For each pattern:
///    - Attempts construction at current context position
///    - On success: returns `(constructed_value, value.clone())`
///    - On failure: resets context and tries next pair
/// 3. Returns [`Error::PairSlice`] if no patterns match
///
/// The cloned associated value `V` provides semantic meaning without consuming the original mapping.
/// This enables patterns to carry metadata like:
/// - Token types in lexers
/// - Precedence levels in parsers  
/// - Handler functions in routers
/// - State transitions in state machines
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum Kind {
///         A,
///         B,
///         C,
///     }
///     let pairs = [("a", Kind::A), ("b", Kind::B), ("c", Kind::C)];
///     let vec = regex::pair_slice(&pairs);
///
///     assert_eq!(CharsCtx::new("cab").ctor(&vec)?, ("c", Kind::C));
///
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PairSlice<'a, K, V>(&'a [(K, V)]);

impl<K, V> std::ops::Not for PairSlice<'_, K, V> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<'a, K, V> PairSlice<'a, K, V> {
    pub fn new(val: &'a [(K, V)]) -> Self {
        Self(val)
    }
}

impl<K, V> Deref for PairSlice<'_, K, V> {
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, C, K, O, V, H> Ctor<'a, C, (O, V), H> for PairSlice<'_, K, V>
where
    V: Clone,
    K: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairSlice);

        debug_ctor_beg!("PairSlice", g.beg());
        for (regex, value) in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok((res, value.clone()));
                break;
            } else {
                g.reset();
            }
        }
        debug_ctor_reval!("PairSlice", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, K, V> Regex<C> for PairSlice<'_, K, V>
where
    K: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairSlice);

        debug_regex_beg!("PairSlice", g.beg());
        for (regex, _) in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
            } else {
                g.reset();
            }
        }
        debug_regex_reval!("PairSlice", ret)
    }
}
