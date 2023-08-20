mod cap;
mod mat;
mod quote;
mod term;

use crate::{err::Error, Context, MatchPolicy, Parser, SpanStore};
use cap::CapThen;
use mat::MatThen;
use quote::Quote;
use term::Term;

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
