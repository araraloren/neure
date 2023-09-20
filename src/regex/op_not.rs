use std::marker::PhantomData;

use super::Regex;

#[derive(Debug, Clone, Default, Copy)]
pub struct Not<R, T>
where
    R: Regex<T>,
{
    regex: R,
    marker: PhantomData<T>,
}

impl<R, T> Not<R, T>
where
    R: Regex<T>,
{
    pub fn new(regex: R) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<R, T> Regex<T> for Not<R, T>
where
    R: Regex<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.regex.is_match(other)
    }
}
