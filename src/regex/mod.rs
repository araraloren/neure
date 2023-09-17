mod equal;
pub mod ext;
pub mod func;
pub mod r#macro;
mod range;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::equal::Equal;
pub use self::ext::RegexExtension;
pub use self::func::*;
pub use self::range::Range;

pub trait Regex<T> {
    fn is_match(&self, other: &T) -> bool;
}

impl<T, F> Regex<T> for F
where
    F: Fn(&T) -> bool,
{
    fn is_match(&self, other: &T) -> bool {
        (self)(other)
    }
}

impl<T> Regex<T> for char
where
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

impl<'a, T> Regex<T> for &'a str
where
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

impl<T> Regex<T> for u8
where
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

impl<'a, T> Regex<T> for &'a [u8]
where
    Self: PartialEq<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self == other
    }
}

impl<'a, T> Regex<T> for Box<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<'a, R, T> Regex<T> for RefCell<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(&*self.borrow(), other)
    }
}

impl<'a, R, T> Regex<T> for Cell<R>
where
    R: Regex<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(&self.get(), other)
    }
}

impl<'a, R, T> Regex<T> for Mutex<R>
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

impl<'a, R, T> Regex<T> for Arc<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<'a, T> Regex<T> for Arc<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<'a, R, T> Regex<T> for Rc<R>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}

impl<'a, T> Regex<T> for Rc<dyn Regex<T>> {
    fn is_match(&self, other: &T) -> bool {
        Regex::is_match(self.as_ref(), other)
    }
}
