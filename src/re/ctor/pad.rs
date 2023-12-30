use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// First try to match `P`. If the match succeeds, then try to match `T`.
///
/// # Ctor
///
/// It will return result of `P`, and ignoring the result of `T`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
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
#[derive(Copy)]
pub struct Pad<C, P, T> {
    pat: P,
    tail: T,
    marker: PhantomData<C>,
}

def_not!(Pad<C, P, T>);

impl<C, P, T> Debug for Pad<C, P, T>
where
    P: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PadUnit")
            .field("pat", &self.pat)
            .field("tail", &self.tail)
            .finish()
    }
}

impl<C, P, T> Clone for Pad<C, P, T>
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

impl<C, P, T> Pad<C, P, T> {
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

impl<'a, C, P, T, M, O> Ctor<'a, C, M, O> for Pad<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let ret = trace!("pad", beg @ "pat", self.pat.constrct(g.ctx(), func));

        if ret.is_ok() {
            let _ = trace!("pad", beg @ "tail", g.try_mat(&self.tail)?);
        }
        trace!("pad", beg -> g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, P, T> Regex<C> for Pad<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("pad", beg @ "pat", g.try_mat(&self.pat)?);

        ret.add_assign(trace!("pad", beg @ "tail", g.try_mat(&self.tail)?));
        trace!("pad", beg => g.end(), Ok(ret))
    }
}

///
/// First try to match `T`. If it succeeds, try to match `P`.
///
/// # Ctor
///
/// It will return result of `P`, ignoring the result of `T`.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
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
#[derive(Copy)]
pub struct Padded<C, P, T> {
    pat: P,
    head: T,
    marker: PhantomData<C>,
}

def_not!(Padded<C, P, T>);

impl<C, P, T> Debug for Padded<C, P, T>
where
    P: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaddedUnit")
            .field("pat", &self.pat)
            .field("head", &self.head)
            .finish()
    }
}

impl<C, P, T> Clone for Padded<C, P, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            head: self.head.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, T> Padded<C, P, T> {
    pub fn new(pat: P, tail: T) -> Self {
        Self {
            pat,
            head: tail,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn head(&self) -> &T {
        &self.head
    }

    pub fn head_mut(&mut self) -> &mut T {
        &mut self.head
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_head(&mut self, head: T) -> &mut Self {
        self.head = head;
        self
    }
}

impl<'a, C, P, T, M, O> Ctor<'a, C, M, O> for Padded<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let _ = trace!("padded", beg @ "head", g.try_mat(&self.head)?);
        let r = trace!("padded", beg @ "pat", self.pat.constrct(g.ctx(), func));

        trace!("padded", beg -> g.end(), r.is_ok());
        g.process_ret(r)
    }
}

impl<'a, C, P, T> Regex<C> for Padded<C, P, T>
where
    T: Regex<C, Ret = Span>,
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let mut ret = trace!("padded", beg @ "head", g.try_mat(&self.head)?);

        ret.add_assign(trace!("padded", beg @ "pat", g.try_mat(&self.pat)?));
        trace!("padded", beg => g.end(), Ok(ret))
    }
}
