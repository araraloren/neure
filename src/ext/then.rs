use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::err::Error;
use crate::ext::Extract;
use crate::ext::Handler;
use crate::parser;
use crate::prelude::Ret;

use super::CtxGuard;
use super::HandlerV;

#[derive(Debug)]
pub struct LazyPattern<'a, Ctx, Pa, Pr, Po> {
    pre: Pr,
    post: Po,
    pat: Pa,
    ctx: &'a mut Ctx,
}

impl<'a, Ctx, Pa, Pr, Po> LazyPattern<'a, Ctx, Pa, Pr, Po> {
    pub fn new(ctx: &'a mut Ctx, pre: Pr, post: Po, pat: Pa) -> Self {
        Self {
            ctx,
            pre,
            post,
            pat,
        }
    }
}

impl<'a, 'b, Ctx, Pa, Pr, Po> LazyPattern<'a, Ctx, Pa, Pr, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Ctx::Orig: 'b,
    Pa: Pattern<Ctx, Ret = Ctx::Ret>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn run(self) -> Result<Ctx::Ret, Error> {
        self.ctx.try_mat(self.pre)?;
        let ret = self.ctx.try_mat(self.pat)?;

        self.ctx.try_mat(self.post)?;
        Ok(ret)
    }

    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(self.pat)?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func.invoke(A::extract(self.ctx, beg, &ret)?)
    }

    pub fn and(
        self,
        pat: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> LazyPattern<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pat;
        let snd = pat;

        LazyPattern::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            parser::and(fst, snd).try_parse(ctx)
        })
    }

    pub fn and_if(
        self,
        r#if: impl Pattern<Ctx, Ret = Ctx::Ret>,
        pat: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> LazyPattern<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pat;
        let snd = pat;

        LazyPattern::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            let mut guard = CtxGuard::new(ctx);
            let mut ret = guard.try_mat(fst)?;

            if let Ok(if_ret) = guard.try_mat(r#if) {
                ret.add_assign(if_ret);
                ret.add_assign(guard.try_mat(snd)?);
            }
            Ok(ret)
        })
    }

    pub fn or(
        self,
        parser: impl Pattern<Ctx, Ret = Ctx::Ret>,
    ) -> LazyPattern<'a, Ctx, impl Pattern<Ctx, Ret = Ctx::Ret>, Pr, Po> {
        let fst = self.pat;
        let snd = parser;

        LazyPattern::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            parser::or(fst, snd).try_parse(ctx)
        })
    }

    pub fn with<V>(
        self,
        val: V,
    ) -> LazyPatternValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    {
        let fst = self.pat;

        LazyPatternValue::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            fst.try_parse(ctx).map(|v| (val, v))
        })
    }

    pub fn with_mapper<H, A, O>(
        self,
        mut func: H,
    ) -> LazyPatternValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(O, Ctx::Ret), Error>, Pr, Po, O>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let fst = self.pat;

        LazyPatternValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
            let beg = ctx.offset();
            let ret = fst.try_parse(ctx)?;

            Ok((func.invoke(A::extract(ctx, beg, &ret)?)?, ret))
        })
    }
}

#[derive(Debug)]
pub struct LazyPatternValue<'a, Ctx, Pa, Pr, Po, V> {
    pre: Pr,
    post: Po,
    pat: Pa,
    ctx: &'a mut Ctx,
    marker: PhantomData<V>,
}

impl<'a, Ctx, Pa, Pr, Po, V> LazyPatternValue<'a, Ctx, Pa, Pr, Po, V> {
    pub fn new(ctx: &'a mut Ctx, pre: Pr, post: Po, pat: Pa) -> Self {
        Self {
            ctx,
            pre,
            post,
            pat,
            marker: PhantomData,
        }
    }
}

