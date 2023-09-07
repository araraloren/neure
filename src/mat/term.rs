use super::quote::Quote;
use super::then::Then;
use super::CtxGuard;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::policy::Policy;
use crate::policy::Ret;

pub struct TermIter<'a, Ctx, Pa, Sep, Pr, Po> {
    pattern: Pa,
    sep: Sep,
    pre: Option<Pr>,
    post: Option<Po>,
    optional: bool,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Pa, Sep, Pr, Po> TermIter<'a, Ctx, Pa, Sep, Pr, Po> {
    pub fn new(
        ctx: &'a mut Ctx,
        pre: Option<Pr>,
        post: Option<Po>,
        pattern: Pa,
        sep: Sep,
        optional: bool,
    ) -> Self {
        Self {
            ctx,
            pattern,
            sep,
            pre,
            post,
            optional,
        }
    }
}

impl<'a, 'd, Ctx, Pa, Sep, Pr, Po> TermIter<'a, Ctx, Pa, Sep, Pr, Po>
where
    Ctx: Context<'d> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Pa: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn next<'b: 'c, 'c>(
        &'b mut self,
    ) -> Then<
        'c,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
    > {
        let pre = self.pre.take();
        let pre = |ctx: &mut Ctx| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            }
        };
        let post = self.post.clone();
        let sep = self.sep.clone();
        let post = |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(sep);

            if let Some(post) = post {
                if let Ok(ret) = &mut ret {
                    if let Ok(post_ret) = guard.try_mat(post) {
                        *ret += post_ret;
                    }
                } else if self.optional {
                    return guard.try_mat(post);
                }
            }
            ret
        };

        Then::new(self.ctx, pre, post, self.pattern.clone())
    }
}

pub struct Term<'a, Ctx, Sep, Pr, Po> {
    sep: Sep,
    pre: Option<Pr>,
    post: Option<Po>,
    optional: bool,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Sep, Pr, Po> Term<'a, Ctx, Sep, Pr, Po> {
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

impl<'a, 'd, Ctx, Sep, Pr, Po> Term<'a, Ctx, Sep, Pr, Po>
where
    Ctx: Context<'d> + Policy<Ctx>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
    Sep: Pattern<Ctx, Ret = Ctx::Ret> + Clone,
{
    pub fn iter<P>(self, parser: P) -> TermIter<'a, Ctx, P, Sep, Pr, Po>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
    {
        TermIter::new(
            self.ctx,
            self.pre,
            self.post,
            parser,
            self.sep,
            self.optional,
        )
    }

    pub fn next_with<'b: 'c, 'c, P: Pattern<Ctx, Ret = Ctx::Ret> + Clone + 'c>(
        &'b mut self,
        parser: P,
    ) -> Then<
        'c,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
    > {
        let pre = self.pre.take();
        let pre = |ctx: &mut Ctx| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            }
        };
        let post = self.post.take();
        let cont_post = post.clone();
        let parser = |ctx: &mut Ctx| {
            parser.try_parse(ctx).or_else(|e| {
                if let Some(post) = cont_post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };
        let sep = self.sep.clone();
        let post = |ctx: &mut Ctx| {
            sep.try_parse(ctx).or_else(|e| {
                if self.optional {
                    Ok(<Ctx::Ret>::new_from((0, 0)))
                } else if let Some(post) = post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };

        Then::new(self.ctx, pre, post, parser)
    }

    pub fn next_quote<'b, 'c, L, R>(
        &'b mut self,
        left: L,
        right: R,
    ) -> Quote<
        'c,
        Ctx,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone + 'c,
    >
    where
        'b: 'c,
        L: Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        R: Pattern<Ctx, Ret = Ctx::Ret> + Clone + 'c,
    {
        let pre = self.pre.take();
        let pre = |ctx: &mut Ctx| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            }
        };
        let left = |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(pre)?;

            ret += guard.try_mat(left)?;
            Ok(ret)
        };
        let sep = self.sep.clone();
        let sep = |ctx: &mut Ctx| {
            let ret = sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };
        let right = |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(right)?;

            ret += guard.try_mat(sep)?;
            Ok(ret)
        };

        Quote::new(self.ctx, left, right)
    }

    pub fn next_term<'b, 'c, P>(
        &'b mut self,
        sep: P,
        optional: bool,
    ) -> Term<
        'c,
        Ctx,
        P,
        impl Pattern<Ctx, Ret = Ctx::Ret> + 'c,
        impl Pattern<Ctx, Ret = Ctx::Ret> + Clone + 'c,
    >
    where
        'b: 'c,
        P: Pattern<Ctx, Ret = Ctx::Ret> + Clone + 'c,
    {
        let pre = self.pre.take();
        let pre = |ctx: &mut Ctx| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            }
        };
        let prev_sep = self.sep.clone();
        let prev_sep = |ctx: &mut Ctx| {
            let ret = prev_sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<Ctx::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };

        Term::new(self.ctx, Some(pre), Some(prev_sep), sep, optional)
    }
}
