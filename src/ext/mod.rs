mod collect;
mod guard;
mod handler;
mod map;
mod parse;
mod pattern;
mod quote;
mod term;

pub use self::collect::Collect;
pub use self::guard::CtxGuard;
pub use self::handler::*;
pub use self::map::MapValue;
pub use self::parse::ParseExtension;
pub use self::pattern::Pattern;
pub use self::quote::Quote;
pub use self::term::Terminated;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub trait Mapper<'a, C, M, O>
where
    C: Context<'a>,
{
    fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

impl<'a, C, O, F> Mapper<'a, C, O, O> for F
where
    C: Context<'a> + Policy<C>,
    F: Fn(&mut C) -> Result<Span, Error>,
{
    fn map<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
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
        self.map(ctx, &mut |orig: &'a C::Orig| Ok(orig))
    }
}
