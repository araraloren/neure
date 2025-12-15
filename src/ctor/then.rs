use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctor::Map;
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
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Sequentially composes two expressions, requiring **both** to match in order.
///
/// The [`Then`] combinator implements **sequence semantics** where:
/// 1. The `left` expression must match first
/// 2. Upon success, the `right` expression must match at the **advanced position**
/// 3. Both expressions must succeed for the entire combinator to succeed
/// 4. On any failure, the context is **atomically reset** to the initial position
///
/// This is the fundamental building block for constructing compound patterns,
/// similar to concatenation in regular expressions (`ab` matches 'a' followed by 'b').
/// Unlike manual sequencing, [`Then`] automatically handles context management,
/// error propagation, and span concatenation with zero runtime overhead.
///
/// # Regex
///
/// In regex mode, [`Then`] returns a **combined span** covering both expressions:
/// 1. Attempts to match `left` at current position
/// 2. On success, attempts to match `right` at **advanced position**
/// 3. On both successes:
///    - Concatenates spans using `Span::add_assign`
///    - Returns single span covering entire matched region
/// 4. On any failure:
///    - Resets context to initial position
///    - Returns first error encountered
///
/// The resulting span is **contiguous** and covers exactly the input consumed by both expressions.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let str = neu::ascii_alphabetic().many1();
///     let str = str.enclose("\"", "\"");
///     let int = neu::digit(10).many1();
///     let tuple = str.then(int.prefix(" "));
///
///     assert_eq!(
///         CharsCtx::new(r#""Galaxy" 42"#).try_mat(&tuple)?,
///         Span::new(0, 11)
///     );
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// In constructor mode, [`Then`] returns a **tuple of values** `(O1, O2)`:
/// 1. Constructs value from `left` expression
/// 2. On success, constructs value from `right` expression at advanced position
/// 3. On both successes:
///    - Returns tuple `(left_value, right_value)`
///    - Context advances past both expressions
/// 4. On any failure:
///    - Resets context to initial position
///    - Returns first error encountered
///
/// This enables structured data extraction from sequential patterns, such as:
/// - Parsing `(key, value)` pairs
/// - Extracting components from compound identifiers
/// - Building AST nodes from multiple tokens
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let year = neu::digit(10).many1();
///     let desc = neu::word().many1();
///     let desc = desc.sep(neu::whitespace().many0()).prefix(" ");
///
///     let parser = year.then(desc);
///     let (answer, desc) = CharsCtx::new("42 is the answer").ctor(&parser)?;
///
///     assert_eq!(answer, "42");
///     assert_eq!(desc, ["is", "the", "answer"]);
///
/// #   Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Then<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Then<C, L, R>);

