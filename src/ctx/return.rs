use crate::ctx::Context;
use crate::err::Error;
use crate::ext::Extract;

use super::Ret;

#[derive(Debug, Clone, Copy)]
pub struct Return(usize, usize);

impl Ret for Return {
    fn count(&self) -> usize {
        self.0
    }

    fn length(&self) -> usize {
        self.1
    }

    fn is_zero(&self) -> bool {
        self.length() == 0 && self.count() == 0
    }

    fn new_from(ret: (usize, usize)) -> Self {
        Return(ret.0, ret.1)
    }

    fn add_assign(&mut self, other: Self) -> &mut Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }
}

impl<'a, C: Context<'a>> Extract<'a, C, Return> for Return {
    type Out<'b> = Return;

    type Error = Error;

    fn extract(_: &C, _: usize, ret: &Return) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ret))
    }
}
