use std::fmt::Debug;

use crate::{ctx::Context, err::Error, regex::Regex};

pub trait Parser<C> {
    type Error: Into<Error>;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error>;
}

impl<H, E, C> Parser<C> for H
where
    H: Fn(&mut C) -> Result<usize, E>,
    E: Into<Error>,
{
    type Error = E;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        self(ctx)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Count<const M: usize, const N: usize, R>(R);

impl<const M: usize, const N: usize, R> Count<M, N, R> {
    pub fn new(r: R) -> Self {
        Self(r)
    }

    pub fn min(&self) -> usize {
        M
    }

    pub fn max(&self) -> usize {
        N
    }
}

impl<const M: usize, const N: usize, R, C> Parser<C> for Count<M, N, R>
where
    R: Regex + Debug,
    C: Context,
{
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        let mut len = 0;
        let mut start = None;
        let mut chars = ctx.peek_chars()?;

        loop {
            if len == N - 1 {
                break;
            }
            if let Some((idx, ch)) = chars.next() {
                if self.0.mat(&ch) {
                    if start.is_none() {
                        start = Some(idx);
                    }
                    len += 1;
                    continue;
                }
            }
            if len < M {
                return Err(Error::Match(format!("Can not match enough `{:?}`", self.0)));
            } else {
                break;
            }
        }
        if let Some(start) = start {
            let next_offset = chars.next().map(|v| v.0).unwrap_or(ctx.len());

            Ok(next_offset - start)
        } else {
            Ok(0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Start;

impl<C: Context> Parser<C> for Start {
    type Error = Error;

    fn parse(&self, _: &mut C) -> Result<usize, Self::Error> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct End;

impl<C: Context> Parser<C> for End {
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        if ctx.peek().map_err(Into::into)?.len() > 0 {
            Err(Error::Match(format!("Here is not end of the context")))
        } else {
            Ok(0)
        }
    }
}
