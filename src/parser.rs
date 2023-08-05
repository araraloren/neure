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
            Err(Error::NeedOne)
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
            Err(Error::NeedOneMore)
        }
    }
}

pub fn count<const M: usize, const N: usize, T: Context>(
    re: impl Fn(&char) -> bool,
) -> impl Fn(&mut T) -> Result<usize, Error> {
    debug_assert!(M <= N, "M must little than N");
    move |dat: &mut T| {
        let mut count = 0;
        let mut start = 0;
        let mut next = None;

        if let Ok(mut chars) = dat.peek_chars() {
            loop {
                if count == N {
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
                return Err(Error::NeedMore);
            } else if count > 0 {
                let total_len = dat.len() - dat.offset();
                let next_offset = next.unwrap_or(chars.next().map(|v| v.0).unwrap_or(total_len));

                return Ok(next_offset - start);
            }
        }
        Ok(0)
    }
}

pub fn count_if<const M: usize, const N: usize, T: Context>(
    re: impl Fn(&char) -> bool,
    validator: impl Fn(&T, usize, char) -> bool,
) -> impl Fn(&mut T) -> Result<usize, Error> {
    debug_assert!(M <= N, "M must little than N");
    move |dat: &mut T| {
        let mut count = 0;
        let mut start = 0;
        let mut next = None;

        if let Ok(chars) = dat.peek_chars() {
            let mut chars = chars.peekable();

            loop {
                if count == N {
                    break;
                }
                if let Some((offset, ch)) = chars.next() {
                    if re(&ch) && validator(dat, dat.offset() + offset, ch) {
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
                return Err(Error::NeedMore);
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
    |dat: &mut T| {
        if dat.offset() == 0 {
            Ok(0)
        } else {
            Err(Error::NotStart)
        }
    }
}

pub fn end<T: Context>() -> impl Fn(&mut T) -> Result<usize, Error> {
    |dat: &mut T| {
        if !dat.peek()?.is_empty() {
            Err(Error::NotEnd)
        } else {
            Ok(0)
        }
    }
}

pub fn string<T: Context>(lit: &'static str) -> impl Fn(&mut T) -> Result<usize, Error> {
    move |dat: &mut T| {
        if !dat.peek()?.starts_with(lit) {
            Err(Error::NotEnd)
        } else {
            Ok(lit.len())
        }
    }
}
