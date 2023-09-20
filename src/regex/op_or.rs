use std::marker::PhantomData;

use super::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    left: L,
    regex: R,
    marker: PhantomData<T>,
}

impl<L, R, T> Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            regex: right,
            marker: PhantomData,
        }
    }
}

impl<L, R, T> Regex<T> for Or<L, R, T>
where
    L: Regex<T>,
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        self.left.is_match(other) || self.regex.is_match(other)
    }
}
