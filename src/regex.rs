use std::{fmt::Debug, ops::RangeBounds};

pub trait Regex {
    fn mat(&self, dat: &char) -> bool;
}

impl<H> Regex for H
where
    H: Fn(&char) -> bool,
{
    fn mat(&self, dat: &char) -> bool {
        self(dat)
    }
}

impl Regex for char {
    fn mat(&self, dat: &char) -> bool {
        dat == self
    }
}

impl<const N: usize> Regex for [char; N] {
    fn mat(&self, dat: &char) -> bool {
        self.contains(dat)
    }
}

impl Regex for Vec<char> {
    fn mat(&self, dat: &char) -> bool {
        self.contains(dat)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Space;

impl Regex for Space {
    fn mat(&self, dat: &char) -> bool {
        dat.is_whitespace()
    }
}

impl Regex for (std::ops::Bound<char>, std::ops::Bound<char>) {
    fn mat(&self, dat: &char) -> bool {
        self.contains(dat)
    }
}

macro_rules! impl_regex_for_range {
    ($type:path) => {
        impl Regex for $type {
            fn mat(&self, dat: &char) -> bool {
                self.contains(dat)
            }
        }
    };
}

impl_regex_for_range!(std::ops::Range<char>);

impl_regex_for_range!(std::ops::RangeFrom<char>);

impl_regex_for_range!(std::ops::RangeFull);

impl_regex_for_range!(std::ops::RangeInclusive<char>);

impl_regex_for_range!(std::ops::RangeTo<char>);

impl_regex_for_range!(std::ops::RangeToInclusive<char>);

#[derive(Debug)]
pub struct Not<T>(T);

impl<T> Not<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> Regex for Not<T>
where
    T: Regex,
{
    fn mat(&self, dat: &char) -> bool {
        !self.0.mat(dat)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Dot;

impl Regex for Dot {
    fn mat(&self, dat: &char) -> bool {
        dat != &'\n'
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Digit;

impl Regex for Digit {
    fn mat(&self, dat: &char) -> bool {
        dat.is_ascii_digit()
    }
}
