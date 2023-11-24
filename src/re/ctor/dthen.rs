use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::ctor::Map;
use crate::re::map::Select0;
use crate::re::map::Select1;
use crate::re::map::SelectEq;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct DynamicCreateCtorThen<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

impl<C, P, F> Clone for DynamicCreateCtorThen<C, P, F>
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

impl<C, P, F> DynamicCreateCtorThen<C, P, F> {
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

impl<'a, C, P, F, T, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for DynamicCreateCtorThen<C, P, F>
where
    P: Ctor<'a, C, M, O1>,
    T: Ctor<'a, C, M, O2>,
    C: Context<'a> + Policy<C>,
    F: Fn(&O1) -> Result<T, Error>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.constrct(g.ctx(), func) {
            Ok(ret1) => {
                let ctor = (self.func)(&ret1)?;
                let ret = ctor.constrct(g.ctx(), func);
                let ret2 = g.process_ret(ret)?;

                Ok((ret1, ret2))
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, F> Regex<C> for DynamicCreateCtorThen<C, P, F>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unimplemented!("Can't not using DynamicThen for regex")
    }
}

pub trait DynamicCreateCtorThenHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn dyn_then_ctor<F>(self, func: F) -> DynamicCreateCtorThen<C, Self, F>;
}

impl<'a, C, T> DynamicCreateCtorThenHelper<'a, C> for T
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn dyn_then_ctor<F>(self, func: F) -> DynamicCreateCtorThen<C, Self, F> {
        DynamicCreateCtorThen::new(self, func)
    }
}
