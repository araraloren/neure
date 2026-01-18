use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctor::Map;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::debug_ctor_stage;
use crate::err::Error;
use crate::map::Select0;
use crate::map::Select1;
use crate::map::SelectEq;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

///
/// Dynamically constructs a second pattern based on the result of the first match.
///
/// This combinator enables context-sensitive parsing where the second pattern depends on
/// the result of the first match. It's designed primarily for construction scenarios rather
/// than simple pattern matching.
///
/// # Regex
///
/// This struct does not support direct regex matching via the `Regex` trait. Any attempt to
/// use it as a regular expression pattern will panic with an unimplemented error. It exists
/// solely as a type system requirement and should not be used for pattern matching operations.
///
/// # Ctor
///
/// Performs a two-step construction process:
/// 1. First constructs a value `O1` using the initial pattern `pat`
/// 2. Then calls the function `func` with the context and the first result to dynamically
///    generate a second pattern
/// 3. Finally constructs a value `O2` using the dynamically generated pattern
/// 4. Returns the tuple `(O1, O2)` containing both results
///
/// The context position is advanced sequentially through both matches. If either step fails,
/// the context position is rolled back to its original state before the combinator was applied.
///
/// ## Example
///
/// ```
/// # use neure::{ctor::DynamicCtorThenBuilderHelper, prelude::*};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let len = regex::consume(2).try_map(map::from_le_bytes::<i16>());
///     let data = len.into_ctor_then_builder(|_, v| Ok(regex::consume(*v as usize)));
///     let ret = BytesCtx::new(b"\x1f\0Hello there, where are you from?").ctor(&data)?;
///
///     assert_eq!(ret, (0x1f, b"Hello there, where are you from".as_ref()));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Helper Trait
///
/// The [`DynamicCtorThenBuilderHelper`] trait provides a convenient builder interface.
///
/// ## Example
///
/// ```
/// # use neure::{ctor::DynamicCtorThenBuilderHelper, prelude::*};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let num = u8::is_ascii_digit
///         .once()
///         .try_map(map::from_utf8::<String>())
///         .try_map(map::from_str::<usize>());
///     let num = num.clone().sep_once(b",", num);
///     let regex =
///         num.into_ctor_then_builder(|_, (a, b)| Ok(b'+'.times(a).then(b'-'.times(b))));
///
///     assert_eq!(
///         BytesCtx::new(b"3,0+++").ctor(&regex)?,
///         ((3, 0), ([43, 43, 43].as_ref(), [].as_ref()))
///     );
///     assert_eq!(
///         BytesCtx::new(b"2,1++-").ctor(&regex)?,
///         ((2, 1), ([43, 43].as_ref(), [45].as_ref()))
///     );
///     assert_eq!(
///         BytesCtx::new(b"0,3---").ctor(&regex)?,
///         ((0, 3), ([].as_ref(), [45, 45, 45].as_ref()))
///     );
///
/// #   Ok(())
/// # }
/// ```
#[derive(Copy)]
pub struct DynamicCtorThenBuilder<C, P, F> {
    pat: P,
    func: F,
    marker: PhantomData<C>,
}

impl_not_for_regex!(DynamicCtorThenBuilder<C, P, F>);

impl<C, P, F> Debug for DynamicCtorThenBuilder<C, P, F>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicCtorBuilder")
            .field("pat", &self.pat)
            .field("func", &self.func)
            .finish()
    }
}

impl<C, P, F> Default for DynamicCtorThenBuilder<C, P, F>
where
    P: Default,
    F: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            func: Default::default(),
            marker: Default::default(),
        }
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
    C: Match<'a>,
{
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("DynamicCtorThenBuilder not support Regex trait")
    }
}

impl<'a, C, P, F, T, O1, O2, H> Ctor<'a, C, (O1, O2), H> for DynamicCtorThenBuilder<C, P, F>
where
    C: Match<'a>,
    P: Ctor<'a, C, O1, H>,
    T: Ctor<'a, C, O2, H>,
    F: Fn(&mut C, &O1) -> Result<T, Error>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let mut ctx = CtxGuard::new(ctx);

        crate::debug_ctor_beg!("DynamicCtorThenBuilder", ctx.beg());

        let l = {
            debug_ctor_stage!(
                "DynamicCtorThenBuilder",
                "left",
                self.pat.construct(ctx.ctx(), func)
            )
        };
        let l = ctx.process_ret(l)?;
        let r = {
            debug_ctor_stage!(
                "DynamicCtorThenBuilder",
                "right",
                (self.func)(ctx.ctx(), &l)?.construct(ctx.ctx(), func)
            )
        };
        let r = ctx.process_ret(r)?;

        crate::debug_ctor_reval!("DynamicCtorThenBuilder", ctx.beg(), ctx.end(), true);
        Ok((l, r))
    }
}

pub trait DynamicCtorThenBuilderHelper<'a, C>
where
    Self: Sized,
    C: Match<'a>,
{
    fn into_ctor_then_builder<F, O1, R>(self, func: F) -> DynamicCtorThenBuilder<C, Self, F>
    where
        F: Fn(&mut C, &O1) -> Result<R, Error>;
}

impl<'a, C, T> DynamicCtorThenBuilderHelper<'a, C> for T
where
    Self: Sized,
    C: Match<'a>,
{
    fn into_ctor_then_builder<F, O1, R>(self, func: F) -> DynamicCtorThenBuilder<C, Self, F>
    where
        F: Fn(&mut C, &O1) -> Result<R, Error>,
    {
        DynamicCtorThenBuilder::new(self, func)
    }
}
