use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::neu::CRange;
use crate::re::trace_v;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct RegexCollect<C, P, O> {
    pat: P,
    min: usize,
    marker: PhantomData<(O, C)>,
}

impl<C, P, O> Clone for RegexCollect<C, P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, O> RegexCollect<C, P, O> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            min: 0,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, P, O> Regex<C> for RegexCollect<C, P, O>
where
    P: Regex<C>,
    O: FromIterator<P::Ret>,
    C: Context<'a> + Policy<C>,
{
    type Ret = O;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut ret = Err(Error::Collect);
        let beg = g.beg();
        let range: CRange<usize> = (self.min..).into();
        let val = trace_v!(
            "collect",
            range,
            beg,
            O::from_iter(std::iter::from_fn(|| match g.try_mat(&self.pat) {
                Ok(ret) => {
                    cnt += 1;
                    Some(ret)
                }
                Err(_) => None,
            }))
        );

        if cnt >= self.min {
            ret = Ok(val);
        }
        trace_v!("collect", range, beg => g.end(), ret.is_ok(), cnt);
        g.process_ret(ret)
    }
}
