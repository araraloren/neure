use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

#[derive(Debug, Default, Copy)]
pub struct IfRegex<C, T, I, E> {
    pat: T,
    r#if: I,
    r#else: E,
    marker: PhantomData<C>,
}

impl<C, T, I, E> Clone for IfRegex<C, T, I, E>
where
    T: Clone,
    I: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            r#if: self.r#if.clone(),
            r#else: self.r#else.clone(),
            marker: self.marker,
        }
    }
}

impl<C, T, I, E> IfRegex<C, T, I, E> {
    pub fn new(regex: T, r#if: I, r#else: E) -> Self {
        Self {
            pat: regex,
            r#if,
            r#else,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &T {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut T {
        &mut self.pat
    }

    pub fn r#if(&self) -> &I {
        &self.r#if
    }

    pub fn r#if_mut(&mut self) -> &mut I {
        &mut self.r#if
    }

    pub fn r#else(&self) -> &E {
        &self.r#else
    }

    pub fn else_mut(&mut self) -> &mut E {
        &mut self.r#else
    }

    pub fn set_pat(&mut self, pat: T) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_if(&mut self, r#if: I) -> &mut Self {
        self.r#if = r#if;
        self
    }

    pub fn set_else(&mut self, r#else: E) -> &mut Self {
        self.r#else = r#else;
        self
    }
}

impl<'a, C, T, I, E, M, O> Ctor<'a, C, M, O> for IfRegex<C, T, I, E>
where
    T: Ctor<'a, C, M, O>,
    E: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("if", beg, (self.r#if)(g.ctx())?);
        let ret = if ret {
            trace!("if", beg @ "true", self.pat.constrct(g.ctx(), func))
        } else {
            trace!("if", beg @ "false", self.r#else.constrct(g.reset().ctx(), func))
        };

        trace!("if", beg -> g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, T, I, E> Regex<C> for IfRegex<C, T, I, E>
where
    T: Regex<C, Ret = Span>,
    E: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    type Ret = T::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("if", beg, (self.r#if)(g.ctx())?);
        let ret = if ret {
            trace!("if", beg @ "true", g.try_mat(&self.pat))
        } else {
            trace!("if", beg @ "false", g.try_mat(&self.r#else))
        };

        trace!("if", beg => g.end(), ret)
    }
}

pub fn branch<'a, C>(
    r#if: impl Fn(&C) -> Result<bool, Error>,
    re: impl Regex<C, Ret = Span>,
    r#else: impl Regex<C, Ret = Span>,
) -> impl Regex<C, Ret = Span>
where
    C: Context<'a> + Match<C>,
{
    IfRegex::new(re, r#if, r#else)
}
