use super::mat::MatchThen;
use super::quote::Quote;
use super::CtxGuard;
use crate::policy::Ret;
use crate::Context;
use crate::MatchPolicy;
use crate::NullParser;
use crate::Parser;

pub struct TermIter<'a, C, T, S, PR, PO> {
    cont: T,
    sep: S,
    pre: Option<PR>,
    post: Option<PO>,
    optional: bool,
    ctx: &'a mut C,
}

impl<'a, C, T, S, PR, PO> TermIter<'a, C, T, S, PR, PO> {
    pub fn new(
        ctx: &'a mut C,
        pre: Option<PR>,
        post: Option<PO>,
        cont: T,
        sep: S,
        optional: bool,
    ) -> Self {
        Self {
            ctx,
            cont,
            sep,
            pre,
            post,
            optional,
        }
    }
}

impl<'a, C, T, S, PR, PO> TermIter<'a, C, T, S, PR, PO>
where
    C: Context + MatchPolicy,
    PR: Parser<C, Ret = C::Ret>,
    PO: Parser<C, Ret = C::Ret> + Clone,
    T: Parser<C, Ret = C::Ret> + Clone,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    pub fn next<'b: 'c, 'c>(
        &'b mut self,
    ) -> MatchThen<
        'c,
        C,
        impl Parser<C, Ret = C::Ret> + 'c,
        impl Parser<C, Ret = C::Ret> + 'c,
        impl Parser<C, Ret = C::Ret> + 'c,
    > {
        let pre = self.pre.take();
        let pre = |ctx: &mut C| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<C::Ret>::new_from((0, 0)))
            }
        };
        let post = self.post.take();
        let cont_post = post.clone();
        let cont = self.cont.clone();
        let parser = |ctx: &mut C| {
            cont.try_parse(ctx).or_else(|e| {
                if let Some(post) = cont_post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };
        let sep = self.sep.clone();
        let post = |ctx: &mut C| {
            sep.try_parse(ctx).or_else(|e| {
                if self.optional {
                    Ok(<C::Ret>::new_from((0, 0)))
                } else if let Some(post) = post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };

        MatchThen::new(self.ctx, pre, post, parser)
    }
}

pub struct Term<'a, C, S, PR, PO>
where
    C: MatchPolicy + Context,
    PR: Parser<C, Ret = C::Ret>,
    PO: Parser<C, Ret = C::Ret> + Clone,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    sep: S,
    pre: Option<PR>,
    post: Option<PO>,
    optional: bool,
    ctx: &'a mut C,
}

impl<'a, C, S, PR, PO> Term<'a, C, S, PR, PO>
where
    C: MatchPolicy + Context,
    PR: Parser<C, Ret = C::Ret>,
    PO: Parser<C, Ret = C::Ret> + Clone,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    pub fn new(ctx: &'a mut C, pre: Option<PR>, post: Option<PO>, sep: S, optional: bool) -> Self {
        Self {
            ctx,
            sep,
            pre,
            post,
            optional,
        }
    }
}

impl<'a, C, S, PR, PO> Term<'a, C, S, PR, PO>
where
    C: MatchPolicy + Context,
    PR: Parser<C, Ret = C::Ret>,
    PO: Parser<C, Ret = C::Ret> + Clone,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    pub fn iter<P>(self, parser: P) -> TermIter<'a, C, P, S, PR, PO>
    where
        P: Parser<C, Ret = C::Ret>,
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

    pub fn next_with<'b: 'c, 'c, P: Parser<C, Ret = C::Ret> + Clone + 'c>(
        &'b mut self,
        parser: P,
    ) -> MatchThen<
        'c,
        C,
        impl Parser<C, Ret = C::Ret> + 'c,
        impl Parser<C, Ret = C::Ret> + 'c,
        impl Parser<C, Ret = C::Ret> + 'c,
    > {
        let pre = self.pre.take();
        let pre = |ctx: &mut C| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<C::Ret>::new_from((0, 0)))
            }
        };
        let post = self.post.take();
        let cont_post = post.clone();
        let parser = |ctx: &mut C| {
            parser.try_parse(ctx).or_else(|e| {
                if let Some(post) = cont_post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };
        let sep = self.sep.clone();
        let post = |ctx: &mut C| {
            sep.try_parse(ctx).or_else(|e| {
                if self.optional {
                    Ok(<C::Ret>::new_from((0, 0)))
                } else if let Some(post) = post {
                    post.try_parse(ctx)
                } else {
                    Err(e)
                }
            })
        };

        MatchThen::new(self.ctx, pre, post, parser)
    }

    pub fn next_quote<'b, 'c, L, R>(
        &'b mut self,
        left: L,
        right: R,
    ) -> Quote<'c, C, impl Parser<C, Ret = C::Ret> + 'c, impl Parser<C, Ret = C::Ret> + 'c>
    where
        'b: 'c,
        L: Parser<C, Ret = C::Ret> + 'c,
        R: Parser<C, Ret = C::Ret> + 'c,
    {
        let pre = self.pre.take();
        let pre = |ctx: &mut C| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<C::Ret>::new_from((0, 0)))
            }
        };
        let left = |ctx: &mut C| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(pre)?;

            ret += guard.try_mat(left)?;
            Ok(ret)
        };
        let sep = self.sep.clone();
        let sep = |ctx: &mut C| {
            let ret = sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<C::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };
        let right = |ctx: &mut C| {
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
    ) -> Term<'c, C, P, impl Parser<C, Ret = C::Ret> + 'c, impl Parser<C, Ret = C::Ret> + Clone + 'c>
    where
        'b: 'c,
        P: Parser<C, Ret = C::Ret> + Clone + 'c,
    {
        let pre = self.pre.take();
        let pre = |ctx: &mut C| {
            if let Some(pre) = pre {
                pre.try_parse(ctx)
            } else {
                Ok(<C::Ret>::new_from((0, 0)))
            }
        };
        let prev_sep = self.sep.clone();
        let prev_sep = |ctx: &mut C| {
            let ret = prev_sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<C::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };

        Term::new(self.ctx, Some(pre), Some(prev_sep), sep, optional)
    }
}
