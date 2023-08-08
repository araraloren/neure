use crate::err::Error;
use crate::iter::Char;
use crate::peek::{CharPeek, StrPeek};
use crate::policy::Ret;

pub trait Parser<T> {
    fn try_parse(&mut self, ctx: &mut T) -> Result<Ret, Error>;

    fn parse(&mut self, ctx: &mut T) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<T, H> Parser<T> for H
where
    H: Fn(&mut T) -> Result<Ret, Error>,
{
    fn try_parse(&mut self, ctx: &mut T) -> Result<Ret, Error> {
        (self)(ctx)
    }
}

pub fn one<T: CharPeek>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<Ret, Error> {
    move |dat: &mut T| {
        let mut chars = dat.peek()?;

        if let Some(&Char {
            offset: _,
            len,
            char,
        }) = chars.next()
        {
            if re(&char) {
                Ok(Ret::from((1, len)))
            } else {
                Err(Error::Match)
            }
        } else {
            Err(Error::NeedOne)
        }
    }
}

pub fn zero_one<T: CharPeek>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<Ret, Error> {
    move |dat: &mut T| {
        if let Ok(mut chars) = dat.peek() {
            if let Some(&Char {
                offset: _,
                len,
                char,
            }) = chars.next()
            {
                if re(&char) {
                    return Ok(Ret::from((1, len)));
                }
            }
        }
        Ok(Ret::from((0, 0)))
    }
}

pub fn zero_more<T: CharPeek>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<Ret, Error> {
    move |dat: &mut T| {
        let mut count = 0;
        let mut length = 0;

        if let Ok(mut chars) = dat.peek() {
            for char in chars.by_ref() {
                if re(&char.char) {
                    count += 1;
                    length += char.len;
                } else {
                    break;
                }
            }
        }
        Ok(Ret::from((count, length)))
    }
}

pub fn one_more<T: CharPeek>(re: impl Fn(&char) -> bool) -> impl Fn(&mut T) -> Result<Ret, Error> {
    move |dat: &mut T| {
        let mut count = 0;
        let mut length = 0;
        let mut chars = dat.peek()?;

        for char in chars.by_ref() {
            if re(&char.char) {
                count += 1;
                length += char.len;
            } else {
                break;
            }
        }
        if count > 0 {
            Ok(Ret::from((count, length)))
        } else {
            Err(Error::NeedOneMore)
        }
    }
}

pub fn count<const M: usize, const N: usize, T: CharPeek>(
    re: impl Fn(&char) -> bool,
) -> impl Fn(&mut T) -> Result<Ret, Error> {
    debug_assert!(M <= N, "M must little than N");
    move |dat: &mut T| {
        let mut count = 0;
        let mut length = 0;

        if let Ok(mut chars) = dat.peek() {
            while count < N {
                if let Some(char) = chars.next() {
                    if re(&char.char) {
                        count += 1;
                        length += char.len;
                        continue;
                    }
                }
                break;
            }
            if count < M {
                return Err(Error::NeedMore);
            }
        }
        Ok(Ret::from((count, length)))
    }
}

pub fn count_if<const M: usize, const N: usize, T: CharPeek + StrPeek>(
    re: impl Fn(&char) -> bool,
    validator: impl Fn(&T, &Char) -> bool,
) -> impl Fn(&mut T) -> Result<Ret, Error> {
    debug_assert!(M <= N, "M must little than N");
    move |dat: &mut T| {
        let mut count = 0;
        let mut length = 0;

        if let Ok(mut chars) = CharPeek::peek(dat) {
            while count < N {
                if let Some(char) = chars.next() {
                    if re(&char.char) && validator(dat, char) {
                        count += 1;
                        length += char.len;
                        continue;
                    }
                }
                break;
            }
            if count < M {
                return Err(Error::NeedMore);
            }
        }
        Ok(Ret::from((count, length)))
    }
}

pub fn start<T: CharPeek>() -> impl Fn(&mut T) -> Result<Ret, Error> {
    |dat: &mut T| {
        if dat.offset() == 0 {
            Ok(Ret::from((0, 0)))
        } else {
            Err(Error::NotStart)
        }
    }
}

pub fn end<T: CharPeek>() -> impl Fn(&mut T) -> Result<Ret, Error> {
    |dat: &mut T| {
        if dat.len() != dat.offset() {
            Err(Error::NotEnd)
        } else {
            Ok(Ret::from((0, 0)))
        }
    }
}

pub fn string<T: StrPeek>(lit: &'static str) -> impl Fn(&mut T) -> Result<Ret, Error> {
    move |dat: &mut T| {
        if !dat.peek()?.starts_with(lit) {
            Err(Error::NotEnd)
        } else {
            Ok(Ret::from((lit.chars().count(), lit.len())))
        }
    }
}
