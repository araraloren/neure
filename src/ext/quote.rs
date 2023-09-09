use super::term::Term;
use super::then::Then;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;

pub struct Quote<'a, Ctx, Pl, Pr> {
    ctx: &'a mut Ctx,
    pattern_l: Pl,
    pattern_r: Pr,
}

impl<'a, Ctx, Pl, Pr> Quote<'a, Ctx, Pl, Pr> {
    pub fn new(ctx: &'a mut Ctx, pattern_l: Pl, pattern_r: Pr) -> Self {
        Self {
            ctx,
            pattern_l,
            pattern_r,
        }
    }
}

impl<'a, 'b, Ctx, Pl, Pr> Quote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn pattern<P>(self, pattern: P) -> Then<'a, Ctx, P, Pl, Pr>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        Then::new(self.ctx, self.pattern_l, self.pattern_r, pattern)
    }
}

impl<'a, 'b, Ctx, Pl, Pr> Quote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn quote_once(
        self,
        left: impl Pattern<Ctx, Ret = Ctx::Ret>,
        right: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> Quote<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, impl Pattern<Ctx, Ret = Ctx::Ret>> {
        let next_l = left;
        let next_r = right;
        let left = self.pattern_l;
        let right = self.pattern_r;

        Quote::new(
            self.ctx,
            move |ctx: &mut Ctx| super::and(left, next_l).try_parse(ctx),
            move |ctx: &mut Ctx| super::and(next_r, right).try_parse(ctx),
        )
    }
}

impl<'a, 'b, Ctx, Pl, Pr> Quote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn quote(
        self,
        left: impl Pattern<Ctx, Ret = Ctx::Ret>,
        right: impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    ) -> Quote<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, impl Pattern<Ctx, Ret = Ctx::Ret> + Clone>
    {
        let next_l = left;
        let next_r = right;
        let left = self.pattern_l;
        let right = self.pattern_r;

        Quote::new(
            self.ctx,
            move |ctx: &mut Ctx| super::and(left, next_l).try_parse(ctx),
            move |ctx: &mut Ctx| super::and(next_r, right).try_parse(ctx),
        )
    }

    pub fn term<S>(self, sep: S, optional: bool) -> Term<'a, Ctx, S, Pl, Pr>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        Term::new(
            self.ctx,
            Some(self.pattern_l),
            Some(self.pattern_r),
            sep,
            optional,
        )
    }
}
