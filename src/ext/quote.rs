use super::term::LazyTerm;
use super::term::NonLazyTerm;
use super::then::LazyPattern;
use super::NonLazyPattern;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::err::Error;

pub struct LazyQuote<'a, Ctx, Pl, Pr> {
    pattern_l: Pl,
    pattern_r: Pr,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Pl, Pr> LazyQuote<'a, Ctx, Pl, Pr> {
    pub fn new(ctx: &'a mut Ctx, pattern_l: Pl, pattern_r: Pr) -> Self {
        Self {
            ctx,
            pattern_l,
            pattern_r,
        }
    }
}

impl<'a, 'b, Ctx, Pl, Pr> LazyQuote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn pattern<P>(self, pattern: P) -> LazyPattern<'a, Ctx, P, Pl, Pr>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        LazyPattern::new(self.ctx, self.pattern_l, self.pattern_r, pattern)
    }
}

impl<'a, 'b, Ctx, Pl, Pr> LazyQuote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn quote_once(
        self,
        left: impl Pattern<Ctx, Ret = Ctx::Ret>,
        right: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> LazyQuote<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, impl Pattern<Ctx, Ret = Ctx::Ret>>
    {
        let next_l = left;
        let next_r = right;
        let left = self.pattern_l;
        let right = self.pattern_r;

        LazyQuote::new(
            self.ctx,
            move |ctx: &mut Ctx| super::and(left, next_l).try_parse(ctx),
            move |ctx: &mut Ctx| super::and(next_r, right).try_parse(ctx),
        )
    }
}

impl<'a, 'b, Ctx, Pl, Pr> LazyQuote<'a, Ctx, Pl, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pl: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn quote(
        self,
        left: impl Pattern<Ctx, Ret = Ctx::Ret>,
        right: impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    ) -> LazyQuote<
        'a,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    > {
        let next_l = left;
        let next_r = right;
        let left = self.pattern_l;
        let right = self.pattern_r;

        LazyQuote::new(
            self.ctx,
            move |ctx: &mut Ctx| super::and(left, next_l).try_parse(ctx),
            move |ctx: &mut Ctx| super::and(next_r, right).try_parse(ctx),
        )
    }

    pub fn term<S>(self, sep: S) -> LazyTerm<'a, Ctx, S, Pl, Pr>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    pub fn term_opt<S>(self, sep: S, optional: bool) -> LazyTerm<'a, Ctx, S, Pl, Pr>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        LazyTerm::new(
            self.ctx,
            Some(self.pattern_l),
            Some(self.pattern_r),
            sep,
            optional,
        )
    }
}

pub struct NonLazyQuote<'a, Ctx: Policy<Ctx>, Pr> {
    pattern_r: Pr,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx: Policy<Ctx>, Pr> NonLazyQuote<'a, Ctx, Pr> {
    pub fn new(ctx: &'a mut Ctx, pattern_r: Pr) -> Self {
        Self { ctx, pattern_r }
    }
}

impl<'a, 'b, Ctx, Pr> NonLazyQuote<'a, Ctx, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn pat<P>(self, pattern: P) -> Result<NonLazyPattern<'a, Ctx, Pr>, Error>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        let beg = self.ctx.offset();
        let ret = self.ctx.try_mat(pattern);
        let post = self.pattern_r;

        Ok(NonLazyPattern::new(self.ctx, post, beg, ret))
    }
}

impl<'a, 'b, Ctx, Pr> NonLazyQuote<'a, Ctx, Pr>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn quote(
        self,
        left: impl Pattern<Ctx, Ret = Ctx::Ret>,
        right: impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    ) -> Result<NonLazyQuote<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret> + Clone>, Error> {
        self.ctx.try_mat(left)?;

        Ok(NonLazyQuote::new(self.ctx, move |ctx: &mut Ctx| {
            super::and(right, self.pattern_r).try_parse(ctx)
        }))
    }

    pub fn term<S>(self, sep: S) -> Result<NonLazyTerm<'a, Ctx, S, Pr>, Error>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        self.term_opt(sep, true)
    }

    pub fn term_opt<S>(self, sep: S, optional: bool) -> Result<NonLazyTerm<'a, Ctx, S, Pr>, Error>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        let post = Some(self.pattern_r);

        Ok(NonLazyTerm::new(self.ctx, post, sep, optional))
    }
}
