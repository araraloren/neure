use std::marker::PhantomData;

use super::Neu;

///
/// Return true if the value matched any `Neu`.
///  
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let aorb = 'a'.or('b').repeat::<1, 2>();
///     let mut ctx = CharsCtx::new("abc");
///
///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
///
///     let aorb = re!(['a' 'b']{1,2});
///     let mut ctx = CharsCtx::new("abc");
///
///     assert_eq!(ctx.try_mat(&aorb)?, Span::new(0, 2));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
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
    pub fn new(left: L, right: R) -> Self {
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

impl<L, R, T> Neu<T> for Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = self.left.is_match(other) || self.right.is_match(other);

        crate::trace_log!("neu logical `or` -> {ret}");
        ret
    }
}

///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let re = u8::is_ascii_hexdigit.or(b'g'.or(b'G')).repeat_times::<4>();
///     let re = re.padded(b"0x");
///
///     assert_eq!(BytesCtx::new(b"0xcfag").ctor(&re)?, b"cfag");
///     Ok(())
/// # }
/// ```
pub fn or<T, L: Neu<T>, R: Neu<T>>(left: L, right: R) -> Or<L, R, T> {
    Or::new(left, right)
}
