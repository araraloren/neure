use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

///
/// Repeatedly match the regex `P` at least [`min`](crate::ctor::Collect#tymethod.min) times.
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
/// #     color_eyre::install()?;
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

impl<'a, C, P, M, O, V, H, A> Ctor<'a, C, M, V, H, A> for Collect<C, P, O, V>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<'a>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<V, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let val = {
            crate::debug_ctor_beg!("Collect", g.beg());
            V::from_iter(std::iter::from_fn(|| {
                self.pat.construct(g.ctx(), func).ok().inspect(|_| {
                    cnt += 1;
                })
            }))
        };
        let ret = if cnt >= self.min {
            Ok(val)
        } else {
            Err(Error::Collect)
        };

        crate::debug_ctor_reval!("Collect", g.beg(), g.end(), ret.is_ok());
        g.process_ret(ret)
    }
}

impl<'a, C, P, O, V> Regex<C> for Collect<C, P, O, V>
where
    P: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut cnt = 0;
        let mut span = Span::new(g.beg(), 0);

        // don't use g.try_mat, it will set reset when failed
        crate::debug_regex_beg!("Collect", g.beg());
        while let Ok(ret) = g.ctx().try_mat(&self.pat) {
            cnt += 1;
            span.add_assign(ret);
        }
        let ret = if cnt >= self.min {
            Ok(span)
        } else {
            Err(Error::Collect)
        };

        crate::debug_regex_reval!("Collect", g.process_ret(ret))
    }
}
