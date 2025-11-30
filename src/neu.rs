mod bool;
mod cond;
mod equal;
mod op_and;
mod op_not;
mod op_one;
mod op_or;
mod op_repeat;
mod op_then;
mod op_zero;
mod prefix;
mod range;
mod units;

use crate::ctx::Context;
use crate::trace_retval;
use crate::MayDebug;

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Bound;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::bool::any;
pub use self::bool::none;
pub use self::bool::False;
pub use self::bool::True;
pub use self::cond::re_cond;
pub use self::cond::Condition;
pub use self::cond::NeuCond;
pub use self::cond::NullCond;
pub use self::cond::RegexCond;
pub use self::equal::equal;
pub use self::equal::Equal;
pub use self::op_and::and;
pub use self::op_and::And;
pub use self::op_not::not;
pub use self::op_not::Not;
pub use self::op_one::NeureOne;
pub use self::op_one::NeureOneMore;
pub use self::op_or::or;
pub use self::op_or::Or;
pub use self::op_repeat::NeureRepeat;
pub use self::op_repeat::NeureRepeatRange;
pub use self::op_then::NeureThen;
pub use self::op_zero::NeureZeroMore;
pub use self::op_zero::NeureZeroOne;
pub use self::prefix::prefix;
pub use self::prefix::prefix_cnt;
pub use self::prefix::prefix_sync;
pub use self::prefix::prefix_sync_cnt;
pub use self::prefix::Prefix;
pub use self::prefix::PrefixSync;
pub use self::range::range;
pub use self::range::CRange;
pub use self::units::alphabetic;
pub use self::units::alphanumeric;
pub use self::units::ascii;
pub use self::units::ascii_alphabetic;
pub use self::units::ascii_alphanumeric;
pub use self::units::ascii_control;
pub use self::units::ascii_digit;
pub use self::units::ascii_graphic;
pub use self::units::ascii_hexdigit;
pub use self::units::ascii_lowercase;
pub use self::units::ascii_punctuation;
pub use self::units::ascii_uppercase;
pub use self::units::ascii_whitespace;
pub use self::units::control;
pub use self::units::digit;
pub use self::units::lowercase;
pub use self::units::numeric;
pub use self::units::uppercase;
pub use self::units::whitespace;
pub use self::units::wild;
pub use self::units::Alphabetic;
pub use self::units::Alphanumeric;
pub use self::units::Ascii;
pub use self::units::AsciiAlphabetic;
pub use self::units::AsciiAlphanumeric;
pub use self::units::AsciiControl;
pub use self::units::AsciiDigit;
pub use self::units::AsciiGraphic;
pub use self::units::AsciiHexDigit;
pub use self::units::AsciiLowercase;
pub use self::units::AsciiPunctuation;
pub use self::units::AsciiUppercase;
pub use self::units::AsciiWhiteSpace;
pub use self::units::Control;
pub use self::units::Digit;
pub use self::units::Lowercase;
pub use self::units::Numeric;
pub use self::units::Uppercase;
pub use self::units::WhiteSpace;
pub use self::units::Wild;

pub trait Neu<T: ?Sized> {
    fn is_match(&self, other: &T) -> bool;
}

impl<T, F> Neu<T> for F
where
    T: MayDebug,
    F: Fn(&T) -> bool,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = (self)(other);

        trace_retval!("F", other, ret)
    }
}

impl<T> Neu<T> for char
where
    T: MayDebug,
    Self: PartialEq<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("char", self, other, self == other)
    }
}

impl<T> Neu<T> for u8
where
    T: MayDebug,
    Self: PartialEq<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("u8", self, other, self == other)
    }
}

impl<T> Neu<T> for Box<dyn Neu<T>> {
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }
}

impl<U, T> Neu<T> for RefCell<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(&*self.borrow(), other)
    }
}

impl<U, T> Neu<T> for Cell<U>
where
    U: Neu<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(&self.get(), other)
    }
}

