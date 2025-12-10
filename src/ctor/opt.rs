use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

///
/// Makes a pattern optional, returning `None` (for [`Ctor`]) or an empty span (for [`Regex`]) when the pattern fails.
///
/// This combinator transforms a required pattern into an optional one. It attempts to match
/// or construct using the inner pattern, but instead of propagating errors when the pattern
/// fails, it returns a neutral value (`None` for [`Ctor`], zero-length span for [`Regex`]).
///
/// # Regex
///
/// Attempts to match the inner pattern. If successful, returns the matched span normally.
/// If the inner pattern fails to match, returns a zero-length span starting at the current
/// context position (consuming no input) instead of returning an error. This ensures the
/// optional pattern never fails to match.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let num = regex!((neu::digit(16))+).opt();
///
///     assert_eq!(CharsCtx::new("f1").try_mat(&num)?, Span::new(0, 2));
///     assert_eq!(CharsCtx::new("p8").try_mat(&num)?, Span::default());
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Attempts to construct a value using the inner pattern. If successful, returns `Some(O)`.
/// If the inner pattern fails (returns an error), the error is consumed and `None` is returned
/// instead. The context position is restored to its original state when the inner pattern fails,
/// ensuring no partial consumption of input on failure.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let num = regex!((neu::digit(10))+).try_map(map::from_str()).opt();
///
///     assert_eq!(CharsCtx::new("42").ctor(&num)?, Some(42i32));
///     assert_eq!(CharsCtx::new("f1").ctor(&num)?, None);
///
/// #   Ok(())
/// # }
/// ```
///
/// # Behavior Notes
///
/// - This combinator never fails in the `Regex` implementation (always returns `Ok(Span)`)
/// - In the `Ctor` implementation, it never propagates errors from the inner pattern
/// - When the inner pattern fails:
///   - For `Regex`: Returns `Span::new(offset, 0)` (zero-length span at current position)
///   - For `Ctor`: Returns `Ok(None)` and restores context position
/// - When the inner pattern succeeds:
///   - For `Regex`: Returns the actual span of the match
///   - For `Ctor`: Returns `Ok(Some(value))` with the constructed value
///
/// # Performance
///
/// The optional pattern adds minimal overhead. When the inner pattern fails, the context guard
/// efficiently restores the position without additional allocations. The zero-cost abstraction
/// principles of Rust ensure this combinator has performance comparable to manual optional handling.
#[derive(Default, Copy)]
pub struct OptionPat<C, P> {
    pat: P,
    marker: PhantomData<C>,
}

def_not!(OptionPat<C, P>);

impl<C, P> Debug for OptionPat<C, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionPat").field("pat", &self.pat).finish()
    }
}

impl<C, P> Clone for OptionPat<C, P>
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

impl<C, P> OptionPat<C, P> {
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
}

impl<'a, C, M, O, P, H> Ctor<'a, C, M, Option<O>, H> for OptionPat<C, P>
where
    P: Ctor<'a, C, M, O, H>,
    C: Match<'a>,
    H: Handler<C, Out = M,>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<Option<O>, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("OptionPat", g.beg());
        let ret = self.pat.construct(g.ctx(), func);
        let ret = g.process_ret(ret).ok();

        debug_ctor_reval!("OptionPat", g.beg(), g.end(), ret.is_some());
        Ok(ret)
    }
}

impl<'a, C, P> Regex<C> for OptionPat<C, P>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        debug_regex_beg!("OptionPat", ctx.offset());

        let ret = ctx.try_mat(&self.pat);
        let ret = ret.unwrap_or(Span::new(ctx.offset(), 0));

        debug_regex_reval!("OptionPat", Ok(ret))
    }
}
