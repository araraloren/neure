use std::fmt::Debug;
use std::marker::PhantomData;

use crate::MayDebug;

use super::Neu;

///
/// Return true if the given value not matches `U`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let not_digit = neu::digit(10).not().between::<1, 3>();
///     let mut ctx = CharsCtx::new("cc9");
///
///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 2));
///
///     let not_digit = regex!((neu::digit(10).not()){1,3});
///     let mut ctx = CharsCtx::new("c99");
///
///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 1));
///
///     Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct Not<U, T>
where
    U: Neu<T>,
{
    unit: U,
    marker: PhantomData<T>,
}

impl<U, T> Debug for Not<U, T>
where
    U: Neu<T> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Not").field("unit", &self.unit).finish()
    }
}

impl<U, T> Default for Not<U, T>
where
    U: Neu<T> + Default,
{
    fn default() -> Self {
        Self {
            unit: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<U, T> Clone for Not<U, T>
where
    U: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            marker: self.marker,
        }
    }
}

impl<U, T> Not<U, T>
where
    U: Neu<T>,
{
    pub fn new(unit: U) -> Self {
        Self {
            unit,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<U, T> Neu<T> for Not<U, T>
where
    T: MayDebug,
    U: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = !self.unit.is_match(other);

        crate::trace_retval!("Not", other, ret)
    }

    fn min_length(&self) -> usize {
        self.unit.min_length()
    }
}

///
/// Return true if the given value not matches `U`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let item = neu::not(u8::is_ascii_uppercase);
///     let str = item.times(6..);
///     let mut ctx = BytesCtx::new(br#"abcedfgABCEE"#);
///
///     assert_eq!(ctx.try_mat(&str)?, Span::new(0, 7));
///     Ok(())
/// # }
/// ```
pub fn not<T, U: Neu<T>>(unit: U) -> Not<U, T> {
    Not::new(unit)
}
