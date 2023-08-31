use crate::{err::Error, policy::Ret, CharsCtx, Context, MatchPolicy, Parser, SpanStore};

pub trait PolicyExt<C>
where
    C: MatchPolicy + Context,
{
    fn ctx(&mut self) -> &mut C;

    fn mat<P>(&mut self, parser: P) -> Map<'_, C, P>
    where
        P: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>,
    {
        Map {
            ctx: self.ctx(),
            parser,
        }
    }

    fn term<T, S>(&mut self, cont: T, sep: S) -> Term<'_, C, T, S>
    where
        T: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
        S: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
    {
        Term {
            ctx: self.ctx(),
            cont,
            sep,
        }
    }
}

pub struct Term<'a, C, T, S>
where
    C: MatchPolicy + Context,
    T: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
    S: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
{
    ctx: &'a mut C,
    cont: T,
    sep: S,
}

impl<'a, C, T, S> Term<'a, C, T, S>
where
    C: MatchPolicy + Context,
    T: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
    S: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error> + Clone,
{
    fn next(
        &mut self,
    ) -> Map<'_, C, impl FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>> {
        let cont = self.cont.clone();
        let sep = self.sep.clone();

        Map {
            ctx: self.ctx,
            parser: move |ctx: &mut C| -> Result<<C as MatchPolicy>::Ret, Error> {
                let ret = cont.try_parse(ctx)?;

                sep.parse(ctx);
                Ok(ret)
            },
        }
    }
}

pub struct Map<'a, C, P>
where
    C: MatchPolicy + Context,
    P: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>,
{
    ctx: &'a mut C,
    parser: P,
}

impl<'a, C, P> Map<'a, C, P>
where
    C: MatchPolicy + Context,
    P: FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>,
{
    pub fn run(self) -> Result<C::Ret, Error> {
        (self.parser)(self.ctx)
    }

    pub fn and(
        self,
        parser: impl FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>,
    ) -> Map<'a, C, impl FnOnce(&mut C) -> Result<<C as MatchPolicy>::Ret, Error>> {
        let fst = self.parser;

        Map {
            ctx: self.ctx,
            parser: move |ctx: &mut C| -> Result<<C as MatchPolicy>::Ret, Error> {
                let fst = fst.try_parse(ctx)?;
                let snd = parser.try_parse(ctx)?;

                Ok(<C as MatchPolicy>::Ret::new_from((
                    fst.count() + snd.count(),
                    fst.length() + snd.length(),
                )))
            },
        }
    }
}

impl PolicyExt<Self> for CharsCtx<'_> {
    fn ctx(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let mut c = CharsCtx::new("++++++");
        let value = neure!('+');
        let mut map = c.term(&value, &value);
        let mut and = map.next();

        dbg!(and.run());
    }
}

// pub trait PolicyExt<C>
// where
//     C: MatchPolicy + Context,
// {
//     fn ctx(&mut self) -> &mut C;

//     fn mat(&mut self, parser: impl Parser<C, Ret = C::Ret>) -> MatThen<'_, C> {
//         let ctx = self.ctx();
//         let beg = ctx.offset();

//         MatThen::new(ctx, beg, ctx.try_mat(parser))
//     }

//     fn cap<'a, P, S>(
//         &'a mut self,
//         id: S::Id,
//         storer: &mut S,
//         parser: impl Parser<C, Ret = C::Ret>,
//     ) -> Result<MatThen<'a, C>, Error>
//     where
//         S: SpanStore,
//     {
//         let ctx = self.ctx();
//         let start = ctx.offset();
//         let ret = ctx.try_cap(id, storer, parser);

//         if ret.is_ok() {
//             Ok(MatThen::new(ctx, start, ret.unwrap()))
//         } else {
//             Err(ret.err().unwrap())
//         }
//     }

//     // match left, then return quote with right
//     fn quote<L, R>(&mut self, left: L, right: R) -> Result<Quote<'_, C, R>, Error>
//     where
//         L: Parser<C, Ret = C::Ret>,
//         R: Parser<C, Ret = C::Ret>,
//     {
//         let ctx = self.ctx();
//         let ret = ctx.try_mat(left);

