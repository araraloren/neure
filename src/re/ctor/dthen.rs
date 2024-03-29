use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::re::ctor::Map;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Like [`Then`](crate::re::ctor::Then), but create with [`.dyn_then_ctor`](crate::re::ctor::DynamicCreateCtorThenHelper#tymethod.dyn_then_ctor) of regex.
///
/// # Ctor
///
/// Return a tuple of `P`'s result and new regex result.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, re::DynamicCreateCtorThenHelper};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let len = re::consume(2).map(map::from_le_bytes::<i16>());
///     let data = len.dyn_then_ctor(|v: &i16| Ok(re::consume(*v as usize)));
///     let ret = BytesCtx::new(b"\x1f\0Hello there, where are you from?").ctor(&data)?;
///
///     assert_eq!(ret, (0x1f, b"Hello there, where are you from".as_ref()));
///
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct DynamicCreateCtorThen<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

def_not!(DynamicCreateCtorThen<C, P, F>);

impl<C, P, F> Debug for DynamicCreateCtorThen<C, P, F>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicCreateCtorThen")
            .field("pat", &self.pat)
            .field("func", &self.func)
            .finish()
    }
}

impl<C, P, F> Clone for DynamicCreateCtorThen<C, P, F>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            func: self.func.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F> DynamicCreateCtorThen<C, P, F> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn func(&self) -> &F {
        &self.func
    }

    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_func(&mut self, func: F) -> &mut Self {
        self.func = func;
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

impl<'a, C, P, F, T, M, O1, O2, H, A> Ctor<'a, C, M, (O1, O2), H, A>
    for DynamicCreateCtorThen<C, P, F>
where
    P: Ctor<'a, C, M, O1, H, A>,
    T: Ctor<'a, C, M, O2, H, A>,
    C: Context<'a> + Match<C>,
    F: Fn(&O1) -> Result<T, Error>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);
        let beg = g.beg();
        let l = trace!("dynamic_create_ctor_then", beg @ "pat", self.pat.construct(g.ctx(), func));
        let l = g.process_ret(l)?;
        let r = trace!("dynamic_create_ctor_then", beg @ "dynamic ctor", (self.func)(&l)?.construct(g.ctx(), func));
        let r = g.process_ret(r)?;

        trace!("dynamic_create_ctor_then", beg -> g.end(), true);
        Ok((l, r))
    }
}

impl<'a, C, P, F> Regex<C> for DynamicCreateCtorThen<C, P, F>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    fn try_parse(&self, _: &mut C) -> Result<Self::Ret, Error> {
        unimplemented!("Can't not using DynamicThen for regex")
    }
}

pub trait DynamicCreateCtorThenHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Match<C>,
{
    fn dyn_then_ctor<F>(self, func: F) -> DynamicCreateCtorThen<C, Self, F>;
}

impl<'a, C, T> DynamicCreateCtorThenHelper<'a, C> for T
where
    Self: Sized,
    C: Context<'a> + Match<C>,
{
    ///
    /// Construct a new regex with `Ctor` implementation based on previous result.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*, re::DynamicCreateCtorThenHelper};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let num = u8::is_ascii_digit
    ///         .repeat_one()
    ///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .map(map::from_str::<usize>());
    ///     let num = num.clone().sep_once(b",", num);
    ///     let re = num.dyn_then_ctor(|a: &(usize, usize)| {
    ///         // leave the a's type empty cause rustc reject compile
    ///         Ok(b'+'
    ///             .repeat_range(a.0..a.0 + 1)
    ///             .then(b'-'.repeat_range(a.1..a.1 + 1)))
    ///     });
    ///
    ///     assert_eq!(
    ///         BytesCtx::new(b"3,0+++").ctor(&re)?,
    ///         ((3, 0), ([43, 43, 43].as_slice(), [].as_slice()))
    ///     );
    ///     assert_eq!(
    ///         BytesCtx::new(b"2,1++-").ctor(&re)?,
    ///         ((2, 1), ([43, 43].as_slice(), [45].as_slice()))
    ///     );
    ///     assert_eq!(
    ///         BytesCtx::new(b"0,3---").ctor(&re)?,
    ///         ((0, 3), ([].as_slice(), [45, 45, 45].as_slice()))
    ///     );
    ///     Ok(())
    /// # }
    /// ```
    fn dyn_then_ctor<F>(self, func: F) -> DynamicCreateCtorThen<C, Self, F> {
        DynamicCreateCtorThen::new(self, func)
    }
}
