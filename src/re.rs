mod extract;
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

pub use self::ctor::branch;
pub use self::ctor::into_boxed_ctor;
pub use self::ctor::into_dyn_ctor;
pub use self::ctor::rec_parser;
pub use self::ctor::rec_parser_sync;
pub use self::ctor::BoxedCtorHelper;
pub use self::ctor::ConstructOp;
pub use self::ctor::Ctor;
pub use self::ctor::DynamicCreateCtorThenHelper;
pub use self::ctor::DynamicCtorHelper;
pub use self::ctor::RecursiveCtor;
pub use self::ctor::RecursiveCtorSync;
pub use self::extract::Extract;
pub use self::extract::Handler;
pub use self::into::BoxedRegex;
pub use self::into::RegexIntoOp;
pub use self::null::NullRegex;
pub use self::regex::and;
pub use self::regex::collect;
pub use self::regex::into_dyn_regex;
pub use self::regex::ltm;
pub use self::regex::or;
pub use self::regex::quote;
pub use self::regex::repeat;
pub use self::regex::sep;
pub use self::regex::sep_collect;
pub use self::regex::sep_once;
pub use self::regex::DynamicCreateRegexThenHelper;
pub use self::regex::DynamicRegexHelper;

use self::ctor::PairVector;
use self::ctor::Vector;
use self::regex::RegexConsume;
use self::regex::RegexConsumeAll;
use self::regex::RegexEnd;
use self::regex::RegexNot;
use self::regex::RegexSlice;
use self::regex::RegexStart;
use self::regex::RegexString;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::Condition;
use crate::neu::Neu;
use crate::neu::Neu2Re;
use crate::neu::NeureOne;
use crate::neu::NeureOneMore;
use crate::neu::NeureZeroMore;
use crate::neu::NeureZeroOne;
use crate::neu::NullCond;

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
    C: Context<'a, Orig = str> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::string(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for String
where
    C: Context<'a, Orig = str> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &String
where
    C: Context<'a, Orig = str> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &[u8]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::slice(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for &[u8; N]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for [u8; N]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let pattern = crate::re::slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for Vec<u8>
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<'a, C> Regex<C> for &Vec<u8>
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<'a, P, C> Regex<C> for Option<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().ok_or(Error::Option)?.try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (*self.borrow()).try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.get().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
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
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Box<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Match<C>,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Arc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Match<C>,
{
    type Ret = Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, Ret, C> Regex<C> for Rc<dyn Regex<C, Ret = Ret>>
where
    C: Context<'a> + Match<C>,
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
pub fn one<'a, C, U>(unit: U) -> NeureOne<C, U, C::Item, NullCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.repeat_one()
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
pub fn zero_one<'a, C, U>(unit: U) -> NeureZeroOne<C, U, C::Item, NullCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.repeat_zero_one()
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
pub fn zero_more<'a, C, U>(unit: U) -> NeureZeroMore<C, U, C::Item, NullCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.repeat_zero_more()
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
pub fn count<'a, const M: usize, const N: usize, C, U>(
    unit: U,
) -> crate::neu::NeureRepeat<M, N, C, U, NullCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.repeat::<M, N>()
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
pub fn count_if<'a, const M: usize, const N: usize, C, U, F>(
    re: U,
    r#if: F,
) -> crate::neu::NeureRepeat<M, N, C, U, F>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    F: crate::neu::NeuCond<'a, C>,
{
    re.repeat::<M, N>().set_cond(r#if)
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
pub fn start() -> RegexStart {
    RegexStart::new()
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
pub fn end() -> RegexEnd {
    RegexEnd::new()
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
pub fn string(lit: &str) -> RegexString<'_> {
    RegexString::new(lit)
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
///     let head = re::slice(&[0xff, 0xff]);
///     let mut ctx = BytesCtx::new(&[0xff, 0xff, 0x12]);
///
///     assert_eq!(ctx.try_mat(&head)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
pub fn slice<T>(lit: &[T]) -> RegexSlice<'_, T> {
    RegexSlice::new(lit)
}

///
/// Consume given length datas.
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
pub fn consume(len: usize) -> RegexConsume {
    RegexConsume::new(len)
}

///
/// Consume all the left datas.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let str = re::string("aabb");
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&str.then(re::consume_all()))?, Span::new(0, 8));
///
///     Ok(())
/// # }
/// ```
pub fn consume_all() -> RegexConsumeAll {
    RegexConsumeAll::new()
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
pub fn null<R>() -> NullRegex<R> {
    NullRegex::new()
}

///
/// Return a regex which reverse the result of `re`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let re = re::not("]]]");
///     let mut ctx = CharsCtx::new("[123,456,789]");
///
///     assert_eq!(ctx.try_mat(&re)?, Span::new(0, 0));
///     Ok(())
/// # }
/// ```
pub fn not<T>(re: T) -> RegexNot<T> {
    RegexNot::new(re)
}

pub fn vector<T>(val: Vec<T>) -> Vector<T> {
    Vector::new(val)
}

pub fn pair_vector<K, V>(val: Vec<(K, V)>) -> PairVector<K, V> {
    PairVector::new(val)
}

#[cfg(feature = "log")]
macro_rules! trace {
    ($name:literal, $beg:ident, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}`@{} start", $name, $beg);
        ret
    }};
    ($name:literal, $beg:ident @ $stage:literal, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}`@{} try stage `{}`", $name, $beg, $stage);
        ret
    }};
    ($name:literal, $beg:ident -> $end:expr, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}`@{} -> {{end: {}, ret: {}}}", $name, $beg, $end, ret);
        ret
    }};
    ($name:literal, $beg:ident => $end:expr, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}`@{} => {{end: {}, ret: {:?}}}", $name, $beg, $end, ret);
        ret
    }};
}

