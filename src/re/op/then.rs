use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::map::Select0;
use crate::re::map::Select1;
use crate::re::map::SelectEq;
use crate::re::op::Map;
use crate::re::Ctor;
use crate::re::CtxGuard;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

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
pub struct Then<C, P, T> {
    pat: P,
    then: T,
    marker: PhantomData<C>,
}

impl<C, P, T> Clone for Then<C, P, T>
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

impl<C, P, T> Then<C, P, T> {
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

    pub fn _0<O>(self) -> Map<C, Self, Select0, O> {
        Map::new(self, Select0)
    }

    pub fn _1<O>(self) -> Map<C, Self, Select1, O> {
        Map::new(self, Select1)
    }

    pub fn _eq<I1, I2>(self) -> Map<C, Self, SelectEq, (I1, I2)> {
        Map::new(self, SelectEq)
    }
}

impl<'a, C, P, T, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for Then<C, P, T>
where
    P: Ctor<'a, C, M, O1>,
    T: Ctor<'a, C, M, O2>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.constrct(g.ctx(), func) {
            Ok(ret1) => {
                let ret = self.then.constrct(g.ctx(), func);
                let ret2 = g.process_ret(ret)?;

                Ok((ret1, ret2))
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, T> Regex<C> for Then<C, P, T>
where
    P: Regex<C, Ret = Span>,
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.pat)?;

        ret.add_assign(g.try_mat(&self.then)?);
        Ok(ret)
    }
}
