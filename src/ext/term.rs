use super::quote::LazyQuote;
use super::quote::NonLazyQuote;
use super::then::LazyPattern;
use super::CtxGuard;
use super::NonLazyPattern;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::err::Error;

pub struct LazyTermIter<'a, Ctx, Pa, Sep, Pr, Po> {
    pattern: Pa,
    term: LazyTerm<'a, Ctx, Sep, Pr, Po>,
}

impl<'a, Ctx, Pa, Sep, Pr, Po> LazyTermIter<'a, Ctx, Pa, Sep, Pr, Po> {
    pub fn new(term: LazyTerm<'a, Ctx, Sep, Pr, Po>, pattern: Pa) -> Self {
        Self { term, pattern }
    }
}

impl<'a, 'b, Ctx, Pa, Sep, Pr, Po> LazyTermIter<'a, Ctx, Pa, Sep, Pr, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Pa: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn next(
        &mut self,
    ) -> LazyPattern<
        '_,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
    > {
        self.term.next_pat(self.pattern.clone())
    }
}

pub struct LazyTerm<'a, Ctx, Sep, Pr, Po> {
    sep: Sep,
    pre: Option<Pr>,
    post: Option<Po>,
    optional: bool,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Sep, Pr, Po> LazyTerm<'a, Ctx, Sep, Pr, Po> {
    pub fn new(
        ctx: &'a mut Ctx,
        pre: Option<Pr>,
        post: Option<Po>,
        sep: Sep,
        optional: bool,
    ) -> Self {
        Self {
            ctx,
            sep,
            pre,
            post,
            optional,
        }
    }
}

impl<'a, 'b, Ctx, Sep, Pr, Po> LazyTerm<'a, Ctx, Sep, Pr, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn iter<P>(self, pattern: P) -> LazyTermIter<'a, Ctx, P, Sep, Pr, Po>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        LazyTermIter::new(self, pattern)
    }

    fn take_pre_pattern(&mut self) -> impl Pattern<Ctx, Ret = Ctx::Ret> {
        let pre = self.pre.take();
        move |ctx: &mut Ctx| {
            pre.and_then(|v| Some(v.try_parse(ctx)))
                .unwrap_or(Ok(<Ctx::Ret>::new_from((0, 0))))
        }
    }

    pub fn next_pat<P>(
        &mut self,
        pattern: P,
    ) -> LazyPattern<
        '_,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
    >
    where
        P: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        let pre = self.take_pre_pattern();
        let post = self.post.clone();
        let sep = self.sep.clone();
        let optional = self.optional;
        let post = move |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(sep);

            if let Some(post) = post {
                if let Ok(ret) = &mut ret {
                    if let Ok(post_ret) = guard.try_mat(post) {
                        ret.add_assign(post_ret);
                    }
                } else if optional {
                    return guard.try_mat(post);
                }
            }
            ret
        };

        LazyPattern::new(self.ctx, pre, post, pattern)
    }

    pub fn next_quote<L, R>(
        &mut self,
        left: L,
        right: R,
    ) -> LazyQuote<
        '_,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    >
    where
        L: Pattern<Ctx, Ret = Ctx::Ret>,
        R: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        let pre = self.take_pre_pattern();
        let left = move |ctx: &mut Ctx| super::and(pre, left).try_parse(ctx);
        let sep = self.sep.clone();
        let optional = self.optional;
        let sep = move |ctx: &mut Ctx| {
            sep.try_parse(ctx).or_else(|e| {
                if optional {
                    Ok(<Ctx::Ret>::new_from((0, 0)))
                } else {
                    Err(e)
                }
            })
        };
        let right = |ctx: &mut Ctx| super::and(right, sep).try_parse(ctx);

        LazyQuote::new(self.ctx, left, right)
    }

    pub fn next_term<S>(
        &mut self,
        sep: S,
    ) -> LazyTerm<
        '_,
        Ctx,
        S,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    >
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        self.next_term_opt(sep, true)
    }

    pub fn next_term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> LazyTerm<
        '_,
        Ctx,
        S,
        impl Pattern<Ctx, Ret = Ctx::Ret>,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    >
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        let pre = self.take_pre_pattern();
        let prev_sep = self.sep.clone();
        let prev_sep = move |ctx: &mut Ctx| {
            prev_sep.try_parse(ctx).or_else(|e| {
                if optional {
                    Ok(<Ctx::Ret>::new_from((0, 0)))
                } else {
                    Err(e)
                }
            })
        };

        LazyTerm::new(self.ctx, Some(pre), Some(prev_sep), sep, optional)
    }
}

