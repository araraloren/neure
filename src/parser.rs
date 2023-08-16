use crate::ctx::Context;
use crate::err::Error;
use crate::MatchPolicy;

pub trait Parser<T> {
    type Ret;

    fn try_parse(&mut self, ctx: &mut T) -> Result<Self::Ret, Error>;

    fn parse(&mut self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H, R> Parser<T> for H
where
    H: Fn(&mut T) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(&mut self, ctx: &mut T) -> Result<Self::Ret, Error> {
        (self)(ctx)
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
                Ok(<C::Ret>::from((1, calc_len(offset, ctx, iter.next()))))
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
                    return Ok(<C::Ret>::from((1, calc_len(offset, ctx, iter.next()))));
                }
            }
        }
        Ok(<C::Ret>::from((0, 0)))
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
            Ok(<C::Ret>::from((cnt, calc_len(start, ctx, end))))
        } else {
            Ok(<C::Ret>::from((0, 0)))
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
            Ok(<C::Ret>::from((cnt, calc_len(start, ctx, end))))
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

                return Ok(<C::Ret>::from((
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
            Ok(<C::Ret>::from((0, 0)))
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
            Ok(<C::Ret>::from((0, 0)))
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
            Ok(<C::Ret>::from((1, lit.len())))
        }
    }
}

pub fn bytes<C>(lit: &'static [u8]) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context<Orig = [u8]> + MatchPolicy,
{
    move |dat: &mut C| {
        println!("matcing. ..`{}`", std::str::from_utf8(lit).unwrap());
        println!("...`{}`", std::str::from_utf8(dat.orig_sub(dat.offset(), 200)?).unwrap());
        if !dat.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            Ok(<C::Ret>::from((1, lit.len())))
        }
    }
}

pub fn consume<C>(length: usize) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        if ctx.len() - ctx.offset() >= length {
            Ok(<C::Ret>::from((1, length)))
        } else {
            Err(Error::Consume)
        }
    }
}

pub fn seq<C>(
    parser1: impl Fn(&mut C) -> Result<C::Ret, Error>,
    parser2: impl Fn(&mut C) -> Result<C::Ret, Error>,
) -> impl Fn(&mut C) -> Result<C::Ret, Error>
where
    C: Context + MatchPolicy,
{
    move |ctx: &mut C| {
        let start = ctx.offset();
        let ret1 = parser1(ctx);

        if ret1.is_ok() {
            let ret2 = parser2(ctx);

            if ret2.is_ok() {
                Ok(<C::Ret>::from((1, ctx.offset() - start)))
            } else {
                ret2
            }
        } else {
            ret1
        }
    }
}
