mod adapter;
mod anchor;
mod assert;
mod builder;
mod consume;
mod empty;
mod fail;
mod into;
mod literal;
#[cfg(feature = "alloc")]
mod rec;

use core::cell::Cell;
use core::cell::RefCell;

pub use self::adapter::Adapter;
pub use self::adapter::RefAdapter;
pub use self::anchor::AnchorEnd;
pub use self::anchor::AnchorStart;
pub use self::anchor::end;
pub use self::anchor::start;
pub use self::assert::Assert;
pub use self::assert::assert;
pub use self::assert::not;
pub use self::assert::peek;
pub use self::builder::DynamicRegexBuilderHelper;
pub use self::builder::into_regex_builder;
pub use self::consume::Consume;
pub use self::consume::ConsumeAll;
pub use self::consume::consume;
pub use self::consume::consume_all;
pub use self::empty::EmptyRegex;
pub use self::empty::empty;
pub use self::fail::FailRegex;
pub use self::fail::fail;
pub use self::into::RegexIntoHelper;
pub use self::literal::LitSlice;
pub use self::literal::LitString;
pub use self::literal::lit_slice;
pub use self::literal::string;

#[cfg(feature = "alloc")]
pub use self::rec::*;

use crate::ctor::Array;
use crate::ctor::PairArray;
use crate::ctor::PairSlice;
use crate::ctor::Slice;
use crate::ctx::Context;
use crate::err::Error;
use crate::neu::Condition;
use crate::neu::EmptyCond;
use crate::neu::Many0;
use crate::neu::Many1;
use crate::neu::Neu;
use crate::neu::NeuIntoRegexOps;
use crate::neu::Once;
use crate::neu::Opt;
use crate::span::Span;

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

macro_rules! impl_orig_regex {
    ($self:ident, $regex:expr, $type:ty, $orig:ty) => {
        impl<'a, C> Regex<C> for $type
        where
            C: Context<'a, Orig<'a> = &'a $orig>,
        {
            #[inline(always)]
            fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
                let $self = self;

                Regex::try_parse($regex, ctx)
            }
        }
    };
}

macro_rules! impl_forward_regex {
    ($self:ident, $regex:expr, $type:ty) => {
        impl<P, C> Regex<C> for $type
        where
            P: Regex<C>,
        {
            #[inline(always)]
            fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
                let $self = self;

                Regex::try_parse($regex, ctx)
            }
        }
    };
}

#[cfg(feature = "alloc")]
mod alloc_regex_impls {

    use crate::alloc::Arc;
    use crate::alloc::Box;
    use crate::alloc::Rc;
    use crate::alloc::String;
    use crate::alloc::Vec;
    use crate::ctor::Ctor;
    use crate::ctx::Context;
    use crate::err::Error;
    use crate::regex::Regex;
    use crate::regex::lit_slice;
    use crate::regex::string;
    use crate::span::Span;

    impl_orig_regex!(self_, &string(self_), String, str);

    impl_orig_regex!(self_, &string(self_), &String, str);

    impl_orig_regex!(self_, &lit_slice(self_.as_slice()), Vec<u8>, [u8]);

    impl_orig_regex!(self_, &lit_slice(self_.as_slice()), &Vec<u8>, [u8]);

    impl_forward_regex!(self_, self_.as_ref(), Arc<P>);

    impl_forward_regex!(self_, self_.as_ref(), Rc<P>);

    macro_rules! impl_dyn_regex {
        ($type:ty) => {
            impl<'b, C> Regex<C> for $type {
                fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
                    self.as_ref().try_parse(ctx)
                }
            }
        };
    }

    macro_rules! impl_dyn_ctor {
        ($ctor:ty) => {
            impl<'a, 'b, C, O, H> Regex<C> for $ctor {
                fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
                    self.as_ref().try_parse(ctx)
                }
            }
        };
    }

    impl_dyn_regex!(Box<dyn Regex<C> + 'b>);

    impl_dyn_regex!(Box<dyn Regex<C> + Send + 'b>);

    impl_dyn_regex!(Box<dyn Regex<C> + Send + Sync + 'b>);

    impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + 'b>);

    impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + Send + 'b>);

    impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + Send + Sync + 'b>);

    impl_dyn_regex!(Arc<dyn Regex<C> + 'b>);

    impl_dyn_regex!(Arc<dyn Regex<C> + Send + 'b>);

    impl_dyn_regex!(Arc<dyn Regex<C> + Send + Sync + 'b>);

    impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + 'b>);

    impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + Send + 'b>);

    impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + Send + Sync + 'b>);

    impl_dyn_regex!(Rc<dyn Regex<C> + 'b>);

    impl_dyn_regex!(Rc<dyn Regex<C> + Send + 'b>);

    impl_dyn_ctor!(Rc<dyn Ctor<'a, C, O, H> + 'b>);

    impl_dyn_ctor!(Rc<dyn Ctor<'a, C, O, H> + Send + 'b>);
}

impl_orig_regex!(self_, &string(self_), &str, str);

impl_orig_regex!(self_, &lit_slice(self_), &[u8], [u8]);

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

impl_forward_regex!(self_, self_.as_ref().ok_or(Error::Option)?, Option<P>);

impl_forward_regex!(self_, &*self_.borrow(), RefCell<P>);

#[cfg(feature = "std")]
impl_forward_regex!(
    self_,
    &*self_.lock().map_err(|_| Error::Mutex)?,
    crate::std::Mutex<P>
);

impl<P, C> Regex<C> for Cell<P>
where
    P: Regex<C> + Copy,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.get().try_parse(ctx)
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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
#[cfg(feature = "alloc")]
pub fn vector<T>(val: impl IntoIterator<Item = T>) -> crate::ctor::Vector<T> {
    crate::ctor::Vector::new(val.into_iter().collect())
}

/// Matches the first successful expression from a dynamic sequence while carrying associated data.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
#[cfg(feature = "alloc")]
pub fn pair_vector<T, V: Clone>(
    val: impl IntoIterator<Item = (T, V)>,
) -> crate::ctor::PairVector<T, V> {
    crate::ctor::PairVector::new(val.into_iter().collect())
}

/// Iterate over the array and match the [`regex`](crate::regex::Regex) against the [`Context`].
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
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

pub trait AsCtor<C>
where
    Self: Regex<C>,
{
    fn as_ctor(&self) -> RefAdapter<'_, C, Self>;
}

impl<T: ?Sized, C> AsCtor<C> for T
where
    Self: Regex<C>,
{
    fn as_ctor(&self) -> RefAdapter<'_, C, Self> {
        RefAdapter::new(self)
    }
}

macro_rules! impl_not_for_regex {
    (@$ty:ident [ ]  [ ]) => {
        impl core::ops::Not for $ty {
            type Output = $crate::regex::Assert<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ ]  [ $($p:ident),+ ]) => {
        impl<$($p),+> core::ops::Not for $ty<$($p),+> {
            type Output = $crate::regex::Assert<Self>;

            fn not(self) -> Self::Output { $crate::regex::not(self) }
        }
    };
    (@$ty:ident [ $($l:lifetime),+ ]  [ $($p:ident),* ]) => {
        impl<$($l),+ , $($p),*> core::ops::Not for $ty<$($l),+ , $($p),*> {
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
