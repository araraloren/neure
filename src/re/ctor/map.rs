use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::map::MapSingle;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

///
/// Map the result to another type.
///
/// # Example 1
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let str = neu::ascii_alphabetic()
///         .repeat_times::<3>()
///         // map &str to String
///         .map(|v: &str| Ok(String::from(v)));
///     let num = neu::digit(10)
///         .repeat_times::<3>()
///         // map &str to i32
///         .map(|v: &str| v.parse::<i32>().map_err(|_| Error::Uid(0)));
///     let (who, id) = CharsCtx::new("foo777").ctor(&str.then(num))?;
///
///     assert_eq!(who, "foo");
///     assert_eq!(id, 777);
///
///     Ok(())
/// # }
/// ```
///
/// # Exampl 2
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let str = neu::ascii_alphabetic()
///         .repeat_times::<3>()
///         // map &str to String
///         .map(|v: (Span, _)| Ok(v));
///     let num = neu::digit(10)
///         .repeat_times::<3>()
///         // map &str to i32
///         .map(|(span, v): (Span, &str)| Ok((span, v.parse::<i32>().map_err(|_| Error::Uid(0))?)));
///     let (who, id) = // you can pass the Span to callback of map
///         CharsCtx::new("foo777").ctor_with(&str.then(num), &mut |span: Span, v: _| Ok((span, v)))?;
///
///     assert_eq!(who, (Span::new(0, 3), "foo"));
///     assert_eq!(id, (Span::new(3, 3), 777));
///
///     Ok(())
/// # }
/// ```
///
/// # Example 3
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     let num = neu::digit(10).repeat_times::<3>();
///     let id =
///         CharsCtx::new("777").map(&num, |v: &str| v.parse::<i32>().map_err(|_| Error::Uid(0)))?;
///
///     assert_eq!(id, 777);
///
///     let (span, id) = CharsCtx::new("777").map(&num, |v: &str, span: Span| {
///         Ok((span, v.parse::<i32>().map_err(|_| Error::Uid(0))?))
///     })?;
///
///     assert_eq!(id, 777);
///     assert_eq!(span, Span::new(0, 3));
///
///     Ok(())
/// # }
/// ```
#[derive(Default, Copy)]
pub struct Map<C, P, F, O> {
    pat: P,
    mapper: F,
    marker: PhantomData<(C, O)>,
}

impl<C, P, F, O> Debug for Map<C, P, F, O>
where
    P: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("pat", &self.pat)
            .field("mapper", &self.mapper)
            .finish()
    }
}

impl<C, P, F, O> Clone for Map<C, P, F, O>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            pat: self.pat.clone(),
            mapper: self.mapper.clone(),
            marker: self.marker,
        }
    }
}

impl<C, P, F, O> Map<C, P, F, O> {
    pub fn new(pat: P, func: F) -> Self {
        Self {
            pat,
            mapper: func,
            marker: PhantomData,
        }
    }

    pub fn pat(&self) -> &P {
        &self.pat
    }

    pub fn mapper(&self) -> &F {
        &self.mapper
    }

    pub fn pat_mut(&mut self) -> &mut P {
        &mut self.pat
    }

    pub fn mapper_mut(&mut self) -> &mut F {
        &mut self.mapper
    }

    pub fn set_pat(&mut self, pat: P) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_mapper(&mut self, func: F) -> &mut Self {
        self.mapper = func;
        self
    }

    pub fn map_to<H, V>(self, mapper: H) -> Map<C, P, H, O>
    where
        H: MapSingle<O, V>,
    {
        Map {
            pat: self.pat,
            mapper,
            marker: self.marker,
        }
    }
}

impl<'a, C, M, O, V, P, F> Ctor<'a, C, M, V> for Map<C, P, F, O>
where
    P: Ctor<'a, C, M, O>,
    F: MapSingle<O, V>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<V, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        self.mapper.map_to(self.pat.constrct(ctx, func)?)
    }
}

impl<'a, C, P, F, O> Regex<C> for Map<C, P, F, O>
where
    P: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
{
    type Ret = P::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        ctx.try_mat(&self.pat)
    }
}
