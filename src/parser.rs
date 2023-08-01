use crate::{ctx::Context, err::Error};

pub trait Parser<T: Context> {
    fn try_parse(&mut self, ctx: &mut T) -> Result<usize, Error>;

    fn parse(&mut self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H> Parser<T> for H
where
    T: Context,
    H: Fn(&mut T) -> Result<usize, Error>,
{
    fn try_parse(&mut self, ctx: &mut T) -> Result<usize, Error> {
        (self)(ctx)
    }
}

pub fn one<T: Context>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        let mut chars = dat.peek_chars()?;

        if let Some((idx, ch)) = chars.next() {
            if re(&ch) {
                let total_len = dat.len() - dat.offset();
                let next_offset = chars.next().map(|v| v.0).unwrap_or(total_len);

                Ok(next_offset - idx)
            } else {
                Err(Error::Match)
            }
        } else {
            Err(Error::NeedMore)
        }
    }
}

pub fn zero_one<T: Context>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        if let Ok(mut chars) = dat.peek_chars() {
            if let Some((idx, ch)) = chars.next() {
                if re(&ch) {
                    let total_len = dat.len() - dat.offset();
                    let next_offset = chars.next().map(|v| v.0).unwrap_or(total_len);

                    return Ok(next_offset - idx);
                }
            }
        }
        Ok(0)
    }
}

pub fn zero_more<T: Context>(
    re: impl Fn(&char) -> bool,
) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        let mut start = None;
        let mut next = None;

        if let Ok(mut chars) = dat.peek_chars() {
            for (idx, ch) in chars.by_ref() {
                if re(&ch) {
                    if start.is_none() {
                        start = Some(idx);
                    }
                } else {
                    next = Some(idx);
                    break;
                }
            }
            if let Some(start) = start {
                let total_len = dat.len() - dat.offset();
                let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

                return Ok(next_offset - start);
            }
        }
        Ok(0)
    }
}

pub fn one_more<T: Context>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        let mut start = None;
        let mut next = None;
        let mut chars = dat.peek_chars()?;

        for (idx, ch) in chars.by_ref() {
            if re(&ch) {
                if start.is_none() {
                    start = Some(idx);
                }
            } else {
                next = Some(idx);
                break;
            }
        }
        if let Some(start) = start {
            let total_len = dat.len() - dat.offset();
            let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

            Ok(next_offset - start)
        } else {
            Err(Error::Match)
        }
    }
}

pub fn count<const M: usize, const N: usize, T: Context>(
    re: impl Fn(&char) -> bool,
) -> impl Fn(&mut T) -> Result<usize, Error> {
    debug_assert!(M < N, "M must little than N");
    move |dat: &mut T| {
        let mut count = 0;
        let mut start = 0;
        let mut next = None;

        if let Ok(mut chars) = dat.peek_chars() {
            loop {
                if count == N - 1 {
                    break;
                }
                if let Some((offset, ch)) = chars.next() {
                    if re(&ch) {
                        if count == 0 {
                            start = offset;
                        }
                        count += 1;
                        continue;
                    } else {
                        next = Some(offset);
                    }
                }
                break;
            }
            if count < M {
                return Err(Error::Match);
            } else if count > 0 {
                let total_len = dat.len() - dat.offset();
                let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

                return Ok(next_offset - start);
            }
        }
        Ok(0)
    }
}

pub fn start<T: Context>() -> impl Fn(&mut T) -> Result<usize, Error> {
    |_: &mut T| Ok(0)
}

pub fn end<T: Context>() -> impl Fn(&mut T) -> Result<usize, Error> {
    |dat: &mut T| {
        if !dat.peek()?.is_empty() {
            Err(Error::NotReachEnd)
        } else {
            Ok(0)
        }
    }
}
