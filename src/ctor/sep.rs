use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::neu::CRange;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

use super::Map;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;

///
/// Splits input into two parts at a single separator pattern, discarding the separator value.
///
/// This combinator matches a **left pattern**, followed by a **separator pattern**, followed by a
/// **right pattern**. It's designed for parsing key-value pairs (`key=value`), headers
/// (`Content-Type: text/plain`), path segments (`dir/file`), and similar two-part structures with
/// an intervening delimiter. The separator serves as a structural boundary but its value is
/// discarded in value construction.
///
/// # Regex
///
/// Matches all three components sequentially and returns a **single merged span** covering:
/// 1. The left pattern match
/// 2. The separator pattern match  
/// 3. The right pattern match
///
/// The returned span represents the complete matched structure from start of left pattern to end
/// of right pattern. If any component fails to match, the entire match fails and the context
/// position remains unchanged. Unlike [`Separate`], this combinator requires exactly one
/// separator occurrence.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let key = '='.not().many1();
///     let val = key;
///     let pair = key.sep_once("=", val);
///
///     assert_eq!(CharsCtx::new("lang=rust").try_mat(&pair)?, Span::new(0, 9));
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// 1. Constructs the left pattern's value
/// 2. Matches the separator pattern (value discarded)
/// 3. Constructs the right pattern's value
/// 4. Returns a **tuple `(left_value, right_value)`**
///
/// The separator's matched value is intentionally discarded—only the left and right values are
/// preserved. This follows the principle that separators define structure but don't contribute to
/// semantic values. For example, in `key=value`, we want `(key, value)` not `(key, '=', value)`.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ele = neu::digit(10).count::<2>();
///     let sep = ":";
///     let time = ele.sep_once(sep, ele).sep_once(sep, ele);
///     let time = time.map(|((h, m), s)| (h, m, s));
///
///     assert_eq!(CharsCtx::new("20:31:42").ctor(&time)?, ("20", "31", "42"));
/// #   Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct SepOnce<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(SepOnce<C, L, S, R>);

impl<C, L, S, R> Debug for SepOnce<C, L, S, R>
where
    L: Debug,
    S: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SepOnce")
            .field("left", &self.left)
            .field("sep", &self.sep)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, S, R> Clone for SepOnce<C, L, S, R>
where
    L: Clone,
    S: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            sep: self.sep.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, S, R> SepOnce<C, L, S, R> {
    pub fn new(left: L, sep: S, right: R) -> Self {
        Self {
            left,
            sep,
            right,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn _0<O>(self) -> Map<C, Self, Select0, O> {
        Map::new(self, Select0)
    }

    pub fn _1<O>(self) -> Map<C, Self, Select1, O> {
        Map::new(self, Select1)
    }

    pub fn _eq<I1, I2>(self) -> Map<C, Self, SelectEq, (I1, I2)> {
        Map::new(self, SelectEq)
    }
}

impl<'a, C, L, S, R, M, O1, O2, H> Ctor<'a, C, M, (O1, O2), H> for SepOnce<C, L, S, R>
where
    L: Ctor<'a, C, M, O1, H>,
    R: Ctor<'a, C, M, O2, H>,
    S: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("SepOnce", g.beg());

        let r = self.left.construct(g.ctx(), func);
        let r = g.process_ret(r)?;
        let _ = g.try_mat(&self.sep)?;
        let l = self.right.construct(g.ctx(), func);
        let l = g.process_ret(l)?;

        debug_ctor_reval!("SepOnce", g.beg(), g.end(), true);
        Ok((r, l))
    }
}

impl<'a, C, L, S, R> Regex<C> for SepOnce<C, L, S, R>
where
    S: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = Span::new(g.ctx().offset(), 0);

        debug_regex_beg!("SepOnce", g.beg());
        span.add_assign(g.try_mat(&self.left)?);
        span.add_assign(g.try_mat(&self.sep)?);
        span.add_assign(g.try_mat(&self.right)?);
        debug_regex_reval!("SepOnce", Ok(span))
    }
}

