use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Match `P` and `T` which padded by `P`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return result of `P`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let protocol = "https".or("http".or("ftp"));
///     let protocol = protocol.pad("://");
///     let domain = neu::alphabetic().repeat_one_more();
///     let domain = domain.sep(".").at_least(2);
///     let url = domain.padded(protocol);
///     let mut ctx = CharsCtx::new(r#"https://www.mozilla.org"#);
///
///     assert_eq!(ctx.ctor(&url)?, ["www", "mozilla", "org"]);
///     Ok(())
/// # }
/// ```
#[derive(Debug, Copy)]
pub struct PadUnit<C, P, T> {
    pat: P,
    tail: T,
    marker: PhantomData<C>,
}

impl<C, P, T> Clone for PadUnit<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            tail: self.tail.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> PadUnit<C, P, T> {
    pub fn new(pat: P, tail: T) -> Self {
        Self {
            pat,
            tail,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn tail(&self) -> &T {
        &self.tail
    }

    pub fn tail_mut(&mut self) -> &mut T {
        &mut self.tail
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_tail(&mut self, tail: T) -> &mut Self {
        self.tail = tail;
        self
    }
}

impl<'a, C, P, T, M, O> Ctor<'a, C, M, O> for PadUnit<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);

        match self.pat.constrct(g.ctx(), func) {
            Ok(ret1) => {
                g.try_mat(&self.tail)?;
                Ok(ret1)
            }
            Err(e) => {
                g.reset();
                Err(e)
            }
        }
    }
}

impl<'a, C, P, T> Regex<C> for PadUnit<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.pat)?;

        ret.add_assign(g.try_mat(&self.tail)?);
        Ok(ret)
    }
}

///
/// Match `P` which padded by `T`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return result of `P`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     let protocol = "https".or("http".or("ftp"));
///     let protocol = protocol.pad("://");
///     let domain = neu::alphabetic().repeat_one_more();
///     let domain = domain.sep(".").at_least(2);
///     let url = domain.padded(protocol);
///     let mut ctx = CharsCtx::new(r#"https://www.mozilla.org"#);
///
///     assert_eq!(ctx.ctor(&url)?, ["www", "mozilla", "org"]);
///     Ok(())
/// # }
/// ```
#[derive(Debug, Copy)]
pub struct PaddedUnit<C, P, T> {
    pat: P,
    tail: T,
    marker: PhantomData<C>,
}

impl<C, P, T> Clone for PaddedUnit<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            tail: self.tail.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> PaddedUnit<C, P, T> {
    pub fn new(pat: P, tail: T) -> Self {
        Self {
            pat,
            tail,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn tail(&self) -> &T {
        &self.tail
    }

    pub fn tail_mut(&mut self) -> &mut T {
        &mut self.tail
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_tail(&mut self, tail: T) -> &mut Self {
        self.tail = tail;
        self
    }
}

impl<'a, C, P, T, M, O> Ctor<'a, C, M, O> for PaddedUnit<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let _ = g.try_mat(&self.tail)?;
        let ret = self.pat.constrct(g.ctx(), func);

        g.process_ret(ret)
    }
}

impl<'a, C, P, T> Regex<C> for PaddedUnit<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = g.try_mat(&self.tail)?;

        ret.add_assign(g.try_mat(&self.pat)?);
        Ok(ret)
    }
}
