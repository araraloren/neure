use std::ops::Deref;
use std::ops::DerefMut;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::Ctor;

#[derive(Debug, Clone)]
pub struct Vector<T>(Vec<T>);

impl<T> Vector<T> {
    pub fn new(val: Vec<T>) -> Self {
        Self(val)
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, C, T, M, O> Ctor<'a, C, M, O> for Vector<T>
where
    T: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("vector", beg, regex.constrct(g.ctx(), func));

            if ret.is_ok() {
                trace!("vector", beg -> g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

impl<'a, C, T> Regex<C> for Vector<T>
where
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = T::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("vector", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("vector", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

#[derive(Debug, Clone)]
pub struct PairVector<K, V>(Vec<(K, V)>);

impl<K, V> PairVector<K, V> {
    pub fn new(val: Vec<(K, V)>) -> Self {
        Self(val)
    }
}

impl<K, V> Deref for PairVector<K, V> {
    type Target = Vec<(K, V)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for PairVector<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, C, K, V, M> Ctor<'a, C, M, V> for PairVector<K, V>
where
    V: Clone,
    K: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, value) in self.0.iter() {
            let ret = trace!("pair_vec", beg, g.try_mat(regex));

            if ret.is_ok() {
                let _ = func.invoke(A::extract(g.ctx(), &ret?)?)?;

                trace!("pair_vec", beg -> g.end(), true);
                return Ok(value.clone());
            }
        }
        Err(Error::PairVec)
    }
}

impl<'a, C, K, V> Regex<C> for PairVector<K, V>
where
    K: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = K::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, _) in self.0.iter() {
            let ret = trace!("pair_vec", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("pair_vec", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::PairVec)
    }
}
