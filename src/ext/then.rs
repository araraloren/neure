use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::err::Error;
use crate::ext::Extract;
use crate::ext::Handler;

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
    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(self.pattern)?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func.invoke(A::extract(&self.ctx, beg, &ret)?)
    }

    pub fn and(
        self,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> Then<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pattern;
        let snd = parser;

        Then::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            super::and(fst, snd).try_parse(ctx)
        })
    }

    pub fn or(
        self,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> Then<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pattern;
        let snd = parser;

        Then::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            super::or(fst, snd).try_parse(ctx)
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

    pub fn with_mapper<H, A, O>(
        self,
        mut func: H,
    ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(O, Ctx::Ret), Error>, Pr, Po, O>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let fst = self.pattern;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
            let beg = ctx.offset();
            let ret = fst.try_parse(ctx)?;

            Ok((func.invoke(A::extract(ctx, beg, &ret)?)?, ret))
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

impl<'a, 'b, Ctx, Pa, Pr, Po, V> MatThenValue<'a, Ctx, Pa, Pr, Po, V>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Pa: FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let mut val = None;
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(|ctx: &mut Ctx| {
                (self.pattern)(ctx).and_then(|(ret_val, ret)| {
                    val = Some(ret_val);
                    Ok(ret)
                })
            })?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func.invoke(A::extract(&self.ctx, beg, &ret)?)
    }

    pub fn or_with(
        self,
        val: V,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    {
        let fst = self.pattern;
        let snd = parser;

        MatThenValue::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                snd.try_parse(ctx).map(|v| (val, v))
            }
        })
    }

    pub fn or_map<H, A, P>(
        self,
        pattern: P,
        mut func: H,
    ) -> MatThenValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
        H: Handler<A, Out = V, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let fst = self.pattern;
        let snd = pattern;

        MatThenValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                let beg = ctx.offset();
                let ret = snd.try_parse(ctx)?;

                Ok((func.invoke(A::extract(ctx, beg, &ret)?)?, ret))
            }
        })
    }
}
