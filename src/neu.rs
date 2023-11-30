mod bool;
mod char;
mod cond;
mod equal;
mod may;
mod op_and;
mod op_not;
mod op_one;
mod op_or;
mod op_repeat;
mod op_zero;
mod range;

use crate::ctx::Context;
use crate::ctx::Ret;
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
pub use self::char::alphabetic;
pub use self::char::alphanumeric;
pub use self::char::ascii;
pub use self::char::ascii_alphabetic;
pub use self::char::ascii_alphanumeric;
pub use self::char::ascii_control;
pub use self::char::ascii_digit;
pub use self::char::ascii_graphic;
pub use self::char::ascii_hexdigit;
pub use self::char::ascii_lowercase;
pub use self::char::ascii_punctuation;
pub use self::char::ascii_uppercase;
pub use self::char::ascii_whitespace;
pub use self::char::control;
pub use self::char::digit;
pub use self::char::lowercase;
pub use self::char::numeric;
pub use self::char::uppercase;
pub use self::char::whitespace;
pub use self::char::wild;
pub use self::char::Alphabetic;
pub use self::char::Alphanumeric;
pub use self::char::Ascii;
pub use self::char::AsciiAlphabetic;
pub use self::char::AsciiAlphanumeric;
pub use self::char::AsciiControl;
pub use self::char::AsciiDigit;
pub use self::char::AsciiGraphic;
pub use self::char::AsciiHexDigit;
pub use self::char::AsciiLowercase;
pub use self::char::AsciiPunctuation;
pub use self::char::AsciiUppercase;
pub use self::char::AsciiWhiteSpace;
pub use self::char::Control;
pub use self::char::Digit;
pub use self::char::Lowercase;
pub use self::char::Numeric;
pub use self::char::Uppercase;
pub use self::char::WhiteSpace;
pub use self::char::Wild;
pub use self::cond::re_cond;
pub use self::cond::Condition;
pub use self::cond::NeuCond;
pub use self::cond::NullCond;
pub use self::cond::RegexCond;
pub use self::equal::equal;
pub use self::equal::Equal;
pub use self::may::MayUnit;
pub use self::op_and::And;
pub use self::op_not::Not;
pub use self::op_one::NeureOne;
pub use self::op_one::NeureOneMore;
pub use self::op_or::Or;
pub use self::op_repeat::NeureRepeat;
pub use self::op_repeat::NeureRepeatRange;
pub use self::op_zero::NeureZeroMore;
pub use self::op_zero::NeureZeroOne;
pub use self::range::range;
pub use self::range::CRange;

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
        trace_u!("func", "func", other, (self)(other))
    }
}

impl<T> Neu<T> for char
where
    T: MayDebug,
    Self: PartialEq<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("char", self, other, self == other)
    }
}

impl<T> Neu<T> for u8
where
    T: MayDebug,
    Self: PartialEq<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("u8", self, other, self == other)
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

impl<const N: usize, T: PartialEq + MayDebug> Neu<T> for [T; N] {
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
    ///   assert_eq!(ctx1.try_mat(&arr.repeat(2..6)).unwrap(), Span::new(0, 5));
    ///   assert_eq!(ctx2.try_mat(&arr.repeat(2..6)).unwrap(), Span::new(0, 2));
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("array", self, other, self.contains(other))
    }
}

impl<'a, T: PartialEq + MayDebug> Neu<T> for &'a [T] {
    ///
    /// Match any value in the array.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() {
    ///   let arr = &[b'a', b'c', b'f', b'e'] as &[u8];
    ///   let arr = UnitOp::repeat(arr, 2..6);
    ///   let mut ctx1 = BytesCtx::new(b"aaffeeeaccc");
    ///   let mut ctx2 = BytesCtx::new(b"acdde");
    ///
    ///   assert_eq!(ctx1.try_mat(&arr).unwrap(), Span::new(0, 5));
    ///   assert_eq!(ctx2.try_mat(&arr).unwrap(), Span::new(0, 2));
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("slice", self, other, self.contains(other))
    }
}

impl<T: PartialEq + MayDebug> Neu<T> for Vec<T> {
    ///
    /// Match any value in the vector.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    ///     let hex = re!((hex){1,6});
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
    ///
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("vector", self, other, self.contains(other))
    }
}

