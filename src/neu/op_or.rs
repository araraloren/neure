use core::fmt::Debug;
use core::marker::PhantomData;

use crate::MayDebug;

use super::Neu;

///
/// Return true if the value matches `L` or `R`.
///  
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let aorb = 'a'.or('b').between::<1, 2>();
///     let mut ctx = CharsCtx::new("abc");
///
///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
///
///     let aorb = regex!(['a' 'b']{1,2});
///     let mut ctx = CharsCtx::new("abc");
///
///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T: MayDebug> core::ops::Not for Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<L, R, T> Debug for Or<L, R, T>
where
    L: Neu<T> + Debug,
    R: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Or")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<L, R, T> Default for Or<L, R, T>
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

impl<L, R, T> Clone for Or<L, R, T>
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

impl<L, R, T> Or<L, R, T>
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

    pub const fn left(&self) -> &L {
        &self.left
    }

    pub const fn right(&self) -> &R {
        &self.right
    }

    pub const fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub const fn right_mut(&mut self) -> &mut R {
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

impl<L, R, T> Neu<T> for Or<L, R, T>
where
    T: MayDebug,
    L: Neu<T>,
    R: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = self.left.is_match(other) || self.right.is_match(other);

        crate::trace_retval!("Or", other, ret)
    }

    fn min_length(&self) -> usize {
        self.left.min_length().min(self.right.min_length())
    }
}

///
/// Return true if the value matches `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let re = u8::is_ascii_hexdigit.or(b'g'.or(b'G')).count::<4>();
///     let re = re.prefix(b"0x");
///
///     assert_eq!(BytesCtx::new(b"0xcfag").ctor(&re)?, b"cfag");
///     Ok(())
/// # }
/// ```
pub const fn or<T, L: Neu<T>, R: Neu<T>>(left: L, right: R) -> Or<L, R, T> {
    Or::new(left, right)
}
