use std::fmt::Debug;
use std::marker::PhantomData;

use super::Unit;

use crate::trace_log;
use crate::LogOrNot;

pub struct True<T>(PhantomData<T>);

impl<T> Debug for True<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("True").field(&self.0).finish()
    }
}

impl<T> Default for True<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for True<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for True<T> {}

impl<T> True<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: LogOrNot> Unit<T> for True<T> {
    fn is_match(&self, _other: &T) -> bool {
        trace_log!("always true: ({:?})(in)", _other);
        true
    }
}

/// Always return true.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// # use neure::unit::*;
/// #
/// # fn main() {
///   let any = any();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert_eq!(ctx.try_mat(&any.repeat(6)).unwrap(), Span::new(0, 6));
/// # }
/// ```
pub fn any<T: LogOrNot>() -> True<T> {
    True::default()
}

pub struct False<T>(PhantomData<T>);

impl<T> Debug for False<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("True").field(&self.0).finish()
    }
}

impl<T> Default for False<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for False<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for False<T> {}

impl<T> False<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: LogOrNot> Unit<T> for False<T> {
    fn is_match(&self, _other: &T) -> bool {
        trace_log!("always false: ({:?})(in)", _other);
        false
    }
}

/// Always return false.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// # use neure::unit::*;
/// #
/// # fn main() {
///   let none = none();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert!(ctx.try_mat(&none.repeat(6)).is_err());
/// # }
/// ```
pub fn none<T: LogOrNot>() -> False<T> {
    False::default()
}
