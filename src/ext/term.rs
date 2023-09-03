use super::mat::MatchThen;
use super::quote::Quote;
use super::CtxGuard;
use crate::policy::Ret;
use crate::Context;
use crate::MatchPolicy;
use crate::NullParser;
use crate::Parser;

pub struct TermIter<'a, C, T, S> {
    cont: T,
    sep: S,
    optional: bool,
    ctx: &'a mut C,
}

impl<'a, C, T, S> TermIter<'a, C, T, S> {
    pub fn new(ctx: &'a mut C, cont: T, sep: S, optional: bool) -> Self {
        Self {
            ctx,
            cont,
            sep,
            optional,
        }
    }
}

impl<'a, C, T, S> TermIter<'a, C, T, S>
where
    C: Context + MatchPolicy,
    T: Parser<C, Ret = C::Ret> + Clone,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    pub fn next(
        &mut self,
    ) -> MatchThen<'_, C, T, NullParser<C>, impl Parser<C, Ret = C::Ret> + '_> {
        let cont = self.cont.clone();
        let sep = self.sep.clone();
        let post = |ctx: &mut C| {
            let ret = sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<C::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };

        MatchThen::new(self.ctx, NullParser::default(), post, cont)
    }
}

// todo!() ADD PRE
pub struct Term<'a, C, S> {
    sep: S,
    optional: bool,
    ctx: &'a mut C,
}

impl<'a, C, S> Term<'a, C, S> {
    pub fn new(ctx: &'a mut C, sep: S, optional: bool) -> Self {
        Self { ctx, sep, optional }
    }
}

impl<'a, C, S> Term<'a, C, S>
where
    C: MatchPolicy + Context,
    S: Parser<C, Ret = C::Ret> + Clone,
{
    pub fn iter<P>(self, parser: P) -> TermIter<'a, C, P, S>
    where
        P: Parser<C, Ret = C::Ret>,
    {
        TermIter::new(self.ctx, parser, self.sep, self.optional)
    }

    pub fn next_with<P>(
        &mut self,
        cont: P,
    ) -> MatchThen<'_, C, P, NullParser<C>, impl Parser<C, Ret = C::Ret> + '_>
    where
        P: Parser<C, Ret = C::Ret>,
    {
        let sep = self.sep.clone();
        let sep = |ctx: &mut C| {
            let ret = sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<C::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };

        MatchThen::new(self.ctx, NullParser::default(), sep, cont)
    }

    pub fn next_quote<'b, L, R>(
        &'a mut self,
        left: L,
        right: R,
    ) -> Quote<'b, C, L, impl Parser<C, Ret = C::Ret> + 'b>
    where
        'a: 'b,
        L: Parser<C, Ret = C::Ret>,
        R: Parser<C, Ret = C::Ret> + 'b,
    {
        let sep = self.sep.clone();
        let sep = |ctx: &mut C| {
            let ret = sep.try_parse(ctx);

            if ret.is_err() && self.optional {
                Ok(<C::Ret>::new_from((0, 0)))
            } else {
                ret
            }
        };

        Quote::new(self.ctx, left, |ctx: &mut C| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(right)?;

            ret += guard.try_mat(sep)?;
            Ok(ret)
        })
    }

    pub fn next_term<'b, P>(
        &'a mut self,
        sep: P,
        optional: bool,
    ) -> Term<'_, C, impl Parser<C, Ret = C::Ret> + 'b>
    where
        'a: 'b,
        P: Parser<C, Ret = C::Ret> + 'b,
    {
        let prev_optional = self.optional;
        let prev_sep = self.sep.clone();
        let sep = move |ctx: &mut C| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(sep)?;

            match guard.try_mat(prev_sep) {
                Ok(prev_ret) => {
                    ret += prev_ret;
                    Ok(ret)
                }
                Err(e) => {
                    if prev_optional {
                        Ok(ret)
                    } else {
                        Err(e)
                    }
                }
            }
        };

        Term::new(self.ctx, sep, optional)
    }
}
