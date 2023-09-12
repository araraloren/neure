mod guard;
mod handler;
mod op;

pub use self::guard::CtxGuard;
pub use self::handler::*;
pub use self::op::OpExtension;

use crate::ctx::Context;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;

pub trait Mapper<'a, C, M, O>
where
    C: Context<'a>,
{
    fn map<H, A>(&self, ctx: &mut C, func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

pub trait MapperOrig<'a, C, O>
where
    C: Context<'a>,
{
    fn map_orig(&self, ctx: &mut C) -> Result<O, Error>;
}

impl<'a, C, O, T> MapperOrig<'a, C, O> for T
where
    C: Context<'a>,
    T: Mapper<'a, C, &'a C::Orig, O>,
    &'a C::Orig: Extract<'a, C, Span, Out<'a> = &'a C::Orig, Error = Error> + 'a,
{
    fn map_orig(&self, ctx: &mut C) -> Result<O, Error> {
        self.map(ctx, |orig: &'a C::Orig| Ok(orig))
    }
}

impl<'a, C: Context<'a, Orig = str>, R: Ret> Extract<'a, C, R> for &'a str {
    type Out<'b> = &'b str;

    type Error = Error;

    fn extract(ctx: &C, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(ret.fst(), ret.snd())
    }
}

impl<'a, C: Context<'a, Orig = [u8]>, R: Ret> Extract<'a, C, R> for &'a [u8] {
    type Out<'b> = &'b [u8];

    type Error = Error;

    fn extract(ctx: &C, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(ret.fst(), ret.snd())
    }
}

impl<'a, C: Context<'a, Orig = str>, R: Ret> Extract<'a, C, R> for String {
    type Out<'b> = String;

    type Error = Error;

    fn extract(ctx: &C, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(String::from(ctx.orig_sub(ret.fst(), ret.snd())?))
    }
}
