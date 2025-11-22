use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Match `P` and return the result wrapped by `Option`, ignoring the error.
///
/// # Ctor
///
/// If the regex `P` matches, return `Some(T)`; otherwise return None.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let num = neu::digit(10)
///         .repeat_one_more()
///         .map(map::from_str())
///         .opt();
///
///     assert_eq!(CharsCtx::new("8922").ctor(&num)?, Some(8922i32));
///     assert_eq!(CharsCtx::new("f122").ctor(&num)?, None);
///
///     Ok(())
/// # }
/// ```
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

impl<'a, C, M, O, P, H, A> Ctor<'a, C, M, Option<O>, H, A> for OptionPat<C, P>
where
    P: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<Option<O>, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("or", beg @ "left", self.pat.construct(g.ctx(), func));

        Ok(g.process_ret(ret).ok())
    }
}

impl<'a, C, P> Regex<C> for OptionPat<C, P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let beg = ctx.offset();
        let ret = ctx.try_mat(&self.pat);

        trace!("option", beg => ctx.offset(), Ok(ret.unwrap_or(<Span as Ret>::from_ctx(ctx, (0, 0)))))
    }
}
