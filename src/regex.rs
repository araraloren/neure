mod anchor;
mod builder;
mod consume;
mod literal;
mod not;
mod null;
mod rec;
mod wrap;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::anchor::AnchorEnd;
pub use self::anchor::AnchorStart;
pub use self::builder::into_regex_builder;
pub use self::builder::DynamicRegexBuilderHelper;
pub use self::consume::Consume;
pub use self::consume::ConsumeAll;
pub use self::literal::LitSlice;
pub use self::literal::LitString;
pub use self::not::RegexNot;
pub use self::null::NullRegex;
pub use self::rec::rec_parser;
pub use self::rec::rec_parser_sync;
pub use self::rec::rec_parser_with;
pub use self::rec::rec_parser_with_sync;
pub use self::rec::RecParser;
pub use self::rec::RecParserSync;
pub use self::rec::RecursiveCtor;
pub use self::rec::RecursiveCtorSync;
pub use self::rec::RecursiveCtorWith;
pub use self::rec::RecursiveCtorWithSync;
pub use self::rec::RecursiveParser;
pub use self::rec::RecursiveParserSync;
pub use self::wrap::Wrap;

use crate::ctor::Array;
use crate::ctor::PairArray;
use crate::ctor::PairSlice;
use crate::ctor::PairVector;
use crate::ctor::Slice;
use crate::ctor::Vector;
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
use crate::regex::wrap::BoxedRegex;

pub trait Regex<C> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error>;

    fn parse(&self, ctx: &mut C) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<C, F> Regex<C> for F
where
    F: Fn(&mut C) -> Result<Span, Error>,
{
    #[inline]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        (self)(ctx)
    }
}

impl<'a, C> Regex<C> for ()
where
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Ok(Span::new(ctx.offset(), 0))
    }
}

impl<'a, C> Regex<C> for &str
where
    C: Context<'a, Orig<'a> = &'a str> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for String
where
    C: Context<'a, Orig<'a> = &'a str> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &String
where
    C: Context<'a, Orig<'a> = &'a str> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &[u8]
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for &[u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for [u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for Vec<u8>
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<'a, C> Regex<C> for &Vec<u8>
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<'a, P, C> Regex<C> for Option<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().ok_or(Error::Option)?.try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        (*self.borrow()).try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.get().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let ret = self.lock().map_err(|_| Error::Mutex)?;
        (*ret).try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, 'b, C> Regex<C> for Box<dyn Regex<C> + 'b>
where
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, 'b, C> Regex<C> for Arc<dyn Regex<C> + 'b>
where
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'a, 'b, C> Regex<C> for Rc<dyn Regex<C> + 'b>
where
    C: Context<'a> + Match<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
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
/// #     color_eyre::install()?;
///     let sign = regex::one('+');
///     let num = regex::one_more('0'..='9');
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
/// #     color_eyre::install()?;
///     let sign = regex::zero_one('+');
///     let num = regex::one_more('0'..='9');
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
/// #     color_eyre::install()?;
///     let num = regex::zero_more('0'..='9');
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
/// #     color_eyre::install()?;
///     let num = regex::one_more('0'..='9');
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
/// #     color_eyre::install()?;
///     let website = regex::count::<0, { usize::MAX }, _, _>(('a'..'{'));
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
/// #     color_eyre::install()?;
///     let website = regex::count_if::<0, { usize::MAX }, _, _, _>(
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
/// #     color_eyre::install()?;
///     let pos = regex::start();
///     let rust = regex::string("rust");
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
pub fn start() -> AnchorStart {
    AnchorStart::new()
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
/// #     color_eyre::install()?;
///     let rust = regex::string("rust");
///     let year = neu::digit(10).repeat_times::<4>();
///     let end = regex::end();
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&year)?, Span::new(4, 4));
///     assert_eq!(ctx.try_mat(&end)?, Span::new(8, 0));
///
///     Ok(())
/// # }
/// ```
pub fn end() -> AnchorEnd {
    AnchorEnd::new()
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
/// #     color_eyre::install()?;
///     let rust = regex::string("rust");
///     let mut ctx = CharsCtx::new("rust2023");
///
///     assert_eq!(ctx.try_mat(&rust)?, Span::new(0, 4));
///
///     Ok(())
/// # }
/// ```
pub fn string(lit: &str) -> LitString<'_> {
    LitString::new(lit)
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
/// #     color_eyre::install()?;
///     let head = regex::lit_slice(&[0xff, 0xff]);
///     let mut ctx = BytesCtx::new(&[0xff, 0xff, 0x12]);
///
///     assert_eq!(ctx.try_mat(&head)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
pub fn lit_slice<T>(lit: &[T]) -> LitSlice<'_, T> {
    LitSlice::new(lit)
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
/// #     color_eyre::install()?;
///     let null = regex::consume(6);
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&null)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
pub fn consume(len: usize) -> Consume {
    Consume::new(len)
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
/// #     color_eyre::install()?;
///     let str = regex::string("aabb");
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&str.then(regex::consume_all()))?, Span::new(0, 8));
///
///     Ok(())
/// # }
/// ```
pub fn consume_all() -> ConsumeAll {
    ConsumeAll::new()
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
/// #     color_eyre::install()?;
///     let null = regex::null();
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&null)?, Span::new(0, 0));
///
///     Ok(())
/// # }
/// ```
pub fn null() -> NullRegex {
    NullRegex::new()
}

