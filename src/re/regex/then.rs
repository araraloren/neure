use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::re::Regex;
use crate::trace_log;

///
/// Match `P` and then match `T`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return a tuple of results of `L` and `R`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let str = neu::ascii_alphabetic().repeat_one_more();
///     let str = str.quote("\"", "\"").map(Ok);
///     let int = neu::digit(10).repeat_one_more();
///     let int = int.map(re::map::from_str_radix::<i32>(10));
///     let tuple = str.ws().then(",".ws())._0().then(int.ws());
///     let tuple = tuple.quote("(", ")");
///     let mut ctx = CharsCtx::new(r#"("Galaxy", 42)"#);
///
///     assert_eq!(ctx.ctor(&tuple)?, ("Galaxy", 42));
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct RegexThen<C, P, T> {
    pat: P,
    then: T,
    marker: PhantomData<C>,
}

impl<C, P, T> Clone for RegexThen<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            then: self.then.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> RegexThen<C, P, T> {
    pub fn new(pat: P, then: T) -> Self {
        Self {
            pat,
            then,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn then(&self) -> &T {
        &self.then
    }

    pub fn then_mut(&mut self) -> &mut T {
        &mut self.then
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_then(&mut self, then: T) -> &mut Self {
        self.then = then;
        self
    }
}

impl<'a, C, P, T> Regex<C> for RegexThen<C, P, T>
where
    P: Regex<C>,
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    type Ret = (P::Ret, T::Ret);

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        trace_log!("(`then`: @{}) => try match left", g.beg());
        let ret1 = g.try_mat(&self.pat)?;

        trace_log!("(`then`: @{}) => try match left", g.beg());
        let ret2 = g.try_mat(&self.then)?;

        Ok((ret1, ret2))
    }
}
