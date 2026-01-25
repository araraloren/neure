use core::fmt::Debug;
use core::marker::PhantomData;

use super::Neu;

use crate::MayDebug;

///
/// Return true if the value matches both `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let large_than = |c: &char| *c > '7';
///     let digit = neu::digit(10).and(large_than).between::<1, 3>();
///     let mut ctx = CharsCtx::new("899");
///
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 3));
///
///     let digit = regex!((neu::digit(10).and(large_than)){1,3});
///     let mut ctx = CharsCtx::new("99c");
///
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct And<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T: MayDebug> core::ops::Not for And<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<L, R, T> Debug for And<L, R, T>
where
    L: Neu<T> + Debug,
    R: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("And")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<L, R, T> Default for And<L, R, T>
where
    L: Neu<T> + Default,
    R: Neu<T> + Default,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<L, R, T> Clone for And<L, R, T>
where
    L: Neu<T> + Clone,
    R: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<L, R, T> And<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    pub const fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<L, R, T> Neu<T> for And<L, R, T>
where
    T: MayDebug,
    L: Neu<T>,
    R: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = self.left.is_match(other) && self.right.is_match(other);

        crate::trace_retval!("And", other, ret)
    }

    fn min_length(&self) -> usize {
        self.left.min_length().max(self.right.min_length())
    }
}

///
/// Return true if the value matches both `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     const BEG: u8 = b'a' - 1;
///     const END: u8 = b'z' + 1;
///
///     let char = neu::and(|a: &u8| a > &BEG, |a: &u8| a < &END);
///     let str = char.times(6..);
///     let mut ctx = BytesCtx::new(br#"abcedfgABCEE"#);
///
///     assert_eq!(ctx.try_mat(&str)?, Span::new(0, 7));
///     Ok(())
/// # }
/// ```
pub fn and<T, L: Neu<T>, R: Neu<T>>(left: L, right: R) -> And<L, R, T> {
    And::new(left, right)
}
