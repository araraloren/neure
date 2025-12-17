use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Adapts a [`Regex`]-only pattern to be usable in [`Ctor`] contexts by providing span-to-value conversion.
///
/// This adapter bridges the gap between pure matching patterns and value-constructing parsers.
/// It wraps any pattern that implements [`Regex<C>`] and adds [`Ctor`] capability by using a handler
/// to convert matched spans into concrete values. This enables seamless integration of basic
/// regex patterns into complex construction pipelines without requiring them to implement [`Ctor`]
/// directly.
///
/// # Regex
///
/// Delegates matching directly to the wrapped pattern. Returns exactly the same span that the
/// underlying pattern would return. No additional behavior or transformation is applied.
/// This maintains the exact matching semantics of the wrapped pattern while making it available
/// through the [`Regex`] trait interface.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let website = "abc".sep_once(".", "com");
///     let pat = website.pat();
///
///     assert_eq!(CharsCtx::new("abc.com").try_mat(&website)?, Span::new(0, 7));
///     assert_eq!(CharsCtx::new("abc.com").try_mat(&pat)?, Span::new(0, 7));
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// First matches the wrapped pattern to obtain a [`Span`], returns the handler's result
/// as the constructed value.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let website = "abc".sep_once(".", "com");
///     let pat = website.pat();
///
///     assert_eq!(CharsCtx::new("abc.com").ctor(&website)?, ("abc", "com"));
///     assert_eq!(CharsCtx::new("abc.com").ctor(&pat)?, "abc.com");
/// #   Ok(())
/// # }
/// ```
///
/// Optimization tips:
/// - Use simple handlers for high-frequency patterns
/// - Avoid expensive string allocations in handlers when possible
/// - Consider caching handler logic for repeated pattern usage
#[derive(Default, Copy)]
pub struct Pattern<C, P> {
    pat: P,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Pattern<C, P>);

impl<C, P> Debug for Pattern<C, P>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern").field("pat", &self.pat).finish()
    }
}

impl<C, P> Clone for Pattern<C, P>
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

impl<C, P> Pattern<C, P> {
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

impl<'a, C, O, P, H> Ctor<'a, C, O, H> for Pattern<C, P>
where
    P: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(&self.pat)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, P> Regex<C> for Pattern<C, P>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        ctx.try_mat(&self.pat)
    }
}
