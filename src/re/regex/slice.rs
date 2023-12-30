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
pub struct RegexSlice<'a, T> {
    val: &'a [T],
}

def_not!(RegexSlice<'a, T>);

impl<'a, T> RegexSlice<'a, T> {
    pub fn new(val: &'a [T]) -> Self {
        Self { val }
    }
}

impl<'a, 'b, C, O, T> Ctor<'a, C, O, O> for RegexSlice<'b, T>
where
    T: PartialOrd + 'a,
    C: Context<'a, Orig = [T]> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, 'b, C, T> Regex<C> for RegexSlice<'b, T>
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
pub struct RegexString<'a> {
    val: &'a str,
}

def_not!(RegexString<'a>);

impl<'a> RegexString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, 'b, C, O> Ctor<'a, C, O, O> for RegexString<'b>
where
    C: Context<'a, Orig = str> + Match<C>,
{
    #[inline(always)]
    fn constrct<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, 'b, C> Regex<C> for RegexString<'b>
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
