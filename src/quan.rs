use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::err::Error;

fn calc_len<'a, T, C: Context<'a>>(offset: usize, ctx: &C, next: Option<(usize, T)>) -> usize {
    let next_offset = next.map(|v| v.0).unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

pub fn one<'a, C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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

pub fn zero_one<'a, C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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

pub fn zero_more<'a, C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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

pub fn one_more<'a, C>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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

pub fn count<'a, const M: usize, const N: usize, C>(
    re: impl Fn(&C::Item) -> bool,
) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
{
    count_if::<'_, M, N, C>(re, |_, _| true)
}

pub fn count_if<'a, const M: usize, const N: usize, C>(
    re: impl Fn(&C::Item) -> bool,
    r#if: impl Fn(&C, &(usize, <C as Context<'_>>::Item)) -> bool,
) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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

pub fn start<'a, C>() -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
{
    |dat: &mut C| {
        if dat.offset() == 0 {
            Ok(<C::Ret>::new_from((0, 0)))
        } else {
            Err(Error::NotStart)
        }
    }
}

pub fn end<'a, C>() -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
{
    |dat: &mut C| {
        if dat.len() != dat.offset() {
            Err(Error::NotEnd)
        } else {
            Ok(<C::Ret>::new_from((0, 0)))
        }
    }
}

pub fn string<'a, C>(lit: &'static str) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a, Orig = str> + Policy<C> + 'a,
{
    move |dat: &mut C| {
        if !dat.orig()?.starts_with(lit) {
            Err(Error::String)
        } else {
            Ok(<C::Ret>::new_from((1, lit.len())))
        }
    }
}

pub fn bytes<'a, C>(lit: &'static [u8]) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a, Orig = [u8]> + Policy<C> + 'a,
{
    move |dat: &mut C| {
        if !dat.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            Ok(<C::Ret>::new_from((1, lit.len())))
        }
    }
}

pub fn consume<'a, C>(length: usize) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<'a> + Policy<C> + 'a,
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
