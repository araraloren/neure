use std::fmt::Debug;
use std::marker::PhantomData;

use super::Neu;

use crate::MayDebug;

pub struct Always<T>(PhantomData<T>);

impl<T: MayDebug> std::ops::Not for Always<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T> Debug for Always<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Always").field(&self.0).finish()
    }
}

impl<T> Default for Always<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for Always<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Always<T> {}

impl<T> Always<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: MayDebug> Neu<T> for Always<T> {
    #[inline(always)]
    fn is_match(&self, _other: &T) -> bool {
        crate::trace_retval!("Always", _other, true)
    }

    fn min_length(&self) -> usize {
        0
    }
}

/// Always return true.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let always = neu::always();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert_eq!(ctx.try_mat(&always.count::<6>()).unwrap(), Span::new(0, 6));
/// # }
/// ```
pub const fn always<T: MayDebug>() -> Always<T> {
    Always(PhantomData)
}

pub struct Never<T>(PhantomData<T>);

impl<T: MayDebug> std::ops::Not for Never<T> {
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T> Debug for Never<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Never").field(&self.0).finish()
    }
}

impl<T> Default for Never<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for Never<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Never<T> {}

impl<T> Never<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: MayDebug> Neu<T> for Never<T> {
    #[inline(always)]
    fn is_match(&self, _other: &T) -> bool {
        crate::trace_retval!("Never", _other, false)
    }

    fn min_length(&self) -> usize {
        0
    }
}

/// Always return false.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() {
///   let never = neu::never();
///   let mut ctx = CharsCtx::new("abc%$#&");
///
///   assert!(ctx.try_mat(&never.count::<6>()).is_err());
/// # }
/// ```
pub const fn never<T: MayDebug>() -> Never<T> {
    Never(PhantomData)
}
