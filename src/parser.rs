use std::fmt::Debug;
use std::marker::PhantomData;

use crate::err::Error;
use crate::policy::Context;
use crate::policy::Ret;
use crate::MatchPolicy;

pub trait Parser<T>
where
    Self: Sized,
{
    type Ret: Ret;

    fn try_parse(self, ctx: &mut T) -> Result<Self::Ret, Error>;

    fn parse(self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H, R> Parser<T> for H
where
    R: Ret,
    H: FnOnce(&mut T) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(self, ctx: &mut T) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

pub struct True<T>(PhantomData<T>);

impl<T> Debug for True<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("True").field(&self.0).finish()
    }
}

impl<T> Clone for True<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for True<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Parser<T> for True<T>
where
    T: Context + MatchPolicy,
{
    type Ret = T::Ret;

    fn try_parse(self, _: &mut T) -> Result<Self::Ret, Error> {
        Ok(<Self::Ret>::new_from((0, 0)))
    }
}

fn calc_len<T, C: Context>(offset: usize, ctx: &C, next: Option<(usize, T)>) -> usize {
    let next_offset = next.map(|v| v.0).unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

pub fn one<C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        let mut iter: C::Iter<'_> = ctx.peek()?;

        if let Some((offset, item)) = iter.next() {
            if re(&item) {
                Ok(<C::Ret>::new_from((1, calc_len(offset, ctx, iter.next()))))
            } else {
                Err(Error::Match)
            }
        } else {
            Err(Error::NeedOne)
        }
    }
}

pub fn zero_one<C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if re(&item) {
                    return Ok(<C::Ret>::new_from((1, calc_len(offset, ctx, iter.next()))));
                }
            }
        }
        Ok(<C::Ret>::new_from((0, 0)))
    }
}

pub fn zero_more<C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;

        if let Ok(mut iter) = ctx.peek() {
            for (offset, item) in iter.by_ref() {
                if !re(&item) {
                    end = Some((offset, item));
                    break;
                }
                cnt += 1;
                if beg.is_none() {
                    beg = Some(offset);
                }
            }
        }
        if let Some(start) = beg {
            Ok(<C::Ret>::new_from((cnt, calc_len(start, ctx, end))))
        } else {
            Ok(<C::Ret>::new_from((0, 0)))
        }
    }
}

pub fn one_more<C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut iter = ctx.peek()?;

        for (offset, item) in iter.by_ref() {
            if !re(&item) {
                end = Some((offset, item));
                break;
            }
            cnt += 1;
            if beg.is_none() {
                beg = Some(offset);
            }
        }
        if let Some(start) = beg {
            Ok(<C::Ret>::new_from((cnt, calc_len(start, ctx, end))))
        } else {
            Err(Error::NeedOneMore)
        }
    }
}

pub fn count<const M: usize, const N: usize, C>(
    re: impl Fn(&C::Item) -> bool,
) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    count_if::<M, N, C>(re, |_, _| true)
}

pub fn count_if<const M: usize, const N: usize, C>(
    re: impl Fn(&C::Item) -> bool,
    r#if: impl Fn(&C, &(usize, C::Item)) -> bool,
) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    debug_assert!(M <= N, "M must little than N");
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        if let Ok(mut iter) = iter {
            while cnt < N {
                if let Some(pair) = iter.next() {
                    if re(&pair.1) && r#if(ctx, &pair) {
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
            if cnt >= M {
                let end = end.or(iter.next());

                return Ok(<C::Ret>::new_from((
                    cnt,
                    beg.map(|v| calc_len(v, ctx, end)).unwrap_or(0),
                )));
            }
        }
        Err(Error::NeedMore)
    }
}

pub fn start<C>() -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    |dat: &mut C| {
        if dat.offset() == 0 {
            Ok(<C::Ret>::new_from((0, 0)))
        } else {
            Err(Error::NotStart)
        }
    }
}

pub fn end<C>() -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    |dat: &mut C| {
        if dat.len() != dat.offset() {
            Err(Error::NotEnd)
        } else {
            Ok(<C::Ret>::new_from((0, 0)))
        }
    }
}

pub fn string<C>(lit: &'static str) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<Orig = str> + MatchPolicy,
{
    move |dat: &mut C| {
        if !dat.orig()?.starts_with(lit) {
            Err(Error::String)
        } else {
            Ok(<C::Ret>::new_from((1, lit.len())))
        }
    }
}

pub fn bytes<C>(lit: &'static [u8]) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<Orig = [u8]> + MatchPolicy,
{
    move |dat: &mut C| {
        if !dat.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            Ok(<C::Ret>::new_from((1, lit.len())))
        }
    }
}

pub fn consume<C>(length: usize) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        if ctx.len() - ctx.offset() >= length {
            Ok(<C::Ret>::new_from((1, length)))
        } else {
            Err(Error::Consume)
        }
    }
}

// todo! add `and` `or` `quote` `terminated`