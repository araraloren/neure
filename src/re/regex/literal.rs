use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::def_not;
use crate::re::trace;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

/// Match given slice in the [`Context`].
///
/// # Regex
///
/// Return a [`Span`] as match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitSlice<'a, T> {
    val: &'a [T],
}

def_not!(LitSlice<'a, T>);

impl<'a, T> LitSlice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self { val }
    }
}

impl<'a, 'b, C, O, T, H, A> Ctor<'a, C, O, O, H, A> for LitSlice<'b, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig = [T]> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, 'b, C, T> Regex<C> for LitSlice<'b, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig = [T]>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut ret = Err(Error::Slice);
        let len = self.val.len();
        let beg = ctx.offset();

        if ctx.orig()?.starts_with(self.val) {
            ctx.inc(len);
            ret = Ok(Span::new(beg, len));
        }
        trace!("slice", beg => ctx.offset(), ret)
    }
}

/// Match given string in the [`Context`].
///
/// # Regex
///
/// Return a [`Span`] as match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitString<'a> {
    val: &'a str,
}

def_not!(LitString<'a>);

impl<'a> LitString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, 'b, C, O, H, A> Ctor<'a, C, O, O, H, A> for LitString<'b>
where
    C: Context<'a, Orig = str> + Match<C>,
    H: Handler<A, Out = O, Error = Error>,
    A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
{
    #[inline(always)]
    fn constrct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, 'b, C> Regex<C> for LitString<'b>
where
    C: Context<'a, Orig = str>,
{
    type Ret = Span;

    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut ret = Err(Error::String);
        let len = self.val.len();
        let beg = ctx.offset();

        if ctx.orig()?.starts_with(self.val) {
            ctx.inc(len);
            ret = Ok(Span::new(beg, len));
        }
        trace!("string", beg => ctx.offset(), ret)
    }
}
