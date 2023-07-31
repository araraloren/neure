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

pub mod utils {
    use std::ops::RangeBounds;

    use crate::{err::Error, Context};

    pub fn char(ch: char) -> impl Fn(&char) -> bool {
        move |dat: &char| dat == &ch
    }

    pub fn array<const N: usize>(chars: [char; N]) -> impl Fn(&char) -> bool {
        move |dat: &char| chars.contains(dat)
    }

    pub fn vector(chars: Vec<char>) -> impl Fn(&char) -> bool {
        move |dat: &char| chars.contains(dat)
    }

    pub fn not(func: impl Fn(&char) -> bool) -> impl Fn(&char) -> bool {
        move |dat: &char| !func(dat)
    }

    pub fn range(bound: impl RangeBounds<char>) -> impl Fn(&char) -> bool {
        move |dat: &char| bound.contains(dat)
    }

    pub fn any() -> impl Fn(&char) -> bool {
        |_: &char| true
    }

    pub fn space() -> impl Fn(&char) -> bool {
        |dat: &char| dat.is_whitespace()
    }

    pub fn digit() -> impl Fn(&char) -> bool {
        |dat: &char| dat.is_ascii_digit()
    }

    pub fn start<T: Context>() -> impl Fn(&mut T) -> Result<usize, Error> {
        |_: &mut T| Ok(0)
    }

    pub fn end<T: Context>() -> impl Fn(&mut T) -> Result<usize, Error> {
        |dat: &mut T| {
            if dat.peek().map_err(Into::into)?.len() > 0 {
                Err(Error::Match("123".to_owned()))
            } else {
                Ok(0)
            }
        }
    }
}
