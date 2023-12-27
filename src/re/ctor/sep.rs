use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::CRange;
use crate::re::map::Select0;
use crate::re::map::Select1;
use crate::re::map::SelectEq;
use crate::re::trace;
use crate::re::trace_v;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::Map;

///
/// Match `L` and `R` separated by `S`.
///
/// # Ctor
///
/// It will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let ele = neu::digit(10).repeat_times::<2>();
///     let sep = ":";
///     let time = ele.sep_once(sep, ele).sep_once(sep, ele);
///     let time = time.map(|((h, m), s)| Ok((h, m, s)));
///     let mut ctx = CharsCtx::new("20:31:42");
///
///     assert_eq!(ctx.ctor(&time)?, ("20", "31", "42"));
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct SepOnce<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, S, R> Debug for SepOnce<C, L, S, R>
where
    L: Debug,
    S: Debug,
    R: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SepOnce")
            .field("left", &self.left)
            .field("sep", &self.sep)
            .field("right", &self.right)
            .finish()
    }
}

impl<C, L, S, R> Clone for SepOnce<C, L, S, R>
where
    L: Clone,
    S: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            sep: self.sep.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, S, R> SepOnce<C, L, S, R> {
    pub fn new(left: L, sep: S, right: R) -> Self {
        Self {
            left,
            sep,
            right,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
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

impl<'a, C, L, S, R, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for SepOnce<C, L, S, R>
where
    L: Ctor<'a, C, M, O1>,
    R: Ctor<'a, C, M, O2>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let r = trace!("sep_once", beg @ "left", self.left.constrct(g.ctx(), func));
        let r = g.process_ret(r)?;
        let _ = trace!("sep_once", beg @ "sep",  g.try_mat(&self.sep)?);
        let l = trace!("sep_once", beg @ "right", self.right.constrct(g.ctx(), func));
        let l = g.process_ret(l)?;

        trace!("sep_once", beg => g.end(), true);
        Ok((r, l))
    }
}

impl<'a, C, L, S, R> Regex<C> for SepOnce<C, L, S, R>
where
    S: Regex<C, Ret = Span>,
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = <Span as Ret>::from_ctx(g.ctx(), (0, 0));
        let beg = g.beg();

        span.add_assign(trace!("sep_once", beg @ "left", g.try_mat(&self.left)?));
        span.add_assign(trace!("sep_once", beg @ "sep", g.try_mat(&self.sep)?));
        span.add_assign(trace!("sep_once", beg @ "right", g.try_mat(&self.right)?));

        trace!("sep_once", beg => g.end(), Ok(span))
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
///
/// # Ctor
///
/// It will return a [`Vec`] of `P`'s match results.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     #[derive(Debug, PartialEq, PartialOrd)]
///     pub struct Tp<'a>(&'a str);
///
///     let ascii = neu::alphabetic().repeat_one_more();
///     let ty = ascii.map(|v| Ok(Tp(v)));
///     let ele = ty.sep(",".ws());
///     let arr = ele.quote("<", ">");
///     let mut ctx = CharsCtx::new("<A, B, Len, Size>");
///
///     assert_eq!(ctx.ctor(&arr)?, [Tp("A"), Tp("B"), Tp("Len"), Tp("Size")]);
///
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Separate<C, P, S> {
    pat: P,
    sep: S,
    skip: bool,
    capacity: usize,
    min: usize,
    marker: PhantomData<C>,
}

impl<C, P, S> Debug for Separate<C, P, S>
where
    P: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Separate")
            .field("pat", &self.pat)
            .field("sep", &self.sep)
            .field("skip", &self.skip)
            .field("capacity", &self.capacity)
            .field("min", &self.min)
            .finish()
    }
}

impl<C, P, S> Clone for Separate<C, P, S>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            capacity: self.capacity,
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, S> Separate<C, P, S> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
            capacity: 0,
            min: 1,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn set_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, S, P, M, O> Ctor<'a, C, M, Vec<O>> for Separate<C, P, S>
where
    P: Ctor<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<Vec<O>, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let mut res = Vec::with_capacity(self.capacity.max(self.min));
        let beg = g.beg();
        let range: CRange<usize> = (self.min..).into();

        trace_v!("separate", range, beg, ());
        while let Ok(ret) = self.pat.constrct(g.ctx(), func) {
            let sep_ret = trace_v!("separate", range, beg @ "sep", g.ctx().try_mat(&self.sep));

            if sep_ret.is_ok() || self.skip {
                res.push(ret);
            }
            if sep_ret.is_err() {
                break;
            }
        }
        let len = res.len();
        let ret = g.process_ret(if len >= self.min {
            Ok(res)
        } else {
            Err(Error::Separate)
        });

        trace_v!("separate", range, beg -> g.end(), ret.is_ok(), len);
        ret
    }
}

