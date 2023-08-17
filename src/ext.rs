mod cap;
mod mat;
mod quote;
mod term;

use crate::{err::Error, Context, MatchPolicy, Parser, SpanStore};
use cap::CapThen;
use mat::MatThen;
use quote::Quote;
use term::Term;

pub trait PolicyExtension
where
    Self: Sized + MatchPolicy + Context,
{
    fn mat(
        &mut self,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> Result<MatThen<'_, Self, Self::Ret>, Error> {
        let start = self.offset();
        let ret = self.try_mat(parser);

        if ret.is_ok() {
            Ok(MatThen::new(self, start, ret.unwrap()))
        } else {
            Err(ret.err().unwrap())
        }
    }

    fn cap<'a, P, S>(
        &'a mut self,
        id: S::Id,
        storer: &mut S,
        parser: impl Parser<Self, Ret = Self::Ret>,
    ) -> Result<MatThen<'a, Self, Self::Ret>, Error>
    where
        S: SpanStore,
    {
        let start = self.offset();
        let ret = self.try_cap(id, storer, parser);

        if ret.is_ok() {
            Ok(MatThen::new(self, start, ret.unwrap()))
        } else {
            Err(ret.err().unwrap())
        }
    }

    // match left, then return quote with right
    fn quote<L, R>(&mut self, left: L, right: R) -> Result<Quote<'_, Self, R>, Error>
    where
        L: Parser<Self, Ret = Self::Ret>,
        R: Parser<Self, Ret = Self::Ret>,
    {
        let ret = self.try_mat(left);

        if ret.is_ok() {
            Ok(Quote::new(self, right))
        } else {
            Err(ret.err().unwrap())
        }
    }

    fn new_term<T, S>(&mut self, cont: T, sep: S) -> Term<'_, Self, T, S>
    where
        T: Parser<Self, Ret = Self::Ret>,
        S: Parser<Self, Ret = Self::Ret>,
    {
        Term::new(self, cont, sep)
    }
}

// map

// and

// or

// not

// seq

// then

// and_then

pub trait Map<C>
where
    C: MatchPolicy + Context,
{
    fn map<R>(self, map: impl FnMut(&C, usize, &C::Ret) -> Result<R, Error>) -> Result<R, Error>;

    fn map_orig<R>(self, map: impl FnMut(&C::Orig) -> Result<R, Error>) -> Result<R, Error>;
}

// pub trait PolicyExtension: MatchPolicy + Context
// where
//     Self: Sized,
// {
//     fn map<R>(
//         &mut self,
//         parser: impl Parser<Self, Ret = Self::Ret>,
//         mut map: impl FnMut(&Self, usize, Self::Ret) -> Result<R, Error>,
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
//         parser: impl Parser<Self, Ret = Self::Ret>,
//         mut map: impl FnMut(&<Self as Context>::Orig) -> Result<R, Error>,
//     ) -> Result<R, Error>
//     where
//         Self::Ret: Into<(usize, usize)>,
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
//         left: impl Parser<Self, Ret = Self::Ret>,
//         right: impl Parser<Self, Ret = Self::Ret>,
//         mut cont: impl FnMut(&mut Self) -> Result<R, Error>,
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
//         left: impl Parser<Self, Ret = Self::Ret>,
//         right: impl Parser<Self, Ret = Self::Ret>,
//         cont: impl Parser<Self, Ret = Self::Ret>,
//         mut map: impl FnMut(&Self, usize, Self::Ret) -> Result<R, Error>,
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
