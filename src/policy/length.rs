use std::ops::Add;
use std::ops::AddAssign;

use super::Ret;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(usize);

impl Ret for Length {
    fn count(&self) -> usize {
        0
    }

    fn length(&self) -> usize {
        self.0
    }

    fn is_zero(&self) -> bool {
        self.length() == 0
    }

    fn new_from(ret: (usize, usize)) -> Self {
        Self(ret.1)
    }
}

impl Add<Length> for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length(self.0 + rhs.0)
    }
}

impl AddAssign<Length> for Length {
    fn add_assign(&mut self, rhs: Length) {
        self.0 += rhs.length();
    }
}