#[cfg(feature = "log")]
macro_rules! trace_v {
    ($name:literal, $inner:expr, $beg:ident, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}({:?})`@{} start", $name, $inner, $beg);
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident @ $stage:literal, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("r`{}({:?})`@{} try stage `{}`", $name, $inner, $beg, $stage);
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident => $end:expr, $ret:expr, $cnt:expr) => {{
        let ret = $ret;
        $crate::trace_log!(
            "r`{}({:?})`@{} => {{end: {}, ret: {:?}, cnt = {}}}",
            $name,
            $inner,
            $beg,
            $end,
            ret,
            $cnt
        );
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident -> $end:expr, $ret:expr, $cnt:expr) => {{
        let ret = $ret;
        $crate::trace_log!(
            "r`{}({:?})`@{} -> {{end: {}, ret: {}, cnt: {}}}",
            $name,
            $inner,
            $beg,
            $end,
            ret,
            $cnt
        );
        ret
    }};
}

#[cfg(not(feature = "log"))]
macro_rules! trace {
    ($name:literal, $beg:ident, $ret:expr) => {{
        let (_, _, ret) = ($name, $beg, $ret);
        ret
    }};
    ($name:literal, $beg:ident @ $stage:literal, $ret:expr) => {{
        let (_, _, _, ret) = ($name, $beg, $stage, $ret);
        ret
    }};
    ($name:literal, $beg:ident -> $end:expr, $ret:expr) => {{
        let (_, _, _, ret) = ($name, $beg, $end, $ret);
        ret
    }};
    ($name:literal, $beg:ident => $end:expr, $ret:expr) => {{
        let (_, _, _, ret) = ($name, $beg, $end, $ret);
        ret
    }};
}

#[cfg(not(feature = "log"))]
macro_rules! trace_v {
    ($name:literal, $inner:expr, $beg:ident, $ret:expr) => {{
        let (_, _, _, ret) = ($name, $inner, $beg, $ret);
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident @ $stage:literal, $ret:expr) => {{
        let (_, _, _, _, ret) = ($name, $inner, $beg, $stage, $ret);
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident => $end:expr, $ret:expr, $cnt:expr) => {{
        let (_, _, _, _, ret) = ($name, $inner, $beg, $end, $ret);
        ret
    }};
    ($name:literal, $inner:expr, $beg:ident -> $end:expr, $ret:expr, $cnt:expr) => {{
        let (_, _, _, _, ret) = ($name, $inner, $beg, $end, $ret);
        ret
    }};
}

pub(crate) use trace;
pub(crate) use trace_v;
