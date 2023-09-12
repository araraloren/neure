use crate::ctx::Context;
use crate::err::Error;
use crate::ext::Extract;

use super::Ret;

#[derive(Debug, Clone, Copy)]
pub struct Return(usize, usize);

impl Ret for Return {
    fn fst(&self) -> usize {
        self.0
    }

    fn snd(&self) -> usize {
        self.1
    }

    fn is_zero(&self) -> bool {
        self.snd() == 0 && self.fst() == 0
    }

    fn add_assign(&mut self, other: Self) -> &mut Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }

    fn from<'a, C>(_: &mut C, info: (usize, usize)) -> Self
    where
        C: Context<'a>,
    {
        Return(info.0, info.1)
    }
}

impl<'a, C: Context<'a>> Extract<'a, C, Return> for Return {
    type Out<'b> = Return;

    type Error = Error;

    fn extract(_: &C, ret: &Return) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ret))
    }
}
