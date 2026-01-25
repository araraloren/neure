use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Matches a pattern followed by a mandatory suffix, returning only the main pattern's value during construction.
///
/// This combinator requires both the main pattern and suffix pattern to match consecutively.
/// During construction, it returns only the value from the main pattern while still enforcing
/// the presence of the suffix. During matching, it returns the combined span of both patterns.
///
/// # Regex
///
/// Attempts to match the main pattern first, then immediately attempts to match the suffix
/// pattern at the new position. Returns a single span covering both matches concatenated
/// together. Fails if either pattern fails to match. The combined span is created by merging
/// the two spans sequentially, requiring them to be adjacent with no gaps.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let protocol = "https".or("http".or("ftp"));
///     let protocol = protocol.suffix("://");
///     let domain = neu::alphabetic().many1();
///     let domain = domain.sep(".").at_least(2);
///     let url = protocol.then(domain);
///     let mut ctx = CharsCtx::new(r#"ftp://ftp.kernel.org"#);
///
///     assert_eq!(ctx.try_mat(&url)?, Span::new(0, ctx.len()));
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// First constructs a value using the main pattern. If successful, it then attempts to match
/// the suffix pattern (without using its value). Returns the main pattern's constructed value
/// only if both patterns succeed. Fails if either pattern fails, with errors from the suffix
/// taking precedence when both fail. The context position advances past both patterns on success.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let protocol = "https".or("http".or("ftp"));
///     let protocol = protocol.suffix("://");
///     let domain = neu::alphabetic().many1();
///     let domain = domain.sep(".").at_least(2);
///     let url = protocol.then(domain);
///     let mut ctx = CharsCtx::new(r#"https://www.mozilla.org"#);
///
///     assert_eq!(ctx.ctor(&url)?, ("https", vec!["www", "mozilla", "org"]));
/// #   Ok(())
/// # }
/// ```
///
/// # Behavior Notes
///
/// - Both patterns must match consecutively with no gaps between them
/// - The suffix pattern is mandatory - failure to match it causes the entire pattern to fail
/// - During construction:
///   - Only the main pattern's value is returned
///   - The suffix pattern's value is ignored (but its match is required)
/// - During matching:
///   - Returns a single span covering both patterns
///   - The span length equals the sum of both pattern spans
/// - Context position is advanced past both patterns only on complete success
/// - Errors from the suffix pattern mask errors from the main pattern when both fail
///
/// # Performance
///
/// Both patterns are always evaluated in sequence. For optimal performance:
/// - Place cheaper patterns first when possible
/// - Ensure the main pattern fails quickly on invalid inputs
/// - Avoid expensive operations in the suffix pattern
#[derive(Copy)]
pub struct Suffix<C, P, T> {
    pat: P,
    suffix: T,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Suffix<C, P, T>);

impl<C, P, T> Debug for Suffix<C, P, T>
where
    P: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PadUnit")
            .field("pat", &self.pat)
            .field("suffix", &self.suffix)
            .finish()
    }
}

impl<C, P, T> Default for Suffix<C, P, T>
where
    P: Default,
    T: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            suffix: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P, T> Clone for Suffix<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            suffix: self.suffix.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> Suffix<C, P, T> {
    pub const fn new(pat: P, suffix: T) -> Self {
        Self {
            pat,
            suffix,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn suffix(&self) -> &T {
        &self.suffix
    }

    pub fn suffix_mut(&mut self) -> &mut T {
        &mut self.suffix
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_suffix(&mut self, suffix: T) -> &mut Self {
        self.suffix = suffix;
        self
    }
}

impl<'a, C, P, T, O, H> Ctor<'a, C, O, H> for Suffix<C, P, T>
where
    T: Regex<C>,
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_ctor_beg!("Suffix", ctx.beg());

        let ret = debug_ctor_stage!("Suffix", "pat", self.pat.construct(ctx.ctx(), func));

        if ret.is_ok() {
            let _ = debug_ctor_stage!("Suffix", "suffix", ctx.try_mat(&self.suffix)?);
        }
        debug_ctor_reval!("Suffix", ctx.beg(), ctx.end(), ret.is_ok());
        ctx.process_ret(ret)
    }
}

impl<'a, C, P, T> Regex<C> for Suffix<C, P, T>
where
    T: Regex<C>,
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("Suffix", ctx.beg());
        let mut ret = debug_regex_stage!("Suffix", "pat", ctx.try_mat(&self.pat)?);

        ret.add_assign(debug_regex_stage!(
            "Suffix",
            "suffix",
            ctx.try_mat(&self.suffix)?
        ));
        debug_regex_reval!("Suffix", Ok(ret))
    }
}