///
/// Return a regex that reverses the result of `re`.
/// It will return zero-length [`Span`] when matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let re = regex::not("]]]");
///     let mut ctx = CharsCtx::new("[123,456,789]");
///
///     assert_eq!(ctx.try_mat(&re)?, Span::new(0, 0));
///     Ok(())
/// # }
/// ```
pub fn not<T>(re: T) -> RegexNot<T> {
    RegexNot::new(re)
}

/// Iterate over the vector and match the regex against the [`Context`].
/// It will return the result of first regex that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let ty = neu::ascii_alphabetic().repeat_one_more();
///     let id = neu::ascii_alphabetic().repeat_one_more();
///     let var = ty.sep_once("", id);
///     let ptr = ty.sep_once("*", id);
///     let r#ref = ty.sep_once("&", id);
///     let vec = regex::vector([var, ptr, r#ref]);
///     let sp = neu::whitespace().repeat_full();
///
///     assert_eq!(CharsCtx::new("int a").skip_before(sp).ctor(&vec)?, ("int", "a"));
///     assert_eq!(CharsCtx::new("int *a").skip_before(sp).ctor(&vec)?, ("int", "a"));
///     assert_eq!(CharsCtx::new("int &a").skip_before(sp).ctor(&vec)?, ("int", "a"));
///     Ok(())
/// # }
/// ```
pub fn vector<T>(val: impl IntoIterator<Item = T>) -> Vector<T> {
    Vector::new(val.into_iter().collect())
}

/// Iterate over the vector and match the regex against the [`Context`].
/// It will return the value of first pair that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum C {
///         Var,
///         Ptr,
///         Ref,
///     }
///
///     let ty = neu::ascii_alphabetic().repeat_one_more();
///     let id = neu::ascii_alphabetic().repeat_one_more();
///     let var = ty.sep_once("", id);
///     let ptr = ty.sep_once("*", id);
///     let r#ref = ty.sep_once("&", id);
///     let vec = regex::pair_vector([(var, C::Var), (ptr, C::Ptr), (r#ref, C::Ref)]);
///     let sp = neu::whitespace().repeat_full();
///
///     assert_eq!(
///         CharsCtx::new("int a").skip_before(sp).ctor(&vec)?,
///         (("int", "a"), C::Var)
///     );
///     assert_eq!(
///         CharsCtx::new("int *a").skip_before(sp).ctor(&vec)?,
///         (("int", "a"), C::Ptr)
///     );
///     assert_eq!(
///         CharsCtx::new("int &a").skip_before(sp).ctor(&vec)?,
///         (("int", "a"), C::Ref)
///     );
///     Ok(())
/// # }
/// ```
pub fn pair_vector<K, V: Clone>(val: impl IntoIterator<Item = (K, V)>) -> PairVector<K, V> {
    PairVector::new(val.into_iter().collect())
}

