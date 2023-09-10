mod guard;
mod handler;
mod quote;
mod term;
mod then;

pub use self::guard::CtxGuard;
pub use self::handler::*;
pub use self::quote::LazyQuote;
pub use self::quote::NonLazyQuote;
pub use self::term::LazyTerm;
pub use self::term::NonLazyTerm;
pub use self::term::NonLazyTermIter;
pub use self::then::LazyPattern;
pub use self::then::LazyPatternValue;
pub use self::then::NonLazyPattern;
pub use self::then::NonLazyPatternValue;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::True;
use crate::err::Error;

pub trait LazyCtxExtension<'a, C: Context<'a> + Policy<C>> {
    fn quote<L, R>(&mut self, left: L, right: R) -> LazyQuote<'_, C, L, R>
    where
        L: Pattern<C, Ret = <C as Policy<C>>::Ret>,
        R: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn pat<P>(&mut self, pattern: P) -> LazyPattern<'_, C, P, True<C>, True<C>>
    where
        P: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn term<S>(&mut self, sep: S) -> LazyTerm<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    fn term_opt<S>(&mut self, sep: S, optional: bool) -> LazyTerm<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone;
}

pub trait NonLazyCtxExtension<'a, C: Context<'a> + Policy<C>> {
    fn quote<L, R>(&mut self, left: L, right: R) -> Result<NonLazyQuote<'_, C, R>, Error>
    where
        L: Pattern<C, Ret = <C as Policy<C>>::Ret>,
        R: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn pat<P>(&mut self, pattern: P) -> Result<NonLazyPattern<'_, C, True<C>>, Error>
    where
        P: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn term<S>(&mut self, sep: S) -> Result<NonLazyTerm<'_, C, S, True<C>>, Error>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    fn term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> Result<NonLazyTerm<'_, C, S, True<C>>, Error>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone;
}

pub fn and<'a, C, P1, P2>(p1: P1, p2: P2) -> impl FnOnce(&mut C) -> Result<C::Ret, Error>
where
    C::Ret: Ret,
    C: Context<'a> + Policy<C>,
    P1: Pattern<C, Ret = C::Ret>,
    P2: Pattern<C, Ret = C::Ret>,
{
    move |ctx: &mut C| {
        let mut guard = CtxGuard::new(ctx);
        let mut ret = guard.try_mat(p1)?;

        ret.add_assign(guard.try_mat(p2)?);
        Ok(ret)
    }
}

pub fn or<'a, C, P1, P2>(p1: P1, p2: P2) -> impl FnOnce(&mut C) -> Result<C::Ret, Error>
where
    C::Ret: Ret,
    C: Context<'a> + Policy<C>,
    P1: Pattern<C, Ret = C::Ret>,
    P2: Pattern<C, Ret = C::Ret>,
{
    move |ctx: &mut C| p1.try_parse(ctx).or(p2.try_parse(ctx))
}

pub fn quote<'a, C, L, R, P>(l: L, r: R, p: P) -> impl FnOnce(&mut C) -> Result<C::Ret, Error>
where
    C::Ret: Ret,
    C: Context<'a> + Policy<C>,
    L: Pattern<C, Ret = C::Ret>,
    R: Pattern<C, Ret = C::Ret>,
    P: Pattern<C, Ret = C::Ret>,
{
    move |ctx: &mut C| {
        let mut guard = CtxGuard::new(ctx);
        let mut ret = guard.try_mat(l)?;

        ret.add_assign(guard.try_mat(p)?);
        ret.add_assign(guard.try_mat(r)?);
        Ok(ret)
    }
}

pub fn terminated<'a, C, S, P>(sep: S, p: P) -> impl FnOnce(&mut C) -> Result<C::Ret, Error>
where
    C::Ret: Ret,
    C: Context<'a> + Policy<C>,
    S: Pattern<C, Ret = C::Ret> + Clone,
    P: Pattern<C, Ret = C::Ret> + Clone,
{
    terminated_opt(sep, p, true)
}

pub fn terminated_opt<'a, C, S, P>(
    sep: S,
    p: P,
    optional: bool,
) -> impl FnOnce(&mut C) -> Result<C::Ret, Error>
where
    C::Ret: Ret,
    C: Context<'a> + Policy<C>,
    S: Pattern<C, Ret = C::Ret> + Clone,
    P: Pattern<C, Ret = C::Ret> + Clone,
{
    move |ctx: &mut C| {
        let mut guard = CtxGuard::new(ctx);
        let mut ret = <C::Ret>::new_from((0, 0));

        while let Ok(p_ret) = guard.try_mat(p.clone()) {
            ret.add_assign(p_ret);
            match guard.try_mat(sep.clone()) {
                Ok(sep_ret) => {
                    ret.add_assign(sep_ret);
                }
                Err(e) => {
                    if optional {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(ret)
    }
}
