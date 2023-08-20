use crate::{err::Error, policy::Ret, Context, MatchPolicy, Parser,};



#[derive(Debug)]
pub struct MatThen<'a, C, P> {
    ctx: &'a mut C,
    parsers: Option<P>,
}

impl<'a, C, P> MatThen<'a, C, P> {
    pub fn new(ctx: &'a mut C, parsers: P) -> Self {
        Self { ctx, parsers: Some(parsers) }
    }
}

// impl<'a, C> PolicyExt<C> for MatThen<'a, C>
// where
//     C: MatchPolicy + Context,
// {
//     fn ctx(&mut self) -> &mut C {
//         self.ctx
//     }
// }

// impl<'a, C, P> MatThen<'a, C, (P, )>
// where
//     C: MatchPolicy + Context,
//     P: Parser<C, Ret = C::Ret>,
// {
//     fn map<R>(
//         self,
//         mut func: impl FnMut(&C, usize, &<C as MatchPolicy>::Ret) -> R,
//     ) -> Result<R, crate::err::Error> {
//         let beg = self.ctx.offset();
//         let ret = self.ctx.try_mat(self.parsers.0);

//         ret.map(|ret| func(self.ctx, beg, &ret))
//     }

//     fn and<R>(
//         self,
//         parser: impl crate::Parser<C, Ret = <C as MatchPolicy>::Ret>,
//     ) -> MatThen<'a, C, (impl Parser<C, Ret = C::Ret>, impl Parser<C, Ret = C::Ret>)> {
//         MatThen::new(self.ctx, (self.parsers.0, parser))
//     }
// }

impl<'a, C, P> MatThen<'a, C, P>
where
    C: MatchPolicy + Context,
    P: FnOnce(&mut C) -> Result<C::Ret, Error>,
{
    fn map<R>(
        self,
        mut func: impl FnMut(&C, usize, &<C as MatchPolicy>::Ret) -> R,
    ) -> Result<R, crate::err::Error> {
        let beg = self.ctx.offset();
        let ret = self.ctx.try_mat(self.parsers.unwrap());

        ret.map(|ret| func(self.ctx, beg, &ret))
    }

    fn and<R>(
        mut self,
        parser: impl FnMut(&mut C) -> Result<C::Ret, Error>,
    ) -> MatThen<'a, C, impl FnOnce(&mut C) -> Result<C::Ret, Error>> {
        let parser2 = self.parsers.take().unwrap();

        MatThen::new(self.ctx, move |ctx: &mut C| {
            let ret = ctx.try_mat(parser2);

            if ret.is_ok() {
                ctx.try_mat(parser)
            }
            else {
                Err(ret.err().unwrap())
            }
        })
    }
}