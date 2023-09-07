use crate::err::Error;

pub trait Extract<'a>
where
    Self: Sized,
{
    type Out<'b>;

    fn extract<'a>() -> Result<Self::Out, Self::Error>;
}

impl<Set, Ser> Extract<Set, Ser> for () {
    type Error = Error;

    fn extract(_set: &Set, _ser: &Ser, _ctx: &Ctx) -> Result<Self, Self::Error> {
        Ok(())
    }
}

/// Supress the error result.
/// Return the `Ok(Some(T))` if successful, otherwise return `Ok(None)`.
impl<T, Err, Set, Ser> Extract<Set, Ser> for Option<T>
where
    Err: Into<Error>,
    T: Extract<Set, Ser, Error = Err>,
{
    type Error = Err;

    fn extract(set: &Set, ser: &Ser, ctx: &Ctx) -> Result<Self, Self::Error> {
        match T::extract(set, ser, ctx) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

macro_rules! impl_extracter_for {
    ($($arg:ident)*) => {
        impl<Set, Ser, $($arg,)*> Extract<Set, Ser> for ($($arg,)*)
        where
            $(
                $arg: Extract<Set, Ser, Error = Error>,
            )*
        {
            type Error = Error;

            fn extract(set: &Set, ser: &Ser, ctx: &Ctx) -> Result<Self, Self::Error> {
                Ok(($($arg::extract(set, ser, ctx)?,)*))
            }
        }
    };
}