impl<'a, T: 'a + ?Sized + PartialOrd + MayDebug> Neu<T> for (Bound<&'a T>, Bound<&'a T>) {
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
    ///     color_eyre::install()?;
    ///     let hex = (Bound::Included(&'a'), Bound::Excluded(&'g'));
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("bound(&T)", self, other, self.contains(other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for (Bound<T>, Bound<T>) {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use std::ops::Bound;
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = (Bound::Included('a'), Bound::Excluded('g'));
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("bound(T)", self, other, self.contains(other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::Range<&T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = &'a' .. &'g';
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range(&T)", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::Range<T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a' .. 'g';
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range(T)", self, other, self.contains(other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeFrom<&T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let from = &'a' ..;
    ///     let from = re!((from){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&from)?, Span::new(0, 8));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_from(&T)", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeFrom<T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let from = 'a' ..;
    ///     let from = re!((from){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&from)?, Span::new(0, 8));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_from(T)", self, other, self.contains(other))
    }
}

impl<T: ?Sized + PartialOrd + MayDebug> Neu<T> for std::ops::RangeFull {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let full = ..;
    ///     let full = re!((full){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&full)?, Span::new(0, 8));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_full", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeInclusive<&T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = &'a' ..= &'f';
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_inclusive(&T)", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeInclusive<T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a' ..= 'f';
    ///     let hex = re!((hex){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_inclusive(T)", self, other, self.contains(other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeTo<&T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let to = .. &'g';
    ///     let to = re!((to){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_to(&T)", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeTo<T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let to = .. 'g';
    ///     let to = re!((to){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_to(T)", self, other, self.contains(other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeToInclusive<&T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let to = ..= &'f';
    ///     let to = re!((to){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_to_inclusive(&T)", self, other, self.contains(&other))
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeToInclusive<T> {
    ///
    /// Match value in the range.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let to = ..= 'f';
    ///     let to = re!((to){1,6});
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&to)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_u!("range_to_inclusive(T)", self, other, self.contains(other))
    }
}

pub trait NeuOp<C> {
    fn or<U>(self, regex: U) -> Or<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized;

    fn and<U>(self, regex: U) -> And<Self, U, C>
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
    ///
    /// Return true if the value matched any `Neu`.
    ///  
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let aorb = 'a'.or('b').repeat::<1, 2>();
    ///     let mut ctx = CharsCtx::new("abc");
    ///
    ///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
    ///
    ///     let aorb = re!(['a' 'b']{1,2});
    ///     let mut ctx = CharsCtx::new("abc");
    ///
    ///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn or<U>(self, unit: U) -> Or<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized,
    {
        Or::new(self, unit)
    }

    ///
    /// Return true if the value matched all `Neu`.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let large_than = |c: &char| *c > '7';
    ///     let digit = neu::digit(10).and(large_than).repeat::<1, 3>();
    ///     let mut ctx = CharsCtx::new("899");
    ///
    ///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 3));
    ///
    ///     let digit = re!((neu::digit(10).and(large_than)){1,3});
    ///     let mut ctx = CharsCtx::new("99c");
    ///
    ///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 2));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn and<U>(self, unit: U) -> And<Self, U, C>
    where
        U: Neu<C>,
        Self: Neu<C> + Sized,
    {
        And::new(self, unit)
    }

    ///
    /// Return true if the value not matched.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let not_digit = neu::digit(10).not().repeat::<1, 3>();
    ///     let mut ctx = CharsCtx::new("cc9");
    ///
    ///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 2));
    ///
    ///     let not_digit = re!((neu::digit(10).not()){1,3});
    ///     let mut ctx = CharsCtx::new("c99");
    ///
    ///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 1));
    ///
    ///     Ok(())
    /// # }
    /// ```
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

#[inline(always)]
pub(crate) fn ret_and_inc<'a, C: Context<'a>, R: Ret>(ctx: &mut C, count: usize, len: usize) -> R {
    let ret = R::from_ctx(ctx, (count, len));

    ctx.inc(len);
    ret
}

pub trait Neu2Re<'a, C>
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
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
    ///
    /// Repeat the match M ..= N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat::<1, 6>();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat<const M: usize, const N: usize>(self) -> NeureRepeat<M, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    ///
    /// Repeat the match M times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_times::<6>();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_times<const M: usize>(self) -> NeureRepeat<M, M, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    ///
    /// Repeat the match minimum M times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_from::<1>();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_from<const M: usize>(self) -> NeureRepeat<M, { usize::MAX }, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_to::<6>();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_to<const N: usize>(self) -> NeureRepeat<0, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_full();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_full(self) -> NeureRepeat<0, { usize::MAX }, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_one();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_one(self) -> NeureOne<C, Self, C::Item, NullCond> {
        NeureOne::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_one_more();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_one_more(self) -> NeureOneMore<C, Self, C::Item, NullCond> {
        NeureOneMore::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_zero_one();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 1));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_zero_one(self) -> NeureZeroOne<C, Self, C::Item, NullCond> {
        NeureZeroOne::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_zero_more();
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_zero_more(self) -> NeureZeroMore<C, Self, C::Item, NullCond> {
        NeureZeroMore::new(self, NullCond)
    }

    ///
    /// Repeat the match maxinmum N times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let hex = 'a'..'g';
    ///     let hex = hex.repeat_range(1..7);
    ///     let mut ctx = CharsCtx::new("aabbccgg");
    ///
    ///     assert_eq!(ctx.try_mat(&hex)?, Span::new(0, 6));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn repeat_range(self, range: impl Into<CRange<usize>>) -> NeureRepeatRange<C, Self, NullCond> {
        NeureRepeatRange::new(self, range.into(), NullCond)
    }
}

/// Match `if` and remember the result, then match `unit`
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let digit = neu::digit(10);
///     let hex = neu::digit(16);
///
///     let tests = [
///         ("99EF", Some(Span::new(0, 2))),
///         ("0x99EF", Some(Span::new(0, 6))),
///         ("099EF", Some(Span::new(0, 3))),
///         ("9899", Some(Span::new(0, 4))),
///         ("x99EF", None),
///     ];
///
///     for test in tests {
///         // `may` is not reuseable
///         let num = neu::may('0', neu::may('x', hex).or(neu::none())).or(digit);
///         let num = re!((num){1,6});
///         let mut ctx = CharsCtx::new(test.0);
///
///         if let Some(span) = test.1 {
///             assert_eq!(ctx.try_mat(&num)?, span, "at {}", test.0);
///         } else {
///             assert!(ctx.try_mat(&num).is_err());
///         }
///     }
///
///     Ok(())
/// # }
/// ```
pub fn may<T, I, U>(r#if: I, unit: U) -> MayUnit<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
{
    MayUnit::new(r#if, 1, unit)
}

pub fn may_count<T, I, U>(r#if: I, count: usize, unit: U) -> MayUnit<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
{
    MayUnit::new(r#if, count, unit)
}

macro_rules! trace_u {
    ($name:literal, $self:expr, $other:ident, $ret:expr) => {{
        let ret = $ret;
        $crate::trace_log!("{:?} -> u`{}-{:?}` -> {}", $other, $name, $self, ret);
        ret
    }};
}

pub(crate) use trace_u;
