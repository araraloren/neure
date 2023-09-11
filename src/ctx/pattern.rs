use crate::ctx::Policy;
use crate::err::Error;

use super::Span;

pub trait Pattern<T>
where
    Self: Sized,
{
    type Ret;

    fn try_parse(&self, ctx: &mut T) -> Result<Self::Ret, Error>;

    fn parse(&self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H, R> Pattern<T> for H
where
    H: Fn(&mut T) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(&self, ctx: &mut T) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

// pub struct True;

// impl<'a, T> Debug for True<'a, T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("True").field(&self.0).finish()
//     }
// }

// impl<'a, T> Clone for True<'a, T> {
//     fn clone(&self) -> Self {
//         Self(self.0)
//     }
// }

// impl<'a, T> Default for True<'a, T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

// impl<T, Ret> Pattern<T> for True
// where
//     T: Policy<T>,
// {
//     type Ret = Ret;

//     fn try_parse(&self, _: &mut T) -> Result<Self::Ret, Error> {
//         Ok(Span::new(0, 0))
//     }
// }
