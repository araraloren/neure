use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;
use crate::span::Span;

///
/// Repeats a pattern a specified number of times, collecting results or spans based on context.
///
/// This combinator matches a pattern repeatedly within a defined count range, supporting both
/// value construction ([`Ctor`]) and span matching ([`Regex`]). It optimizes performance through
/// pre-allocation and early termination, while providing precise range validation and context
/// safety. Designed for parsing lists, sequences, and repeated structures with explicit bounds.
/// Unlike its dynamic counterpart [`Repeat`](crate::ctor::Repeat), it uses compile-time
/// fixed bounds (`M` and `N`) and stores
/// results in a stack-allocated array(\[`Option<O>`; N\]) rather than a heap-allocated vector.
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
/// 1. Collects constructed values from each successful pattern match into a **fixed-size array** (`[Option<O>; N]`), where `N` is the maximum capacity specified at compile time
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
///     let num = char.skip_ws().repeat2::<1, 6>();
///     let mut ctx = CharsCtx::new(r#"你好，世界？"#);
///
///     assert_eq!(ctx.ctor(&num)?, [Some("你"), Some("好"), Some("，"), Some("世"), Some("界"), Some("？")]);
/// #   Ok(())
/// # }
/// ```
///
/// Optimization tips:
/// - Set capacity close to expected match count
/// - Use tight ranges to limit unnecessary matching attempts
#[derive(Copy)]
pub struct Repeat2<C, P, const M: usize, const N: usize> {
    pat: P,
    marker: PhantomData<C>,
}

impl<C, P, const M: usize, const N: usize> core::ops::Not for Repeat2<C, P, M, N> {
    type Output = crate::regex::Assert<Self>;

    fn not(self) -> Self::Output {
        crate::regex::not(self)
    }
}

impl<C, P, const M: usize, const N: usize> Debug for Repeat2<C, P, M, N>
where
    P: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Repeat2").field("pat", &self.pat).finish()
    }
}

impl<C, P, const M: usize, const N: usize> Default for Repeat2<C, P, M, N>
where
    P: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P, const M: usize, const N: usize> Clone for Repeat2<C, P, M, N>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, const M: usize, const N: usize> Repeat2<C, P, M, N> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn with_pat(mut self, pat: P) -> Self {
        self.pat = pat;
        self
    }
}

impl<'a, C, P, const M: usize, const N: usize, O, H> Ctor<'a, C, [Option<O>; N], H>
    for Repeat2<C, P, M, N>
where
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<[Option<O>; N], Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let mut vals = [const { None }; N];
        let range = M..=N;

        crate::debug_ctor_beg!("Repeat2", &range, offset);
        while cnt <= N {
            if let Ok(val) = self.pat.construct(ctx, handler) {
                vals[cnt] = Some(val);
                cnt += 1;
            } else {
                break;
            }
        }
        let ret = if range.contains(&cnt) {
            Ok(vals)
        } else {
            Err(Error::Repeat2)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_ctor_reval!("Repeat2", offset, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, P, const M: usize, const N: usize> Regex<C> for Repeat2<C, P, M, N>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let mut total = Span::new(offset, 0);
        let range = M..=N;

        crate::debug_regex_beg!("Repeat2", &range, offset);
        while cnt <= N {
            if let Ok(p_span) = ctx.try_mat(&self.pat) {
                total.add_assign(p_span);
                cnt += 1;
            } else {
                break;
            }
        }
        let ret = if range.contains(&cnt) {
            Ok(total)
        } else {
            Err(Error::Repeat2)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_regex_reval!("Repeat2", ret)
    }
}
