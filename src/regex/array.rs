use super::Regex;

use crate::trace_log;
use crate::LogOrNot;

#[derive(Debug, Clone, Copy)]
pub struct Array<const N: usize, T> {
    vals: [T; N],
}

impl<const N: usize, T> Array<N, T> {
    pub fn new(vals: [T; N]) -> Self {
        Self { vals }
    }
}

impl<const N: usize, T: PartialEq + LogOrNot> Regex<T> for Array<N, T> {
    fn is_match(&self, other: &T) -> bool {
        trace_log!("match ({:?})([..]) with value ({:?})(in)", self.vals, other);
        self.vals.contains(other)
    }
}
