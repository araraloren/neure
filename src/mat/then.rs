use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::err::Error;
use crate::policy::Policy;
use crate::policy::Ret;

use super::CtxGuard;

#[derive(Debug)]
pub struct Then<'a, Ctx, Pa, Pr, Po> {
    pre: Pr,
    post: Po,
    pattern: Pa,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Pa, Pr, Po> Then<'a, Ctx, Pa, Pr, Po> {
    pub fn new(ctx: &'a mut Ctx, pre: Pr, post: Po, pattern: Pa) -> Self {
        Self {
            ctx,
            pre,
            post,
            pattern,
        }
    }
}

impl<'a, 'b, Ctx, Pa, Pr, Po> Then<'a, Ctx, Pa, Pr, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Ctx::Orig: 'b,
    Pa: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn map<R>(
        self,
        mut func: impl FnMut(&'b Ctx::Orig) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(self.pattern)?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func(self.ctx.orig_sub(beg, ret.length())?)
    }

    // pub fn map_pos<R>(
    //     self,
    //     mut func: impl FnMut(&Ctx, usize, &Ctx::Ret) -> Result<R, Error>,
    // ) -> Result<R, Error> {
    //     let (beg, ret) = {
    //         self.ctx.try_mat(self.pre)?;

    //         let beg = self.ctx.offset();
    //         let ret = self.ctx.try_mat(self.parser)?;

    //         self.ctx.try_mat(self.post)?;
    //         (beg, ret)
    //     };

    //     func(self.ctx, beg, &ret)
    // }

    pub fn and(
        self,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> Then<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pattern;
        let snd = parser;

        Then::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            let mut gurad = CtxGuard::new(ctx);
            let fst = gurad.try_mat(fst)?;
            let snd = gurad.try_mat(snd)?;

            Ok(<Ctx::Ret>::new_from((
                fst.count() + snd.count(),
                fst.length() + snd.length(),
            )))
        })
    }

    pub fn or(
        self,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> Then<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pattern;
        let snd = parser;

        Then::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            fst.try_parse(ctx).or(snd.try_parse(ctx))
        })
    }

    pub fn with_value<V>(
        self,
        val: V,
    ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    {
        let fst = self.pattern;

        MatThenValue::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            fst.try_parse(ctx).map(|v| (val, v))
        })
    }

    // pub fn with_mapper<V>(
    //     self,
    //     mut mapper: impl FnMut(&Ctx::Orig) -> Result<V, Error>,
    // ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    // {
    //     let fst = self.pattern;

    //     MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
    //         let beg = ctx.offset();
    //         let ret = fst.try_parse(ctx)?;

    //         Ok((mapper(ctx.orig_sub(beg, ret.length())?)?, ret))
    //     })
    // }

    pub fn with_pos_mapper<V>(
        self,
        mut mapper: impl FnMut(&Ctx, usize, &Ctx::Ret) -> Result<V, Error>,
    ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    {
        let fst = self.pattern;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
            let beg = ctx.offset();
            let ret = fst.try_parse(ctx)?;

            Ok((mapper(ctx, beg, &ret)?, ret))
        })
    }
}

#[derive(Debug)]
pub struct MatThenValue<'a, Ctx, Pa, Pr, Po, V> {
    ctx: &'a mut Ctx,
    pre: Pr,
    post: Po,
    pattern: Pa,
    marker: PhantomData<V>,
}

impl<'a, Ctx, Pa, Pr, Po, V> MatThenValue<'a, Ctx, Pa, Pr, Po, V> {
    pub fn new(ctx: &'a mut Ctx, pre: Pr, post: Po, pattern: Pa) -> Self {
        Self {
            ctx,
            pre,
            post,
            pattern,
            marker: PhantomData,
        }
    }
}

impl<'a, 'b, C, P, PR, PO, V> MatThenValue<'a, C, P, PR, PO, V>
where
    C: Context<'b> + Policy<C>,
    C::Orig: 'b,
    P: FnOnce(&mut C) -> Result<(V, C::Ret), Error>,
    PO: Pattern<C, Ret = C::Ret>,
    PR: Pattern<C, Ret = C::Ret>,
{
    pub fn map<R>(
        self,
        mut func: impl FnMut(V, &'b C::Orig) -> Result<R, Error>,
    ) -> Result<R, Error> {
        let mut val = None;
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(|ctx: &mut C| {
                (self.pattern)(ctx).and_then(|(ret_val, ret)| {
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
                (self.pattern)(ctx).and_then(|(ret_val, ret)| {
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
        parser: impl Pattern<C, Ret = C::Ret>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.pattern;
        let snd = parser;

        MatThenValue::new(self.ctx, self.pre, self.post, |ctx: &mut C| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                snd.try_parse(ctx).map(|v| (val, v))
            }
        })
    }

    // pub fn or_map(
    //     self,
    //     parser: impl Pattern<C, Ret = C::Ret>,
    //     mut mapper: impl FnMut(&C::Orig) -> Result<V, Error>,
    // ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
    //     let fst = self.pattern;
    //     let snd = parser;

    //     MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut C| {
    //         if let Ok((ret_val, ret)) = (fst)(ctx) {
    //             Ok((ret_val, ret))
    //         } else {
    //             let beg = ctx.offset();
    //             let ret = snd.try_parse(ctx)?;

    //             Ok((mapper(ctx.orig_sub(beg, ret.length())?)?, ret))
    //         }
    //     })
    // }

    pub fn or_map_pos(
        self,
        parser: impl Pattern<C, Ret = C::Ret>,
        mut mapper: impl FnMut(&C, usize, &C::Ret) -> Result<V, Error>,
    ) -> MatThenValue<'a, C, impl FnOnce(&mut C) -> Result<(V, C::Ret), Error>, PR, PO, V> {
        let fst = self.pattern;
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
