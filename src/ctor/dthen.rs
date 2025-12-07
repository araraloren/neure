use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctor::Map;
use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::debug_ctor_stage;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::regex::def_not;
use crate::regex::Regex;

///
/// [`DynamicCtorThenBuilder`] is a type similar to [`Then`](crate::ctor::Then).
/// It can dynamically construct a new type based on the result type of given `P`,
/// and upon success, it will return a tuple of result type of `P` and result of the newly type.
///
/// # Regex
///
/// Not support.
///
/// # Ctor
///
/// Return a tuple of `P`'s result and newly type result.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, ctor::DynamicCtorThenBuilderHelper};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #     color_eyre::install()?;
///     let len = regex::consume(2).map(map::from_le_bytes::<i16>());
///     let data = len.into_ctor_then_builder(|_, v| Ok(regex::consume(*v as usize)));
///     let ret = BytesCtx::new(b"\x1f\0Hello there, where are you from?").ctor(&data)?;
///
///     assert_eq!(ret, (0x1f, b"Hello there, where are you from".as_ref()));
///
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct DynamicCtorThenBuilder<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

def_not!(DynamicCtorThenBuilder<C, P, F>);

impl<C, P, F> Debug for DynamicCtorThenBuilder<C, P, F>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicCtorBuilder")
            .field("pat", &self.pat)
            .field("func", &self.func)
            .finish()
    }
}

impl<C, P, F> Clone for DynamicCtorThenBuilder<C, P, F>
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

impl<C, P, F> DynamicCtorThenBuilder<C, P, F> {
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

impl<'a, C, P, F> Regex<C> for DynamicCtorThenBuilder<C, P, F>
where
    P: Regex<C>,
    C: Context<'a> + Match<'a>,
{
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicCtorThenBuilder not support Regex trait")
    }
}

impl<'a, C, P, F, T, M, O1, O2, H, A> Ctor<'a, C, M, (O1, O2), H, A>
    for DynamicCtorThenBuilder<C, P, F>
where
    P: Ctor<'a, C, M, O1, H, A>,
    T: Ctor<'a, C, M, O2, H, A>,
    C: Context<'a> + Match<'a>,
    F: Fn(&mut C, &O1) -> Result<T, Error>,
    H: Handler<A, Out = M, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut g = CtxGuard::new(ctx);

        crate::debug_ctor_beg!("DynamicCtorThenBuilder", g.beg());

        let l = {
            debug_ctor_stage!(
                "DynamicCtorThenBuilder",
                "left",
                self.pat.construct(g.ctx(), func)
            )
        };
        let l = g.process_ret(l)?;
        let r = {
            debug_ctor_stage!(
                "DynamicCtorThenBuilder",
                "right",
                (self.func)(g.ctx(), &l)?.construct(g.ctx(), func)
            )
        };
        let r = g.process_ret(r)?;

        crate::debug_ctor_reval!("DynamicCtorThenBuilder", g.beg(), g.end(), true);
        Ok((l, r))
    }
}

pub trait DynamicCtorThenBuilderHelper<'a, C>
where
    Self: Sized,
    C: Context<'a> + Match<'a>,
{
    fn into_ctor_then_builder<F, O1, R>(self, func: F) -> DynamicCtorThenBuilder<C, Self, F>
    where
        F: Fn(&mut C, &O1) -> Result<R, Error>;
}

impl<'a, C, T> DynamicCtorThenBuilderHelper<'a, C> for T
where
    Self: Sized,
    C: Context<'a> + Match<'a>,
{
    ///
    /// Construct a new regex based on previous result.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*, ctor::DynamicCtorThenBuilderHelper};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let num = u8::is_ascii_digit
    ///         .repeat_one()
    ///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .map(map::from_str::<usize>());
    ///     let num = num.clone().sep_once(b",", num);
    ///     let re = num.into_ctor_then_builder(|_, a: &(usize, usize)| {
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
    fn into_ctor_then_builder<F, O1, R>(self, func: F) -> DynamicCtorThenBuilder<C, Self, F>
    where
        F: Fn(&mut C, &O1) -> Result<R, Error>,
    {
        DynamicCtorThenBuilder::new(self, func)
    }
}
