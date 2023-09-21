use std::marker::PhantomData;

use super::Unit;

#[derive(Debug, Clone, Default, Copy)]
pub struct Not<R, T>
where
    R: Unit<T>,
{
    regex: R,
    marker: PhantomData<T>,
}

impl<R, T> Not<R, T>
where
    R: Unit<T>,
{
    pub fn new(regex: R) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<R, T> Unit<T> for Not<R, T>
where
    R: Unit<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.regex.is_match(other)
    }
}
