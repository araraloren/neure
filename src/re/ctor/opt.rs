use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
///
///
#[derive(Debug, Default, Copy)]
pub struct OptionPat<C, P> {
    pat: P,
    marker: PhantomData<C>,
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

impl<'a, C, M, O, P> Ctor<'a, C, M, Option<O>> for OptionPat<C, P>
where
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<Option<O>, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("or", beg @ "left", self.pat.constrct(g.ctx(), func));

        Ok(if let Ok(ret) = g.process_ret(ret) {
            Some(ret)
        } else {
            None
        })
    }
}

impl<'a, C, P> Regex<C> for OptionPat<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let beg = ctx.offset();
        let ret = ctx.try_mat(&self.pat);

        trace!("option", beg => ctx.offset(), Ok(ret.unwrap_or(<Span as Ret>::from_ctx(ctx, (0, 0)))))
    }
}
