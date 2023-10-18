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
use crate::neu::CRange;
use crate::neu::Neu;
use crate::neu::length_of;
use crate::neu::ret_and_inc;
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
        let pattern = crate::re::string(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, 'b, C> Regex<C> for &'b [u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C> + 'a,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::bytes(self);
        ctx.try_mat(&pattern)
    }
}

impl<'a, Ret, C> Regex<C> for Box<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(&*self.borrow())
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(&self.get())
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret: std::sync::MutexGuard<'_, P> =
            self.lock().expect("Oops ?! Can not unwrap mutex ...");
        ctx.try_mat_t(&*ret)
    }
}

impl<'a, P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Arc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(self.as_ref())
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(self.as_ref())
    }
}

impl<'a, Ret, C> Regex<C> for Rc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C> + 'a,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat_t(self.as_ref())
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

pub fn one<'a, C, R>(re: impl Neu<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        trace_log!("match data in one(1..2)");
        let mut iter: C::Iter<'_> = ctx.peek()?;

        if let Some((offset, item)) = iter.next() {
            if re.is_match(&item) {
                return Ok(ret_and_inc(
                    ctx,
                    1,
                    length_of(offset, ctx, iter.next().map(|v| v.0)),
                ));
            }
        }
        Err(Error::One)
    }
}

pub fn zero_one<'a, C, R>(re: impl Neu<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + 'a,
{
    move |ctx: &mut C| {
        trace_log!("match data in zero_one(0..2)");
        if let Ok(mut iter) = ctx.peek() {
            if let Some((offset, item)) = iter.next() {
                if re.is_match(&item) {
                    return Ok(ret_and_inc(
                        ctx,
                        1,
                        length_of(offset, ctx, iter.next().map(|v| v.0)),
                    ));
                }
            }
        }
        Ok(R::from(ctx, (0, 0)))
    }
}

pub fn zero_more<'a, C, R>(re: impl Neu<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
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
            Ok(ret_and_inc(
                ctx,
                cnt,
                length_of(start, ctx, end.map(|v| v.0)),
            ))
        } else {
            Ok(R::from(ctx, (0, 0)))
        }
    }
}

pub fn one_more<'a, C, R>(re: impl Neu<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
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
            Ok(ret_and_inc(
                ctx,
                cnt,
                length_of(start, ctx, end.map(|v| v.0)),
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

pub fn count<'a, const M: usize, const N: usize, C, U: Neu<C::Item>>(
    re: U,
) -> crate::neu::NeureRepeat<'a, M, N, C, U, crate::neu::NullCond>
where
    C: Context<'a>,
{
    crate::neu::Neu2Re::repeat::<M, N>(re)
}

pub fn count_if<'a, const M: usize, const N: usize, C, U: Neu<C::Item>, F>(
    re: U,
    r#if: F,
) -> crate::neu::NeureRepeat<'a, M, N, C, U, F>
where
    C: Context<'a> + 'a,
    F: crate::neu::NeuCond<'a, C>,
{
    crate::neu::NeureRepeat::new(re, r#if)
}

///
/// Match the start position of data.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let pos = re::start();
///     let rust = re::string("rust");
///     let year = neu::digit(10).repeat_times::<4>();
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&pos)?, Span::new(0, 0));
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&year)?, Span::new(4, 4));
///
///     Ok(())
/// # }
/// ```
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

///
/// Match the end position of data.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let rust = re::string("rust");
///     let year = neu::digit(10).repeat_times::<4>();
///     let end = re::end();
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&year)?, Span::new(4, 4));
///     assert_eq!(ctx.try_mat(&end)?, Span::new(8, 0));
///
///     Ok(())
/// # }
/// ```
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

///
/// Match given string.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let rust = re::string("rust");
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///
///     Ok(())
/// # }
/// ```
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

///
/// Match given data.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let head = re::bytes(&[0xff, 0xff]);
///     let mut ctx = BytesCtx::new(&[0xff, 0xff, 0x12]);
///
///     assert_eq!(ctx.try_mat(&head)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
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

///
/// Consume given length items.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let null = re::consume(6);
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&null)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
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

///
/// Match nothing, simple return `R::from(_, (0, 0))`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     let null = re::null();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&null)?, Span::new(0, 0));
///
///     Ok(())
/// # }
/// ```
pub fn null<'a, C, R>() -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a>,
{
    move |ctx: &mut C| Ok(R::from(ctx, (0, 0)))
}