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
/// Repeatedly match the regex `P` at least [`min`](crate::re::ctor::Collect#tymethod.min) times.
///
/// # Ctor
///
/// Return a type `V` that collects the result of regex `P`.
/// `Collect` will always succeed if the minimum size is 0, be careful to use it with `.sep` faimly APIs.
/// The default size is 1.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let re = b'+'.repeat_one().collect::<_, Vec<_>>();
///
///     assert!(BytesCtx::new(b"---A").ctor(&re).is_err());
///     assert_eq!(BytesCtx::new(b"+++A").ctor(&re)?, vec![b"+", b"+", b"+"]);
///     assert_eq!(BytesCtx::new(b"++-A").ctor(&re)?, vec![b"+", b"+"]);
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Collect<C, P, O, V> {
    pat: P,
    min: usize,
    marker: PhantomData<(O, V, C)>,
}

def_not!(Collect<C, P, O, V>);

impl<C, P, O, V> Debug for Collect<C, P, O, V>
where
    P: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Collect")
            .field("pat", &self.pat)
            .field("min", &self.min)
            .finish()
    }
}

impl<C, P, O, V> Clone for Collect<C, P, O, V>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            min: self.min,
            marker: self.marker,
        }
    }
}

impl<C, P, O, V> Collect<C, P, O, V> {
    pub fn new(pat: P) -> Self {
        Self {
            pat,
            min: 1,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    pub fn at_least(mut self, min: usize) -> Self {
        self.min = min;
        self
    }
}

impl<'a, C, P, M, O, V> Ctor<'a, C, M, V> for Collect<C, P, O, V>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut ret = Err(Error::Collect);
        let beg = g.beg();
        let val = trace!(
            "collect",
            beg,
            V::from_iter(std::iter::from_fn(|| {
                match self.pat.constrct(g.ctx(), func) {
                    Ok(ret) => {
                        cnt += 1;
                        Some(ret)
                    }
                    Err(_) => None,
                }
            }))
        );

        if cnt >= self.min {
            ret = Ok(val);
        }
        trace!("collect", beg -> g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, P, O, V> Regex<C> for Collect<C, P, O, V>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = <Span as Ret>::from_ctx(g.ctx(), (0, 0));
        let mut ret = Err(Error::Collect);
        let beg = g.beg();

        // don't use g.try_mat
        trace!("collect", beg, ());
        while let Ok(ret) = g.ctx().try_mat(&self.pat) {
            cnt += 1;
            span.add_assign(ret);
        }
        if cnt >= self.min {
            ret = Ok(span);
        }
        trace!("collect", beg => g.end(), g.process_ret(ret))
    }
}
