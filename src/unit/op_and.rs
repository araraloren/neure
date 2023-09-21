use std::marker::PhantomData;

use super::Unit;

#[derive(Debug, Clone, Default, Copy)]
pub struct And<L, R, T>
where
    L: Unit<T>,
    R: Unit<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T> And<L, R, T>
where
    L: Unit<T>,
    R: Unit<T>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            marker: PhantomData,
        }
    }
}

impl<L, R, T> Unit<T> for And<L, R, T>
where
    L: Unit<T>,
    R: Unit<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) && self.right.is_match(other)
    }
}
