use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Extract;
use crate::regex::Handler;
use crate::regex::Invoke;
use crate::regex::Regex;

use super::Unit;

#[derive(Debug, Clone, Default, Copy)]
pub struct RepeatIf<R, B, O, T, F>
where
    R: Unit<T>,
    B: RangeBounds<usize>,
{
    r#if: F,
    regex: R,
    range: B,
    marker: PhantomData<(O, T)>,
}

impl<R, B, O, T, F> RepeatIf<R, B, O, T, F>
where
    R: Unit<T>,
    B: RangeBounds<usize>,
{
    pub fn new(regex: R, range: B, r#if: F) -> Self {
        Self {
            r#if,
            regex,
            range,
            marker: PhantomData,
        }
    }

    pub fn is_contain(&self, count: usize) -> bool
    where
        B: RangeBounds<usize>,
    {
        match self.range.end_bound() {
            std::ops::Bound::Included(max) => count < *max,
            std::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, R, B, C, M, F> Invoke<'a, C, M, M> for RepeatIf<R, B, Span, C::Item, F>
where
    C: Context<'a> + 'a,
    R: Unit<C::Item>,
    B: RangeBounds<usize>,
    C: Context<'a> + Policy<C>,
    F: Fn(&C, &(usize, C::Item)) -> bool,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<M, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, R, B, C, F> Regex<C> for RepeatIf<R, B, Span, C::Item, F>
where
    C: Context<'a> + 'a,
    R: Unit<C::Item>,
    B: RangeBounds<usize>,
    F: Fn(&C, &(usize, C::Item)) -> bool,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        if let Ok(mut iter) = iter {
            while self.is_contain(cnt) {
                if let Some(pair) = iter.next() {
                    if self.regex.is_match(&pair.1) && (self.r#if)(ctx, &pair) {
                        cnt += 1;
                        if beg.is_none() {
                            beg = Some(pair.0);
                        }
                        continue;
                    } else {
                        end = Some(pair);
                    }
                }
                break;
            }
            if self.range.contains(&cnt) {
                let end = end.or_else(|| iter.next()).map(|v| v.0);
                let length = beg
                    .map(|beg| end.unwrap_or(ctx.len() - ctx.offset()) - beg)
                    .unwrap_or(0);

                let ret = <Self::Ret as Ret>::from(ctx, (cnt, length));

                ctx.inc(length);
                return Ok(ret);
            }
        }
        Err(crate::err::Error::NeedMore)
    }
}
