use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::Neu;
use crate::neu::NeureOneMore;
use crate::neu::NullCond;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Copy)]
pub struct PadUnit<C, P, U: Neu<T>, T> {
    pat: P,
    unit: NeureOneMore<C, U, T, NullCond>,
    marker: PhantomData<C>,
}

impl<C, P, U, T> Clone for PadUnit<C, P, U, T>
where
    P: Clone,
    U: Clone + Neu<T>,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            unit: self.unit.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, U, T> PadUnit<C, P, U, T>
where
    U: Neu<T>,
{
    pub fn new(pat: P, unit: NeureOneMore<C, U, T, NullCond>) -> Self {
        Self {
            pat,
            unit,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn unit(&self) -> &NeureOneMore<C, U, T, NullCond> {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut NeureOneMore<C, U, T, NullCond> {
        &mut self.unit
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_unit(&mut self, unit: NeureOneMore<C, U, T, NullCond>) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<'a, C, P, U, T, M, O> Ctor<'a, C, M, O> for PadUnit<C, P, U, T>
where
    U: Neu<T>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a, Item = T> + Policy<C> + 'a,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.constrct(g.ctx(), func) {
            Ok(ret1) => {
                let _ = g.try_mat(&self.unit);
                Ok(ret1)
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, U, T> Regex<C> for PadUnit<C, P, U, T>
where
    U: Neu<T>,
    P: Regex<C, Ret = Span>,
    C: Context<'a, Item = T> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.pat)?;

        if let Ok(ws_ret) = g.try_mat(&self.unit) {
            ret.add_assign(ws_ret);
        }
        Ok(ret)
    }
}

#[derive(Debug, Copy)]
pub struct PaddedUnit<C, P, U: Neu<T>, T> {
    pat: P,
    unit: NeureOneMore<C, U, T, NullCond>,
    marker: PhantomData<C>,
}

impl<C, P, U, T> Clone for PaddedUnit<C, P, U, T>
where
    P: Clone,
    U: Clone + Neu<T>,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            unit: self.unit.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, U, T> PaddedUnit<C, P, U, T>
where
    U: Neu<T>,
{
    pub fn new(pat: P, unit: NeureOneMore<C, U, T, NullCond>) -> Self {
        Self {
            pat,
            unit,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn unit(&self) -> &NeureOneMore<C, U, T, NullCond> {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut NeureOneMore<C, U, T, NullCond> {
        &mut self.unit
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_unit(&mut self, unit: NeureOneMore<C, U, T, NullCond>) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<'a, C, P, U, T, M, O> Ctor<'a, C, M, O> for PaddedUnit<C, P, U, T>
where
    U: Neu<T>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a, Item = T> + Policy<C> + 'a,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let _ = g.try_mat(&self.unit);
        let ret = self.pat.constrct(g.ctx(), func);

        g.process_ret(ret)
    }
}

impl<'a, C, P, U, T> Regex<C> for PaddedUnit<C, P, U, T>
where
    U: Neu<T>,
    P: Regex<C, Ret = Span>,
    C: Context<'a, Item = T> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = match g.try_mat(&self.unit) {
            Ok(ret) => ret,
            Err(_) => Span::default(),
        };

        ret.add_assign(g.try_mat(&self.pat)?);
        Ok(ret)
    }
}
