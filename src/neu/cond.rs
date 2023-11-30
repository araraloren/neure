use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::err::Error;
use crate::re::Regex;
use crate::trace_log;

pub trait NeuCond<'a, C>
where
    C: Context<'a>,
{
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error>;
}

pub trait Condition<'a, C>
where
    C: Context<'a>,
{
    type Out<F>;

    fn set_cond<F>(self, r#if: F) -> Self::Out<F>
    where
        F: NeuCond<'a, C>;
}

impl<'a, C, F> NeuCond<'a, C> for F
where
    C: Context<'a>,
    F: Fn(&C, &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, C::Item)) -> Result<bool, Error> {
        let ret = (self)(ctx, item);

        trace_log!("running cond -> {:?}", ret);
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
pub struct RegexCond<'a, C, T> {
    regex: T,
    marker: PhantomData<(&'a (), C)>,
}

impl<'a, C, T> RegexCond<'a, C, T> {
    pub fn new(regex: T) -> Self {
        Self {
            regex,
            marker: PhantomData,
        }
    }
}

impl<'a, C, T> NeuCond<'a, C> for RegexCond<'a, C, T>
where
    C::Orig: 'a,
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn check(&self, ctx: &C, item: &(usize, <C as Context<'a>>::Item)) -> Result<bool, Error> {
        let mut ctx = ctx.clone_with(ctx.orig_at(ctx.offset() + item.0)?);
        let ret = {
            trace_log!("running regex cond");
            ctx.try_mat_t(&self.regex)
        };

        crate::trace_log!("running regex cond -> {:?}", ret.is_ok());
        Ok(ret.is_ok())
    }
}

pub fn re_cond<'a, C, T>(regex: T) -> RegexCond<'a, C, T> {
    RegexCond::new(regex)
}
