use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Match;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

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
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, T> Regex<C> for RegexNot<T>
where
    T: Regex<C>,
    <T as Regex<C>>::Ret: Ret,
    C: Context<'a> + Match<C>,
{
    type Ret = T::Ret;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut g = CtxGuard::new(ctx);
        let mut ret = Err(Error::Not);
        let beg = g.beg();
        let r = trace!("not", beg, g.try_mat(&self.val));

        if r.is_err() {
            ret = Ok(<T::Ret as Ret>::from_ctx(g.ctx(), (0, 0)));
        }
        trace!("not", beg => g.reset().end(), ret)
    }
}
