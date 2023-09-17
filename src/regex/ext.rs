use std::{marker::PhantomData, ops::RangeBounds};

use super::Regex;

use crate::ctx::Context;
use crate::ctx::Parse;
use crate::ctx::Ret;

pub trait RegexExtension<T> {
    fn or<R>(self, regex: R) -> Or<Self, R>
    where
        Self: Sized;

    fn and<R>(self, regex: R) -> And<Self, R>
    where
        Self: Sized;

    fn not(self) -> Not<Self>
    where
        Self: Sized;
}

impl<T, Regex> RegexExtension<T> for Regex
where
    Regex: super::Regex<T>,
{
    fn or<R>(self, regex: R) -> Or<Self, R>
    where
        Self: Sized,
    {
        Or::new(self, regex)
    }

    fn and<R>(self, regex: R) -> And<Self, R>
    where
        Self: Sized,
    {
        And::new(self, regex)
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not::new(self)
    }
}

pub struct Or<L, R> {
    left: L,
    regex: R,
}

impl<L, R> Or<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, regex: right }
    }
}

impl<L, R, T> Regex<T> for Or<L, R>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) || self.regex.is_match(other)
    }
}

pub struct And<L, R> {
    left: L,
    right: R,
}

impl<L, R> And<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R, T> Regex<T> for And<L, R>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) && self.right.is_match(other)
    }
}

pub struct Not<R> {
    regex: R,
}

impl<R> Not<R> {
    pub fn new(regex: R) -> Self {
        Self { regex }
    }
}

impl<R, T> Regex<T> for Not<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.regex.is_match(other)
    }
}

pub struct Repeat<R, B, O> {
    regex: R,
    range: B,
    marker: PhantomData<O>,
}

impl<R, B, O> Repeat<R, B, O> {
    pub fn new(regex: R, range: B) -> Self {
        Self {
            regex,
            range,
            marker: PhantomData,
        }
    }

    pub fn check_range(&self, count: usize) -> bool
    where
        B: RangeBounds<usize>,
    {
        match self.range.end_bound() {
            std::ops::Bound::Included(max) => count <= *max,
            std::ops::Bound::Excluded(max) => count < *max,
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, R, B, C, O> Parse<C> for Repeat<R, B, O>
where
    O: Ret,
    C: Context<'a> + 'a,
    R: Regex<C::Item>,
    B: RangeBounds<usize>,
{
    type Ret = O;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        if let Ok(mut iter) = iter {
            while self.check_range(cnt) {
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

                let ret = O::from(ctx, (cnt, length));

                ctx.inc(length);
                return Ok(ret);
            }
        }
        Err(crate::err::Error::NeedMore)
    }
}