impl<C, L, R> Debug for Then<C, L, R>
where
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Then")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, R> Clone for Then<C, L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R> Then<C, L, R> {
    pub fn new(pat: L, then: R) -> Self {
        Self {
            left: pat,
            right: then,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, pat: L) -> &mut Self {
        self.left = pat;
        self
    }

    pub fn set_right(&mut self, then: R) -> &mut Self {
        self.right = then;
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

impl<'a, C, L, R, M, O1, O2, H> Ctor<'a, C, M, (O1, O2), H> for Then<C, L, R>
where
    L: Ctor<'a, C, M, O1, H>,
    R: Ctor<'a, C, M, O2, H>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("Then", g.beg());

        let ret =
            debug_ctor_stage!("Then", "l", self.left.construct(g.ctx(), func)).and_then(|ret1| {
                debug_ctor_stage!("Then", "r", self.right.construct(g.ctx(), func))
                    .map(|ret2| (ret1, ret2))
            });
        let ret = g.process_ret(ret);

        debug_ctor_reval!("Then", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, L, R> Regex<C> for Then<C, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("Then", g.beg());

        let mut ret = debug_regex_stage!("Then", "l", g.try_mat(&self.left)?);

        ret.add_assign(debug_regex_stage!("Then", "r", g.try_mat(&self.right)?));
        debug_regex_reval!("Then", Ok(ret))
    }
}

///
/// Conditionally extends a match with an optional suffix **only if** a test expression succeeds.
///
/// [`IfThen`] implements a **conditional sequence** pattern where:
/// 1. First matches the `left` expression (required)
/// 2. Then tests the `test` expression at the current position
/// 3. **Only if `test` succeeds**, matches the `right` expression
/// 4. Crucially: **`test` failure does NOT cause overall failure** - only `left` is required
///
/// This differs from standard sequence combinators by making the suffix conditional:
/// - Unlike [`Then<L, R>`]: The suffix (`test`+`right`) is optional
///
/// # Regex
///
/// Returns a [`Span`] covering:
/// - **Always** includes `left`'s matched text
/// - **Only if `test` succeeds**: Includes `test` + `right` matched text
/// - If `test` fails: Returns only `left`'s span (no error)
/// - If `test` succeeds but `right` fails: **Entire match fails**
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let name = neu::word().many1();
///     let paras = name.sep(", ").enclose("<", ">");
///     let test = regex::assert("<", true);
///     let parser = name.if_then(test, paras);
///
///     assert_eq!(CharsCtx::new("Vec").try_mat(&parser)?, Span::new(0, 3));
///     assert_eq!(
///         CharsCtx::new("Vec<A, B>").try_mat(&parser)?,
///         Span::new(0, 9)
///     );
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
/// Returns a tuple `(O1, Option<O2>)` where:
/// - `O1`: Value constructed from `left` (always present)
/// - `Option<O2>`:
///   - `Some(O2)` if **both `test` and `right` succeeded**
///   - `None` if `test` failed
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let val = neu::ascii_alphabetic().many1();
///     let tuple = val.if_then(",".skip_ws(), val).enclose("(", ")");
///
///     assert_eq!(CharsCtx::new("(abc)").ctor(&tuple)?, ("abc", None));
///     assert_eq!(
///         CharsCtx::new("(abc, cde)").ctor(&tuple)?,
///         ("abc", Some("cde"))
///     );
///
/// #   Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct IfThen<C, L, I, R> {
    left: L,
    test: I,
    right: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(IfThen<C, L, I, R>);

impl<C, L, I, R> Debug for IfThen<C, L, I, R>
where
    L: Debug,
    R: Debug,
    I: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfThen")
            .field("left", &self.left)
            .field("test", &self.test)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, I, R> Clone for IfThen<C, L, I, R>
where
    L: Clone,
    R: Clone,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            test: self.test.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, I, R> IfThen<C, L, I, R> {
    pub fn new(left: L, test: I, right: R) -> Self {
        Self {
            test,
            left,
            right,
            marker: PhantomData,
        }
    }

    pub fn test(&self) -> &I {
        &self.test
    }

    pub fn test_mut(&mut self) -> &mut I {
        &mut self.test
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_test(&mut self, test: I) -> &mut Self {
        self.test = test;
        self
    }

    pub fn set_left(&mut self, pat: L) -> &mut Self {
        self.left = pat;
        self
    }

    pub fn set_right(&mut self, then: R) -> &mut Self {
        self.right = then;
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

impl<'a, C, L, I, R, M, O1, O2, H> Ctor<'a, C, M, (O1, Option<O2>), H> for IfThen<C, L, I, R>
where
    L: Ctor<'a, C, M, O1, H>,
    R: Ctor<'a, C, M, O2, H>,
    I: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, Option<O2>), Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("IfThen", g.beg());

        let r_l = debug_ctor_stage!("IfThen", "l", self.left.construct(g.ctx(), func));
        let r_l = g.process_ret(r_l)?;
        let r_i = debug_ctor_stage!("IfThen", "test", g.try_mat(&self.test));
        let ret = if r_i.is_ok() {
            let r_r = debug_ctor_stage!("IfThen", "r", self.right.construct(g.ctx(), func));
            let r_r = g.process_ret(r_r)?;

            // if matched, return (01, Some(O2))
            (r_l, Some(r_r))
        } else {
            // not matched, return None
            (r_l, None)
        };

        debug_ctor_reval!("IfThen", g.beg(), g.end(), true);
        Ok(ret)
    }
}

impl<'a, C, L, I, R> Regex<C> for IfThen<C, L, I, R>
where
    I: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("IfThen", g.beg());

        let mut ret = debug_regex_stage!("IfThen", "l", g.try_mat(&self.left)?);

        if let Ok(span) = debug_regex_stage!("IfThen", "test", g.try_mat(&self.test)) {
            ret.add_assign(span);
            ret.add_assign(debug_regex_stage!("IfThen", "r", g.try_mat(&self.right)?));
        }
        debug_regex_reval!("IfThen", Ok(ret))
    }
}
