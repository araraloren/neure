use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::ctor::Map;
use crate::re::map::Select0;
use crate::re::map::Select1;
use crate::re::map::SelectEq;
use crate::re::trace;
use crate::re::Ctor;
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
pub struct And<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Clone for And<C, L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R> And<C, L, R> {
    pub fn new(pat: L, then: R) -> Self {
        Self {
            left: pat,
            right: then,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, pat: L) -> &mut Self {
        self.left = pat;
        self
    }

    pub fn set_right(&mut self, then: R) -> &mut Self {
        self.right = then;
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

impl<'a, C, L, R, M, O1, O2> Ctor<'a, C, M, (O1, O2)> for And<C, L, R>
where
    L: Ctor<'a, C, M, O1>,
    R: Ctor<'a, C, M, O2>,
    C: Context<'a> + Policy<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("and", beg @ "left", self.left.constrct(g.ctx(), func).map(|ret1| {
            trace!("and", beg @ "right", self.right.constrct(g.ctx(), func).map(|ret2| (ret1, ret2)))   
        }) );
        let ret = g.process_ret(ret)?;

        trace!("and", beg => g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, L, R> Regex<C> for And<C, L, R>
where
    L: Regex<C, Ret = Span>,
    R: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = L::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("and", beg @ "left", g.try_mat(&self.left)?);

        ret.add_assign(trace!("and", beg @ "right", g.try_mat(&self.right)?));
        trace!("and", beg => g.end(), Ok(ret))
    }
}
