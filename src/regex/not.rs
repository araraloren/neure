use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

/// Reverse the result, return zero length [`Span`] if match failed.
///
/// # Regex
///
/// Return zero length [`Span`] if `T` match failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Not<T> {
    val: T,
}

impl<T> Not<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl_not_for_regex!(Not<T>);

impl<'a, C, O, T, H> Ctor<'a, C, O, O, H> for Not<T>
where
    T: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C, T> Regex<C> for Not<T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Not);

        crate::debug_regex_beg!("Not", g.beg());
        if g.try_mat(&self.val).is_err() {
            ret = Ok(Span::new(g.beg(), 0));
        }
        crate::debug_regex_reval!("Not", ret)
    }
}
