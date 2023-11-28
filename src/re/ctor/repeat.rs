use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::CRange;
use crate::re::trace_v;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
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
pub struct Repeat<C, P> {
    pat: P,
    range: CRange<usize>,
    capacity: usize,
    marker: PhantomData<C>,
}

impl<C, P> Clone for Repeat<C, P>
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

impl<C, P> Repeat<C, P> {
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

impl<'a, C, P, M, O> Ctor<'a, C, M, Vec<O>> for Repeat<C, P>
where
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<Vec<O>, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut res = Vec::with_capacity(self.capacity);
        let mut ret = Err(Error::RegexRepeat);
        let beg = g.beg();

        trace_v!("repeat", self.range, beg, ());
        while self.is_contain(cnt) {
            let ret = self.pat.constrct(g.ctx(), handler);

            match ret {
                Ok(ret) => {
                    res.push(ret);
                    cnt += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            ret = Ok(res);
        }
        trace_v!("repeat", self.range, beg -> g.end(), ret.is_ok(), cnt);
        g.process_ret(ret)
    }
}

impl<'a, C, P> Regex<C> for Repeat<C, P>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = <Span as Ret>::from_ctx(g.ctx(), (0, 0));
        let mut ret = Err(Error::RegexRepeat);
        let beg = g.beg();

        trace_v!("repeat", self.range, beg, ());
        while self.is_contain(cnt) {
            match g.ctx().try_mat(&self.pat) {
                Ok(ret) => {
                    span.add_assign(ret);
                    cnt += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        if std::ops::RangeBounds::contains(&self.range, &cnt) {
            ret = Ok(span);
        }
        trace_v!("repeat", self.range, beg => g.end(), g.process_ret(ret), cnt)
    }
}
