use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::neu::CRange;
use crate::re::trace_v;
use crate::re::Regex;

///
/// Match `P` repeatedly.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return a collection of `P`'s match results.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let char = neu::any().repeat_one();
///     let num = char.ws().repeat(1..);
///     let mut ctx = CharsCtx::new(r#"你好，世界？"#);
///
///     assert_eq!(ctx.ctor(&num)?, ["你", "好", "，", "世", "界", "？"]);
///     Ok(())
/// # }
/// ```
#[derive(Debug, Copy)]
pub struct RegexRepeat<C, P> {
    pat: P,
    range: CRange<usize>,
    capacity: usize,
    marker: PhantomData<C>,
}

impl<C, P> Clone for RegexRepeat<C, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            range: self.range,
            capacity: self.capacity,
            marker: self.marker,
        }
    }
}

impl<C, P> RegexRepeat<C, P> {
    pub fn new(pat: P, range: impl Into<CRange<usize>>) -> Self {
        let range = range.into();
        let capacity = Self::guess_capacity(&range, 0);

        Self {
            pat,
            range,
            capacity,
            marker: PhantomData,
        }
    }

    pub fn guess_capacity(range: &CRange<usize>, val: usize) -> usize {
        let start = match range.start_bound() {
            std::ops::Bound::Included(v) => *v,
            std::ops::Bound::Excluded(v) => *v,
            std::ops::Bound::Unbounded => val,
        };
        start.max(val)
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn range(&self) -> &CRange<usize> {
        &self.range
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_range(&mut self, range: impl Into<CRange<usize>>) -> &mut Self {
        self.range = range.into();
        self
    }

    pub fn set_capacity(&mut self, cap: usize) -> &mut Self {
        self.capacity = cap;
        self
    }

    pub fn with_pat(mut self, pat: P) -> Self {
        self.pat = pat;
        self
    }

    pub fn with_range(mut self, range: impl Into<CRange<usize>>) -> Self {
        self.range = range.into();
        self
    }

    pub fn with_capacity(mut self, cap: usize) -> Self {
        self.capacity = cap;
        self
    }

    fn is_contain(&self, count: usize) -> bool {
        match std::ops::RangeBounds::end_bound(&self.range) {
            std::ops::Bound::Included(max) => count < *max,
            std::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, C, P> Regex<C> for RegexRepeat<C, P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Vec<P::Ret>;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut all_rets = Vec::with_capacity(self.capacity);
        let mut cnt = 0;
        let mut ret = Err(Error::RegexRepeat);
        let beg = g.beg();

        trace_v!("regex_repeat", self.range, beg, ());
        while self.is_contain(cnt) {
            match g.try_mat(&self.pat) {
                Ok(ret) => {
                    all_rets.push(ret);
                    cnt += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            ret = Ok(all_rets);
        }
        trace_v!("regex_repeat", self.range, beg => g.end(), ret.is_ok(), cnt);
        g.process_ret(ret)
    }
}
