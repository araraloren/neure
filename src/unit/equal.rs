use super::Unit;

use crate::trace_log;
use crate::LogOrNot;

#[derive(Debug, Clone, Default, Copy)]
pub struct Equal<T> {
    val: T,
}

impl<T> Equal<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: PartialEq + LogOrNot> Unit<T> for Equal<T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({:?}) with value ({:?})(in)", self.val, other);
        &self.val == other
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
pub fn equal<T: PartialEq + LogOrNot>(val: T) -> Equal<T> {
    Equal::new(val)
}
