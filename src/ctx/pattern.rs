use std::fmt::Debug;
use std::marker::PhantomData;

use crate::err::Error;
use crate::policy::Policy;
use crate::policy::Ret;

pub trait Pattern<T>
where
    Self: Sized,
{
    type Ret: Ret;

    fn try_parse(self, ctx: &mut T) -> Result<Self::Ret, Error>;

    fn parse(self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H, R> Pattern<T> for H
where
    R: Ret,
    H: FnOnce(&mut T) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(self, ctx: &mut T) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}

pub struct True<'a, T>(PhantomData<(&'a (), T)>);

impl<'a, T> Debug for True<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("True").field(&self.0).finish()
    }
}

impl<'a, T> Clone for True<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, T> Default for True<'a, T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, T> Pattern<T> for True<'a, T>
where
    T: Policy<T>,
{
    type Ret = T::Ret;

    fn try_parse(self, _: &mut T) -> Result<Self::Ret, Error> {
        Ok(<Self::Ret>::new_from((0, 0)))
    }
}
