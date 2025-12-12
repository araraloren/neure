use std::fmt::Debug;
use std::marker::PhantomData;

use super::Neu;

use crate::MayDebug;

pub struct True<T>(PhantomData<T>);

impl<T: MayDebug> std::ops::Not for True<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

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

impl<T: MayDebug> Neu<T> for True<T> {
    #[inline(always)]
    fn is_match(&self, _other: &T) -> bool {
        crate::trace_retval!("True", _other, true)
    }
}

/// Always return true.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let any = neu::any();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert_eq!(ctx.try_mat(&any.repeat_times::<6>()).unwrap(), Span::new(0, 6));
/// # }
/// ```
pub const fn any<T: MayDebug>() -> True<T> {
    True(PhantomData)
}

pub struct False<T>(PhantomData<T>);

impl<T: MayDebug> std::ops::Not for False<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T> Debug for False<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("False").field(&self.0).finish()
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

impl<T: MayDebug> Neu<T> for False<T> {
    #[inline(always)]
    fn is_match(&self, _other: &T) -> bool {
        crate::trace_retval!("False", _other, false)
    }
}

/// Always return false.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let none = neu::none();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert!(ctx.try_mat(&none.repeat_times::<6>()).is_err());
/// # }
/// ```
pub const fn none<T: MayDebug>() -> False<T> {
    False(PhantomData)
}
