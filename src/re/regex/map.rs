use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::err::Error;
use crate::re::map::MapSingle;
use crate::re::Regex;

/// Map the result to another type.
#[derive(Default, Copy)]
pub struct RegexMap<C, P, F, O> {
    pat: P,
    mapper: F,
    marker: PhantomData<(C, O)>,
}

impl<C, P, F, O> Debug for RegexMap<C, P, F, O>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegexMap")
            .field("pat", &self.pat)
            .field("mapper", &self.mapper)
            .finish()
    }
}

impl<C, P, F, O> Clone for RegexMap<C, P, F, O>
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

impl<C, P, F, O> RegexMap<C, P, F, O> {
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

    pub fn map_to<H, V>(self, mapper: H) -> RegexMap<C, P, H, O>
    where
        H: MapSingle<O, V>,
    {
        RegexMap {
            pat: self.pat,
            mapper,
            marker: self.marker,
        }
    }
}

impl<'a, C, P, F, O> Regex<C> for RegexMap<C, P, F, O>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
    F: MapSingle<P::Ret, O>,
{
    type Ret = O;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(&self.pat);

        self.mapper.map_to(g.process_ret(ret)?)
    }
}
