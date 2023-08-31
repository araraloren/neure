use crate::err::Error;
use crate::parser::Parser;
use crate::policy::Context;
use crate::policy::MatchPolicy;
use crate::policy::Ret;
use crate::SpanStore;

pub trait MatchExt<C>
where
    C: MatchPolicy + Context,
{
    fn mat<P>(&mut self, parser: P) -> Match<'_, C, P>
    where
        P: Parser<C, Ret = C::Ret>;

    fn cap<P, S>(&mut self, idx: S::Index, storer: &mut S, parser: P) -> Capture<'_, C, S, P>
    where
        S: SpanStore,
        P: Parser<C, Ret = C::Ret>;
}

#[derive(Debug)]
pub struct Capture<'a, C, S, P>
where
    S: SpanStore,
{
    parser: P,

    id: S::Id,

    ctx: &'a mut C,

    storer: &'a mut S,
}

impl<'a, C, S, P> Capture<'a, C, S, P>
where
    S: SpanStore,
{
    pub fn new(ctx: &'a mut C, id: S::Id, storer: &'a mut S, parser: P) -> Self {
        Self {
            ctx,
            id,
            storer,
            parser,
        }
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

    pub fn and_mat(
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

    pub fn and_cap<S>(
        self,
        id: S::Id,
        storer: &'a mut S,
        parser: impl Parser<C, Ret = C::Ret>,
    ) -> Capture<'a, C, S, impl FnOnce(&mut C, S::Id, &mut S) -> Result<C::Ret, Error>>
    where
        S: SpanStore,
    {
        let fst = self.parser;
        let snd = parser;
        let ctx = self.ctx;

        Capture::new(
            ctx,
            id,
            storer,
            move |ctx: &mut C, id: S::Id, storer: &mut S| -> Result<C::Ret, Error> {
                let fst = ctx.try_mat(fst)?;
                let snd = ctx.try_cap(id, storer, snd)?;

                Ok(<C::Ret>::new_from((
                    fst.count() + snd.count(),
                    fst.length() + snd.length(),
                )))
            },
        )
    }
}
