use std::marker::PhantomData;

use super::CtxGuard;
use super::Extract;
use super::Handler;
use super::Invoke;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;
use crate::trace_log;

#[derive(Debug, Default, Copy)]
pub struct IfRegex<C, T, I, E> {
    regex: T,
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
            regex: self.regex.clone(),
            r#if: self.r#if.clone(),
            r#else: self.r#else.clone(),
            marker: self.marker,
        }
    }
}

impl<'a, C, T, I, E> IfRegex<C, T, I, E>
where
    C: Context<'a> + Policy<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    pub fn new(regex: T, r#if: I, r#else: E) -> Self {
        Self {
            regex,
            r#if,
            r#else,
            marker: PhantomData,
        }
    }

    pub fn regex(&self) -> &T {
        &self.regex
    }

    pub fn regex_mut(&mut self) -> &mut T {
        &mut self.regex
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

    pub fn set_regex(&mut self, regex: T) -> &mut Self {
        self.regex = regex;
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

impl<'a, C, T, I, E, M, O> Invoke<'a, C, M, O> for IfRegex<C, T, I, E>
where
    T: Invoke<'a, C, M, O>,
    E: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let ret = if (self.r#if)(g.ctx())? {
            self.regex.invoke(g.ctx(), func)
        } else {
            self.r#else.invoke(g.reset().ctx(), func)
        };

        g.process_ret(ret)
    }
}

impl<'a, C, T, I, E> Regex<C> for IfRegex<C, T, I, E>
where
    T: Regex<C, Ret = Span>,
    E: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    type Ret = T::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let ret = (self.r#if)(ctx)?;

        trace_log!("running if logical: {}", ret);
        if ret {
            ctx.try_mat(&self.regex)
        } else {
            ctx.try_mat(&self.r#else)
        }
    }
}
