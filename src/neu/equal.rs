use super::trace_u;
use super::Neu;

use crate::MayDebug;

#[derive(Debug, Clone, Default, Copy)]
pub struct Equal<T> {
    val: T,
}

impl<T> Equal<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: PartialEq + MayDebug> Neu<T> for Equal<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_u!("equal", self.val, other, &self.val == other)
    }
}

///
/// Match a character equal to given value.
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use unit::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let letter = equal('A');
///     let letter = letter.repeat(3);
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
