use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
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
/// Conditional branching combinator that selects between two patterns based on a test function.
///
/// This struct enables context-sensitive parsing decisions by evaluating a test function before
/// attempting to match or construct values. The test function examines the current context without
/// consuming any input, allowing for lookahead-based decisions.
///
/// # Regex
///
/// Evaluates the test function on the current context. If the test returns `true`, attempts to
/// match pattern `P`. If the test returns `false`, attempts to match pattern `E` instead. The
/// context position is only advanced if the selected pattern successfully matches.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = ctor::branch(
///         |ctx: &CharsCtx| Ok(ctx.len() - ctx.offset() >= 3),
///         neu::ascii_digit().count::<3>(),
///         neu::ascii_digit().many0(),
///     );
///
///     assert_eq!(CharsCtx::new("21345").try_mat(&regex)?, Span::new(0, 3));
///     assert_eq!(CharsCtx::new("42").try_mat(&regex)?, Span::new(0, 2));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Similar to the `Regex` behavior, but constructs values instead of just matching. Evaluates
/// the test function first, then constructs a value using either pattern `P` or pattern `E`
/// depending on the test result. The context position is restored to its original state if
/// the selected pattern fails to construct a value.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let re1 = "google".sep_once(".", "com".or("is")).pat();
///     let re2 = "google"
///         .sep_once(".", "co".sep_once(".", "kr".or("jp")))
///         .pat();
///     let regex = re2.if_else(
///         |ctx: &CharsCtx| ctx.orig().map(|v| v.ends_with("jp") || v.ends_with("kr")),
///         re1,
///     );
///
///     assert_eq!(CharsCtx::new("google.com").ctor(&regex)?, "google.com");
///     assert_eq!(CharsCtx::new("google.is").ctor(&regex)?, "google.is");
///     assert_eq!(CharsCtx::new("google.co.jp").ctor(&regex)?, "google.co.jp");
///     assert_eq!(CharsCtx::new("google.co.kr").ctor(&regex)?, "google.co.kr");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Test Function
///
/// The test function receives an immutable reference to the context and must return either:
/// - `Ok(true)` to select pattern `P`
/// - `Ok(false)` to select pattern `E`
/// - `Err(Error)` to immediately fail the entire branch operation
///
/// The test function should not modify the context state, as it receives an immutable reference.
/// It's ideal for examining lookahead data, checking remaining input length, or inspecting the
/// original input string at the current position.
///
/// # Performance
///
/// The test function is evaluated exactly once per branch attempt. Both patterns are only
/// compiled and stored, not executed, until needed. This makes branch selection very efficient,
/// especially when the test function is simple.
pub struct Branch<C, P, F, E> {
    pat: P,
    test: F,
    other: E,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Branch<C, P, I, E>);

impl<C, P, F, E> Debug for Branch<C, P, F, E>
where
    P: Debug,
    F: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Branch")
            .field("pat", &self.pat)
            .field("test", &self.test)
            .field("other", &self.other)
            .finish()
    }
}

impl<C, P, F, E> Default for Branch<C, P, F, E>
where
    P: Default,
    F: Default,
    E: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            test: Default::default(),
            other: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P, F, E> Clone for Branch<C, P, F, E>
where
    P: Clone,
    F: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            test: self.test.clone(),
            other: self.other.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F, E> Copy for Branch<C, P, F, E>
where
    P: Copy,
    F: Copy,
    E: Copy,
{
}

impl<C, P, F, E> Branch<C, P, F, E> {
    pub const fn new(test: F, pat: P, other: E) -> Self {
        Self {
            pat,
            test,
            other,
            marker: PhantomData,
        }
    }

    pub const fn pat(&self) -> &P {
        &self.pat
    }

    pub const fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub const fn test(&self) -> &F {
        &self.test
    }

    pub const fn test_mut(&mut self) -> &mut F {
        &mut self.test
    }

    pub const fn other(&self) -> &E {
        &self.other
    }

    pub const fn other_mut(&mut self) -> &mut E {
        &mut self.other
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_test(&mut self, test: F) -> &mut Self {
        self.test = test;
        self
    }

    pub fn set_other(&mut self, other: E) -> &mut Self {
        self.other = other;
        self
    }
}

impl<'a, C, P, F, E, O, H> Ctor<'a, C, O, H> for Branch<C, P, F, E>
where
    P: Ctor<'a, C, O, H>,
    E: Ctor<'a, C, O, H>,
    C: Match<'a>,
    F: Fn(&C) -> Result<bool, Error>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let offset = ctx.offset();

        debug_ctor_beg!("Branch", offset);

        let ret = debug_ctor_stage!("Branch", "test", (self.test)(ctx)?);
        let ret = if ret {
            debug_ctor_stage!("Branch", "true", self.pat.construct(ctx, func))
        } else {
            debug_ctor_stage!("Branch", "false", self.other.construct(ctx, func))
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        debug_ctor_reval!("Branch", offset, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, P, F, E> Regex<C> for Branch<C, P, F, E>
where
    P: Regex<C>,
    E: Regex<C>,
    C: Match<'a>,
    F: Fn(&C) -> Result<bool, Error>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let offset = ctx.offset();

        debug_regex_beg!("Branch", offset);

        let ret = debug_regex_stage!("Branch", "test", (self.test)(ctx)?);
        let ret = if ret {
            debug_regex_stage!("Branch", "true", ctx.try_mat(&self.pat))
        } else {
            debug_regex_stage!("Branch", "false", ctx.try_mat(&self.other))
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        debug_regex_reval!("Branch", ret)
    }
}

pub const fn branch<'a, C, P, F, E>(test: F, pat: P, other: E) -> Branch<C, P, F, E>
where
    C: Match<'a>,
    E: Regex<C>,
    P: Regex<C>,
    F: Fn(&C) -> Result<bool, Error>,
{
    Branch::new(test, pat, other)
}
