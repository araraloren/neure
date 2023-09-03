use std::marker::PhantomData;

use crate::{err::Error, policy::Ret, Context, MatchPolicy, Parser};

use super::CtxGuard;

#[derive(Debug)]
pub struct MatchThen<'a, C, P, PR, PO> {
    pre: PR,
    post: PO,
    parser: P,
    ctx: &'a mut C,
}

impl<'a, C, P, PR, PO> MatchThen<'a, C, P, PR, PO> {
    pub fn new(ctx: &'a mut C, pre: PR, post: PO, parser: P) -> Self {
        Self {
            ctx,
            pre,
            post,
            parser,
        }
    }
}

impl<'a, C, P, PR, PO> MatchThen<'a, C, P, PR, PO>
where
    C: MatchPolicy + Context,
    P: Parser<C, Ret = C::Ret>,
    PO: Parser<C, Ret = C::Ret>,
    PR: Parser<C, Ret = C::Ret>,
{
    pub fn map<R>(self, mut func: impl FnMut(&C::Orig) -> Result<R, Error>) -> Result<R, Error> {
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(self.parser)?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func(self.ctx.orig_sub(beg, ret.length())?)
    }

    pub fn map_pos<R>(
        self,
        mut func: impl FnMut(&C, usize, &C::Ret) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(self.parser)?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func(self.ctx, beg, &ret)
    }

    pub fn and(
        self,
        parser: impl crate::Parser<C, Ret = C::Ret>,
    ) -> MatchThen<'a, C, impl Parser<C, Ret = C::Ret>, PR, PO> {
        let fst = self.parser;
        let snd = parser;

        MatchThen::new(self.ctx, self.pre, self.post, |ctx: &mut C| {
            let mut gurad = CtxGuard::new(ctx);
            let fst = gurad.try_mat(fst)?;
            let snd = gurad.try_mat(snd)?;

            Ok(<C::Ret>::new_from((
                fst.count() + snd.count(),
                fst.length() + snd.length(),
            )))
        })
    }

    pub fn or(
        self,
        parser: impl crate::Parser<C, Ret = C::Ret>,
    ) -> MatchThen<'a, C, impl Parser<C, Ret = C::Ret>, PR, PO> {
        let fst = self.parser;
        let snd = parser;

        MatchThen::new(self.ctx, self.pre, self.post, |ctx: &mut C| {
            fst.try_parse(ctx).or(snd.try_parse(ctx))
        })
    }

    pub fn with_value<V>(
        self,
        val: V,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;

        MatThenValue::new(self.ctx, self.pre, self.post, |ctx: &mut C| {
            fst.try_parse(ctx).map(|v| (val, v))
        })
    }

    pub fn with_mapper<V>(
        self,
        mut mapper: impl FnMut(&C::Orig) -> Result<V, Error>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut C| {
            let beg = ctx.offset();
            let ret = fst.try_parse(ctx)?;

            Ok((mapper(ctx.orig_sub(beg, ret.length())?)?, ret))
        })
    }

    pub fn with_pos_mapper<V>(
        self,
        mut mapper: impl FnMut(&C, usize, &C::Ret) -> Result<V, Error>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut C| {
            let beg = ctx.offset();
            let ret = fst.try_parse(ctx)?;

            Ok((mapper(ctx, beg, &ret)?, ret))
        })
    }
}

#[derive(Debug)]
pub struct MatThenValue<'a, C, P, PR, PO, V> {
    ctx: &'a mut C,
    pre: PR,
    post: PO,
    parser: P,
    marker: PhantomData<V>,
}

impl<'a, C, P, PR, PO, V> MatThenValue<'a, C, P, PR, PO, V> {
    pub fn new(ctx: &'a mut C, pre: PR, post: PO, parser: P) -> Self {
        Self {
            ctx,
            pre,
            post,
            parser,
            marker: PhantomData,
        }
    }
}

impl<'a, C, P, PR, PO, V> MatThenValue<'a, C, P, PR, PO, V>
where
    C: MatchPolicy + Context,
    P: FnOnce(&mut C) -> Result<(V, C::Ret), Error>,
    PO: Parser<C, Ret = C::Ret>,
    PR: Parser<C, Ret = C::Ret>,
{
    pub fn map<R>(self, mut func: impl FnMut(V, &C::Orig) -> Result<R, Error>) -> Result<R, Error> {
        let mut val = None;
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(|ctx: &mut C| {
                (self.parser)(ctx).and_then(|(ret_val, ret)| {
                    val = Some(ret_val);
                    Ok(ret)
                })
            })?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func(val.unwrap(), self.ctx.orig_sub(beg, ret.length())?)
    }

    pub fn map_pos<R>(
        self,
        mut func: impl FnMut(V, &C, usize, &C::Ret) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let mut val = None;
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(|ctx: &mut C| {
                (self.parser)(ctx).and_then(|(ret_val, ret)| {
                    val = Some(ret_val);
                    Ok(ret)
                })
            })?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func(val.unwrap(), self.ctx, beg, &ret)
    }

    pub fn or_with(
        self,
        val: V,
        parser: impl crate::Parser<C, Ret = C::Ret>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;
        let snd = parser;

        MatThenValue::new(self.ctx, self.pre, self.post, |ctx: &mut C| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                snd.try_parse(ctx).map(|v| (val, v))
            }
        })
    }

    pub fn or_map(
        self,
        parser: impl crate::Parser<C, Ret = C::Ret>,
        mut mapper: impl FnMut(&C::Orig) -> Result<V, Error>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;
        let snd = parser;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut C| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                let beg = ctx.offset();
                let ret = snd.try_parse(ctx)?;

                Ok((mapper(ctx.orig_sub(beg, ret.length())?)?, ret))
            }
        })
    }

    pub fn or_map_pos(
        self,
        parser: impl crate::Parser<C, Ret = C::Ret>,
        mut mapper: impl FnMut(&C, usize, &C::Ret) -> Result<V, Error>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.parser;
        let snd = parser;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut C| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                let beg = ctx.offset();
                let ret = snd.try_parse(ctx)?;

                Ok((mapper(ctx, beg, &ret)?, ret))
            }
        })
    }
}
