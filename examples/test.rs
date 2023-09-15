use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use neure::err::Error;
use neure::ext::*;
use neure::prelude::*;
use neure::*;

// pub struct Terminated<S, P, C> {
//     sep: S,
//     pat: P,
//     marker: PhantomData<C>,
// }

// impl<'a, S, P, C> Terminated<S, P, C>
// where
//     P: Parse<C>,
//     C: Context<'a> + Policy<C>,
// {
//     pub fn map<H, A, I, O>(
//         self,
//         mut func: H,
//     ) -> Map<Self, C, impl FnMut(&mut C, &I) -> Result<O, Error>, O>
//     where
//         I: Parse<C>,
//         I::Ret: Ret,
//         H: Handler<A, Out = O, Error = Error>,
//         A: Extract<'a, C, I::Ret, Out<'a> = A, Error = Error>,
//     {
//         Map {
//             pat: self,
//             func: move |ctx: &mut C, pat: &I| {
//                 let ret = ctx.try_mat(pat)?;

//                 func.invoke(A::extract(ctx, ret.fst(), &ret)?)
//             },
//             marker: PhantomData,
//         }
//     }
// }

// impl<'a, C, S, P> Parse<C> for Terminated<S, P, C>
// where
//     P: Parse<C>,
//     C: Context<'a> + Policy<C>,
//     S: Parse<C, Ret = P::Ret>,
// {
//     type Ret = P::Ret;

//     fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, err::Error> {
//         let ret = ctx.try_mat(&self.pat)?;

//         ctx.try_mat(&self.sep)?;
//         Ok(ret)
//     }
// }

// pub struct Map<P, C, F, T> {
//     pat: P,
//     func: F,
//     marker: PhantomData<(C, T)>,
// }

// impl<P, C, F, T> Map<P, C, F, T> {
//     pub fn collect(self) -> Collect<Self, C, T> {
//         Collect {
//             pat: self,
//             marker: PhantomData,
//         }
//     }
// }

// impl<'a, P, C, F, T> Map<P, C, F, T>
// where
//     P: Parse<C, Ret = Span>,
//     C: Context<'a> + Policy<C>,
//     F: FnMut(&mut C, &P) -> Result<T, Error>,
// {
//     fn next(&mut self, ctx: &mut C) -> Result<T, Error> {
//         (self.func)(ctx, &self.pat)
//     }
// }

// pub struct Collect<P, C, T> {
//     pat: P,
//     marker: PhantomData<(C, T)>,
// }

// impl<'a, P, C, T> Collect<P, C, T>
// where
//     P: Parse<C>,
//     C: Context<'a> + Policy<C>,
// {
//     pub fn map<H, A, I>(
//         self,
//         mut func: H,
//     ) -> Map<Self, C, impl FnMut(&mut C, &I) -> Result<Vec<T>, Error>, Vec<T>>
//     where
//         I: Parse<C>,
//         I::Ret: Ret,
//         H: Handler<A, Out = Vec<T>, Error = Error>,
//         A: Extract<'a, C, I::Ret, Out<'a> = A, Error = Error>,
//     {
//         Map {
//             pat: self,
//             func: move |ctx: &mut C, pat: &I| {
//                 let ret = ctx.try_mat(pat)?;

//                 func.invoke(A::extract(ctx, ret.fst(), &ret)?)
//             },
//             marker: PhantomData,
//         }
//     }
// }

// pub struct Quote<L, R, P, C> {
//     pat: P,
//     left: L,
//     right: R,
//     marker: PhantomData<C>,
// }

// impl<'a, L, R, P, C> Quote<L, R, P, C>
// where
//     P: Parse<C>,
//     C: Context<'a>,
// {
//     pub fn map<H, A, O>(
//         self,
//         mut func: H,
//     ) -> Map<Self, C, impl FnMut(&C, usize, &P::Ret) -> Result<O, Error>, O>
//     where
//         H: Handler<A, Out = O, Error = Error>,
//         A: Extract<'a, C, P::Ret, Out<'a> = A, Error = Error>,
//     {
//         Map {
//             pat: self,
//             func: move |ctx: &C, beg, ret: &P::Ret| func.invoke(A::extract(ctx, beg, &ret)?),
//             marker: PhantomData,
//         }
//     }
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let digits = neure!(['0' - '9']+);
//     let comma = neure!([',']{0,1});
//     let term = Terminated {
//         sep: comma,
//         pat: digits,
//         marker: PhantomData,
//     };
//     let mut ctx = CharsCtx::new("123,4354,6546,675");
//     let mut map = term.map(|str: &str| Ok(str.to_owned()));
//     let mut collect = map.collect();

//     while let Ok(val) = map.next(&mut ctx) {
//         println!("get return value = {val}");
//     }

//     Ok(())
// }

#[derive(Debug)]
pub enum Value<'a> {
    A(i64),

    B(&'a str),

    C(&'a str),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let digits = neure!(['0' - '9']+);
    // let letters = neure!(['A' - 'Z']+);
    // let comma = neure!(','{0,1});
    let mut ctx = CharsCtx::new("[{[{123}]}]");
    let then: Rc<dyn Parse<_, Ret = _>> = Rc::new(parser::null());
    let quote1 = Rc::new(RefCell::new(digits.quote('{', '}').then(then)));
    let quote2: Rc<dyn Parse<_, Ret = _>> =
        Rc::new(parser::null().quote("[", ']').then(quote1.clone()));

    quote1.borrow_mut().set_then(quote2.clone());

    let val = ctx.try_mat(&quote1)?;

    dbg!(val);

    Ok(())
}
