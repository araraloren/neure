use core::fmt::Debug;
use core::fmt::Display;
use core::ops::Bound;
use core::ops::RangeBounds;

use super::Neu;

use crate::MayDebug;

#[derive(Debug, Copy)]
pub struct CRange<T> {
    start: Bound<T>,

    end: Bound<T>,
}

impl<T> Default for CRange<T> {
    fn default() -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl<T: PartialOrd + MayDebug> core::ops::Not for CRange<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T: Clone> Clone for CRange<T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

impl<T: Display> Display for CRange<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.start {
            Bound::Included(start) => {
                write!(f, "{start} ..")?;
            }
            Bound::Excluded(start) => {
                write!(f, "{start} ..")?;
            }
            Bound::Unbounded => {
                write!(f, " ..",)?;
            }
        }
        match &self.end {
            Bound::Included(end) => {
                write!(f, "= {end}")
            }
            Bound::Excluded(end) => {
                write!(f, " {end}")
            }
            Bound::Unbounded => {
                write!(f, " ",)
            }
        }
    }
}

impl<T> CRange<T> {
    pub const fn new(start: Bound<T>, end: Bound<T>) -> Self {
        Self { start, end }
    }
}

impl CRange<usize> {
    pub fn is_empty(&self) -> bool {
        match &self.start {
            Bound::Included(start) => match &self.end {
                Bound::Included(end) => end < start,
                Bound::Excluded(end) => end <= start,
                Bound::Unbounded => start == &usize::MAX,
            },
            Bound::Excluded(start) => match &self.end {
                Bound::Included(end) => end <= start,
                Bound::Excluded(end) => *end <= *start + 1,
                Bound::Unbounded => false,
            },
            Bound::Unbounded => match &self.end {
                Bound::Included(_) => false,
                Bound::Excluded(end) => *end == 0,
                Bound::Unbounded => false,
            },
        }
    }
}

impl<T: Clone> CRange<T> {
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

impl<T> RangeBounds<T> for CRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.end.as_ref()
    }
}

impl<T: PartialOrd + MayDebug> Neu<T> for CRange<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = self.contains(other);

        crate::trace_retval!("CRange", self, other, ret)
    }
}

macro_rules! impl_copy_range {
    (&$ty:ty) => {
        impl From<&'_ $ty> for $crate::neu::CRange<usize> {
            fn from(value: &$ty) -> Self {
                let value = *value as usize;
                Self::from(value..=value)
            }
        }
    };
    ($ty:ty) => {
        impl From<$ty> for $crate::neu::CRange<usize> {
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
impl_copy_range!(&i8);
impl_copy_range!(&u8);
impl_copy_range!(&i16);
impl_copy_range!(&u16);
impl_copy_range!(&u32);
impl_copy_range!(&i32);
impl_copy_range!(&u64);
impl_copy_range!(&i64);
impl_copy_range!(&isize);
impl_copy_range!(&usize);

impl<T> From<(Bound<T>, Bound<T>)> for CRange<T> {
    fn from(value: (Bound<T>, Bound<T>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<core::ops::Range<T>> for CRange<T> {
    fn from(value: core::ops::Range<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Excluded(value.end))
    }
}

impl<T> From<core::ops::RangeFrom<T>> for CRange<T> {
    fn from(value: core::ops::RangeFrom<T>) -> Self {
        Self::new(Bound::Included(value.start), Bound::Unbounded)
    }
}

impl<T> From<core::ops::RangeFull> for CRange<T> {
    fn from(_: core::ops::RangeFull) -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
}

impl<T> From<core::ops::RangeInclusive<T>> for CRange<T> {
    fn from(value: core::ops::RangeInclusive<T>) -> Self {
        let (start, end) = value.into_inner();
        Self::new(Bound::Included(start), Bound::Included(end))
    }
}

impl<T> From<core::ops::RangeTo<T>> for CRange<T> {
    fn from(value: core::ops::RangeTo<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Excluded(value.end))
    }
}

impl<T> From<core::ops::RangeToInclusive<T>> for CRange<T> {
    fn from(value: core::ops::RangeToInclusive<T>) -> Self {
        Self::new(Bound::Unbounded, Bound::Included(value.end))
    }
}

///
/// Match a character within the specified range.
///
/// # Example
///
/// ```
/// use neure::prelude::*;
///
/// fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let letter = neu::range('a' ..= 'f');
///     let letter = letter.count::<3>();
///     let mut ctx = CharsCtx::new("adfwgh");
///
///     assert_eq!(ctx.try_mat(&letter)?, Span::new(0, 3));
///     assert!(ctx.try_mat(&letter).is_err());
///     Ok(())
/// }
/// ```
pub fn range<T>(val: impl Into<CRange<T>>) -> CRange<T> {
    val.into()
}