impl<U, T> Neu<T> for Mutex<U>
where
    U: Neu<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        let ret = self
            .lock()
            .expect("Oops ?! Can not unwrap mutex for regex ...");

        Neu::is_match(&*ret, other)
    }
}

impl<U, T> Neu<T> for Arc<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }
}

impl<T> Neu<T> for Arc<dyn Neu<T>> {
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }
}

impl<U, T> Neu<T> for Rc<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }
}

impl<T> Neu<T> for Rc<dyn Neu<T>> {
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }
}

///
/// Match any value in the array.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let arr = ['a', 'c', 'f', 'e'];
///   let mut ctx1 = CharsCtx::new("aaffeeeaccc");
///   let mut ctx2 = CharsCtx::new("acdde");
///
///   assert_eq!(ctx1.try_mat(&arr.repeat_range(2..6)).unwrap(), Span::new(0, 5));
///   assert_eq!(ctx2.try_mat(&arr.repeat_range(2..6)).unwrap(), Span::new(0, 2));
/// # }
/// ```
impl<const N: usize, T: PartialEq + MayDebug> Neu<T> for [T; N] {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("array", self, other, self.contains(other))
    }
}

///
/// Match any value in the array.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let arr = &[b'a', b'c', b'f', b'e'] as &[u8];
///   let arr = Neu2Re::repeat_range(arr, 2..6);
///   let mut ctx1 = BytesCtx::new(b"aaffeeeaccc");
///   let mut ctx2 = BytesCtx::new(b"acdde");
///
///   assert_eq!(ctx1.try_mat(&arr).unwrap(), Span::new(0, 5));
///   assert_eq!(ctx2.try_mat(&arr).unwrap(), Span::new(0, 2));
/// # }
/// ```
impl<T: PartialEq + MayDebug> Neu<T> for &'_ [T] {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("slice", self, other, self.contains(other))
    }
}

