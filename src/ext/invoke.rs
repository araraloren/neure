use super::Extract;
use super::Handler;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;

pub trait Invoke<'a, C, M, O>
where
    C: Context<'a>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

impl<'a, C, O, F> Invoke<'a, C, O, O> for F
where
    C: Context<'a> + Policy<C>,
    F: Fn(&mut C) -> Result<Span, Error>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

// pub trait InvokeOrig<'a, C, O>
// where
//     C: Context<'a>,
// {
//     fn invoke(&self, ctx: &mut C) -> Result<O, Error>;
// }

// impl<'a, C, O, T> InvokeOrig<'a, C, O> for T
// where
//     C: Context<'a>,
//     T: Invoke<'a, C, &'a C::Orig, O>,
//     &'a C::Orig: Extract<'a, C, Span, Out<'a> = &'a C::Orig, Error = Error> + 'a,
// {
//     fn invoke(&self, ctx: &mut C) -> Result<O, Error> {
//         self.invoke_with(ctx, &mut |orig: &'a C::Orig| Ok(orig))
//     }
// }

// pub trait InvokeSpan<'a, C, O>
// where
//     C: Context<'a>,
// {
//     fn invoke_span(&self, ctx: &mut C) -> Result<O, Error>;
// }

// impl<'a, C, O, T> InvokeSpan<'a, C, O> for T
// where
//     C: Context<'a>,
//     T: Invoke<'a, C, Span, O>,
//     Span: Extract<'a, C, Span, Out<'a> = Span, Error = Error> + 'a,
// {
//     fn invoke_span(&self, ctx: &mut C) -> Result<O, Error> {
//         self.invoke_with(ctx, &mut |orig: Span| Ok(orig))
//     }
// }
