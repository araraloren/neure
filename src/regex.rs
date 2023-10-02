mod dyna;
mod extract;
mod guard;
mod invoke;
mod op_collect;
mod op_if;
mod op_map;
mod op_or;
mod op_ormap;
mod op_pat;
mod op_quote;
mod op_repeat;
mod op_term;
mod op_then;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::dyna::IntoDynamic;
pub use self::dyna::IntoNonDynamic;
pub use self::extract::*;
pub use self::guard::CtxGuard;
pub use self::invoke::*;
pub use self::op_collect::Collect;
pub use self::op_if::IfRegex;
pub use self::op_map::Map;
pub use self::op_or::Or;
pub use self::op_ormap::OrMap;
pub use self::op_pat::Pattern;
pub use self::op_quote::Quote;
pub use self::op_repeat::Repeat;
pub use self::op_repeat::TryRepeat;
pub use self::op_term::Terminated;
pub use self::op_then::Then;

pub use self::op_if::branch;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::neure::CRange;
use crate::neure::Neure;
use crate::trace_log;

pub trait Regex<C> {
    type Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error>;

    fn parse(&self, ctx: &mut C) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<C, F, R> Regex<C> for F
where
    F: Fn(&mut C) -> Result<R, Error>,
{
    type Ret = R;

    #[inline]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

impl<'a, 'b, C> Regex<C> for &'b str
where
    C: Context<'a, Orig = str> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::regex::string(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, C> Regex<C> for &'b [u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::regex::bytes(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, Ret, C> Regex<C> for Box<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&*self.borrow())
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.get())
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret = self.lock().expect("Oops ?! Can not unwrap mutex ...");
        ctx.try_mat(&*ret)
    }
}

impl<'a, P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Arc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Rc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(self.as_ref())
    }
}

pub trait RegexOp<'a, C>
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn pattern(self) -> Pattern<C, Self>;

    fn map<F, O>(self, f: F) -> Map<C, Self, F, O>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R>;

    fn terminated<S>(self, sep: S) -> Terminated<C, Self, S>;

    fn or<P>(self, pat: P) -> Or<C, Self, P>;

    fn or_map<P, F, O>(self, pat: P, func: F) -> OrMap<C, Self, P, F, O>;

    fn then<T>(self, then: T) -> Then<C, Self, T>;

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self>;

    fn try_repeat(self, range: impl Into<CRange<usize>>) -> TryRepeat<C, Self>;

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>;
}

impl<'a, C, T> RegexOp<'a, C> for T
where
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    fn pattern(self) -> Pattern<C, Self> {
        Pattern::new(self)
    }

    fn map<F, O>(self, func: F) -> Map<C, Self, F, O> {
        Map::new(self, func)
    }

    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R> {
        Quote::new(self, left, right)
    }

    fn terminated<S>(self, sep: S) -> Terminated<C, Self, S> {
        Terminated::new(self, sep)
    }

    fn or<P>(self, pat: P) -> Or<C, Self, P> {
        Or::new(self, pat)
    }

    fn or_map<P, F, O>(self, pat: P, func: F) -> OrMap<C, Self, P, F, O> {
        OrMap::new(self, pat, func)
    }

    fn then<P>(self, then: P) -> Then<C, Self, P> {
        Then::new(self, then)
    }

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self> {
        Repeat::new(self, range)
    }

    fn try_repeat(self, range: impl Into<CRange<usize>>) -> TryRepeat<C, Self> {
        TryRepeat::new(self, range)
    }

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>,
    {
        IfRegex::new(self, r#if, r#else)
    }
}

pub fn and<'a, C, O, P1, P2>(p1: P1, p2: P2) -> impl Fn(&mut C) -> Result<O, Error>
where
    O: Ret,
    P1: Regex<C, Ret = O>,
    P2: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&p1)?;

        ret.add_assign(g.try_mat(&p2)?);
        Ok(ret)
    }
}

pub fn or<'a, C, O, P1, P2>(p1: P1, p2: P2) -> impl Fn(&mut C) -> Result<O, Error>
where
    O: Ret,
    P1: Regex<C, Ret = O>,
    P2: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    move |ctx: &mut C| p1.try_parse(ctx).or_else(|_| p2.try_parse(ctx))
}

pub fn quote<'a, C, L, R, P, O>(l: L, r: R, p: P) -> impl Fn(&mut C) -> Result<O, Error>
where
    L: Regex<C>,
    R: Regex<C>,
    P: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);

        g.try_mat(&l)?;
        let ret = g.try_mat(&p)?;

        g.try_mat(&r)?;
        Ok(ret)
    }
}

pub fn terminated<'a, C, S, P, O>(sep: S, p: P) -> impl Fn(&mut C) -> Result<O, Error>
where
    S: Regex<C>,
    P: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(&p)?;

        g.try_mat(&sep)?;
        Ok(ret)
    }
}

fn length<'a, C: Context<'a>>(offset: usize, ctx: &C, next: Option<usize>) -> usize {
    let next_offset = next.unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

fn make_ret_and_inc<'a, C: Context<'a>, R: Ret>(ctx: &mut C, count: usize, len: usize) -> R {
    let ret = R::from(ctx, (count, len));

    ctx.inc(len);
    ret
}

pub fn one<'a, C, R>(re: impl Neure<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        trace_log!("match data in one(1..=1)");
        let mut iter: C::Iter<'_> = ctx.peek()?;

        if let Some((offset, item)) = iter.next() {
            if re.is_match(&item) {
                return Ok(make_ret_and_inc(
                    ctx,
                    1,
                    length(offset, ctx, iter.next().map(|v| v.0)),
                ));
            }
        }
        Err(Error::One)
    }
}

