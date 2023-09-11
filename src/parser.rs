use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::err::Error;
use crate::ext::CtxGuard;

fn length<'a, T, C: Context<'a>>(offset: usize, ctx: &C, next: Option<(usize, T)>) -> usize {
    let next_offset = next.map(|v| v.0).unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

fn make_ret_and_inc<'a, C: Context<'a>, R: Ret>(ctx: &mut C, count: usize, len: usize) -> R {
    let ret = R::from(ctx, (count, len));

    ctx.inc(len);
    ret
}

pub fn one<'a, C, R>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        let mut iter: C::Iter<'_> = ctx.peek()?;

        if let Some((offset, item)) = iter.next() {
            if re(&item) {
                Ok(make_ret_and_inc(ctx, 1, length(offset, ctx, iter.next())))
            } else {
                Err(Error::Match)
            }
        } else {
            Err(Error::NeedOne)
        }
    }
}

pub fn zero_one<'a, C, R>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if re(&item) {
                    return Ok(make_ret_and_inc(ctx, 1, length(offset, ctx, iter.next())));
                }
            }
        }
        Ok(R::from(ctx, (0, 0)))
    }
}

pub fn zero_more<'a, C, R>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
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
            Ok(make_ret_and_inc(ctx, cnt, length(start, ctx, end)))
        } else {
            Ok(R::from(ctx, (0, 0)))
        }
    }
}

pub fn one_more<'a, C, R>(re: impl Fn(&C::Item) -> bool) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
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
            Ok(make_ret_and_inc(ctx, cnt, length(start, ctx, end)))
        } else {
            Err(Error::NeedOneMore)
        }
    }
}

pub fn count<'a, const M: usize, const N: usize, C, R>(
    re: impl Fn(&C::Item) -> bool + 'a,
) -> impl Fn(&mut C) -> Result<R, Error> + 'a
where
    R: Ret + 'a,
    C: Context<'a> + 'a,
{
    count_if::<'a, M, N, C, R>(re, |_, _| true)
}

pub fn count_if<'a, const M: usize, const N: usize, C, R>(
    re: impl Fn(&C::Item) -> bool,
    r#if: impl Fn(&C, &(usize, <C as Context<'a>>::Item)) -> bool,
) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
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

                return Ok(make_ret_and_inc(
                    ctx,
                    cnt,
                    beg.map(|v| length(v, ctx, end)).unwrap_or(0),
                ));
            }
        }
        Err(Error::NeedMore)
    }
}

pub fn start<'a, C, R>() -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    |ctx: &mut C| {
        if ctx.offset() == 0 {
            Ok(R::from(ctx, (0, 0)))
        } else {
            Err(Error::NotStart)
        }
    }
}

pub fn end<'a, C, R>() -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    |ctx: &mut C| {
        if ctx.len() != ctx.offset() {
            Err(Error::NotEnd)
        } else {
            Ok(R::from(ctx, (0, 0)))
        }
    }
}

pub fn string<'a, C, R>(lit: &'static str) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a, Orig = str>,
{
    move |ctx: &mut C| {
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::String)
        } else {
            Ok(R::from(ctx, (1, lit.len())))
        }
    }
}

pub fn bytes<'a, C, R>(lit: &'static [u8]) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a, Orig = [u8]>,
{
    move |ctx: &mut C| {
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            Ok(R::from(ctx, (1, lit.len())))
        }
    }
}

pub fn consume<'a, C, R>(length: usize) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    move |ctx: &mut C| {
        if ctx.len() - ctx.offset() >= length {
            Ok(R::from(ctx, (1, length)))
        } else {
            Err(Error::Consume)
        }
    }
}

pub fn and<'a, C, R, P1, P2>(p1: P1, p2: P2) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + Policy<C>,
    P1: Pattern<C, Ret = R>,
    P2: Pattern<C, Ret = R>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&p1)?;

        ret.add_assign(g.try_mat(&p2)?);
        Ok(ret)
    }
}

pub fn or<'a, C, R, P1, P2>(p1: P1, p2: P2) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + Policy<C>,
    P1: Pattern<C, Ret = R>,
    P2: Pattern<C, Ret = R>,
{
    move |ctx: &mut C| p1.try_parse(ctx).or(p2.try_parse(ctx))
}

// pub fn quote<'a, C, L, R, P>(l: L, r: R, p: P) -> impl Fn(&mut C) -> Result<P::Ret, Error>
// where
//     C: Context<'a> + Policy<C>,
//     L: Pattern<C>,
//     R: Pattern<C>,
//     P: Pattern<C>,
// {
//     move |ctx: &mut C| {
//         let mut g = CtxGuard::new(ctx);

//         g.try_mat(&l)?;
//         let ret = g.try_mat(&p)?;

//         g.try_mat(&r)?;
//         Ok(ret)
//     }
// }

// pub fn terminated<'a, C, S, P>(sep: S, p: P) -> impl Fn(&mut C) -> Result<C::Ret, Error>
// where
//     C::Ret: Ret,
//     C: Context<'a> + Policy<C>,
//     S: Pattern<C, Ret = C::Ret> + Clone,
//     P: Pattern<C, Ret = C::Ret> + Clone,
// {
//     terminated_opt(sep, p, true)
// }

// pub fn terminated_opt<'a, C, S, P>(
//     sep: S,
//     p: P,
//     optional: bool,
// ) -> impl Fn(&mut C) -> Result<C::Ret, Error>
// where
//     C::Ret: Ret,
//     C: Context<'a> + Policy<C>,
//     S: Pattern<C, Ret = C::Ret>,
//     P: Pattern<C, Ret = C::Ret>,
// {
//     move |ctx: &mut C| {
//         let mut guard = CtxGuard::new(ctx);
//         let mut ret = <C::Ret>::new_from((0, 0));

//         while let Ok(p_ret) = guard.try_mat(&p) {
//             ret.add_assign(p_ret);
//             match guard.try_mat(&sep) {
//                 Ok(sep_ret) => {
//                     ret.add_assign(sep_ret);
//                 }
//                 Err(e) => {
//                     if optional {
//                         break;
//                     } else {
//                         return Err(e);
//                     }
//                 }
//             }
//         }
//         Ok(ret)
//     }
// }