///
/// Matches a mandatory prefix followed by a pattern, returning only the main pattern's value during construction.
///
/// This combinator requires both the prefix pattern and main pattern to match consecutively.
/// During construction, it returns only the value from the main pattern while still enforcing
/// the presence of the prefix. During matching, it returns the combined span of both patterns.
///
/// # Regex
///
/// Attempts to match the prefix pattern first, then immediately attempts to match the main
/// pattern at the new position. Returns a single span covering both matches concatenated
/// together. Fails if either pattern fails to match. The combined span is created by merging
/// the two spans sequentially, requiring them to be adjacent with no gaps.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let protocol = "https".or("http".or("ftp")).suffix("://");
///     let domain = neu::alphabetic().many1();
///     let domain = domain.sep(".").at_least(2);
///     let url = domain.prefix(protocol);
///     let mut ctx = CharsCtx::new(r#"ftp://ftp.kernel.org"#);
///
///     assert_eq!(ctx.try_mat(&url)?, Span::new(0, ctx.len()));
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// First matches the prefix pattern (without using its value). If successful, it then constructs
/// a value using the main pattern. Returns the main pattern's constructed value only if both
/// patterns succeed. Fails if either pattern fails, with errors from the main pattern taking
/// precedence when both fail. The context position advances past both patterns on success.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let protocol = "https".or("http".or("ftp")).suffix("://");
///     let domain = neu::alphabetic().many1();
///     let domain = domain.sep(".").at_least(2);
///     let url = domain.prefix(protocol);
///     let mut ctx = CharsCtx::new(r#"ftp://ftp.kernel.org"#);
///
///     assert_eq!(ctx.ctor(&url)?, ["ftp", "kernel", "org"]);
/// #   Ok(())
/// # }
/// ```
///
/// # Behavior Notes
///
/// - Both patterns must match consecutively with no gaps between them
/// - The prefix pattern is mandatory - failure to match it causes the entire pattern to fail
/// - During construction:
///   - Only the main pattern's value is returned
///   - The prefix pattern's value is ignored (but its match is required)
/// - During matching:
///   - Returns a single span covering both patterns
///   - The span length equals the sum of both pattern spans
/// - Context position is advanced past both patterns only on complete success
/// - Errors from the main pattern mask errors from the prefix pattern when both fail
///
/// # Performance
///
/// Both patterns are evaluated in sequence. For optimal performance:
/// - Place cheaper patterns first (the prefix is evaluated before the main pattern)
/// - Ensure the prefix pattern fails quickly on invalid inputs
/// - Avoid expensive operations in the prefix pattern since it's evaluated unconditionally
#[derive(Copy)]
pub struct Prefix<C, P, T> {
    pat: P,
    prefix: T,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Prefix<C, P, T>);

impl<C, P, T> Debug for Prefix<C, P, T>
where
    P: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PaddedUnit")
            .field("pat", &self.pat)
            .field("prefix", &self.prefix)
            .finish()
    }
}

impl<C, P, T> Default for Prefix<C, P, T>
where
    P: Default,
    T: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            prefix: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P, T> Clone for Prefix<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            prefix: self.prefix.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> Prefix<C, P, T> {
    pub const fn new(pat: P, prefix: T) -> Self {
        Self {
            pat,
            prefix,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn prefix(&self) -> &T {
        &self.prefix
    }

    pub fn prefix_mut(&mut self) -> &mut T {
        &mut self.prefix
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_prefix(&mut self, prefix: T) -> &mut Self {
        self.prefix = prefix;
        self
    }
}

impl<'a, C, P, T, O, H> Ctor<'a, C, O, H> for Prefix<C, P, T>
where
    T: Regex<C>,
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_ctor_beg!("Prefix", ctx.beg());

        let _ = debug_ctor_stage!("Prefix", "head", ctx.try_mat(&self.prefix)?);
        let r = debug_ctor_stage!("Prefix", "prefix", self.pat.construct(ctx.ctx(), func));

        debug_ctor_reval!("Prefix", ctx.beg(), ctx.end(), r.is_ok());
        ctx.process_ret(r)
    }
}

impl<'a, C, P, T> Regex<C> for Prefix<C, P, T>
where
    T: Regex<C>,
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("Prefix", ctx.beg());

        let mut ret = debug_regex_stage!("Prefix", "head", ctx.try_mat(&self.prefix)?);

        ret.add_assign(debug_regex_stage!(
            "Prefix",
            "prefix",
            ctx.try_mat(&self.pat)?
        ));
        debug_regex_reval!("Prefix", Ok(ret))
    }
}
