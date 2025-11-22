use std::ops::Deref;
use std::ops::DerefMut;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

use super::Ctor;

///
/// Iterate over the vector and match the regex against the [`Context`].
///
/// # Ctor
///
/// Return the result of first regex that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let tuple = re::vector(["a".into_dyn_regex(), "b".into_dyn_regex(), "c".into_dyn_regex()]);
///
///     assert_eq!(CharsCtx::new("abc").ctor_span(&tuple)?, Span::new(0, 1));
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Vector<T>(Vec<T>);

def_not!(Vector<T>);

impl<T> Vector<T> {
    pub fn new(val: Vec<T>) -> Self {
        Self(val)
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, C, T, M, O, H, A> Ctor<'a, C, M, O, H, A> for Vector<T>
where
    T: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("vector", beg, regex.construct(g.ctx(), func));

            if ret.is_ok() {
                trace!("vector", beg -> g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

impl<'a, C, T> Regex<C> for Vector<T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for regex in self.0.iter() {
            let ret = trace!("vector", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("vector", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::Vec)
    }
}

///
/// Iterate over the vector and match the regex against the [`Context`].
///
/// # Ctor
///
/// Return a pair of result and the value of first pair that matches.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub enum Kind {
///         A,
///         B,
///         C,
///     }
///     let vec = re::pair_vector([("a", Kind::A), ("b", Kind::B), ("c", Kind::C)]);
///
///     assert_eq!(CharsCtx::new("cab").ctor(&vec)?, ("c", Kind::C));
///
///     Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PairVector<K, V>(Vec<(K, V)>);

def_not!(PairVector<K, V>);

impl<K, V> PairVector<K, V> {
    pub fn new(val: Vec<(K, V)>) -> Self {
        Self(val)
    }
}

impl<K, V> Deref for PairVector<K, V> {
    type Target = Vec<(K, V)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for PairVector<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, C, K, M, O, V, H, A> Ctor<'a, C, M, (O, V), H, A> for PairVector<K, V>
where
    V: Clone,
    K: Ctor<'a, C, M, O, H, A>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error>
where {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, value) in self.0.iter() {
            let ret = trace!("pair_vec", beg, regex.construct(g.ctx(), func));

            if ret.is_ok() {
                trace!("pair_vec", beg -> g.end(), true);
                return Ok((ret?, value.clone()));
            } else {
                g.reset();
            }
        }
        Err(Error::PairVec)
    }
}

impl<'a, C, K, V> Regex<C> for PairVector<K, V>
where
    K: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();

        for (regex, _) in self.0.iter() {
            let ret = trace!("pair_vec", beg, g.try_mat(regex));

            if ret.is_ok() {
                trace!("pair_vec", beg => g.end(), true);
                return ret;
            } else {
                g.reset();
            }
        }
        Err(Error::PairVec)
    }
}