impl<'a, 'b, Ctx, Pa, Pr, Po, V> LazyPatternValue<'a, Ctx, Pa, Pr, Po, V>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
    Pr: Pattern<Ctx, Ret = Ctx::Ret>,
    Pa: FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>,
{
    pub fn run(self) -> Result<(V, Ctx::Ret), Error> {
        let mut val = None;

        self.ctx.try_mat(self.pre)?;
        let ret = self.ctx.try_mat(|ctx: &mut Ctx| {
            (self.pat)(ctx).map(|(ret_val, ret)| {
                val = Some(ret_val);
                ret
            })
        })?;

        self.ctx.try_mat(self.post)?;
        Ok((val.unwrap(), ret))
    }

    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: HandlerV<V, A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let mut val = None;
        let (beg, ret) = {
            self.ctx.try_mat(self.pre)?;

            let beg = self.ctx.offset();
            let ret = self.ctx.try_mat(|ctx: &mut Ctx| {
                (self.pat)(ctx).map(|(ret_val, ret)| {
                    val = Some(ret_val);
                    ret
                })
            })?;

            self.ctx.try_mat(self.post)?;
            (beg, ret)
        };

        func.invoke(val.unwrap(), A::extract(self.ctx, beg, &ret)?)
    }

    pub fn or_with(
        self,
        pat: impl Pattern<Ctx, Ret = Ctx::Ret>,
        val: V,
    ) -> LazyPatternValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    {
        let fst = self.pat;
        let snd = pat;

        LazyPatternValue::new(self.ctx, self.pre, self.post, |ctx: &mut Ctx| {
            if let Ok((ret_val, ret)) = (fst)(ctx) {
                Ok((ret_val, ret))
            } else {
                snd.try_parse(ctx).map(|v| (val, v))
            }
        })
    }

    pub fn or_map<H, A, P>(
        self,
        pat: P,
        mut func: H,
    ) -> LazyPatternValue<'a, Ctx, impl FnOnce(&mut Ctx) -> Result<(V, Ctx::Ret), Error>, Pr, Po, V>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
        H: Handler<A, Out = V, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let fst = self.pat;
        let snd = pat;

        LazyPatternValue::new(self.ctx, self.pre, self.post, move |ctx: &mut Ctx| {
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

#[derive(Debug)]
pub struct NonLazyPattern<'a, Ctx: Policy<Ctx>, Po> {
    post: Po,
    ctx: &'a mut Ctx,
    beg: usize,
    error: Error,
    ret: Option<Ctx::Ret>,
}

impl<'a, Ctx: Policy<Ctx>, Po> NonLazyPattern<'a, Ctx, Po> {
    pub fn new(
        ctx: &'a mut Ctx,
        post: Po,
        beg: usize,
        ret: Option<Ctx::Ret>,
        error: Error,
    ) -> Self {
        Self {
            post,
            beg,
            ret,
            ctx,
            error,
        }
    }
}

impl<'a, 'b, Ctx, Po> NonLazyPattern<'a, Ctx, Po>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn run(self) -> Result<Ctx::Ret, Error> {
        if self.ret.is_none() {
            Err(self.error)
        } else {
            self.ctx.try_mat(self.post)?;

            Ok(self.ret.unwrap())
        }
    }

    #[inline(always)]
    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        if self.ret.is_none() {
            Err(self.error)
        } else {
            self.ctx.try_mat(self.post)?;
            func.invoke(A::extract(self.ctx, self.beg, self.ret.as_ref().unwrap())?)
        }
    }

    pub fn and(self, pat: impl Pattern<Ctx, Ret = Ctx::Ret>) -> Result<Self, Error> {
        if self.ret.is_none() {
            Err(self.error)
        } else {
            let mut prev_ret = self.ret.unwrap();
            let (ret, error) = {
                match self.ctx.try_mat(pat) {
                    Ok(ret) => {
                        prev_ret.add_assign(ret);
                        (Some(prev_ret), Error::Null)
                    }
                    Err(e) => (None, e),
                }
            };

            Ok(Self::new(self.ctx, self.post, self.beg, ret, error))
        }
    }

    pub fn or(self, pat: impl Pattern<Ctx, Ret = Ctx::Ret>) -> Result<Self, Error> {
        if self.ret.is_none() {
            let (ret, error) = match self.ctx.try_mat(pat) {
                Ok(ret) => (Some(ret), Error::Null),
                Err(e) => (None, e),
            };

            Ok(Self::new(self.ctx, self.post, self.beg, ret, error))
        } else {
            Ok(self)
        }
    }

    pub fn with<V>(self, val: V) -> Result<NonLazyPatternValue<'a, Ctx, Po, V>, Error> {
        Ok(NonLazyPatternValue::new(
            self.ctx,
            self.post,
            Some(val),
            self.beg,
            self.ret,
            self.error,
        ))
    }

    pub fn with_mapper<H, A, O>(
        self,
        mut func: H,
    ) -> Result<NonLazyPatternValue<'a, Ctx, Po, O>, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        let val = if let Some(ret) = &self.ret {
            func.invoke(A::extract(self.ctx, self.beg, ret)?).ok()
        } else {
            None
        };

        Ok(NonLazyPatternValue::new(
            self.ctx, self.post, val, self.beg, self.ret, self.error,
        ))
    }
}

