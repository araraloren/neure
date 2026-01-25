use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::RangeBounds;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::neu::CRange;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Repeats a pattern a specified number of times, collecting results or spans based on context.
///
/// This combinator matches a pattern repeatedly within a defined count range, supporting both
/// value construction ([`Ctor`]) and span matching ([`Regex`]). It optimizes performance through
/// pre-allocation and early termination, while providing precise range validation and context
/// safety. Designed for parsing lists, sequences, and repeated structures with explicit bounds.
///
/// # Regex
///
/// Matches the pattern repeatedly and returns a **single merged span** covering all successful
/// matches. The matching continues until either:
/// 1. The pattern fails to match
/// 2. The maximum count in the range is reached
///
/// The result is valid only if the total match count falls within the specified range. If the
/// count is outside the range, matching fails and the context position is restored to its original
/// state. The merged span represents the complete sequence from the first match start to the last
/// match end.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let char = neu::word().count::<2>();
///     let num = char.skip_ws().repeat(1..);
///     let mut ctx = CharsCtx::new(r#"Hello, World!"#);
///
///     assert_eq!(ctx.try_mat(&num)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// 1. Collects constructed values from each successful pattern match into a [`Vec`](crate::alloc::Vec)
/// 2. Continues matching until pattern failure or maximum count reached
/// 3. Validates that the total match count falls within the specified range
/// 4. Returns the collected values only if the count constraint is satisfied
///
/// The inner pattern's handler is invoked for each match, with each result preserved in order.
/// Memory is pre-allocated using the `capacity` field to minimize reallocations. If the final
/// count doesn't satisfy the range constraint, all collected values are discarded and the context
/// is restored to its initial position.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let char = neu::always().once();
///     let num = char.skip_ws().repeat(1..);
///     let mut ctx = CharsCtx::new(r#"你好，世界？"#);
///
///     assert_eq!(ctx.ctor(&num)?, ["你", "好", "，", "世", "界", "？"]);
/// #   Ok(())
/// # }
/// ```
///
/// Optimization tips:
/// - Set capacity close to expected match count
/// - Use tight ranges to limit unnecessary matching attempts
#[derive(Copy)]
pub struct Repeat<C, P> {
    pat: P,
    range: CRange<usize>,
    capacity: usize,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Repeat<C, P>);

impl<C, P> Debug for Repeat<C, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Repeat")
            .field("pat", &self.pat)
            .field("range", &self.range)
            .field("capacity", &self.capacity)
            .finish()
    }
}

impl<C, P> Default for Repeat<C, P>
where
    P: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            range: Default::default(),
            capacity: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P> Clone for Repeat<C, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            range: self.range,
            capacity: self.capacity,
            marker: self.marker,
        }
    }
}

impl<C, P> Repeat<C, P> {
    pub fn new(pat: P, range: impl Into<CRange<usize>>) -> Self {
        let range = range.into();
        let capacity = Self::guess_capacity(&range, 0);

        Self {
            pat,
            range,
            capacity,
            marker: PhantomData,
        }
    }

    pub fn guess_capacity(range: &CRange<usize>, val: usize) -> usize {
        let start = match range.start_bound() {
            core::ops::Bound::Included(v) => *v,
            core::ops::Bound::Excluded(v) => *v,
            core::ops::Bound::Unbounded => val,
        };
        start.max(val)
    }

    pub const fn pat(&self) -> &P {
        &self.pat
    }

    pub const fn range(&self) -> &CRange<usize> {
        &self.range
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub const fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_range(&mut self, range: impl Into<CRange<usize>>) -> &mut Self {
        self.range = range.into();
        self
    }

    pub fn set_capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn with_pat(mut self, pat: P) -> Self {
        self.pat = pat;
        self
    }

    pub fn with_range(mut self, range: impl Into<CRange<usize>>) -> Self {
        self.range = range.into();
        self
    }

    pub fn with_capacity(mut self, cap: usize) -> Self {
        self.capacity = cap;
        self
    }

    fn is_contain(&self, count: usize) -> bool {
        match core::ops::RangeBounds::end_bound(&self.range) {
            core::ops::Bound::Included(max) => count < *max,
            core::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            core::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, C, P, O, H> Ctor<'a, C, crate::alloc::Vec<O>, H> for Repeat<C, P>
where
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<crate::alloc::Vec<O>, Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let mut vals = crate::alloc::Vec::with_capacity(self.capacity);

        crate::debug_ctor_beg!("Repeat", self.range, offset);
        while self.is_contain(cnt) {
            if let Ok(val) = self.pat.construct(ctx, handler) {
                vals.push(val);
                cnt += 1;
            } else {
                break;
            }
        }
        let ret = if core::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(vals)
        } else {
            Err(Error::Repeat)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_ctor_reval!("Repeat", offset, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, P> Regex<C> for Repeat<C, P>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let mut total = Span::new(offset, 0);

        crate::debug_regex_beg!("Repeat", self.range, offset);
        while self.is_contain(cnt) {
            if let Ok(p_span) = ctx.try_mat(&self.pat) {
                total.add_assign(p_span);
                cnt += 1;
            } else {
                break;
            }
        }
        let ret = if core::ops::RangeBounds::contains(&self.range, &cnt) {
            Ok(total)
        } else {
            Err(Error::Repeat)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_regex_reval!("Repeat", ret)
    }
}
