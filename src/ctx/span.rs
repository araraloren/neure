use std::fmt::Display;

use crate::ctx::Context;
use crate::err::Error;
use crate::re::Extract;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

impl Span {
    pub fn new(beg: usize, len: usize) -> Self {
        Self { beg, len }
    }

    pub fn beg(&self) -> usize {
        self.beg
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_zero(&self) -> bool {
        self.len == 0
    }

    pub fn add_assign(&mut self, other: Self) -> &mut Self {
        self.len += other.len + other.beg - (self.beg + self.len);
        self
    }
}

impl<'a, C: Context<'a>> Extract<'a, C> for Span {
    type Out<'b> = Span;

    type Error = Error;

    fn extract(_: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ret))
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{beg: {}, len: {}}}", self.beg, self.len)
    }
}