pub fn zero_one<'a, C, R>(re: impl Neure<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        trace_log!("match data in zero_one(0..=1)");
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if re.is_match(&item) {
                    return Ok(make_ret_and_inc(
                        ctx,
                        1,
                        length(offset, ctx, iter.next().map(|v| v.0)),
                    ));
                }
            }
        }
        Ok(R::from(ctx, (0, 0)))
    }
}

pub fn zero_more<'a, C, R>(re: impl Neure<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;

        trace_log!("match data in zero_more(0..)");
        if let Ok(mut iter) = ctx.peek() {
            for (offset, item) in iter.by_ref() {
                if !re.is_match(&item) {
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
            Ok(make_ret_and_inc(
                ctx,
                cnt,
                length(start, ctx, end.map(|v| v.0)),
            ))
        } else {
            Ok(R::from(ctx, (0, 0)))
        }
    }
}

pub fn one_more<'a, C, R>(re: impl Neure<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;

        trace_log!("match data in one_more(1..)");
        let mut iter = ctx.peek()?;

        for (offset, item) in iter.by_ref() {
            if !re.is_match(&item) {
                end = Some((offset, item));
                break;
            }
            cnt += 1;
            if beg.is_none() {
                beg = Some(offset);
            }
        }
        if let Some(start) = beg {
            Ok(make_ret_and_inc(
                ctx,
                cnt,
                length(start, ctx, end.map(|v| v.0)),
            ))
        } else {
            Err(Error::OneMore)
        }
    }
}

// pub fn count<'a, const M: usize, const N: usize, C, R>(
//     re: impl Unit<C::Item> + 'a,
// ) -> impl Fn(&mut C) -> Result<R, Error> + 'a
// where
//     R: Ret + 'a,
//     C: Context<'a> + 'a,
// {
//     count_if::<'a, M, N, C, R>(re, |_, _| true)
// }

pub fn count<'a, const M: usize, const N: usize, C, U: Neure<C::Item>>(
    re: U,
) -> crate::neure::NeureRepeat<'a, M, N, C, U, crate::neure::NullCond>
where
    C: Context<'a>,
{
    crate::neure::Neure2Regex::repeat::<M, N>(re)
}

pub fn count_if<'a, const M: usize, const N: usize, C, R>(
    re: impl Neure<C::Item>,
    r#if: impl crate::neure::NeureCond<'a, C>,
) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    debug_assert!(M <= N, "M must little equal than N");
    move |ctx: &mut C| {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        trace_log!("match data in count_if({}..={})", M, N);
        if let Ok(mut iter) = iter {
            while cnt < N {
                if let Some(pair) = iter.next() {
                    if re.is_match(&pair.1) && r#if.check(ctx, &pair)? {
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
                let end = end.or_else(|| iter.next()).map(|v| v.0);

                return Ok(make_ret_and_inc(
                    ctx,
                    cnt,
                    beg.map(|v| length(v, ctx, end)).unwrap_or(0),
                ));
            }
        }
        Err(Error::CountIf)
    }
}

pub fn start<'a, C, R>() -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    |ctx: &mut C| {
        if ctx.offset() == 0 {
            trace_log!("match start of context");
            Ok(R::from(ctx, (0, 0)))
        } else {
            Err(Error::Start)
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
            Err(Error::End)
        } else {
            trace_log!("match end of context");
            Ok(R::from(ctx, (0, 0)))
        }
    }
}

pub fn string<'a, 'b, C, R>(lit: &'b str) -> impl Fn(&mut C) -> Result<R, Error> + 'b
where
    R: Ret,
    C: Context<'a, Orig = str>,
{
    move |ctx: &mut C| {
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::String)
        } else {
            let len = lit.len();
            let _str = ctx.orig_sub(ctx.offset(), len)?;
            let ret = R::from(ctx, (1, len));

            trace_log!("match string \"{}\" with {}", lit, _str);
            ctx.inc(len);
            Ok(ret)
        }
    }
}

pub fn bytes<'a, 'b, C, R>(lit: &'b [u8]) -> impl Fn(&mut C) -> Result<R, Error> + 'b
where
    R: Ret,
    C: Context<'a, Orig = [u8]>,
{
    move |ctx: &mut C| {
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            let len = lit.len();
            let _byte = ctx.orig_sub(ctx.offset(), len)?;
            let ret = R::from(ctx, (1, len));

            trace_log!("match bytes \"{:?}\" with {:?}", lit, _byte);
            ctx.inc(len);
            Ok(ret)
        }
    }
}

pub fn consume<'a, C, R>(length: usize) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    move |ctx: &mut C| {
        trace_log!(
            "try to consume length {}, current offset = {}, total = {}",
            length,
            ctx.offset(),
            ctx.len()
        );
        if ctx.len() - ctx.offset() >= length {
            let ret = R::from(ctx, (1, length));

            ctx.inc(length);
            Ok(ret)
        } else {
            Err(Error::Consume)
        }
    }
}

pub fn null<'a, C, R>() -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    move |ctx: &mut C| Ok(R::from(ctx, (0, 0)))
}
