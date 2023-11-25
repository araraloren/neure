use std::fmt::Debug;

use crate::ctx::{Context, Policy, RegexCtx};
use crate::err::Error;
use crate::re::Regex;

pub trait NeuCond<'a, C>
where
    C: Context<'a>,
{
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error>;
}

impl<'a, C, F> NeuCond<'a, C> for F
where
    C: Context<'a>,
    F: Fn(&C, &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error> {
        let ret = (self)(ctx, item);

        crate::trace_log!("check cond: {:?}", ret);
        ret
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NullCond;

impl<'a, C> NeuCond<'a, C> for NullCond
where
    C: Context<'a>,
{
    fn check(&self, _: &C, _: &(usize, C::Item)) -> Result<bool, Error> {
        Ok(true)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReCond<T>(T);

impl<T> ReCond<T> {
    pub fn new(regex: T) -> Self {
        Self(regex)
    }
}

impl<'a, C, R> NeuCond<'a, C> for ReCond<R>
where
    C::Orig: 'a,
    C: Context<'a>,
    R::Ret: crate::ctx::Ret,
    R: Regex<RegexCtx<'a, C::Orig>>,
    RegexCtx<'a, C::Orig>: Context<'a>,
{
    fn check(&self, ctx: &C, item: &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error> {
        let mut ctx = RegexCtx::new(ctx.orig_at(ctx.offset() + item.0)?);
        let ret = {
            crate::trace_log!("regex cond");
            ctx.try_mat_t(&self.0)
        };

        crate::trace_log!("regex cond: {:?}", ret);
        Ok(ret.is_ok())
    }
}

pub fn re_cond<R>(regex: R) -> ReCond<R> {
    ReCond::new(regex)
}
