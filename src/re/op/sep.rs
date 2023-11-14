use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Match `L` and `R` separated by `S`.
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
pub struct SeparateOnce<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, S, R> Clone for SeparateOnce<C, L, S, R>
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

impl<C, L, S, R> SeparateOnce<C, L, S, R> {
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

impl<'a, C, L, S, R, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for SeparateOnce<C, L, S, R>
where
    L: Ctor<'a, C, M, O1>,
    R: Ctor<'a, C, M, O2>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let ret1 = self.left.constrct(g.ctx(), func);
        let ret1 = g.process_ret(ret1)?;

        g.try_mat(&self.sep)?;

        let ret2 = self.right.constrct(g.ctx(), func);
        let ret2 = g.process_ret(ret2)?;

        Ok((ret1, ret2))
    }
}

impl<'a, C, L, S, R> Regex<C> for SeparateOnce<C, L, S, R>
where
    S: Regex<C, Ret = Span>,
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut span = <Span as Ret>::from(ctx, (0, 0));
        let mut g = CtxGuard::new(ctx);

        span.add_assign(g.try_mat(&self.left)?);
        span.add_assign(g.try_mat(&self.sep)?);
        span.add_assign(g.try_mat(&self.right)?);

        Ok(span)
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
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
pub struct Separate<C, P, S> {
    pat: P,
    sep: S,
    skip: bool,
    capacity: usize,
    marker: PhantomData<C>,
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

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}

impl<'a, C, S, P, M, O> Ctor<'a, C, M, Vec<O>> for Separate<C, P, S>
where
    P: Ctor<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<Vec<O>, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut res = Vec::with_capacity(self.capacity);

        while let Ok(ret) = self.pat.constrct(ctx, func) {
            let sep_ret = ctx.try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                res.push(ret);
            } else {
                break;
            }
        }
        Ok(res)
    }
}

impl<'a, C, S, P> Regex<C> for Separate<C, P, S>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut span = <Span as Ret>::from(ctx, (0, 0));

        while let Ok(ret) = ctx.try_mat(&self.pat) {
            let sep_ret = ctx.try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                span.add_assign(ret);
                if let Ok(sep_ret) = sep_ret {
                    span.add_assign(sep_ret);
                }
            } else {
                break;
            }
        }
        Ok(span)
    }
}

///
/// Match regex `P` as many times as possible, with S as the delimiter.
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
pub struct SeparateCollect<C, P, S, O, T> {
    pat: P,
    sep: S,
    skip: bool,
    marker: PhantomData<(C, O, T)>,
}

impl<C, P, S, O, T> Clone for SeparateCollect<C, P, S, O, T>
where
    P: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            sep: self.sep.clone(),
            skip: self.skip,
            marker: self.marker,
        }
    }
}

impl<C, P, S, O, T> SeparateCollect<C, P, S, O, T> {
    pub fn new(pat: P, sep: S) -> Self {
        Self {
            pat,
            sep,
            skip: true,
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

    pub fn with_skip(mut self, skip: bool) -> Self {
        self.skip = skip;
        self
    }
}

impl<'a, C, S, P, M, O, T> Ctor<'a, C, M, T> for SeparateCollect<C, P, S, O, T>
where
    T: FromIterator<O>,
    P: Ctor<'a, C, M, O>,
    S: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<T, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ok(T::from_iter(std::iter::from_fn(|| {
            self.pat.constrct(ctx, func).ok().and_then(|ret| {
                let sep_ret = ctx.try_mat(&self.sep);

                if sep_ret.is_ok() || self.skip {
                    Some(ret)
                } else {
                    None
                }
            })
        })))
    }
}

impl<'a, C, S, P, O, T> Regex<C> for SeparateCollect<C, P, S, O, T>
where
    S: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut span = <Span as Ret>::from(ctx, (0, 0));

        while let Ok(ret) = ctx.try_mat(&self.pat) {
            let sep_ret = ctx.try_mat(&self.sep);

            if sep_ret.is_ok() || self.skip {
                span.add_assign(ret);
                if let Ok(sep_ret) = sep_ret {
                    span.add_assign(sep_ret);
                }
            } else {
                break;
            }
        }
        Ok(span)
    }
}
