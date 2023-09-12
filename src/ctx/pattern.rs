use crate::err::Error;

pub trait Parse<C> {
    type Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error>;

    fn parse(&self, ctx: &mut C) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<C, H, R> Parse<C> for H
where
    H: Fn(&mut C) -> Result<R, Error>,
{
    type Ret = R;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        (self)(ctx)
    }
}
