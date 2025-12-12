use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::map::MapSingle;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Transforms the output type of a pattern through a mapping function.
///
/// This combinator allows you to change the result type of a constructor without affecting
/// its matching behavior. It's particularly useful for converting parsed values into more
/// convenient or domain-specific types immediately after parsing.
///
/// # Regex
///
/// The `Regex` implementation simply delegates to the inner pattern. The mapping function
/// has no effect on matching behavior - it only affects construction results. This ensures
/// that pattern matching and token recognition remain unchanged while allowing flexible
/// transformation of parsed values.
///
/// ## Example
///
/// ```
/// # use neure::err::Error;
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let str = neu::ascii_alphabetic().repeat_times::<3>();
///     let num = neu::digit(10)
///         .repeat_times::<3>()
///         .try_map::<_, (Span, i32)>(|v: (Span, &str)| {
///             v.1.parse::<i32>()
///                 .map_err(|_| Error::Uid(0))
///                 .map(|n| (v.0, n))
///         });
///     let mut ctx = CharsCtx::new("foo777");
///
///     assert_eq!(ctx.try_mat(&str.then(num))?, Span::new(0, 6));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// First constructs a value of type `O` using the inner pattern, then applies the mapping
/// function to transform it into type `V`. If the inner pattern fails to construct a value,
/// the mapping function is never called and the error is propagated directly.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let str = regex!((neu::ascii_alphabetic()){3}).map(String::from);
///     let num = neu::digit(10)
///         .repeat_times::<3>()
///         // map &str to i32
///         .try_map(map::from_str::<i32>());
///     let re = str.then(num);
///
///     assert_eq!(CharsCtx::new("foo777").ctor(&re)?, ("foo".to_string(), 777));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Mapping Function
///
/// The mapping function must implement the [`MapSingle`] trait, which provides the
/// `map_to` method for converting from input type `O` to output type `V`. Common implementations
/// include:
/// - Simple closures: `|x| x.to_uppercase()`
/// - Function pointers: `str::parse`
/// - Struct constructors: `MyType::from`
/// - Complex transformations: `|x| MyType { value: x.trim().to_string() }`
///
/// ## Example
///
/// ```
/// # use neure::err::Error;
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let num = neu::digit(10).repeat_times::<3>();
///     let id =
///         CharsCtx::new("777").map(&num, |v: &str| v.parse::<i32>().map_err(|_| Error::Uid(0)))?;
///
///     assert_eq!(id, 777);
///
///     let (span, id) = CharsCtx::new("777").map_with(&num, |ctx, span| {
///         let orig = span.orig(ctx)?;
///
///         Ok((*span, orig.parse::<i32>().map_err(|_| Error::Uid(0))?))
///     })?;
///
///     assert_eq!(id, 777);
///     assert_eq!(span, Span::new(0, 3));
///
/// #   Ok(())
/// # }
/// ```
///
/// # Performance
///
/// The mapping operation adds negligible overhead when the mapping function itself is efficient.
/// Since the mapping only occurs after successful construction, there is no performance penalty
/// for failed matches. For optimal performance, ensure your mapping function is lightweight
/// or consider moving heavy transformations outside the parsing phase.
///
#[derive(Default, Copy)]
pub struct Map<C, P, F, O> {
    pat: P,
    mapper: F,
    marker: PhantomData<(C, O)>,
}

impl_not_for_regex!(Map<C, P, F, O>);

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

impl<'a, C, M, O, V, P, F, H> Ctor<'a, C, M, V, H> for Map<C, P, F, O>
where
    P: Ctor<'a, C, M, O, H>,
    F: MapSingle<O, V>,
    C: Match<'a>,
    H: Handler<C, Out = M>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<V, Error> {
        self.mapper.map_to(self.pat.construct(ctx, func)?)
    }
}

impl<'a, C, P, F, O> Regex<C> for Map<C, P, F, O>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        ctx.try_mat(&self.pat)
    }
}
