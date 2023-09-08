mod guard;
mod handler;
mod quote;
mod term;
mod then;

pub use self::guard::CtxGuard;
pub use self::handler::*;
pub use self::quote::Quote;
pub use self::term::Term;
pub use self::then::MatThenValue;
pub use self::then::Then;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::err::Error;

// todo! add `and` `or` `quote` `terminated`
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
