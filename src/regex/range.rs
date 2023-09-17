use std::ops::Bound;
use std::ops::RangeBounds;

use super::Regex;

use crate::trace_log;
use crate::PartialOrd;

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    start: Bound<T>,

    end: Bound<T>,
}

impl<T> Range<T> {
    pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
        Self { start, end }
    }
}

impl<T: Clone> Range<T> {
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

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end.as_ref()
    }
}

impl<T: PartialOrd> Regex<T> for Range<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({}) with value ({})(in)", self, other);
        self.contains(other)
    }
}

impl<T> From<(Bound<T>, Bound<T>)> for Range<T> {
    fn from(value: (Bound<T>, Bound<T>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<std::ops::Range<T>> for Range<T> {
    fn from(value: std::ops::Range<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Excluded(value.end))
    }
}

impl<T> From<std::ops::RangeFrom<T>> for Range<T> {
    fn from(value: std::ops::RangeFrom<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Unbounded)
    }
}

impl<T> From<std::ops::RangeFull> for Range<T> {
    fn from(_: std::ops::RangeFull) -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for Range<T> {
    fn from(value: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = value.into_inner();
        Self::new(Bound::Included(start), Bound::Included(end))
    }
}

impl<T> From<std::ops::RangeTo<T>> for Range<T> {
    fn from(value: std::ops::RangeTo<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Excluded(value.end))
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for Range<T> {
    fn from(value: std::ops::RangeToInclusive<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Included(value.end))
    }
}
