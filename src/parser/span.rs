use super::Ret;

use crate::err::Error;
use crate::parser::Context;
use crate::regex::Extract;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

impl Span {
    pub fn new(beg: usize, len: usize) -> Self {
        Self { beg, len }
    }
}

impl Ret for Span {
    fn fst(&self) -> usize {
        self.beg
    }

    fn snd(&self) -> usize {
        self.len
    }

    fn is_zero(&self) -> bool {
        self.len == 0
    }

    fn add_assign(&mut self, other: Self) -> &mut Self {
        self.len += other.len;
        self
    }

    fn from<'a, C>(ctx: &mut C, info: (usize, usize)) -> Self
    where
        C: Context<'a>,
    {
        Span {
            beg: ctx.offset(),
            len: info.1,
        }
    }
}

impl<'a, C: Context<'a>> Extract<'a, C, Span> for Span {
    type Out<'b> = Span;

    type Error = Error;

    fn extract(_: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ret))
    }
}