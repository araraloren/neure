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

pub fn equal<T: PartialEq + LogOrNot>(val: T) -> Equal<T> {
    Equal::new(val)
}
