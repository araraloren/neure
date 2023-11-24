use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::re::Regex;

///
/// Match `L` and `R` separated by `S`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
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
#[derive(Debug, Default, Copy)]
pub struct RegexSeparateOnce<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, S, R> Clone for RegexSeparateOnce<C, L, S, R>
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

impl<C, L, S, R> RegexSeparateOnce<C, L, S, R> {
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
}

impl<'a, C, L, S, R> Regex<C> for RegexSeparateOnce<C, L, S, R>
where
    S: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = (L::Ret, R::Ret);

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let ret1 = g.try_mat(&self.left)?;
        let _ = g.try_mat(&self.sep)?;
        let ret2 = g.try_mat(&self.right)?;

        Ok((ret1, ret2))
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
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
///     #[derive(Debug, PartialEq, PartialOrd)]
///     pub struct Tp<'a>(&'a str);
///
///     let ascii = neu::alphabetic().repeat_one_more();
///     let ty = ascii.map(|v| Ok(Tp(v)));
///     let ele = ty.sep(",".pad(' '));
///     let arr = ele.quote("<", ">");
///     let mut ctx = CharsCtx::new("<A, B, Len, Size>");
///
///     assert_eq!(ctx.ctor(&arr)?, [Tp("A"), Tp("B"), Tp("Len"), Tp("Size")]);
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct RegexSeparate<C, P, S> {
    pat: P,
    sep: S,
    skip: bool,
    capacity: usize,
    min: usize,
    marker: PhantomData<C>,
}

impl<C, P, S> Clone for RegexSeparate<C, P, S>
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

impl<C, P, S> RegexSeparate<C, P, S> {
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

impl<'a, C, S, P> Regex<C> for RegexSeparate<C, P, S>
where
    S: Regex<C>,
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Vec<P::Ret>;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut res = Vec::with_capacity(self.capacity.max(self.min));

        while let Ok(ret) = g.try_mat(&self.pat) {
            let sep_ret = g.try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                res.push(ret);
            } else {
                break;
            }
        }
        g.process_ret(if res.len() >= self.min {
            Ok(res)
        } else {
            Err(Error::Separate)
        })
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return a collection that can constructed from `P`'s match results
/// using [`from_iter`](std::iter::FromIterator::from_iter).
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, re::FromStr};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
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
#[derive(Debug, Default, Copy)]
pub struct RegexSeparateCollect<C, P, S, T> {
    pat: P,
    sep: S,
    skip: bool,
    min: usize,
    marker: PhantomData<(C, T)>,
}

impl<C, P, S, T> Clone for RegexSeparateCollect<C, P, S, T>
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

impl<C, P, S, T> RegexSeparateCollect<C, P, S, T> {
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

impl<'a, C, S, P, T> Regex<C> for RegexSeparateCollect<C, P, S, T>
where
    S: Regex<C>,
    P: Regex<C>,
    T: FromIterator<P::Ret>,
    C: Context<'a> + Policy<C>,
{
    type Ret = T;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let ret = T::from_iter(std::iter::from_fn(|| {
            g.try_mat(&self.pat).ok().and_then(|ret| {
                let sep_ret = g.try_mat(&self.sep);

                if sep_ret.is_ok() || self.skip {
                    cnt += 1;
                    Some(ret)
                } else {
                    None
                }
            })
        }));

        g.process_ret(if cnt >= self.min {
            Ok(ret)
        } else {
            Err(Error::SeparateCollect)
        })
    }
}
