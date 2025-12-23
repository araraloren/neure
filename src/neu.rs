mod bool;
mod cond;
mod equal;
mod once;
mod op_and;
mod op_not;
mod op_or;
mod opt;
mod prefix;
mod range;
mod then;
mod times;
mod units;

use crate::MayDebug;
use crate::ctx::Context;
use crate::trace_retval;

use std::cell::Cell;
use std::cell::RefCell;
use std::ops::Bound;
use std::ops::RangeBounds;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::bool::Always;
pub use self::bool::Never;
pub use self::bool::always;
pub use self::bool::never;
pub use self::cond::Condition;
pub use self::cond::EmptyCond;
pub use self::cond::NeuCond;
pub use self::cond::RegexCond;
pub use self::cond::regex_cond;
pub use self::equal::Equal;
pub use self::equal::equal;
pub use self::once::Many1;
pub use self::once::Once;
pub use self::op_and::And;
pub use self::op_and::and;
pub use self::op_not::Not;
pub use self::op_not::not;
pub use self::op_or::Or;
pub use self::op_or::or;
pub use self::opt::Many0;
pub use self::opt::Opt;
pub use self::prefix::Prefix;
pub use self::prefix::PrefixSync;
pub use self::prefix::prefix;
pub use self::prefix::prefix_cnt;
pub use self::prefix::prefix_sync;
pub use self::prefix::prefix_sync_cnt;
pub use self::range::CRange;
pub use self::range::range;
pub use self::then::NeureThen;
pub use self::times::Between;
pub use self::times::Times;
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
pub use self::units::Word;
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
pub use self::units::word;

pub trait Neu<T> {
    fn is_match(&self, other: &T) -> bool;

    fn min_length(&self) -> usize {
        1
    }
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

    fn min_length(&self) -> usize {
        self.len_utf8()
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

    fn min_length(&self) -> usize {
        self.as_ref().min_length()
    }
}

impl<U, T> Neu<T> for RefCell<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(&*self.borrow(), other)
    }

    fn min_length(&self) -> usize {
        self.borrow().min_length()
    }
}

impl<U, T> Neu<T> for Cell<U>
where
    U: Neu<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(&self.get(), other)
    }

    fn min_length(&self) -> usize {
        self.get().min_length()
    }
}

impl<U, T> Neu<T> for Mutex<U>
where
    U: Neu<T> + Copy,
{
    fn is_match(&self, other: &T) -> bool {
        let ret = self
            .lock()
            .expect("Oops ?! Can not unwrap mutex for Neu ...");

        Neu::is_match(&*ret, other)
    }

    fn min_length(&self) -> usize {
        self.lock()
            .expect("Oops ?! Can not unwrap mutex for Neu ...")
            .min_length()
    }
}

impl<U, T> Neu<T> for Arc<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }

    fn min_length(&self) -> usize {
        self.as_ref().min_length()
    }
}

impl<T> Neu<T> for Arc<dyn Neu<T>> {
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }

    fn min_length(&self) -> usize {
        self.as_ref().min_length()
    }
}

impl<U, T> Neu<T> for Rc<U>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }

    fn min_length(&self) -> usize {
        self.as_ref().min_length()
    }
}

impl<T> Neu<T> for Rc<dyn Neu<T>> {
    fn is_match(&self, other: &T) -> bool {
        Neu::is_match(self.as_ref(), other)
    }

    fn min_length(&self) -> usize {
        self.as_ref().min_length()
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
///   assert_eq!(ctx1.try_mat(&arr.times(2..6)).unwrap(), Span::new(0, 5));
///   assert_eq!(ctx2.try_mat(&arr.times(2..6)).unwrap(), Span::new(0, 2));
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
///   let arr = NeuIntoRegexOps::times(arr, 2..6);
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
impl<'a, T: 'a + PartialOrd + MayDebug> Neu<T> for (Bound<&'a T>, Bound<&'a T>) {
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
impl<T: PartialOrd + MayDebug> Neu<T> for std::ops::RangeFull {
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

pub trait NeuOp<T> {
    fn or<U>(self, unit: U) -> Or<Self, U, T>
    where
        U: Neu<T>,
        Self: Neu<T> + Sized;

    fn and<U>(self, unit: U) -> And<Self, U, T>
    where
        U: Neu<T>,
        Self: Neu<T> + Sized;

    fn not(self) -> Not<Self, T>
    where
        Self: Neu<T> + Sized;
}

impl<T, V> NeuOp<T> for V
where
    V: Neu<T>,
{
    fn or<U>(self, unit: U) -> Or<Self, U, T>
    where
        U: Neu<T>,
        Self: Neu<T> + Sized,
    {
        Or::new(self, unit)
    }

    fn and<U>(self, unit: U) -> And<Self, U, T>
    where
        U: Neu<T>,
        Self: Neu<T> + Sized,
    {
        And::new(self, unit)
    }

    fn not(self) -> Not<Self, T>
    where
        Self: Neu<T> + Sized,
    {
        Not::new(self)
    }
}

// beg: None => match nothing
// end: None => match all
#[inline(always)]
pub(crate) fn calc_length(beg: Option<usize>, end: Option<usize>, remaining_len: usize) -> usize {
    beg.map(|v| end.unwrap_or(remaining_len) - v)
        .unwrap_or_default()
}

// https://github.com/rust-lang/rust-analyzer/issues/21315
const COUNT_MAX: usize = usize::MAX;

pub trait NeuIntoRegexOps<'a, C>
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn then<R>(self, unit: R) -> NeureThen<C, Self, R, C::Item, EmptyCond>
    where
        R: Neu<C::Item>;

    fn between<const M: usize, const N: usize>(self) -> Between<M, N, C, Self>;

    fn count<const M: usize>(self) -> Between<M, M, C, Self> {
        self.between::<M, M>()
    }

    fn at_least<const M: usize>(self) -> Between<M, COUNT_MAX, C, Self> {
        self.between::<M, COUNT_MAX>()
    }

    fn at_most<const N: usize>(self) -> Between<0, N, C, Self> {
        self.between::<0, N>()
    }

    fn once(self) -> Once<C, Self, C::Item>;

    fn many1(self) -> Many1<C, Self, C::Item>;

    fn opt(self) -> Opt<C, Self, C::Item>;

    fn many0(self) -> Many0<C, Self, C::Item>;

    fn times(self, range: impl Into<CRange<usize>>) -> Times<C, Self>;
}

impl<'a, C, U> NeuIntoRegexOps<'a, C> for U
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn then<R>(self, unit: R) -> NeureThen<C, Self, R, C::Item, EmptyCond>
    where
        R: Neu<C::Item>,
    {
        NeureThen::new(self, unit, EmptyCond)
    }

    fn between<const M: usize, const N: usize>(self) -> Between<M, N, C, Self> {
        Between::new(self, EmptyCond)
    }

    fn once(self) -> Once<C, Self, C::Item> {
        Once::new(self, EmptyCond)
    }

    fn many1(self) -> Many1<C, Self, C::Item> {
        Many1::new(self, EmptyCond)
    }

    fn opt(self) -> Opt<C, Self, C::Item> {
        Opt::new(self, EmptyCond)
    }

    fn many0(self) -> Many0<C, Self, C::Item> {
        Many0::new(self, EmptyCond)
    }

    fn times(self, range: impl Into<CRange<usize>>) -> Times<C, Self> {
        let range = range.into();

        debug_assert!(!range.is_empty(), "Invalid CRange for Times");
        Times::new(self, range, EmptyCond)
    }
}