#[derive(Debug)]
pub struct NonLazyPatternValue<'a, Ctx: Policy<Ctx>, Po, Val> {
    post: Po,
    ctx: &'a mut Ctx,
    beg: usize,
    error: Error,
    ret: Option<Ctx::Ret>,
    val: Option<Val>,
}

impl<'a, Ctx: Policy<Ctx>, Po, Val> NonLazyPatternValue<'a, Ctx, Po, Val> {
    pub fn new(
        ctx: &'a mut Ctx,
        post: Po,
        val: Option<Val>,
        beg: usize,
        ret: Option<Ctx::Ret>,
        error: Error,
    ) -> Self {
        Self {
            ctx,
            post,
            beg,
            ret,
            error,
            val,
        }
    }
}

impl<'a, 'b, Ctx, Po, Val> NonLazyPatternValue<'a, Ctx, Po, Val>
where
    Ctx: Context<'b> + Policy<Ctx>,
    Po: Pattern<Ctx, Ret = Ctx::Ret>,
{
    pub fn map<H, A, O>(self, mut func: H) -> Result<O, Error>
    where
        H: HandlerV<Val, A, Out = O, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        if self.ret.is_none() {
            Err(self.error)
        } else {
            let ret = self.ret.unwrap();
            let val = self.val.unwrap();

            func.invoke(val, A::extract(self.ctx, self.beg, &ret)?)
        }
    }

    pub fn or_with(
        self,
        pat: impl Pattern<Ctx, Ret = Ctx::Ret>,
        val: Val,
    ) -> Result<NonLazyPatternValue<'a, Ctx, Po, Val>, Error> {
        if self.ret.is_some() {
            Ok(self)
        } else {
            let beg = self.ctx.offset();
            let (ret, error) = match self.ctx.try_mat(pat) {
                Ok(ret) => (Some(ret), Error::Null),
                Err(e) => (None, e),
            };

            Ok(NonLazyPatternValue::new(
                self.ctx,
                self.post,
                Some(val),
                beg,
                ret,
                error,
            ))
        }
    }

    pub fn or_map<H, A, P>(
        self,
        pat: P,
        mut func: H,
    ) -> Result<NonLazyPatternValue<'a, Ctx, Po, Val>, Error>
    where
        P: Pattern<Ctx, Ret = Ctx::Ret>,
        H: Handler<A, Out = Val, Error = Error>,
        A: Extract<'b, Ctx, Ctx::Ret, Out<'b> = A, Error = Error>,
    {
        if self.ret.is_some() {
            Ok(self)
        } else {
            let beg = self.ctx.offset();
            let (ret, error) = match self.ctx.try_mat(pat) {
                Ok(ret) => (Some(ret), Error::Null),
                Err(e) => (None, e),
            };
            let val = if let Some(ret) = &ret {
                func.invoke(A::extract(self.ctx, beg, ret)?).ok()
            } else {
                None
            };

            Ok(NonLazyPatternValue::new(
                self.ctx, self.post, val, beg, ret, error,
            ))
        }
    }
}
