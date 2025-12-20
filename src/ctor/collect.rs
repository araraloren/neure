use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;

///
/// Collect repeated matches of a pattern into a collection.
///
/// Attempts to match the inner pattern repeatedly until it fails, collecting
/// the results into a container type. Requires at least `min` successful matches
/// to succeed, otherwise fails and resets the context position.
///
/// # Regex
///
/// Matches the inner pattern repeatedly and combines the resulting spans.
/// The combined span covers the entire matched region from the first to the
/// last successful match. Requires at least `min` matches to succeed.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let regex = b"+".collect::<Span, Vec<Span>>();
///
///     assert!(BytesCtx::new(b"---A").try_mat(&regex).is_err());
///     assert_eq!(BytesCtx::new(b"+++A").try_mat(&regex)?, Span::new(0, 3));
///
///     let regex = b"+".collect::<Span, Vec<Span>>().at_least(3);
///
///     assert!(BytesCtx::new(b"++-A").try_mat(&regex).is_err(),);
///
/// #   Ok(())
/// # }
/// ```
///
/// # Ctor
///
/// Constructs values from each successful match of the inner pattern and
/// collects them into a container type `V` (which must implement [`FromIterator`]).
/// The collection process stops when the pattern fails to match. Requires at
/// least `min` successful matches to return a valid collection.
///
/// ## Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let regex = b"+".collect::<_, Vec<_>>();
///
///     assert!(BytesCtx::new(b"---A").ctor(&regex).is_err());
///     assert_eq!(BytesCtx::new(b"+++A").ctor(&regex)?, vec![b"+", b"+", b"+"]);
///     assert_eq!(BytesCtx::new(b"++-A").ctor(&regex)?, vec![b"+", b"+"]);
///
/// #   Ok(())
/// # }
/// ```
///
/// # Greedy Behavior
///
/// This combinator is greedy - it will continue matching until the pattern fails.
/// It does not backtrack between matches. If minimum requirements aren't met,
/// the context position is reset to the starting point.
#[derive(Copy)]
pub struct Collect<C, P, O, V> {
    pat: P,
    min: usize,
    marker: PhantomData<(O, V, C)>,
}

impl_not_for_regex!(Collect<C, P, O, V>);

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

impl<C, P, O, V> Default for Collect<C, P, O, V>
where
    P: Default,
{
    fn default() -> Self {
        Self {
            pat: Default::default(),
            min: Default::default(),
            marker: Default::default(),
        }
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

impl<'a, C, P, O, V, H> Ctor<'a, C, V, H> for Collect<C, P, O, V>
where
    V: FromIterator<O>,
    P: Ctor<'a, C, O, H>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<V, Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let val = {
            crate::debug_ctor_beg!("Collect", offset);
            V::from_iter(std::iter::from_fn(|| {
                self.pat.construct(ctx, func).ok().inspect(|_| {
                    cnt += 1;
                })
            }))
        };
        let ret = if cnt >= self.min {
            Ok(val)
        } else {
            Err(Error::Collect)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_ctor_reval!("Collect", offset, ctx.offset(), ret.is_ok());
        ret
    }
}

impl<'a, C, P, O, V> Regex<C> for Collect<C, P, O, V>
where
    P: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let offset = ctx.offset();
        let mut cnt = 0;
        let mut span = Span::new(offset, 0);

        // don't use g.try_mat, it will set reset when failed
        crate::debug_regex_beg!("Collect", offset);
        while let Ok(ret) = ctx.try_mat(&self.pat) {
            cnt += 1;
            span.add_assign(ret);
        }
        let ret = if cnt >= self.min {
            Ok(span)
        } else {
            Err(Error::Collect)
        }
        .inspect_err(|_| {
            ctx.set_offset(offset);
        });

        crate::debug_regex_reval!("Collect", ret)
    }
}
