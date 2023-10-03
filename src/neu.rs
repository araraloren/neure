mod bool;
mod char;
mod equal;
mod op_and;
mod op_if;
mod op_not;
mod op_one;
mod op_or;
mod op_repeat;
mod op_zero;
mod range;

use crate::ctx::Context;
use crate::ctx::Ret;
use crate::err::Error;
use crate::trace_log;
use crate::LogOrNot;

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Bound;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::bool::False;
pub use self::bool::True;
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
pub use self::equal::Equal;
pub use self::op_and::And;
pub use self::op_if::IfUnit;
pub use self::op_not::Not;
pub use self::op_one::NeureOne;
pub use self::op_one::NeureOneMore;
pub use self::op_or::Or;
pub use self::op_repeat::NeureRepeat;
pub use self::op_repeat::NeureRepeatRange;
pub use self::op_zero::NeureZeroMore;
pub use self::op_zero::NeureZeroOne;
pub use self::range::CRange;

pub use self::bool::any;
pub use self::bool::none;
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
pub use self::equal::equal;
pub use self::range::range;

pub trait Neu<T: ?Sized> {
    fn is_match(&self, other: &T) -> bool;
}

impl<T, F> Neu<T> for F
where
    T: LogOrNot,
    F: Fn(&T) -> bool,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match function with value ({:?})(in)", other);
        (self)(other)
    }
}

impl<T> Neu<T> for char
where
    T: LogOrNot,
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({}) with value ({:?})(in)", self, other);
        self == other
    }
}

impl<T> Neu<T> for u8
where
    T: LogOrNot,
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({}) with value ({:?})(in)", self, other);
        self == other
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

impl<const N: usize, T: PartialEq + LogOrNot> Neu<T> for [T; N] {
    ///
    /// Match any character in the array.
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
    ///   assert_eq!(ctx1.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 5));
    ///   assert_eq!(ctx2.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 2));
    /// # }
    /// ```
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match array({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<'a, T: PartialEq + LogOrNot> Neu<T> for &'a [T] {
    ///
    /// Match any character in the array.
    ///
    /// # Example
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() {
    ///   let arr = &[b'a', b'c', b'f', b'e'] as &[u8];
    ///   let arr = UnitOp::repeat(arr, 2..=5);
    ///   let mut ctx1 = BytesCtx::new(b"aaffeeeaccc");
    ///   let mut ctx2 = BytesCtx::new(b"acdde");
    ///
    ///   assert_eq!(ctx1.try_mat(&arr).unwrap(), Span::new(0, 5));
    ///   assert_eq!(ctx2.try_mat(&arr).unwrap(), Span::new(0, 2));
    /// # }
    /// ```
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match array({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialEq + LogOrNot> Neu<T> for Vec<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match vector({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<'a, T: 'a + ?Sized + PartialOrd + LogOrNot> Neu<T> for (Bound<&'a T>, Bound<&'a T>) {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for (Bound<T>, Bound<T>) {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::Range<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::Range<T> {
    fn is_match(&self, other: &T) -> bool {
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeFrom<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeFrom<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: ?Sized + PartialOrd + LogOrNot> Neu<T> for std::ops::RangeFull {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeInclusive<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeInclusive<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeTo<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeTo<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeToInclusive<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Neu<T> for std::ops::RangeToInclusive<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
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

    fn r#if<I, O>(self, r#if: I, otherwise: O) -> IfUnit<Self, I, O, C>
    where
        Self: Neu<C> + Sized,
        I: Neu<C>,
        O: Neu<C>;
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

    fn r#if<I, O>(self, r#if: I, otherwise: O) -> IfUnit<Self, I, O, C>
    where
        Self: Neu<C> + Sized,
        I: Neu<C>,
        O: Neu<C>,
    {
        IfUnit::new(self, r#if, otherwise)
    }
}

#[inline(always)]
pub(crate) fn length_of<'a, C: Context<'a>>(offset: usize, ctx: &C, next: Option<usize>) -> usize {
    let next_offset = next.unwrap_or(ctx.len() - ctx.offset());
    next_offset - offset
}

#[inline(always)]
pub(crate) fn inc_and_ret<'a, C: Context<'a>, R: Ret>(ctx: &mut C, count: usize, len: usize) -> R {
    let ret = R::from(ctx, (count, len));

    ctx.inc(len);
    ret
}

pub trait NeuCond<'a, C>
where
    C: Context<'a>,
{
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error>;
}

impl<'a, C, F> NeuCond<'a, C> for F
where
    C: Context<'a>,
    F: Fn(&C, &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error> {
        (self)(ctx, item)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullCond;

impl<'a, C> NeuCond<'a, C> for NullCond
where
    C: Context<'a>,
{
    fn check(&self, _: &C, _: &(usize, C::Item)) -> Result<bool, Error> {
        Ok(true)
    }
}

pub trait Neu2Re<'a, C>
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn repeat<const M: usize, const N: usize>(self) -> NeureRepeat<'a, M, N, C, Self, NullCond>;

    fn repeat_from<const M: usize>(self) -> NeureRepeat<'a, M, { usize::MAX }, C, Self, NullCond>;

    fn repeat_to<const N: usize>(self) -> NeureRepeat<'a, 0, N, C, Self, NullCond>;

    fn repeat_full(self) -> NeureRepeat<'a, 0, { usize::MAX }, C, Self, NullCond>;

    fn repeat_one(self) -> NeureOne<C, Self, C::Item, NullCond>;

    fn repeat_one_more(self) -> NeureOneMore<C, Self, C::Item, NullCond>;

    fn repeat_zero_one(self) -> NeureZeroOne<C, Self, C::Item, NullCond>;

    fn repeat_zero_more(self) -> NeureZeroMore<C, Self, C::Item, NullCond>;

    fn repeat_range(
        self,
        range: impl Into<CRange<usize>>,
    ) -> NeureRepeatRange<'a, C, Self, NullCond>;
}

impl<'a, C, U> Neu2Re<'a, C> for U
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn repeat<const M: usize, const N: usize>(self) -> NeureRepeat<'a, M, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_from<const M: usize>(self) -> NeureRepeat<'a, M, { usize::MAX }, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_to<const N: usize>(self) -> NeureRepeat<'a, 0, N, C, Self, NullCond> {
        NeureRepeat::new(self, NullCond)
    }

    fn repeat_full(self) -> NeureRepeat<'a, 0, { usize::MAX }, C, Self, NullCond> {
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

    fn repeat_range(
        self,
        range: impl Into<CRange<usize>>,
    ) -> NeureRepeatRange<'a, C, Self, NullCond> {
        NeureRepeatRange::new(self, range.into(), NullCond)
    }
}
