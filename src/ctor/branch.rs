use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

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
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let regex = ctor::branch(
///         |ctx: &CharsCtx| Ok(ctx.len() - ctx.offset() >= 3),
///         neu::ascii_digit().repeat_times::<3>(),
///         neu::ascii_digit().repeat_full(),
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
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let re1 = "google".sep_once(".", "com".or("is")).pat();
///     let re2 = "google"
///         .sep_once(".", "co".sep_once(".", "kr".or("jp")))
///         .pat();
///     let regex = re2.branch(
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
#[derive(Default, Copy)]
pub struct Branch<C, P, F, E> {
    pat: P,
    test: F,
    other: E,
    marker: PhantomData<C>,
}

def_not!(Branch<C, P, I, E>);

impl<C, P, F, E> Debug for Branch<C, P, F, E>
where
    P: Debug,
    F: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Branch")
            .field("pat", &self.pat)
            .field("test", &self.test)
            .field("other", &self.other)
            .finish()
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

impl<C, P, F, E> Branch<C, P, F, E> {
    pub fn new(test: F, pat: P, other: E) -> Self {
        Self {
            pat,
            test,
            other,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn test(&self) -> &F {
        &self.test
    }

    pub fn test_mut(&mut self) -> &mut F {
        &mut self.test
    }

    pub fn other(&self) -> &E {
        &self.other
    }

    pub fn other_mut(&mut self) -> &mut E {
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

impl<'a, C, P, F, E, M, O, H, A> Ctor<'a, C, M, O, H, A> for Branch<C, P, F, E>
where
    P: Ctor<'a, C, M, O, H, A>,
    E: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<'a>,
    F: Fn(&C) -> Result<bool, Error>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("Branch", g.beg());

        let ret = debug_ctor_stage!("Branch", "test", (self.test)(g.ctx())?);
        let ret = if ret {
            debug_ctor_stage!("Branch", "true", self.pat.construct(g.ctx(), func))
        } else {
            debug_ctor_stage!(
                "Branch",
                "false",
                self.other.construct(g.reset().ctx(), func)
            )
        };

        debug_ctor_reval!("Branch", g.beg(), g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, P, F, E> Regex<C> for Branch<C, P, F, E>
where
    P: Regex<C>,
    E: Regex<C>,
    C: Context<'a> + Match<'a>,
    F: Fn(&C) -> Result<bool, Error>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("Branch", g.beg());

        let ret = debug_regex_stage!("Branch", "test", (self.test)(g.ctx())?);
        let ret = if ret {
            debug_regex_stage!("Branch", "true", g.try_mat(&self.pat))
        } else {
            debug_regex_stage!("Branch", "false", g.try_mat(&self.other))
        };

        debug_regex_reval!("Branch", ret)
    }
}

pub fn branch<'a, C, P, F, E>(test: F, pat: P, other: E) -> Branch<C, P, F, E>
where
    C: Context<'a> + Match<'a>,
    E: Regex<C>,
    P: Regex<C>,
    F: Fn(&C) -> Result<bool, Error>,
{
    Branch::new(test, pat, other)
}