///
/// Matches a pattern repeated with separators, supporting trailing separators and minimum counts.
///
/// This combinator parses **lists of elements separated by delimiters**, like CSV data (`A,B,C`),
/// function arguments (`func(a, b, c)`), or path segments (`/usr/local/bin`). It provides fine-grained
/// control over separator requirements, trailing separator handling, and minimum element
/// counts. Unlike [`SepOnce`] (single split) or [`Repeat`](crate::ctor::Repeat) (no separators),
/// this handles arbitrary-length sequences with explicit delimiter semantics.
///
/// # Regex
///
/// Matches the pattern and separators repeatedly, returning a **single merged span** covering:
/// 1. All successfully matched patterns
/// 2. All successfully matched separators (including trailing ones if allowed)
///
/// The matching continues until:
/// - Pattern matching fails, OR
/// - Separator matching fails (and `skip = false` for trailing separator)
///
/// The result is valid only if the number of matched patterns meets or exceeds the `min` threshold.
/// The returned span represents the complete matched region from start of first pattern to end of
/// last pattern (including all intervening separators).
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ws = neu::whitespace().many0();
///     let num = neu::digit(10).many1();
///     let ele = num.sep(",".suffix(ws));
///     let arr = ele.enclose("{", "}");
///     let mut ctx = CharsCtx::new("{11, 42, 8, 99}");
///
///     assert_eq!(ctx.try_mat(&arr)?, Span::new(0, 15));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// 1. Collects constructed values from each successful pattern match into a `Vec`
/// 2. Discards all separator values (they define structure but not semantics)
/// 3. Handles trailing separators based on `skip` flag:
///    - `skip = true`: Accepts sequences without trailing separator (standard behavior)
///    - `skip = false`: Requires trailing separator after each element (strict format)
/// 4. Validates that the total pattern count meets or exceeds `min`
/// 5. Returns the collected values only if count constraint is satisfied
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ws = neu::whitespace().many0();
///     let ty = ('a'..='z').or('A'..='Z').once();
///     let ty = ty.then(neu::word().many0()).pat();
///     let ele = ty.sep(",".suffix(ws));
///     let arr = ele.enclose("<", ">");
///     let mut ctx = CharsCtx::new("<Ctx, T, O1, O2>");
///
///     assert_eq!(ctx.ctor(&arr)?, vec!["Ctx", "T", "O1", "O2"]);
///
/// #   Ok(())
/// # }
/// ```
///
/// Optimization tips:
/// - Set `capacity` close to expected element count
/// - Use tight `min` bounds to fail early on invalid inputs
/// - Prefer `skip(true)` for more common formats (avoids extra separator checks)
#[derive(Default, Copy)]
pub struct Separate<C, P, S> {
    pat: P,
    sep: S,
    skip: bool,
    capacity: usize,
    min: usize,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Separate<C, P, S>);

impl<C, P, S> Debug for Separate<C, P, S>
where
    P: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Separate")
            .field("pat", &self.pat)
            .field("sep", &self.sep)
            .field("skip", &self.skip)
            .field("capacity", &self.capacity)
            .field("min", &self.min)
            .finish()
    }
}

impl<C, P, S> Clone for Separate<C, P, S>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            capacity: self.capacity,
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, S> Separate<C, P, S> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
            capacity: 0,
            min: 1,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn set_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, S, P, M, O, H> Ctor<'a, C, M, Vec<O>, H> for Separate<C, P, S>
where
    P: Ctor<'a, C, M, O, H>,
    S: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<Vec<O>, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut vals = Vec::with_capacity(self.capacity.max(self.min));
        let range: CRange<usize> = (self.min..).into();

        debug_ctor_beg!("Separate", range, ctx.beg());
        while let Ok(val) = self.pat.construct(ctx.ctx(), func) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                vals.push(val);
            }
            if res.is_err() {
                break;
            }
        }
        let len = vals.len();
        let ret = ctx.process_ret(if len >= self.min {
            Ok(vals)
        } else {
            Err(Error::Separate)
        });

        debug_ctor_reval!("Separate", range, ctx.beg(), ctx.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, S, P> Regex<C> for Separate<C, P, S>
where
    S: Regex<C>,
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = Span::new(ctx.ctx().offset(), 0);
        let range: CRange<usize> = (self.min..).into();

        debug_regex_beg!("Separate", range, ctx.beg());
        while let Ok(ret) = ctx.ctx().try_mat(&self.pat) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = res {
                    span.add_assign(sep_ret);
                }
            }
            if res.is_err() {
                break;
            }
        }
        let ret = if cnt >= self.min {
            Ok(span)
        } else {
            Err(Error::Separate)
        };
        debug_regex_reval!("Separate", range, ctx.process_ret(ret))
    }
}

