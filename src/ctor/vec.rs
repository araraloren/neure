use std::ops::Deref;
use std::ops::DerefMut;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_beg;
use crate::debug_ctor_reval;
use crate::debug_regex_beg;
use crate::debug_regex_reval;
use crate::err::Error;
use crate::regex::def_not;
use crate::regex::Regex;

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
///     let tuple = regex::vector(["a".into_dyn_regex(), "b".into_dyn_regex(), "c".into_dyn_regex()]);
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

impl<'a, C, T, M, O, H> Ctor<'a, C, M, O, H> for Vector<T>
where
    T: Ctor<'a, C, M, O, H>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Vector);

        debug_ctor_beg!("Vector", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }

        debug_ctor_reval!("Vector", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, T> Regex<C> for Vector<T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Vector);

        debug_regex_beg!("Vector", g.beg());
        for regex in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }

        debug_regex_reval!("Vector", ret)
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
///     let vec = regex::pair_vector([("a", Kind::A), ("b", Kind::B), ("c", Kind::C)]);
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

impl<'a, C, K, M, O, V, H> Ctor<'a, C, M, (O, V), H> for PairVector<K, V>
where
    V: Clone,
    K: Ctor<'a, C, M, O, H>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O, V), Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairVector);

        debug_ctor_beg!("PairVector", g.beg());
        for (regex, value) in self.0.iter() {
            if let Ok(res) = regex.construct(g.ctx(), func) {
                ret = Ok((res, value.clone()));
                break;
            } else {
                g.reset();
            }
        }

        debug_ctor_reval!("PairVector", g.beg(), g.end(), ret.is_ok());
        ret
    }
}

impl<'a, C, K, V> Regex<C> for PairVector<K, V>
where
    K: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::PairVector);

        debug_regex_beg!("PairVector", g.beg());
        for (regex, _) in self.0.iter() {
            if let Ok(res) = g.try_mat(regex) {
                ret = Ok(res);
                break;
            } else {
                g.reset();
            }
        }

        debug_regex_reval!("PairVector", ret)
    }
}
