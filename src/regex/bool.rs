use std::marker::PhantomData;

use super::Regex;

use crate::trace_log;
use crate::LogOrNot;

#[derive(Debug, Clone, Default, Copy)]
pub struct True<T>(PhantomData<T>);

impl<T> True<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: LogOrNot> Regex<T> for True<T> {
    fn is_match(&self, _other: &T) -> bool {
        trace_log!("always true: ({:?})(in)", _other);
        true
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct False<T>(PhantomData<T>);

impl<T> False<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: LogOrNot> Regex<T> for False<T> {
    fn is_match(&self, _other: &T) -> bool {
        trace_log!("always false: ({:?})(in)", _other);
        false
    }
}