///
/// Collects separator-delimited patterns into any [`FromIterator`] collection type.
///
/// This combinator extends [`Separate`] with **generic collection support**, allowing parsed
/// elements to be collected into any type implementing [`FromIterator`]—not just [`Vec<T>`]. This
/// enables direct construction of specialized collections like
/// [`HashSet`](std::collections::HashSet), [`BTreeSet`](std::collections::BTreeSet), [`String`],
/// or custom aggregate types without intermediate allocations. It maintains the same core
/// semantics for separator handling and minimum count enforcement, but provides ultimate
/// flexibility in result representation.
///
/// # Regex
///
/// Behaves identically to [`Separate`]:
/// - Matches pattern and separators repeatedly
/// - Returns a **single merged span** covering all patterns and separators
/// - Validates minimum count requirement (`min`)
/// - Includes all separators in the returned span
/// - Fails atomically if count constraint isn't met
///
/// The span covers from start of first pattern to end of last pattern (including all separators).
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let digit = neu::digit(10).many1();
///     let val = digit;
///     // using sep_collect is inconvenient for Regex
///     let vals = val.sep_collect::<_, &str, Vec<&str>>(",".ws());
///     let array = vals.enclose("[", "]");
///     let mut ctx = CharsCtx::new("[18, 24, 42, 58, 69]");
///
///     assert_eq!(ctx.try_mat(&array)?, Span::new(0, 20));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// 1. Creates a lazy iterator over successfully constructed pattern values
/// 2. Applies separator semantics:
///    - `skip = true`: Accepts sequences without trailing separator
///    - `skip = false`: Requires trailing separator after each element
/// 3. Validates that total pattern count meets `min` threshold
/// 4. Delegates collection to `V::from_iter()`, allowing arbitrary aggregation
/// 5. Returns collection only if count constraint is satisfied
///
/// The iterator-based approach enables **lazy evaluation**—elements are constructed on-demand
/// during collection, allowing short-circuiting in custom collectors.
///
/// ## Example
/// ```
/// # use std::collections::HashMap;
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let name = neu::word().many1();
///     let year = neu::digit(10).many1();
///     let year = year.try_map(map::from_str::<u32>());
///     let pair = name.sep_once(" => ", year);
///     let pairs = pair.sep_collect(", ");
///     let parser = pairs.enclose("{", "}");
///     let mut ctx = CharsCtx::new("{rust => 2015, golang => 2012, java => 1996}");
///     let hashmap: HashMap<&str, u32> = ctx.ctor(&parser)?;
///
///     assert_eq!(hashmap.get("rust"), Some(&2015));
///     assert_eq!(hashmap.get("golang"), Some(&2012));
///     assert_eq!(hashmap.get("java"), Some(&1996));
///
/// #   Ok(())
/// # }
/// ```
///
/// Performance guidance:
/// - **Use [`Separate`]** when you need a `Vec` (better optimized for this case)
/// - **Use [`SepCollect`]** when:
///   - Target collection is `HashSet`/`BTreeSet` (avoids `Vec→Set` conversion)
///   - You need custom aggregation logic
///   - Memory constraints favor specialized collections
///   - You want to avoid intermediate allocations
/// - **Avoid** for large collections with expensive `O` types (iterator overhead adds up)
///
/// # Notice
///
/// `SepCollect` will always succeed if the minimum size is 0, be careful to use it with other `.sep` faimly APIs.
/// The default size is 1.
#[derive(Default, Copy)]
pub struct SepCollect<C, P, S, O, V> {
    pat: P,
    sep: S,
    skip: bool,
    min: usize,
    marker: PhantomData<(C, O, V)>,
}

impl_not_for_regex!(SepCollect<C, P, S, O, V>);

impl<C, P, S, O, V> Debug for SepCollect<C, P, S, O, V>
where
    P: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SepCollect")
            .field("pat", &self.pat)
            .field("sep", &self.sep)
            .field("skip", &self.skip)
            .field("min", &self.min)
            .finish()
    }
}

impl<C, P, S, O, V> Clone for SepCollect<C, P, S, O, V>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, S, O, V> SepCollect<C, P, S, O, V> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
            min: 1,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, S, P, M, O, V, H> Ctor<'a, C, M, V, H> for SepCollect<C, P, S, O, V>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, M, O, H>,
    S: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<V, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut end = false;
        let range: CRange<usize> = (self.min..).into();
        let ret = {
            debug_ctor_beg!("SepCollect", range, ctx.beg());
            V::from_iter(std::iter::from_fn(|| {
                self.pat.construct(ctx.ctx(), func).ok().and_then(|ret| {
                    let res = ctx.ctx().try_mat(&self.sep);

                    if !end {
                        if res.is_err() {
                            end = true;
                        }
                        // The current value is captured only
                        // when the current delimiter matches successfully
                        // or the skip flag is true.
                        if res.is_ok() || self.skip {
                            cnt += 1;
                            return Some(ret);
                        }
                    }
                    None
                })
            }))
        };
        let ret = ctx.process_ret(if cnt >= self.min {
            Ok(ret)
        } else {
            Err(Error::SepCollect)
        });

        debug_ctor_reval!("SepCollect", range, ctx.beg(), ctx.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, S, P, O, V> Regex<C> for SepCollect<C, P, S, O, V>
where
    S: Regex<C>,
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = Span::new(ctx.ctx().offset(), 0);
        let mut ret = Err(Error::SepCollect);
        let range: CRange<usize> = (self.min..).into();

        debug_regex_beg!("SepCollect", range, ctx.beg());
        while let Ok(ret) = ctx.ctx().try_mat(&self.pat) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = res {
                    span.add_assign(sep_ret);
                }
            }
            if res.is_err() {
                break;
            }
        }
        if cnt >= self.min {
            ret = Ok(span);
        }
        debug_regex_reval!("SepCollect", range, ctx.process_ret(ret))
    }
}
