mod bool;
mod char;
mod equal;
mod op;
mod range;

pub mod r#macro;

use crate::trace_log;
use crate::LogOrNot;

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Bound;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::ctx::Span;

pub use self::bool::False;
pub use self::bool::True;
pub use self::char::AsciiSpace;
pub use self::char::Digit;
pub use self::char::HexDigit;
pub use self::char::Space;
pub use self::char::Wild;
pub use self::equal::Equal;
pub use self::op::And;
pub use self::op::Not;
pub use self::op::Or;
pub use self::op::Repeat;
pub use self::range::CopyRange;

pub use self::bool::any;
pub use self::bool::none;

pub trait Regex<T: ?Sized> {
    fn is_match(&self, other: &T) -> bool;
}

impl<T, F> Regex<T> for F
where
    T: LogOrNot,
    F: Fn(&T) -> bool,
{
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match function with value ({:?})(in)", other);
        (self)(other)
    }
}

impl<T> Regex<T> for char
where
    T: LogOrNot,
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({}) with value ({:?})(in)", self, other);
        self == other
    }
}

impl<T> Regex<T> for u8
where
    T: LogOrNot,
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({}) with value ({:?})(in)", self, other);
        self == other
    }
}

impl<T> Regex<T> for Box<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<R, T> Regex<T> for RefCell<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(&*self.borrow(), other)
    }
}

impl<R, T> Regex<T> for Cell<R>
where
    R: Regex<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(&self.get(), other)
    }
}

impl<R, T> Regex<T> for Mutex<R>
where
    R: Regex<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        let ret = self
            .lock()
            .expect("Oops ?! Can not unwrap mutex for regex ...");

        Regex::is_match(&*ret, other)
    }
}

impl<R, T> Regex<T> for Arc<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<T> Regex<T> for Arc<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<R, T> Regex<T> for Rc<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<T> Regex<T> for Rc<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<const N: usize, T: PartialEq + LogOrNot> Regex<T> for [T; N] {
    ///
    /// Match any character in the array.
    ///
    /// # Example
    /// ```
    /// use neure::prelude::*;
    ///
    /// fn main() {
    ///     let arr = ['a', 'c', 'f', 'e'];
    ///     let mut ctx1 = CharsCtx::new("aaffeeeaccc");
    ///     let mut ctx2 = CharsCtx::new("acdde");
    ///
    ///     assert_eq!(ctx1.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 5));
    ///     assert_eq!(ctx2.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 2));
    /// }
    /// ```
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match array({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<'a, T: PartialEq + LogOrNot> Regex<T> for &'a [T] {
    ///
    /// Match any character in the array.
    ///
    /// # Example
    /// ```
    /// use neure::prelude::*;
    ///
    /// fn main() {
    ///     let arr = &[b'a', b'c', b'f', b'e'] as &[u8];
    ///     let arr = RegexExt::repeat(arr, 2..=5);
    ///     let mut ctx1 = BytesCtx::new(b"aaffeeeaccc");
    ///     let mut ctx2 = BytesCtx::new(b"acdde");
    ///
    ///     assert_eq!(ctx1.try_mat(&arr).unwrap(), Span::new(0, 5));
    ///     assert_eq!(ctx2.try_mat(&arr).unwrap(), Span::new(0, 2));
    /// }
    /// ```
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match array({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialEq + LogOrNot> Regex<T> for Vec<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match vector({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<'a, T: 'a + ?Sized + PartialOrd + LogOrNot> Regex<T> for (Bound<&'a T>, Bound<&'a T>) {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for (Bound<T>, Bound<T>) {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::Range<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::Range<T> {
    fn is_match(&self, other: &T) -> bool {
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeFrom<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeFrom<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: ?Sized + PartialOrd + LogOrNot> Regex<T> for std::ops::RangeFull {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeInclusive<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeInclusive<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeTo<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeTo<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeToInclusive<&T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(&other)
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for std::ops::RangeToInclusive<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match range({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

pub trait RegexExt<T> {
    fn or<R>(self, regex: R) -> Or<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized;

    fn and<R>(self, regex: R) -> And<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized;

    fn not(self) -> Not<Self, T>
    where
        Self: Regex<T> + Sized;

    fn repeat<R>(self, range: R) -> Repeat<Self, CopyRange<usize>, Span, T>
    where
        Self: Regex<T> + Sized,
        R: Into<CopyRange<usize>>;
}

impl<T, Re> RegexExt<T> for Re
where
    Re: Regex<T>,
{
    fn or<R>(self, regex: R) -> Or<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized,
    {
        Or::new(self, regex)
    }

    fn and<R>(self, regex: R) -> And<Self, R, T>
    where
        R: Regex<T>,
        Self: Regex<T> + Sized,
    {
        And::new(self, regex)
    }

    fn not(self) -> Not<Self, T>
    where
        Self: Regex<T> + Sized,
    {
        Not::new(self)
    }

    fn repeat<R>(self, range: R) -> Repeat<Self, CopyRange<usize>, Span, T>
    where
        Self: Regex<T> + Sized,
        R: Into<CopyRange<usize>>,
    {
        Repeat::new(self, range.into())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_array() {
        let arr = ['a', 'c', 'f', 'e'];
        let mut ctx1 = CharsCtx::new("aaffeeeaccc");
        let mut ctx2 = CharsCtx::new("acdde");

        assert_eq!(ctx1.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 5));
        assert_eq!(ctx2.try_mat(&arr.repeat(2..=5)).unwrap(), Span::new(0, 2));
    }
}
