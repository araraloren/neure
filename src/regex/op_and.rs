use std::marker::PhantomData;

use super::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T> And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            marker: PhantomData,
        }
    }
}

impl<L, R, T> Regex<T> for And<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) && self.right.is_match(other)
    }
}