pub fn array<const N: usize, T>(val: [T; N]) -> Array<N, T> {
    Array::new(val)
}

pub fn slice<const N: usize, T>(val: &[T; N]) -> Slice<'_, N, T> {
    Slice::new(val)
}

pub fn pair_array<const N: usize, K, V>(val: [(K, V); N]) -> PairArray<N, K, V> {
    PairArray::new(val)
}

pub fn pair_slice<const N: usize, K, V>(val: &[(K, V); N]) -> PairSlice<'_, N, K, V> {
    PairSlice::new(val)
}

pub trait RegexIntoOp<C>
where
    Self: Sized + Regex<C>,
{
    fn into_ctor(self) -> Wrap<Self, C>;

    fn into_box_regex(self) -> Wrap<BoxedRegex<Self>, C>;

    fn into_rc_regex(self) -> Wrap<Rc<Self>, C>;

    fn into_arc_regex(self) -> Wrap<Arc<Self>, C>;

    fn into_cell_regex(self) -> Wrap<Cell<Self>, C>;

    fn into_refcell_regex(self) -> Wrap<RefCell<Self>, C>;

    fn into_mutex_regex(self) -> Wrap<Mutex<Self>, C>;

    fn into_dyn_regex<'a, 'b>(self) -> Wrap<Box<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b;

    fn into_dyn_arc_regex<'a, 'b>(self) -> Wrap<std::sync::Arc<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b;

    fn into_dyn_rc_regex<'a, 'b>(self) -> Wrap<std::rc::Rc<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b;
}

impl<C, T> RegexIntoOp<C> for T
where
    T: Regex<C>,
{
    fn into_ctor(self) -> Wrap<Self, C> {
        Wrap::new(self)
    }

    fn into_box_regex(self) -> Wrap<BoxedRegex<Self>, C> {
        Wrap::r#box(self)
    }

    fn into_rc_regex(self) -> Wrap<Rc<Self>, C> {
        Wrap::rc(self)
    }

    fn into_arc_regex(self) -> Wrap<Arc<Self>, C> {
        Wrap::arc(self)
    }

    fn into_cell_regex(self) -> Wrap<Cell<Self>, C> {
        Wrap::cell(self)
    }

    fn into_refcell_regex(self) -> Wrap<RefCell<Self>, C> {
        Wrap::refcell(self)
    }

    fn into_mutex_regex(self) -> Wrap<Mutex<Self>, C> {
        Wrap::mutex(self)
    }

    fn into_dyn_regex<'a, 'b>(self) -> Wrap<Box<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Wrap::dyn_box(self)
    }

    fn into_dyn_arc_regex<'a, 'b>(self) -> Wrap<std::sync::Arc<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Wrap::dyn_arc(self)
    }

    fn into_dyn_rc_regex<'a, 'b>(self) -> Wrap<std::rc::Rc<dyn Regex<C> + 'b>, C>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Wrap::dyn_rc(self)
    }
}

macro_rules! def_not {
    (@$ty:ident [ ]  [ ]) => {
        impl std::ops::Not for $ty {
            type Output = $crate::regex::RegexNot<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ ]  [ $($p:ident),+ ]) => {
        impl<$($p),+> std::ops::Not for $ty<$($p),+> {
            type Output = $crate::regex::RegexNot<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ $($l:lifetime),+ ]  [ $($p:ident),* ]) => {
        impl<$($l),+ , $($p),*> std::ops::Not for $ty<$($l),+ , $($p),*> {
            type Output = $crate::regex::RegexNot<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    ($ty:ident) => {
        def_not! { @$ty [ ] [ ] }
    };
    ($ty:ident < $($l:lifetime),* $(,)? $($p:ident),* >) => {
        def_not! { @$ty [$($l),*] [$($p),*] }
    };
}

pub(crate) use def_not;
