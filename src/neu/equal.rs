use super::Neu;

use crate::MayDebug;

#[derive(Debug, Clone, Default, Copy)]
pub struct Equal<T> {
    val: T,
}

impl<T: PartialEq + MayDebug> core::ops::Not for Equal<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T> Equal<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: PartialEq + MayDebug> Neu<T> for Equal<T> {
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let val = &self.val;
        let ret = val == other;

        crate::trace_retval!("Equal", val, other, ret)
    }
}

///
/// Match a character equal to given value.
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let letter = equal('A');
///     let letter = letter.count::<3>();
///     let mut ctx = CharsCtx::new("AAAabcd");
///
///     assert_eq!(ctx.try_mat(&letter)?, Span::new(0, 3));
///     assert!(ctx.try_mat(&letter).is_err());
///     Ok(())
/// }
/// ```
pub const fn equal<T: PartialEq + MayDebug>(val: T) -> Equal<T> {
    Equal { val }
}
