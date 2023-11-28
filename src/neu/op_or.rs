use std::marker::PhantomData;

use super::Neu;

#[derive(Debug, Default, Copy)]
pub struct Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    left: L,
    right: R,
    marker: PhantomData<T>,
}

impl<L, R, T> Clone for Or<L, R, T>
where
    L: Neu<T> + Clone,
    R: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<L, R, T> Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
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

impl<L, R, T> Neu<T> for Or<L, R, T>
where
    L: Neu<T>,
    R: Neu<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let ret = self.left.is_match(other) || self.right.is_match(other);

        crate::trace_log!("neu logical `or` -> {ret}");
        ret
    }
}
