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
pub struct Assert<T> {
    pat: T,
    value: bool,
}

impl<T> Assert<T> {
    pub fn new(pat: T, value: bool) -> Self {
        Self { pat, value }
    }

    pub fn pat(&self) -> &T {
        &self.pat
    }

    pub fn value(&self) -> bool {
        self.value
    }

    pub fn pat_mut(&mut self) -> &mut T {
        &mut self.pat
    }

    pub fn set_pat(&mut self, pat: T) -> &mut Self {
        self.pat = pat;
        self
    }

    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = value;
        self
    }

    pub fn with_value(mut self, value: bool) -> Self {
        self.value = value;
        self
    }
}

impl_not_for_regex!(Assert<T>);

impl<'a, C, O, T, H> Ctor<'a, C, O, O, H> for Assert<T>
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

impl<'a, C, T> Regex<C> for Assert<T>
where
    T: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::err::Error> {
        let mut ctx = CtxGuard::new(ctx);

        crate::debug_regex_beg!("Assert", ctx.beg());
        let ret = if ctx.try_mat(&self.pat).is_ok() == self.value {
            Ok(Span::new(ctx.beg(), 0))
        } else {
            Err(Error::Assert)
        };

        ctx.reset(); // force reset the offset
        crate::debug_regex_reval!("Assert", ret)
    }
}
