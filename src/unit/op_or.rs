use std::marker::PhantomData;

use super::Unit;

#[derive(Debug, Clone, Default, Copy)]
pub struct Or<L, R, T>
where
    L: Unit<T>,
    R: Unit<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T> Or<L, R, T>
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

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<L, R, T> Unit<T> for Or<L, R, T>
where
    L: Unit<T>,
    R: Unit<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) || self.right.is_match(other)
    }
}
