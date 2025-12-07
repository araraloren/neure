use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::neu::CRange;
use crate::regex::def_not;
use crate::regex::Regex;

use super::Map;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;

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
/// #     color_eyre::install()?;
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

def_not!(SepOnce<C, L, S, R>);

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

impl<'a, C, L, S, R, M, O1, O2, H, A> Ctor<'a, C, M, (O1, O2), H, A> for SepOnce<C, L, S, R>
where
    L: Ctor<'a, C, M, O1, H, A>,
    R: Ctor<'a, C, M, O2, H, A>,
    S: Regex<C>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("SepOnce", g.beg());

        let r = self.left.construct(g.ctx(), func);
        let r = g.process_ret(r)?;
        let _ = g.try_mat(&self.sep)?;
        let l = self.right.construct(g.ctx(), func);
        let l = g.process_ret(l)?;

        debug_ctor_reval!("SepOnce", g.beg(), g.end(), true);
        Ok((r, l))
    }
}

impl<'a, C, L, S, R> Regex<C> for SepOnce<C, L, S, R>
where
    S: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut span = Span::new(g.ctx().offset(), 0);

        debug_regex_beg!("SepOnce", g.beg());
        span.add_assign(g.try_mat(&self.left)?);
        span.add_assign(g.try_mat(&self.sep)?);
        span.add_assign(g.try_mat(&self.right)?);
        debug_regex_reval!("SepOnce", Ok(span))
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
/// #     color_eyre::install()?;
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

def_not!(Separate<C, P, S>);

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

impl<'a, C, S, P, M, O, H, A> Ctor<'a, C, M, Vec<O>, H, A> for Separate<C, P, S>
where
    P: Ctor<'a, C, M, O, H, A>,
    S: Regex<C>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<Vec<O>, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut vals = Vec::with_capacity(self.capacity.max(self.min));
        let range: CRange<usize> = (self.min..).into();

        debug_ctor_beg!("Separate", range, ctx.beg());
        while let Ok(val) = self.pat.construct(ctx.ctx(), func) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                vals.push(val);
            }
            if res.is_err() {
                break;
            }
        }
        let len = vals.len();
        let ret = ctx.process_ret(if len >= self.min {
            Ok(vals)
        } else {
            Err(Error::Separate)
        });

        debug_ctor_reval!("Separate", range, ctx.beg(), ctx.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, S, P> Regex<C> for Separate<C, P, S>
where
    S: Regex<C>,
    P: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = Span::new(ctx.ctx().offset(), 0);
        let range: CRange<usize> = (self.min..).into();

        debug_regex_beg!("Separate", range, ctx.beg());
        while let Ok(ret) = ctx.ctx().try_mat(&self.pat) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = res {
                    span.add_assign(sep_ret);
                }
            }
            if res.is_err() {
                break;
            }
        }
        let ret = if cnt >= self.min {
            Ok(span)
        } else {
            Err(Error::Separate)
        };
        debug_regex_reval!("Separate", range, ctx.process_ret(ret))
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
///
/// # Ctor
///
/// It will return a `V` that can constructed from `P`'s match results
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
/// # use neure::{prelude::*, map::FromStr};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
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
pub struct SepCollect<C, P, S, O, V> {
    pat: P,
    sep: S,
    skip: bool,
    min: usize,
    marker: PhantomData<(C, O, V)>,
}

def_not!(SepCollect<C, P, S, O, V>);

impl<C, P, S, O, V> Debug for SepCollect<C, P, S, O, V>
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

impl<C, P, S, O, V> Clone for SepCollect<C, P, S, O, V>
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

impl<C, P, S, O, V> SepCollect<C, P, S, O, V> {
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

impl<'a, C, S, P, M, O, V, H, A> Ctor<'a, C, M, V, H, A> for SepCollect<C, P, S, O, V>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, M, O, H, A>,
    S: Regex<C>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<V, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut end = false;
        let range: CRange<usize> = (self.min..).into();
        let ret = {
            debug_ctor_beg!("SepCollect", range, ctx.beg());
            V::from_iter(std::iter::from_fn(|| {
                self.pat.construct(ctx.ctx(), func).ok().and_then(|ret| {
                    let res = ctx.ctx().try_mat(&self.sep);

                    if !end {
                        if res.is_err() {
                            end = true;
                        }
                        // The current value is captured only
                        // when the current delimiter matches successfully
                        // or the skip flag is true.
                        if res.is_ok() || self.skip {
                            cnt += 1;
                            return Some(ret);
                        }
                    }
                    None
                })
            }))
        };
        let ret = ctx.process_ret(if cnt >= self.min {
            Ok(ret)
        } else {
            Err(Error::SepCollect)
        });

        debug_ctor_reval!("SepCollect", range, ctx.beg(), ctx.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, S, P, O, V> Regex<C> for SepCollect<C, P, S, O, V>
where
    S: Regex<C>,
    P: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut ctx = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = Span::new(ctx.ctx().offset(), 0);
        let mut ret = Err(Error::SepCollect);
        let range: CRange<usize> = (self.min..).into();

        debug_regex_beg!("SepCollect", range, ctx.beg());
        while let Ok(ret) = ctx.ctx().try_mat(&self.pat) {
            let res = ctx.ctx().try_mat(&self.sep);

            if res.is_ok() || self.skip {
                cnt += 1;
                span.add_assign(ret);
                if let Ok(sep_ret) = res {
                    span.add_assign(sep_ret);
                }
            }
            if res.is_err() {
                break;
            }
        }
        if cnt >= self.min {
            ret = Ok(span);
        }
        debug_regex_reval!("SepCollect", range, ctx.process_ret(ret))
    }
}
