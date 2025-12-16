mod anchor;
mod assert;
mod builder;
mod consume;
mod empty;
mod fail;
mod literal;
mod rec;
mod wrap;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::anchor::end;
pub use self::anchor::start;
pub use self::anchor::AnchorEnd;
pub use self::anchor::AnchorStart;
pub use self::assert::assert;
pub use self::assert::not;
pub use self::assert::peek;
pub use self::assert::Assert;
pub use self::builder::into_regex_builder;
pub use self::builder::DynamicRegexBuilderHelper;
pub use self::consume::consume;
pub use self::consume::consume_all;
pub use self::consume::Consume;
pub use self::consume::ConsumeAll;
pub use self::empty::empty;
pub use self::empty::EmptyRegex;
pub use self::fail::fail;
pub use self::fail::FailRegex;
pub use self::literal::lit_slice;
pub use self::literal::string;
pub use self::literal::LitSlice;
pub use self::literal::LitString;
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
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::Condition;
use crate::neu::EmptyCond;
use crate::neu::Many0;
use crate::neu::Many1;
use crate::neu::Neu;
use crate::neu::NeuIntoRegexOps;
use crate::neu::Once;
use crate::neu::Opt;
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
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Ok(Span::new(ctx.offset(), 0))
    }
}

impl<'a, C> Regex<C> for &str
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for String
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &String
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::string(self.as_str());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for &[u8]
where
    C: Context<'a, Orig<'a> = &'a [u8]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self);
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for &[u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, const N: usize, C> Regex<C> for [u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let pattern = crate::regex::lit_slice(self.as_slice());
        pattern.try_parse(ctx)
    }
}

impl<'a, C> Regex<C> for Vec<u8>
where
    C: Context<'a, Orig<'a> = &'a [u8]>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<'a, C> Regex<C> for &Vec<u8>
where
    C: Context<'a, Orig<'a> = &'a [u8]>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Regex::try_parse(&self.as_slice(), ctx)
    }
}

impl<P, C> Regex<C> for Option<P>
where
    P: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().ok_or(Error::Option)?.try_parse(ctx)
    }
}

impl<P, C> Regex<C> for RefCell<P>
where
    P: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        (*self.borrow()).try_parse(ctx)
    }
}

impl<P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.get().try_parse(ctx)
    }
}

impl<P, C> Regex<C> for Mutex<P>
where
    P: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let ret = self.lock().map_err(|_| Error::Mutex)?;
        (*ret).try_parse(ctx)
    }
}

impl<P, C> Regex<C> for Arc<P>
where
    P: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<P, C> Regex<C> for Rc<P>
where
    P: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Box<dyn Regex<C> + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Box<dyn Regex<C> + Send + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Box<dyn Regex<C> + Send + Sync + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Arc<dyn Regex<C> + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Arc<dyn Regex<C> + Send + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Arc<dyn Regex<C> + Send + Sync + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Rc<dyn Regex<C> + 'b> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.as_ref().try_parse(ctx)
    }
}

