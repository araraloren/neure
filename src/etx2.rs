use crate::err::Error;
use crate::parser::Parser;
use crate::policy::Context;
use crate::policy::MatchPolicy;
use crate::policy::Ret;

///
/// mat -> { map, and, or }
/// 
/// quote -> {
///     mat -> { map, and, or }
///     quote + parser -> ?
///     term
/// }
/// 
/// 
pub trait Extension<C>
where
    C: MatchPolicy + Context,
{
    fn mat<P: Parser<C, Ret = C::Ret>>(&mut self, parser: P) -> Match<'_, C, P>;

    fn quote<L, R>(&mut self, left: L, right: R) -> Quote<'_, L, R, C>
    where
        L: Parser<C, Ret = C::Ret>,
        R: Parser<C, Ret = C::Ret>;

    fn term<T, S>(&mut self, cont: T, sep: S) -> Term<'_, T, S, C>
    where
        T: Parser<C, Ret = C::Ret>,
        S: Parser<C, Ret = C::Ret>;
}

#[derive(Debug)]
pub struct Term<'a, T, S, C> {
    cont: T,

    sep: S,

    ctx: &'a mut C,
}

impl<'a, L, R, C> Term<'a, L, R, C> {
    pub fn new(ctx: &'a mut C, cont: L, sep: R) -> Self {
        Self { ctx, cont, sep }
    }
}

#[derive(Debug)]
pub struct Quote<'a, L, R, C> {
    left: L,

    right: R,

    ctx: &'a mut C,
}

impl<'a, L, R, C> Quote<'a, L, R, C> {
    pub fn new(ctx: &'a mut C, left: L, right: R) -> Self {
        Self { ctx, left, right }
    }
}

#[derive(Debug)]
pub struct Match<'a, C, P> {
    parser: P,

    ctx: &'a mut C,
}

impl<'a, C, P> Match<'a, C, P> {
    pub fn new(ctx: &'a mut C, parser: P) -> Self {
        Self { ctx, parser }
    }
}

impl<'a, C, P> Match<'a, C, P>
where
    C: MatchPolicy + Context,
    P: Parser<C, Ret = C::Ret>,
{
    pub fn map<R>(self, mut func: impl FnMut(&C::Orig) -> Result<R, Error>) -> Result<R, Error> {
        let start = self.ctx.offset();
        let ret = self.ctx.try_mat(self.parser)?;

        func(self.ctx.orig_sub(start, ret.length())?)
    }

    pub fn and(
        self,
        parser: impl Parser<C, Ret = C::Ret>,
    ) -> Match<'a, C, impl Parser<C, Ret = C::Ret>> {
        let fst = self.parser;
        let snd = parser;
        let ctx = self.ctx;

        Match::new(ctx, move |ctx: &mut C| -> Result<C::Ret, Error> {
            let fst = ctx.try_mat(fst)?;
            let snd = ctx.try_mat(snd)?;

            Ok(<C::Ret>::new_from((
                fst.count() + snd.count(),
                fst.length() + snd.length(),
            )))
        })
    }

    pub fn or(
        self,
        parser: impl Parser<C, Ret = C::Ret>,
    ) -> Match<'a, C, impl Parser<C, Ret = C::Ret>> {
        let fst = self.parser;
        let snd = parser;
        let ctx = self.ctx;

        Match::new(ctx, move |ctx: &mut C| -> Result<C::Ret, Error> {
            ctx.try_mat(fst).or(ctx.try_mat(snd))
        })
    }
}
