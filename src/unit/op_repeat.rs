use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Ret;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::Extract;
use crate::regex::Handler;
use crate::regex::Invoke;
use crate::regex::Regex;

use super::Unit;

#[derive(Debug, Default, Copy)]
pub struct UnitRepeat<C, U, B, T>
where
    U: Unit<T>,
    B: RangeBounds<usize>,
{
    unit: U,
    range: B,
    marker: PhantomData<(C, T)>,
}

impl<C, U, B, T> Clone for UnitRepeat<C, U, B, T>
where
    U: Unit<T> + Clone,
    B: RangeBounds<usize> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            range: self.range.clone(),
            marker: self.marker,
        }
    }
}

impl<C, U, B, T> UnitRepeat<C, U, B, T>
where
    U: Unit<T>,
    B: RangeBounds<usize>,
{
    pub fn new(unit: U, range: B) -> Self {
        Self {
            unit,
            range,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn range(&self) -> &B {
        &self.range
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn range_mut(&mut self) -> &mut B {
        &mut self.range
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn set_range(&mut self, range: B) -> &mut Self {
        self.range = range;
        self
    }

    fn is_contain(&self, count: usize) -> bool
    where
        B: RangeBounds<usize>,
    {
        match self.range.end_bound() {
            std::ops::Bound::Included(max) => count < *max,
            std::ops::Bound::Excluded(max) => count < max.saturating_sub(1),
            std::ops::Bound::Unbounded => true,
        }
    }
}

impl<'a, U, B, C, M> Invoke<'a, C, M, M> for UnitRepeat<C, U, B, C::Item>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    B: RangeBounds<usize>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, func: &mut H) -> Result<M, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        func.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, U, B, C> Regex<C> for UnitRepeat<C, U, B, C::Item>
where
    C: Context<'a> + 'a,
    U: Unit<C::Item>,
    B: RangeBounds<usize>,
{
    type Ret = Span;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, crate::err::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let iter = ctx.peek();

        if let Ok(mut iter) = iter {
            while self.is_contain(cnt) {
                if let Some(pair) = iter.next() {
                    if self.unit.is_match(&pair.1) {
                        cnt += 1;
                        if beg.is_none() {
                            beg = Some(pair.0);
                        }
                        continue;
                    } else {
                        end = Some(pair);
                    }
                }
                break;
            }
            if self.range.contains(&cnt) {
                let end = end.or_else(|| iter.next()).map(|v| v.0);
                let length = beg
                    .map(|beg| end.unwrap_or(ctx.len() - ctx.offset()) - beg)
                    .unwrap_or(0);

                let ret = <Self::Ret as Ret>::from(ctx, (cnt, length));

                ctx.inc(length);
                return Ok(ret);
            }
        }
        Err(crate::err::Error::UnitRepeat)
    }
}
