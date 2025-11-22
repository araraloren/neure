use crate::ctx::Context;
use crate::ctx::Span;
use crate::err::Error;

pub trait Extract<'a, C: Context<'a>> {
    type Out<'b>;
    type Error: Into<Error>;

    fn extract(ctx: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error>;
}

impl<'a, C: Context<'a>> Extract<'a, C> for () {
    type Out<'b> = ();

    type Error = Error;

    fn extract(_: &C, _: &Span) -> Result<Self::Out<'a>, Self::Error> {
        Ok(())
    }
}

macro_rules! impl_extracter_for {
    ($($arg:ident)*) => {
        impl<'a, Ctx: Context<'a>, $($arg,)*> Extract<'a, Ctx> for ($($arg,)*)
        where
            $(
                $arg: Extract<'a, Ctx, Error = Error>,
            )*
        {
            type Out<'b> = ($(<$arg as Extract<'a, Ctx>>::Out<'b>,)*);

            type Error = Error;


            fn extract(ctx: &Ctx, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
                Ok(($($arg::extract(ctx, ret)?,)*))
            }
        }
    };
}

impl_extracter_for!(A);

impl_extracter_for!(A B);

impl_extracter_for!(A B C);

impl_extracter_for!(A B C D);

impl_extracter_for!(A B C D E);

impl_extracter_for!(A B C D E F);

pub trait Handler<Args> {
    type Out;
    type Error: Into<Error>;

    fn invoke(&mut self, args: Args) -> Result<Self::Out, Self::Error>;
}

macro_rules! impl_handler_for {
    ($($arg:ident)*) => {
        impl<Func, Out, $($arg,)*> Handler<($($arg,)*)> for Func
        where
            Func: FnMut($($arg),*) -> Result<Out, Error>,
        {
            type Out = Out;
            type Error = Error;


            #[allow(non_snake_case)]
            fn invoke(&mut self, ($($arg,)*): ($($arg,)*)) -> Result<Self::Out, Self::Error> {
                (self)($($arg,)*)
            }
        }
    };
}

impl_handler_for!();

impl_handler_for!(A);

impl_handler_for!(A B);

impl_handler_for!(A B C);

impl_handler_for!(A B C D);

impl_handler_for!(A B C D E);

impl_handler_for!(A B C D E F);

impl<'a, C: Context<'a, Orig = str>> Extract<'a, C> for &'a str {
    type Out<'b> = &'b str;

    type Error = Error;

    fn extract(ctx: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(ret.begin(), ret.length())
    }
}

impl<'a, C: Context<'a, Orig = [u8]>> Extract<'a, C> for &'a [u8] {
    type Out<'b> = &'b [u8];

    type Error = Error;

    fn extract(ctx: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(ret.begin(), ret.length())
    }
}

impl<'a, C: Context<'a, Orig = str>> Extract<'a, C> for String {
    type Out<'b> = String;

    type Error = Error;

    fn extract(ctx: &C, ret: &Span) -> Result<Self::Out<'a>, Self::Error> {
        Ok(String::from(ctx.orig_sub(ret.begin(), ret.length())?))
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pass;

impl<T> Handler<T> for Pass {
    type Out = T;

    type Error = Error;

    fn invoke(&mut self, args: T) -> Result<Self::Out, Self::Error> {
        Ok(args)
    }
}
