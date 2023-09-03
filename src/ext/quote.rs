use super::mat::MatchThen;
use super::CtxGuard;
use super::term::Term;
use crate::Context;
use crate::MatchPolicy;
use crate::Parser;

pub struct Quote<'a, C, L, R> {
    ctx: &'a mut C,
    left: L,
    right: R,
}

impl<'a, C, L, R> Quote<'a, C, L, R> {
    pub fn new(ctx: &'a mut C, left: L, right: R) -> Self {
        Self { ctx, left, right }
    }
}

impl<'a, C, L, R> Quote<'a, C, L, R>
where
    C: MatchPolicy + Context,
    L: Parser<C, Ret = C::Ret>,
    R: Parser<C, Ret = C::Ret>,
{
    pub fn mat<P>(self, parser: P) -> MatchThen<'a, C, P, L, R>
    where
        P: Parser<C, Ret = C::Ret>,
    {
        MatchThen::new(self.ctx, self.left, self.right, parser)
    }

    pub fn quote(
        self,
        left: impl Parser<C, Ret = C::Ret>,
        right: impl Parser<C, Ret = C::Ret>,
    ) -> Quote<'a, C, impl Parser<C, Ret = C::Ret>, impl Parser<C, Ret = C::Ret>> {
        let next_left = left;
        let next_right = right;
        let left = self.left;
        let right = self.right;

        Quote::new(
            self.ctx,
            move |ctx: &mut C| {
                let mut guard = CtxGuard::new(ctx);
                let mut ret = guard.try_mat(left)?;

                ret += guard.try_mat(next_left)?;
                Ok(ret)
            },
            move |ctx: &mut C| {
                let mut guard = CtxGuard::new(ctx);
                let mut ret = guard.try_mat(next_right)?;

                ret += guard.try_mat(right)?;
                Ok(ret)
            },
        )
    }

    // pub fn term<S>(self, sep: S) -> Term<'a, C, impl Parser<C, Ret = C::Ret>> {

    // }
}
