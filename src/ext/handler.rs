use crate::ctx::Context;
use crate::ctx::Ret;
use crate::err::Error;

pub trait Extract<'a, C: Context<'a>, R> {
    type Out<'b>;
    type Error: Into<Error>;

    fn extract(ctx: &C, beg: usize, ret: &R) -> Result<Self::Out<'a>, Self::Error>;
}

impl<'a, C: Context<'a>, R> Extract<'a, C, R> for () {
    type Out<'b> = ();

    type Error = Error;

    fn extract(_: &C, _: usize, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(())
    }
}

macro_rules! impl_extracter_for {
    ($($arg:ident)*) => {
        impl<'a, Ctx: Context<'a>, R, $($arg,)*> Extract<'a, Ctx, R> for ($($arg,)*)
        where
            $(
                $arg: Extract<'a, Ctx, R, Error = Error>,
            )*
        {
            type Out<'b> = ($(<$arg as Extract<'a, Ctx, R>>::Out<'b>,)*);

            type Error = Error;

            fn extract(ctx: &Ctx, beg: usize, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
                Ok(($($arg::extract(ctx, beg, ret)?,)*))
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

            #[inline]
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

impl<'a, C: Context<'a, Orig = str>, R: Ret> Extract<'a, C, R> for &'a str {
    type Out<'b> = &'b str;

    type Error = Error;

    fn extract(ctx: &C, beg: usize, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(beg, ret.length())
    }
}

impl<'a, C: Context<'a, Orig = [u8]>, R: Ret> Extract<'a, C, R> for &'a [u8] {
    type Out<'b> = &'b [u8];

    type Error = Error;

    fn extract(ctx: &C, beg: usize, ret: &R) -> Result<Self::Out<'a>, Self::Error> {
        ctx.orig_sub(beg, ret.length())
    }
}

impl<'a, C: Context<'a>, R> Extract<'a, C, R> for usize {
    type Out<'b> = usize;

    type Error = Error;

    fn extract(_: &C, beg: usize, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(beg)
    }
}

pub trait HandlerV<V, Args> {
    type Out;
    type Error: Into<Error>;

    fn invoke(&mut self, v: V, args: Args) -> Result<Self::Out, Self::Error>;
}

macro_rules! impl_handler_for {
    ($($arg:ident)*) => {
        impl<V, Func, Out, $($arg,)*> HandlerV<V, ($($arg,)*)> for Func
        where
            Func: FnMut(V, $($arg),*) -> Result<Out, Error>,
        {
            type Out = Out;
            type Error = Error;

            #[inline]
            #[allow(non_snake_case)]
            fn invoke(&mut self, v: V, ($($arg,)*): ($($arg,)*)) -> Result<Self::Out, Self::Error> {
                (self)(v, $($arg,)*)
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