///
/// Match any value in the vector.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = vec!['a', 'b', 'c', 'd', 'e', 'f'];
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     let mut ctx = CharsCtx::new("0xbbb");
///
///     assert!(ctx.try_mat(&hex).is_err());
///
///     Ok(())
/// # }
/// ```
impl<T: PartialEq + MayDebug> Neu<T> for Vec<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("vector", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
///
/// ```
/// # use std::ops::Bound;
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = (Bound::Included(&'a'), Bound::Excluded(&'g'));
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<'a, T: 'a + ?Sized + PartialOrd + MayDebug> Neu<T> for (Bound<&'a T>, Bound<&'a T>) {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("bound(&T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use std::ops::Bound;
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = (Bound::Included('a'), Bound::Excluded('g'));
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for (Bound<T>, Bound<T>) {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("bound(T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = &'a' .. &'g';
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::Range<&T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range(&T)", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = 'a' .. 'g';
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::Range<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range(T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let from = &'a' ..;
///     let from = regex!((from){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&from)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeFrom<&T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_from(&T)", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let from = 'a' ..;
///     let from = regex!((from){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&from)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeFrom<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_from(T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let full = ..;
///     let full = regex!((full){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&full)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: ?Sized + PartialOrd + MayDebug> Neu<T> for std::ops::RangeFull {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_full", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = &'a' ..= &'f';
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeInclusive<&T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_inclusive(&T)", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let hex = 'a' ..= 'f';
///     let hex = regex!((hex){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeInclusive<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_inclusive(T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let to = .. &'g';
///     let to = regex!((to){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeTo<&T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_to(&T)", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let to = .. 'g';
///     let to = regex!((to){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeTo<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_to(T)", self, other, self.contains(other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let to = ..= &'f';
///     let to = regex!((to){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeToInclusive<&T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_to_inclusive(&T)", self, other, self.contains(&other))
    }
}

///
/// Match value in the range.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let to = ..= 'f';
///     let to = regex!((to){1,6});
///     let mut ctx = CharsCtx::new("aabbccgg");
///
///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
///
///     Ok(())
/// # }
/// ```
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeToInclusive<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_retval!("range_to_inclusive(T)", self, other, self.contains(other))
    }
}

pub trait NeuOp<C> {
    fn or<U>(self, unit: U) -> Or<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized;

    fn and<U>(self, unit: U) -> And<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized;

    fn not(self) -> Not<Self, C>
    where
        Self: Neu<C> + Sized;
}

impl<C, T> NeuOp<C> for T
where
    T: Neu<C>,
{
    fn or<U>(self, unit: U) -> Or<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized,
    {
        Or::new(self, unit)
    }

    fn and<U>(self, unit: U) -> And<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized,
    {
        And::new(self, unit)
    }

    fn not(self) -> Not<Self, C>
    where
        Self: Neu<C> + Sized,
    {
        Not::new(self)
    }
}

#[inline(always)]
pub(crate) fn length_of<'a, C: Context<'a>>(offset: usize, ctx: &C, next: Option<usize>) -> usize {
    let next_offset = next.unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

pub trait Neu2Re<'a, C>
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn then<R>(self, unit: R) -> NeureThen<C, Self, R, C::Item, NullCond>
    where
        R: Neu<C::Item>;

    fn repeat<const M: usize, const N: usize>(self) -> NeureRepeat<M, N, C, Self, NullCond>;

    fn repeat_times<const M: usize>(self) -> NeureRepeat<M, M, C, Self, NullCond>;

    fn repeat_from<const M: usize>(self) -> NeureRepeat<M, { usize::MAX }, C, Self, NullCond>;

    fn repeat_to<const N: usize>(self) -> NeureRepeat<0, N, C, Self, NullCond>;

    fn repeat_full(self) -> NeureRepeat<0, { usize::MAX }, C, Self, NullCond>;

    fn repeat_one(self) -> NeureOne<C, Self, C::Item, NullCond>;

    fn repeat_one_more(self) -> NeureOneMore<C, Self, C::Item, NullCond>;

    fn repeat_zero_one(self) -> NeureZeroOne<C, Self, C::Item, NullCond>;

    fn repeat_zero_more(self) -> NeureZeroMore<C, Self, C::Item, NullCond>;

    fn repeat_range(self, range: impl Into<CRange<usize>>) -> NeureRepeatRange<C, Self, NullCond>;
}

impl<'a, C, U> Neu2Re<'a, C> for U
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn then<R>(self, unit: R) -> NeureThen<C, Self, R, C::Item, NullCond>
    where
        R: Neu<C::Item>,
    {
        NeureThen::new(self, unit, NullCond)
    }

    fn repeat<const M: usize, const N: usize>(self) -> NeureRepeat<M, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_times<const M: usize>(self) -> NeureRepeat<M, M, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_from<const M: usize>(self) -> NeureRepeat<M, { usize::MAX }, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_to<const N: usize>(self) -> NeureRepeat<0, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_full(self) -> NeureRepeat<0, { usize::MAX }, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_one(self) -> NeureOne<C, Self, C::Item, NullCond> {
        NeureOne::new(self, NullCond)
    }

    fn repeat_one_more(self) -> NeureOneMore<C, Self, C::Item, NullCond> {
        NeureOneMore::new(self, NullCond)
    }

    fn repeat_zero_one(self) -> NeureZeroOne<C, Self, C::Item, NullCond> {
        NeureZeroOne::new(self, NullCond)
    }

    fn repeat_zero_more(self) -> NeureZeroMore<C, Self, C::Item, NullCond> {
        NeureZeroMore::new(self, NullCond)
    }

    fn repeat_range(self, range: impl Into<CRange<usize>>) -> NeureRepeatRange<C, Self, NullCond> {
        NeureRepeatRange::new(self, range.into(), NullCond)
    }
}
