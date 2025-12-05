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

/// Reverse the result, return zero length [`Span`] if match failed.
///
/// # Regex
///
/// Return zero length [`Span`] if `T` match failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegexNot<T> {
    val: T,
}

impl<T> RegexNot<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

def_not!(RegexNot<T>);

impl<'a, C, O, T, H, A> Ctor<'a, C, O, O, H, A> for RegexNot<T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, T> Regex<C> for RegexNot<T>
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::RegexNot);

        crate::debug_regex_beg!("RegexNot", g.beg());
        if g.try_mat(&self.val).is_err() {
            ret = Ok(Span::new(g.beg(), 0));
        }
        crate::debug_regex_reval!("RegexNot", ret)
    }
}
