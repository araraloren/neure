use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Call [`.try_mat`](crate::ctx::Match#tymethod.try_mat) to match regex `P`.
///
/// # Ctor
///
/// Return [`Orig`](crate::ctx::Context::Orig) with the [`Span`] as the index if the match is found.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let abc = "abc";
///     let com = "com";
///     let website = abc.sep_once(".", com);
///     let mut ctx = CharsCtx::new("abc.com");
///
///     assert_eq!(ctx.ctor(&website)?, ("abc", "com"));
///     let pat = website.pat();
///
///     assert_eq!(ctx.reset().ctor(&pat)?, "abc.com");
///
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Pattern<C, P> {
    pat: P,
    marker: PhantomData<C>,
}

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

impl<'a, C, O, P> Ctor<'a, C, O, O> for Pattern<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.pat)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, P> Regex<C> for Pattern<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