impl<'b, C> Regex<C> for Rc<dyn Regex<C> + Send + 'b> {
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
///     let sign = regex::once('+');
///     let num = regex::many1('0'..='9');
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
pub fn once<'a, C, U>(unit: U) -> Once<C, U, C::Item, EmptyCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.once()
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
///     let sign = regex::opt('+');
///     let num = regex::many1('0'..='9');
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
pub fn opt<'a, C, U>(unit: U) -> Opt<C, U, C::Item, EmptyCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.opt()
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
///     let num = regex::many0('0'..='9');
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
pub fn many0<'a, C, U>(unit: U) -> Many0<C, U, C::Item, EmptyCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.many0()
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
///     let num = regex::many1('0'..='9');
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
pub fn many1<'a, C, N>(re: N) -> Many1<C, N, C::Item, EmptyCond>
where
    N: Neu<C::Item>,
    C: Context<'a>,
{
    re.many1()
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
) -> crate::neu::Between<M, N, C, U, EmptyCond>
where
    C: Context<'a>,
    U: Neu<C::Item>,
{
    unit.between::<M, N>()
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
    unit: U,
    test: F,
) -> crate::neu::Between<M, N, C, U, F>
where
    C: Context<'a> + 'a,
    U: Neu<C::Item>,
    F: crate::neu::NeuCond<'a, C>,
{
    unit.between::<M, N>().set_cond(test)
}

/// Matches the **first successful expression** from a dynamic sequence of alternatives.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ty = neu::ascii_alphabetic().many1();
///     let id = neu::ascii_alphabetic().many1();
///     let var = ty.sep_once("", id);
///     let ptr = ty.sep_once("*", id);
///     let r#ref = ty.sep_once("&", id);
///     let vec = regex::vector([var, ptr, r#ref]);
///     let sp = neu::whitespace().many0();
///
///     assert_eq!(CharsCtx::new("int a").skip_before(sp).ctor(&vec)?, ("int", "a"));
///     assert_eq!(CharsCtx::new("int *a").skip_before(sp).ctor(&vec)?, ("int", "a"));
///     assert_eq!(CharsCtx::new("int &a").skip_before(sp).ctor(&vec)?, ("int", "a"));
/// #   Ok(())
/// # }
/// ```
pub fn vector<T>(val: impl IntoIterator<Item = T>) -> Vector<T> {
    Vector::new(val.into_iter().collect())
}

/// Matches the first successful expression from a dynamic sequence while carrying associated data.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum C {
///         Var,
///         Ptr,
///         Ref,
///     }
///
///     let ty = neu::ascii_alphabetic().many1();
///     let id = neu::ascii_alphabetic().many1();
///     let var = ty.sep_once("", id);
///     let ptr = ty.sep_once("*", id);
///     let r#ref = ty.sep_once("&", id);
///     let vec = regex::pair_vector([(var, C::Var), (ptr, C::Ptr), (r#ref, C::Ref)]);
///     let sp = neu::whitespace().many0();
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
/// #   Ok(())
/// # }
/// ```
pub fn pair_vector<K, V: Clone>(val: impl IntoIterator<Item = (K, V)>) -> PairVector<K, V> {
    PairVector::new(val.into_iter().collect())
}

/// Iterate over the array and match the [`regex`](crate::regex::Regex) against the [`Context`].
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let parser = regex::array([b"rust".as_ref(), b"jawa", b"golang"]);
///     let mut ctx = BytesCtx::new(b"rust is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
pub fn array<const N: usize, T>(val: [T; N]) -> Array<N, T> {
    Array::new(val)
}

/// Attempts patterns in sequence, returning the first successful match from a **compile-time fixed array**.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let slice = [b"rust".as_ref(), b"jawa", b"golang"];
///     let parser = regex::slice(&slice);
///     let mut ctx = BytesCtx::new(b"rust is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
pub fn slice<T>(val: &[T]) -> Slice<'_, T> {
    Slice::new(val)
}

/// Iterate over the array and match the regex against the [`Context`].
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     #[derive(Debug, Clone, PartialEq)]
///     enum Lang {
///         Rust,
///         Java,
///         Golang,
///     }
///
///     let parser = regex::pair_array([
///         (b"rust".as_ref(), Lang::Rust),
///         (b"jawa", Lang::Java),
///         (b"golang", Lang::Golang),
///     ]);
///     let mut ctx = BytesCtx::new(b"golang is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 6));
/// #   Ok(())
/// # }
/// ```
pub fn pair_array<const N: usize, K, V>(val: [(K, V); N]) -> PairArray<N, K, V> {
    PairArray::new(val)
}

/// Maps patterns to associated values, returning the first successful match with its paired value.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     #[derive(Debug, Clone, PartialEq)]
///     enum Lang {
///         Rust,
///         Java,
///         Golang,
///     }
///
///     let slice = [
///         (b"rust".as_ref(), Lang::Rust),
///         (b"jawa", Lang::Java),
///         (b"golang", Lang::Golang),
///     ];
///     let parser = regex::pair_slice(&slice);
///     let mut ctx = BytesCtx::new(b"jawa is so awesome!");
///
///     assert_eq!(ctx.try_mat(&parser)?, Span::new(0, 4));
/// #   Ok(())
/// # }
/// ```
pub fn pair_slice<K, V>(val: &[(K, V)]) -> PairSlice<'_, K, V> {
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

macro_rules! impl_not_for_regex {
    (@$ty:ident [ ]  [ ]) => {
        impl std::ops::Not for $ty {
            type Output = $crate::regex::Assert<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ ]  [ $($p:ident),+ ]) => {
        impl<$($p),+> std::ops::Not for $ty<$($p),+> {
            type Output = $crate::regex::Assert<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ $($l:lifetime),+ ]  [ $($p:ident),* ]) => {
        impl<$($l),+ , $($p),*> std::ops::Not for $ty<$($l),+ , $($p),*> {
            type Output = $crate::regex::Assert<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    ($ty:ident) => {
        impl_not_for_regex! { @$ty [ ] [ ] }
    };
    ($ty:ident < $($l:lifetime),* $(,)? $($p:ident),* >) => {
        impl_not_for_regex! { @$ty [$($l),*] [$($p),*] }
    };
}

pub(crate) use impl_not_for_regex;
