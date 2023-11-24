mod extract;
mod guard;
mod into;
mod null;

pub mod ctor;
pub mod map;
pub mod regex;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::ctor::into_boxed_ctor;
pub use self::ctor::into_dyn_ctor;
pub use self::ctor::rec_parser;
pub use self::ctor::rec_parser_sync;
pub use self::ctor::ConstructOp;
pub use self::ctor::Ctor;
pub use self::extract::Extract;
pub use self::extract::Handler;
pub use self::extract::HandlerV;
pub use self::guard::CtxGuard;
pub use self::into::BoxedRegex;
pub use self::into::RegexIntoOp;
pub use self::null::NullRegex;

use self::ctor::Or;
use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::length_of;
use crate::neu::ret_and_inc;
use crate::neu::Neu;
use crate::neu::Neu2Re;
use crate::neu::NeureOneMore;
use crate::neu::NullCond;
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

impl<'a, C> Regex<C> for &str
where
    C: Context<'a, Orig = str> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::string(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &[u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::bytes(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for &[u8; N]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::bytes(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Option<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().ok_or(Error::RegexOption)?.try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (*self.borrow()).try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.get().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret = self.lock().map_err(|_| Error::LockMutex)?;
        (*ret).try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Box<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C>,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Arc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C>,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Rc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Policy<C>,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

///
/// Match one item.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let sign = re::one('+');
///     let num = re::one_more('0'..='9');
///     let mut ctx = CharsCtx::new("+2077");
///
///     assert_eq!(ctx.try_mat(&sign)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&num)?, Span::new(1, 4));
///
///     let mut ctx = CharsCtx::new("2077");
///
///     assert!(ctx.try_mat(&sign).is_err());
///     Ok(())
/// # }
/// ```
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

///
/// Match zero or one item.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let sign = re::zero_one('+');
///     let num = re::one_more('0'..='9');
///     let mut ctx = CharsCtx::new("+2077");
///
///     assert_eq!(ctx.try_mat(&sign)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&num)?, Span::new(1, 4));
///
///     let mut ctx = CharsCtx::new("2077");
///
///     assert_eq!(ctx.try_mat(&sign)?, Span::new(0, 0));
///     assert_eq!(ctx.try_mat(&num)?, Span::new(0, 4));
///     Ok(())
/// # }
/// ```
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

///
/// Match at least zero item.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let num = re::zero_more('0'..='9');
///     let mut ctx = CharsCtx::new("2048mb");
///
///     assert_eq!(ctx.try_mat(&num)?, Span::new(0, 4));
///
///     let mut ctx = CharsCtx::new("rust2021");
///
///     assert_eq!(ctx.try_mat(&num)?, Span::new(0, 0));
///     Ok(())
/// # }
/// ```
///
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

///
/// Match at least one item.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let num = re::one_more('0'..='9');
///     let mut ctx = CharsCtx::new("2048mb");
///
///     assert_eq!(ctx.try_mat(&num)?, Span::new(0, 4));
///
///     let mut ctx = CharsCtx::new("rust2021");
///
///     assert!(ctx.try_mat(&num).is_err());
///     Ok(())
/// # }
/// ```
pub fn one_more<'a, C, N>(re: N) -> NeureOneMore<C, N, C::Item, NullCond>
where
    N: Neu<C::Item>,
    C: Context<'a>,
{
    re.repeat_one_more()
}

// pub fn one_more<'a, C, R>(re: impl Neu<C::Item>) -> impl Fn(&mut C) -> Result<R, Error>
// where
//     R: Ret,
//     C: Context<'a> + 'a,
// {
//     move |ctx: &mut C| {
//         let mut cnt = 0;
//         let mut beg = None;
//         let mut end = None;

//         trace_log!("match data in one_more(1..)");
//         let mut iter = ctx.peek()?;

//         for (offset, item) in iter.by_ref() {
//             if !re.is_match(&item) {
//                 end = Some((offset, item));
//                 break;
//             }
//             cnt += 1;
//             if beg.is_none() {
//                 beg = Some(offset);
//             }
//         }
//         if let Some(start) = beg {
//             Ok(ret_and_inc(
//                 ctx,
//                 cnt,
//                 length_of(start, ctx, end.map(|v| v.0)),
//             ))
//         } else {
//             Err(Error::OneMore)
//         }
//     }
// }

///
/// Match the given `Neu` M ..= N times.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let website = re::count::<0, { usize::MAX }, _, _>(('a'..'{'));
///     let mut ctx = CharsCtx::new("example.com");
///
///     assert_eq!(ctx.try_mat(&website)?, Span::new(0, 7));
///     assert_eq!(ctx.orig_sub(0, 7)?, "example");
///     Ok(())
/// }
/// ```
///
pub fn count<'a, const M: usize, const N: usize, C, U: Neu<C::Item>>(
    re: U,
) -> crate::neu::NeureRepeat<'a, M, N, C, U, crate::neu::NullCond>
where
    C: Context<'a>,
{
    crate::neu::Neu2Re::repeat::<M, N>(re)
}

///
/// Match the given `Neu` M ..= N times.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let website = re::count_if::<0, { usize::MAX }, _, _, _>(
///         ('a'..'{').or('.'),
///         |ctx: &CharsCtx, pair: &(usize, char)| {
///             Ok(pair.1 != '.'
///                 || ctx
///                     .orig()?
///                     .get((pair.0 + 1)..)
///                     .map(|v| v.find('.').is_some())
///                     .unwrap_or(false))
///         },
///     );
///     let mut ctx = CharsCtx::new("domain.example.com");
///
///     assert_eq!(ctx.try_mat(&website)?, Span::new(0, 14));
///     assert_eq!(ctx.orig_sub(0, 14)?, "domain.example");
///     Ok(())
/// }
/// ```
///
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
///     color_eyre::install()?;
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
///     color_eyre::install()?;
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
///     color_eyre::install()?;
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
        let len = lit.len();

        trace_log!(
            "(`string`: @{}) => '{}' <-> '{}'",
            ctx.offset(),
            lit,
            ctx.orig_sub(ctx.offset(), len)?
        );
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::String)
        } else {
            let ret = R::from(ctx, (1, len));

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
///     color_eyre::install()?;
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
        let len = lit.len();

        trace_log!(
            "(`bytes`: @{}) => '{:?}' <-> '{:?}'",
            ctx.offset(),
            lit,
            ctx.orig_sub(ctx.offset(), len)?
        );
        if !ctx.orig()?.starts_with(lit) {
            Err(Error::Bytes)
        } else {
            let ret = R::from(ctx, (1, len));

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
///     color_eyre::install()?;
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
///     color_eyre::install()?;
///     let null = re::null();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&null)?, Span::new(0, 0));
///
///     Ok(())
/// # }
/// ```
pub fn null<R>() -> NullRegex<R>
where
    R: Ret,
{
    NullRegex::new()
}

pub fn nullable<'a, T, C, R>(val: T) -> Or<C, T, NullRegex<R>>
where
    R: Ret,
    T: Regex<C, Ret = R>,
    C: Context<'a> + Policy<C>,
{
    val.or(null())
}

///
pub fn not<'a, C, R>(re: impl Regex<C, Ret = R>) -> impl Fn(&mut C) -> Result<R, Error>
where
    R: Ret,
    C: Context<'a> + Policy<C>,
{
    move |ctx: &mut C| {
        let mut g = CtxGuard::new(ctx);
        let ret = g.try_mat(&re);

        if ret.is_err() {
            Ok(R::from(g.ctx(), (0, 0)))
        } else {
            g.reset();
            Err(Error::Other)
        }
    }
}