//         if ret.is_ok() {
//             Ok(Quote::new(ctx, right))
//         } else {
//             Err(ret.err().unwrap())
//         }
//     }

//     // fn new_term<T, S>(&mut self, cont: T, sep: S) -> Term<'_, C, T, S>
//     // where
//     //     T: Parser<C, Ret = C::Ret>,
//     //     S: Parser<C, Ret = C::Ret>,
//     // {
//     //     Term::new(self, cont, sep)
//     // }
// }

// map

// and

// or

// not

// seq

// then

// and_then

// pub trait Operator<C>
// where
//     C: MatchPolicy + Context,
// {
//     fn map<R>(&mut self, func: impl FnMut(&C, usize, &C::Ret) -> R) -> Result<R, Error>;

//     fn map_orig<R>(&mut self, func: impl FnMut(&C::Orig) -> R) -> Result<R, Error>;

//     fn and<R>(&mut self, parser: impl Parser<C, Ret = C::Ret>) -> Result<MatThen<'_, C>, Error>;
// }

// pub trait PolicyExtension: MatchPolicy + Context
// where
//     C: Sized,
// {
//     fn map<R>(
//         &mut self,
//         parser: impl Parser<C, Ret = C::Ret>,
//         mut map: impl FnMut(&C, usize, C::Ret) -> Result<R, Error>,
//     ) -> Result<R, Error> {
//         let start = self.offset();
//         let ret = self.try_mat(parser);

//         if ret.is_ok() {
//             map(self, start, ret?)
//         } else {
//             Err(ret.err().unwrap())
//         }
//     }

//     fn map_orig<R>(
//         &mut self,
//         parser: impl Parser<C, Ret = C::Ret>,
//         mut map: impl FnMut(&<C as Context>::Orig) -> Result<R, Error>,
//     ) -> Result<R, Error>
//     where
//         C::Ret: Into<(usize, usize)>,
//     {
//         let start = self.offset();
//         let ret = self.try_mat(parser);

//         if ret.is_ok() {
//             let (_, len) = ret?.into();

//             map(self.orig_sub(start, len)?)
//         } else {
//             Err(ret.err().unwrap())
//         }
//     }

//     fn quote_cont<R>(
//         &mut self,
//         left: impl Parser<C, Ret = C::Ret>,
//         right: impl Parser<C, Ret = C::Ret>,
//         mut cont: impl FnMut(&mut C) -> Result<R, Error>,
//     ) -> Result<R, Error> {
//         if self.mat(left) {
//             let ret = cont(self);

//             self.try_mat(right)?;
//             ret
//         } else {
//             Err(Error::Quote)
//         }
//     }

//     fn quote<R>(
//         &mut self,
//         left: impl Parser<C, Ret = C::Ret>,
//         right: impl Parser<C, Ret = C::Ret>,
//         cont: impl Parser<C, Ret = C::Ret>,
//         mut map: impl FnMut(&C, usize, C::Ret) -> Result<R, Error>,
//     ) -> Result<R, Error> {
//         if self.mat(left) {
//             let start = self.offset();
//             let ret = self.try_mat(cont)?;
//             let ret = map(self, start, ret)?;

//             self.try_mat(right)?;
//             Ok(ret)
//         } else {
//             Err(Error::Quote)
//         }
//     }
// }

// impl<T: MatchPolicy + Context> PolicyExtension for T {}

// pub fn terminated<C>(
//     cont: impl Fn(&mut C) -> Result<C::Ret, Error>,
//     sep: impl Fn(&mut C) -> Result<C::Ret, Error>,
//     min: usize,
//     sep_need: bool,
// ) -> impl Fn(&mut C) -> Result<C::Ret, Error>
// where
//     C: Context + MatchPolicy,
// {
//     move |ctx: &mut C| {
//         let ret = cont(ctx);

//         if min == 0 && ret.is_err() {
//             return Ok()
//         }

//         Ok(ret)
//     }
// }