pub struct NonLazyTermIter<'a, Ctx: Policy<Ctx>, Pa, Sep, Po> {
    pattern: Pa,
    term: NonLazyTerm<'a, Ctx, Sep, Po>,
}

impl<'a, Ctx: Policy<Ctx>, Pa, Sep, Po> NonLazyTermIter<'a, Ctx, Pa, Sep, Po> {
    pub fn new(term: NonLazyTerm<'a, Ctx, Sep, Po>, pattern: Pa) -> Self {
        Self { term, pattern }
    }
}

impl<'a, 'b, Ctx, Pa, Sep, Po> NonLazyTermIter<'a, Ctx, Pa, Sep, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Pa: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn next(
        &mut self,
    ) -> Result<NonLazyPattern<'_, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>>, Error> {
        self.term.next_pat(self.pattern.clone())
    }
}

pub struct NonLazyTerm<'a, Ctx: Policy<Ctx>, Sep, Po> {
    sep: Sep,
    post: Option<Po>,
    opt: bool,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx: Policy<Ctx>, Sep, Po> NonLazyTerm<'a, Ctx, Sep, Po> {
    pub fn new(ctx: &'a mut Ctx, post: Option<Po>, sep: Sep, optional: bool) -> Self {
        Self {
            ctx,
            sep,
            post,
            opt: optional,
        }
    }
}

impl<'a, 'b, Ctx, Sep, Po> NonLazyTerm<'a, Ctx, Sep, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn iter<P>(self, pattern: P) -> NonLazyTermIter<'a, Ctx, P, Sep, Po>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        NonLazyTermIter::new(self, pattern)
    }

    pub fn next_pat<P>(
        &mut self,
        pattern: P,
    ) -> Result<NonLazyPattern<'_, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>>, Error>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        let beg = self.ctx.offset();
        let ret = self.ctx.try_mat(pattern);
        let post = self.post.clone();
        let sep = self.sep.clone();
        let opt = self.opt;
        let post = move |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(sep);

            if let Some(post) = post {
                if let Ok(ret) = &mut ret {
                    if let Ok(post_ret) = guard.try_mat(post) {
                        ret.add_assign(post_ret);
                    }
                } else if opt {
                    return guard.try_mat(post);
                }
            }
            ret
        };

        Ok(NonLazyPattern::new(self.ctx, post, beg, ret))
    }

    pub fn next_quote<L, R>(
        &mut self,
        left: L,
        right: R,
    ) -> Result<NonLazyQuote<'_, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret> + Clone>, Error>
    where
        L: Pattern<Ctx, Ret = Ctx::Ret>,
        R: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        self.ctx.try_mat(left)?;

        let sep = self.sep.clone();
        let optional = self.opt;
        let sep = move |ctx: &mut Ctx| {
            sep.try_parse(ctx).or_else(|e| {
                if optional {
                    Ok(<Ctx::Ret>::new_from((0, 0)))
                } else {
                    Err(e)
                }
            })
        };
        let right = |ctx: &mut Ctx| super::and(right, sep).try_parse(ctx);

        Ok(NonLazyQuote::new(self.ctx, right))
    }

    pub fn next_term<S>(
        &mut self,
        sep: S,
    ) -> Result<NonLazyTerm<'_, Ctx, S, impl Pattern<Ctx, Ret = Ctx::Ret> + Clone>, Error>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        self.next_term_opt(sep, true)
    }

    pub fn next_term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> Result<NonLazyTerm<'_, Ctx, S, impl Pattern<Ctx, Ret = Ctx::Ret> + Clone>, Error>
    where
        S: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    {
        let prev_sep = self.sep.clone();
        let prev_sep = move |ctx: &mut Ctx| {
            prev_sep.try_parse(ctx).or_else(|e| {
                if optional {
                    Ok(<Ctx::Ret>::new_from((0, 0)))
                } else {
                    Err(e)
                }
            })
        };
        let post = Some(prev_sep);

        Ok(NonLazyTerm::new(self.ctx, post, sep, optional))
    }
}
