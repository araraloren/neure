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
    R: Regex,
    C: Context,
{
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        Parser::parse(&self, ctx)
    }
}

impl<'a, const M: usize, const N: usize, R, C> Parser<C> for &'a Count<M, N, R>
where
    R: Regex,
    C: Context,
{
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        let mut len = 0;
        let mut start = None;
        let mut next = None;
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
                } else {
                    next = Some(idx);
                }
            }
            if len < M {
                return Err(Error::Match(format!(
                    "Can not match enough letter for regex"
                )));
            } else {
                break;
            }
        }
        if let Some(start) = start {
            let total_len = ctx.len() - ctx.offset();
            let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

            Ok(next_offset - start)
        } else {
            Ok(0)
        }
    }
}

pub fn count<const M: usize, const N: usize, T: Context>(
    func: impl Fn(&char) -> bool,
) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        let mut len = 0;
        let mut start = None;
        let mut next = None;
        let mut chars = dat.peek_chars()?;

        loop {
            if len == N - 1 {
                break;
            }
            if let Some((idx, ch)) = chars.next() {
                if func(&ch) {
                    if start.is_none() {
                        start = Some(idx);
                    }
                    len += 1;
                    continue;
                } else {
                    next = Some(idx);
                }
            }
            if len < M {
                return Err(Error::Match("".to_owned()));
            } else {
                break;
            }
        }
        if let Some(start) = start {
            let total_len = dat.len() - dat.offset();
            let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

            Ok(next_offset - start)
        } else {
            Ok(0)
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Start;

impl<C: Context> Parser<C> for Start {
    type Error = Error;

    fn parse(&self, _: &mut C) -> Result<usize, Self::Error> {
        Ok(0)
    }
}

impl<'a, C: Context> Parser<C> for &'a Start {
    type Error = Error;

    fn parse(&self, _: &mut C) -> Result<usize, Self::Error> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct End;

impl<C: Context> Parser<C> for End {
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        Parser::parse(&self, ctx)
    }
}

impl<'a, C: Context> Parser<C> for &'a End {
    type Error = Error;

    fn parse(&self, ctx: &mut C) -> Result<usize, Self::Error> {
        if ctx.peek().map_err(Into::into)?.len() > 0 {
            Err(Error::Match(format!("Here is not end of the context")))
        } else {
            Ok(0)
        }
    }
}
