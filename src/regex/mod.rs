mod array;
mod bool;
mod char;
mod equal;
mod exts;
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

pub use self::array::Array;
pub use self::bool::False;
pub use self::bool::True;
pub use self::char::AsciiSpace;
pub use self::char::Digit;
pub use self::char::HexDigit;
pub use self::char::Space;
pub use self::char::Wild;
pub use self::equal::Equal;
pub use self::exts::And;
pub use self::exts::Not;
pub use self::exts::Or;
pub use self::exts::RegexExt;
pub use self::exts::Repeat;
pub use self::range::CopyRange;

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
