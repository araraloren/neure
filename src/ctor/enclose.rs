use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::span::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;

//
/// Wraps a pattern between opening and closing delimiters, forming a boundary-enclosed structure.
///
/// This combinator matches a pattern surrounded by opening and closing expressions, commonly used
/// for quoted strings, bracketed arrays, parenthesized expressions, and similar structures.
///
/// # Regex
///
/// Matches all three components sequentially and returns a **single merged span** covering:
/// 1. The opening delimiter match
/// 2. The inner pattern match  
/// 3. The closing delimiter match
///
/// The returned span represents the complete enclosed structure from start of opening delimiter
/// to end of closing delimiter. If any component fails to match, the entire match fails and the
/// context position remains unchanged.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let digit = neu::digit(10).many0();
///     let parser = digit.enclose("(", ")");
///
///     assert_eq!(CharsCtx::new("(42686)").try_mat(&parser)?, Span::new(0, 7));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// 1. Matches the opening delimiter (value discarded)
/// 2. Constructs the inner pattern's value using the provided handler
/// 3. Matches the closing delimiter (value discarded)
/// 4. Returns **only the inner pattern's constructed value**
///
/// The opening/closing delimiters serve purely as syntactic boundaries—their matched values are
/// never exposed to the user. Only the inner pattern's value is preserved and returned. This
/// follows the principle of "syntax vs. semantics separation" where boundary markers define
/// structure but don't contribute to the parsed value.
///
/// ## Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let str_val = '"'.not().many1();
///     let str = str_val.enclose("\"", "\"");
///
///     assert_eq!(CharsCtx::new("\"rust\"").ctor(&str)?, "rust");
///
/// #   Ok(())
/// # }
/// ```
///
/// ## Example2
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let cond = neu::regex_cond(!('r'.then('#')));
///     let str_val = '#'.not().many1().set_cond(cond);
///     let str = str_val.enclose("r#", "#");
///
///     assert_eq!(
///         CharsCtx::new("r#rust raw string#").ctor(&str)?,
///         "rust raw string"
///     );
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Enclose<C, P, L, R> {
    pat: P,
    open: L,
    close: R,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Enclose<C, P, L, R>);

impl<C, P, L, R> Debug for Enclose<C, P, L, R>
where
    P: Debug,
    L: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enclose")
            .field("pat", &self.pat)
            .field("open", &self.open)
            .field("close", &self.close)
            .finish()
    }
}

impl<C, P, L, R> Default for Enclose<C, P, L, R>
where
    P: Default,
    L: Default,
    R: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            open: Default::default(),
            close: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<C, P, L, R> Clone for Enclose<C, P, L, R>
where
    P: Clone,
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            open: self.open.clone(),
            close: self.close.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, L, R> Enclose<C, P, L, R> {
    pub fn new(pat: P, open: L, close: R) -> Self {
        Self {
            pat,
            open,
            close,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn open(&self) -> &L {
        &self.open
    }

    pub fn open_mut(&mut self) -> &mut L {
        &mut self.open
    }

    pub fn close(&self) -> &R {
        &self.close
    }

    pub fn close_mut(&mut self) -> &mut R {
        &mut self.close
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_open(&mut self, open: L) -> &mut Self {
        self.open = open;
        self
    }

    pub fn set_close(&mut self, close: R) -> &mut Self {
        self.close = close;
        self
    }
}

impl<'a, C, L, R, P, O, H> Ctor<'a, C, O, H> for Enclose<C, P, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_ctor_beg!("Enclose", ctx.beg());

        let _ = debug_ctor_stage!("Enclose", "open", ctx.try_mat(&self.open)?);
        let r = debug_ctor_stage!("Enclose", "pat", self.pat.construct(ctx.ctx(), func));
        let r = ctx.process_ret(r)?;
        let _ = debug_ctor_stage!("Enclose", "close", ctx.try_mat(&self.close)?);

        debug_ctor_reval!("Enclose", ctx.beg(), ctx.end(), true);
        Ok(r)
    }
}

impl<'a, C, L, R, P> Regex<C> for Enclose<C, P, L, R>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);

        debug_regex_beg!("Enclose", ctx.beg());

        let mut ret = debug_regex_stage!("Enclose", "open", ctx.try_mat(&self.open)?);

        ret.add_assign(debug_regex_stage!(
            "Enclose",
            "pat",
            ctx.try_mat(&self.pat)?
        ));
        ret.add_assign(debug_regex_stage!(
            "Enclose",
            "close",
            ctx.try_mat(&self.close)?
        ));
        debug_regex_reval!("Enclose", Ok(ret))
    }
}