impl<'a, C, S, P> Regex<C> for Separate<C, P, S>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = <Span as Ret>::from_ctx(g.ctx(), (0, 0));
        let mut ret = Err(Error::Separate);
        let beg = g.beg();
        let range: CRange<usize> = (self.min..).into();

        trace_v!("separate", range, beg, ());
        while let Ok(ret) = g.ctx().try_mat(&self.pat) {
            let sep_ret = g.ctx().try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = sep_ret {
                    span.add_assign(sep_ret);
                }
            }
            if sep_ret.is_err() {
                break;
            }
        }
        if cnt >= self.min {
            ret = Ok(span);
        }
        trace_v!("separate", range, beg => g.end(), g.process_ret(ret), cnt )
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
///
/// # Ctor
///
/// It will return a `T` that can constructed from `P`'s match results
/// using [`from_iter`](std::iter::FromIterator::from_iter).
///
/// # Notice
///
/// `SepCollect` will always succeed if the minimum size is 0, be careful to use it with other `.sep` faimly APIs.
/// The default size is 1.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, re::map::FromStr};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let digit = neu::digit(10).repeat_one_more();
///     let val = digit.map(FromStr::<i64>::new());
///     let vals = val.sep_collect::<_, _, Vec<i64>>(",".ws());
///     let array = vals.quote("[", "]");
///     let mut ctx = CharsCtx::new("[18, 24, 42, 58, 69]");
///
///     assert_eq!(ctx.ctor(&array)?, [18, 24, 42, 58, 69]);
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct SepCollect<C, P, S, O, T> {
    pat: P,
    sep: S,
    skip: bool,
    min: usize,
    marker: PhantomData<(C, O, T)>,
}

impl<C, P, S, O, T> Debug for SepCollect<C, P, S, O, T>
where
    P: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SepCollect")
            .field("pat", &self.pat)
            .field("sep", &self.sep)
            .field("skip", &self.skip)
            .field("min", &self.min)
            .finish()
    }
}

impl<C, P, S, O, T> Clone for SepCollect<C, P, S, O, T>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, S, O, T> SepCollect<C, P, S, O, T> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
            min: 1,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn sep(&self) -> &S {
        &self.sep
    }

    pub fn sep_mut(&mut self) -> &mut S {
        &mut self.sep
    }

    pub fn skip(&self) -> bool {
        self.skip
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_sep(&mut self, sep: S) -> &mut Self {
        self.sep = sep;
        self
    }

    pub fn set_skip(&mut self, skip: bool) -> &mut Self {
        self.skip = skip;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, S, P, M, O, T> Ctor<'a, C, M, T> for SepCollect<C, P, S, O, T>
where
    T: FromIterator<O>,
    P: Ctor<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<T, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut end = false;
        let beg = g.beg();
        let range: CRange<usize> = (self.min..).into();
        let ret = {
            trace_v!("sep_collect", range, beg, ());
            T::from_iter(std::iter::from_fn(|| {
                self.pat.constrct(g.ctx(), func).ok().and_then(|ret| {
                    let sep_ret =
                        trace_v!("sep_collect", range, beg @ "sep", g.ctx().try_mat(&self.sep));

                    if !end {
                        if sep_ret.is_err() {
                            end = true;
                        }
                        if sep_ret.is_ok() || self.skip {
                            cnt += 1;
                            return Some(ret);
                        }
                    }
                    None
                })
            }))
        };
        let ret = g.process_ret(if cnt >= self.min {
            Ok(ret)
        } else {
            Err(Error::SepCollect)
        });

        trace_v!("sep_collect", range, beg -> g.end(), ret.is_ok(), cnt);
        ret
    }
}

impl<'a, C, S, P, O, T> Regex<C> for SepCollect<C, P, S, O, T>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = <Span as Ret>::from_ctx(g.ctx(), (0, 0));
        let mut ret = Err(Error::SepCollect);
        let beg = g.beg();
        let range: CRange<usize> = (self.min..).into();

        trace_v!("sep_collect", range, beg, ());
        while let Ok(ret) = g.ctx().try_mat(&self.pat) {
            let sep_ret = g.ctx().try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = sep_ret {
                    span.add_assign(sep_ret);
                }
            }
            if sep_ret.is_err() {
                break;
            }
        }
        if cnt >= self.min {
            ret = Ok(span);
        }
        trace_v!("sep_collect", range, beg => g.end(), g.process_ret(ret), cnt)
    }
}
