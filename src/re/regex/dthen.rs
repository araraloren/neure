use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::ctor::Map;
use crate::re::map::Select0;
use crate::re::map::Select1;
use crate::re::map::SelectEq;
use crate::re::trace;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct DynamicCreateRegexThen<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

impl<C, P, F> Clone for DynamicCreateRegexThen<C, P, F>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            func: self.func.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F> DynamicCreateRegexThen<C, P, F> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn func(&self) -> &F {
        &self.func
    }

    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_func(&mut self, func: F) -> &mut Self {
        self.func = func;
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

impl<'a, C, P, F, T> Regex<C> for DynamicCreateRegexThen<C, P, F>
where
    P: Regex<C, Ret = Span>,
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
    F: Fn(&Span) -> Result<T, Error>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("dynamic_create_regex_then", beg @ "pat", g.try_mat(&self.pat)?);

        ret.add_assign(
            trace!("dynamic_create_regex_then", beg @ "dynamic regex", g.try_mat(&(self.func)(&ret)?)?)
        );
        trace!("dynamic_create_regex_then", beg => g.end(), Ok(ret))
    }
}

pub trait DynamicCreateRegexThenHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn dyn_then_re<F>(self, func: F) -> DynamicCreateRegexThen<C, Self, F>;
}

impl<'a, C, T> DynamicCreateRegexThenHelper<'a, C> for T
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn dyn_then_re<F>(self, func: F) -> DynamicCreateRegexThen<C, Self, F> {
        DynamicCreateRegexThen::new(self, func)
    }
}
