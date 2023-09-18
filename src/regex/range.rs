use std::fmt::Debug;
use std::ops::Bound;
use std::ops::RangeBounds;

use super::Regex;

use crate::trace_log;
use crate::LogOrNot;

#[derive(Debug, Clone, Copy)]
pub struct CopyRange<T> {
    start: Bound<T>,

    end: Bound<T>,
}

impl<T> CopyRange<T> {
    pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
        Self { start, end }
    }
}

impl<T: Clone> CopyRange<T> {
    pub fn clone_from(range: impl RangeBounds<T>) -> Self {
        Self {
            start: match range.start_bound() {
                Bound::Included(start) => Bound::Included(start.clone()),
                Bound::Excluded(start) => Bound::Excluded(start.clone()),
                Bound::Unbounded => Bound::Unbounded,
            },

            end: match range.start_bound() {
                Bound::Included(start) => Bound::Included(start.clone()),
                Bound::Excluded(start) => Bound::Excluded(start.clone()),
                Bound::Unbounded => Bound::Unbounded,
            },
        }
    }
}

impl<T> RangeBounds<T> for CopyRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end.as_ref()
    }
}

impl<T: PartialOrd + LogOrNot> Regex<T> for CopyRange<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({:?}) with value ({:?})(in)", self, other);
        self.contains(other)
    }
}

macro_rules! impl_copy_range {
    ($ty:ty) => {
        impl From<$ty> for CopyRange<usize> {
            fn from(value: $ty) -> Self {
                let value = value as usize;
                Self::from(value..=value)
            }
        }
    };
}

impl_copy_range!(i8);
impl_copy_range!(u8);
impl_copy_range!(i16);
impl_copy_range!(u16);
impl_copy_range!(u32);
impl_copy_range!(i32);
impl_copy_range!(u64);
impl_copy_range!(i64);
impl_copy_range!(isize);
impl_copy_range!(usize);

impl<T> From<(Bound<T>, Bound<T>)> for CopyRange<T> {
    fn from(value: (Bound<T>, Bound<T>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<std::ops::Range<T>> for CopyRange<T> {
    fn from(value: std::ops::Range<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Excluded(value.end))
    }
}

impl<T> From<std::ops::RangeFrom<T>> for CopyRange<T> {
    fn from(value: std::ops::RangeFrom<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Unbounded)
    }
}

impl<T> From<std::ops::RangeFull> for CopyRange<T> {
    fn from(_: std::ops::RangeFull) -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for CopyRange<T> {
    fn from(value: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = value.into_inner();
        Self::new(Bound::Included(start), Bound::Included(end))
    }
}

impl<T> From<std::ops::RangeTo<T>> for CopyRange<T> {
    fn from(value: std::ops::RangeTo<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Excluded(value.end))
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for CopyRange<T> {
    fn from(value: std::ops::RangeToInclusive<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Included(value.end))
    }
}
