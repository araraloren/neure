use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub trait MapSingle<I, O> {
    fn map_to(&self, val: I) -> Result<O, Error>;
}

#[derive(Debug, Default, Copy)]
pub struct Map<C, P, F, O> {
    pat: P,
    mapper: F,
    marker: PhantomData<(C, O)>,
}

impl<C, P, F, O> Clone for Map<C, P, F, O>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            mapper: self.mapper.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F, O> Map<C, P, F, O> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            mapper: func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn mapper(&self) -> &F {
        &self.mapper
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn mapper_mut(&mut self) -> &mut F {
        &mut self.mapper
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_mapper(&mut self, func: F) -> &mut Self {
        self.mapper = func;
        self
    }

    pub fn map_to<H, V>(self, mapper: H) -> Map<C, P, H, O>
    where
        H: MapSingle<O, V>,
    {
        Map {
            pat: self.pat,
            mapper,
            marker: self.marker,
        }
    }
}

impl<'a, C, M, O, V, P, F> Ctor<'a, C, M, V> for Map<C, P, F, O>
where
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
    F: MapSingle<O, V>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.mapper.map_to(self.pat.constrct(ctx, func)?)
    }
}

impl<'a, C, P, F, O> Regex<C> for Map<C, P, F, O>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}

impl<I, O, F> MapSingle<I, O> for F
where
    F: Fn(I) -> Result<O, Error>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        (self)(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Single;

impl<I> MapSingle<I, I> for Single {
    fn map_to(&self, val: I) -> Result<I, Error> {
        Ok(val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Select0;

impl<I1, I2> MapSingle<(I1, I2), I1> for Select0 {
    fn map_to(&self, val: (I1, I2)) -> Result<I1, Error> {
        Ok(val.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Select1;

impl<I1, I2> MapSingle<(I1, I2), I2> for Select1 {
    fn map_to(&self, val: (I1, I2)) -> Result<I2, Error> {
        Ok(val.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectEq;

impl<I1, I2> MapSingle<(I1, I2), (I1, I2)> for SelectEq
where
    I1: PartialEq<I2>,
{
    fn map_to(&self, val: (I1, I2)) -> Result<(I1, I2), Error> {
        if val.0 == val.1 {
            Ok(val)
        } else {
            Err(Error::SelectEq)
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStr<T>(PhantomData<T>);

impl<T> FromStr<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I, O> MapSingle<I, O> for FromStr<O>
where
    O: std::str::FromStr,
    I: AsRef<str>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        let val: &str = val.as_ref();

        val.parse::<O>().map_err(|_| Error::FromStr)
    }
}
