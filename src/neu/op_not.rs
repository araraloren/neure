use std::marker::PhantomData;

use super::Neu;

///
/// Return true if the value not matched.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let not_digit = neu::digit(10).not().repeat::<1, 3>();
///     let mut ctx = CharsCtx::new("cc9");
///
///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 2));
///
///     let not_digit = re!((neu::digit(10).not()){1,3});
///     let mut ctx = CharsCtx::new("c99");
///
///     assert_eq!(ctx.try_mat(&not_digit)?, Span::new(0, 1));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct Not<U, T>
where
    U: Neu<T>,
{
    unit: U,
    marker: PhantomData<T>,
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
    U: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = !self.unit.is_match(other);

        crate::trace_log!("neu logical `not` -> {ret}");
        ret
    }
}

///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let item = neu::not(u8::is_ascii_uppercase);
///     let str = item.repeat_range(6..);
///     let mut ctx = BytesCtx::new(br#"abcedfgABCEE"#);
///
///     assert_eq!(ctx.try_mat(&str)?, Span::new(0, 7));
///     Ok(())
/// # }
/// ```
pub fn not<T, U: Neu<T>>(unit: U) -> Not<U, T> {
    Not::new(unit)
}
