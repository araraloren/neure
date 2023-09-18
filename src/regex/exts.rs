use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::ext::Extract;
use crate::ext::Handler;
use crate::ext::Invoke;

use super::CopyRange;
use super::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    left: L,
    regex: R,
    marker: PhantomData<T>,
}

impl<L, R, T> Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            regex: right,
            marker: PhantomData,
        }
    }
}

impl<L, R, T> Regex<T> for Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) || self.regex.is_match(other)
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T> And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            marker: PhantomData,
        }
    }
}

impl<L, R, T> Regex<T> for And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) && self.right.is_match(other)
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Not<R, T>
where
    R: Regex<T>,
{
    regex: R,
    marker: PhantomData<T>,
}

impl<R, T> Not<R, T>
where
    R: Regex<T>,
{
    pub fn new(regex: R) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<R, T> Regex<T> for Not<R, T>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.regex.is_match(other)
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Repeat<R, B, O, T>
where
    R: Regex<T>,
    B: RangeBounds<usize>,
{
    regex: R,
    range: B,
    marker: PhantomData<(O, T)>,
}

impl<R, B, O, T> Repeat<R, B, O, T>
where
    R: Regex<T>,
    B: RangeBounds<usize>,
{
    pub fn new(regex: R, range: B) -> Self {
        Self {
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

impl<'a, R, B, C, M> Invoke<'a, C, M, M> for Repeat<R, B, Span, C::Item>
where
    C: Context<'a> + 'a,
    R: Regex<C::Item>,
    B: RangeBounds<usize>,
    C: Context<'a> + Policy<C>,
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

impl<'a, R, B, C> Parse<C> for Repeat<R, B, Span, C::Item>
where
    C: Context<'a> + 'a,
    R: Regex<C::Item>,
    B: RangeBounds<usize>,
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
                    if self.regex.is_match(&pair.1) {
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

pub trait RegexExt<T> {
    fn or<R>(self, regex: R) -> Or<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized;

    fn and<R>(self, regex: R) -> And<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized;

    fn not(self) -> Not<Self, T>
    where
        Self: Regex<T> + Sized;

    fn repeat<R>(self, range: R) -> Repeat<Self, CopyRange<usize>, Span, T>
    where
        Self: Regex<T> + Sized,
        R: Into<CopyRange<usize>>;
}

impl<T, Re> RegexExt<T> for Re
where
    Re: Regex<T>,
{
    fn or<R>(self, regex: R) -> Or<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized,
    {
        Or::new(self, regex)
    }

    fn and<R>(self, regex: R) -> And<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized,
    {
        And::new(self, regex)
    }

    fn not(self) -> Not<Self, T>
    where
        Self: Regex<T> + Sized,
    {
        Not::new(self)
    }

    fn repeat<R>(self, range: R) -> Repeat<Self, CopyRange<usize>, Span, T>
    where
        Self: Regex<T> + Sized,
        R: Into<CopyRange<usize>>,
    {
        Repeat::new(self, range.into())
    }
}
