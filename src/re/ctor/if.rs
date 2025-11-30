use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_ctor_stage;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::debug_regex_stage;
use crate::err::Error;
use crate::re::def_not;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Construct a branch struct base on the test `I`(Fn(&C) -> Result<bool, Error>).
///
/// # Ctor
///
/// Return the result of `P` if `I` return true, otherwise return `E`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let re1 = "google".sep_once(".", "com".or("is")).pat();
///     let re2 = "google"
///         .sep_once(".", "co".sep_once(".", "kr".or("jp")))
///         .pat();
///     // test the `orig` before match
///     let re = re2.r#if(
///         |ctx: &CharsCtx| ctx.orig().map(|v| v.ends_with("jp") || v.ends_with("kr")),
///         re1,
///     );
///
///     assert_eq!(CharsCtx::new("google.com").ctor(&re)?, "google.com");
///     assert_eq!(CharsCtx::new("google.is").ctor(&re)?, "google.is");
///     assert_eq!(CharsCtx::new("google.co.jp").ctor(&re)?, "google.co.jp");
///     assert_eq!(CharsCtx::new("google.co.kr").ctor(&re)?, "google.co.kr");
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct IfRegex<C, P, I, E> {
    pat: P,
    r#if: I,
    r#else: E,
    marker: PhantomData<C>,
}

def_not!(IfRegex<C, P, I, E>);

impl<C, P, I, E> Debug for IfRegex<C, P, I, E>
where
    P: Debug,
    I: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfRegex")
            .field("pat", &self.pat)
            .field("r#if", &self.r#if)
            .field("r#else", &self.r#else)
            .finish()
    }
}

impl<C, P, I, E> Clone for IfRegex<C, P, I, E>
where
    P: Clone,
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

impl<C, P, I, E> IfRegex<C, P, I, E> {
    pub fn new(pat: P, r#if: I, r#else: E) -> Self {
        Self {
            pat,
            r#if,
            r#else,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
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

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
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

impl<'a, C, P, I, E, M, O, H, A> Ctor<'a, C, M, O, H, A> for IfRegex<C, P, I, E>
where
    P: Ctor<'a, C, M, O, H, A>,
    E: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    I: Fn(&C) -> Result<bool, Error>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_ctor_beg!("IfRegex", g.beg());

        let ret = debug_ctor_stage!("IfRegex", "if", (self.r#if)(g.ctx())?);
        let ret = if ret {
            debug_ctor_stage!("IfRegex", "true", self.pat.construct(g.ctx(), func))
        } else {
            debug_ctor_stage!(
                "IfRegex",
                "false",
                self.r#else.construct(g.reset().ctx(), func)
            )
        };

        debug_ctor_reval!("IfRegex", g.beg(), g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, P, I, E> Regex<C> for IfRegex<C, P, I, E>
where
    P: Regex<C>,
    E: Regex<C>,
    C: Context<'a> + Match<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);

        debug_regex_beg!("IfRegex", g.beg());

        let ret = debug_regex_stage!("IfRegex", "if", (self.r#if)(g.ctx())?);
        let ret = if ret {
            debug_regex_stage!("IfRegex", "true", g.try_mat(&self.pat))
        } else {
            debug_regex_stage!("IfRegex", "false", g.try_mat(&self.r#else))
        };

        debug_regex_reval!("IfRegex", ret)
    }
}

pub fn branch<'a, C, P, I, E>(r#if: I, re: P, r#else: E) -> IfRegex<C, P, I, E>
where
    C: Context<'a> + Match<C>,
    E: Regex<C>,
    P: Regex<C>,
    I: Fn(&C) -> Result<bool, Error>,
{
    IfRegex::new(re, r#if, r#else)
}
