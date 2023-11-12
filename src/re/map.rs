use std::{marker::PhantomData, num::ParseIntError};

use crate::err::Error;

pub trait MapSingle<I, O> {
    fn map_to(&self, val: I) -> Result<O, Error>;
}

impl<I, O, F> MapSingle<I, O> for F
where
    F: Fn(I) -> Result<O, Error>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        (self)(val)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Single;

impl Single {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> MapSingle<I, I> for Single {
    fn map_to(&self, val: I) -> Result<I, Error> {
        Ok(val)
    }
}

pub fn single() -> Single {
    Single::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Select0;

impl Select0 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> MapSingle<(I1, I2), I1> for Select0 {
    fn map_to(&self, val: (I1, I2)) -> Result<I1, Error> {
        Ok(val.0)
    }
}

pub fn select0() -> Select0 {
    Select0::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Select1;

impl Select1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> MapSingle<(I1, I2), I2> for Select1 {
    fn map_to(&self, val: (I1, I2)) -> Result<I2, Error> {
        Ok(val.1)
    }
}

pub fn select1() -> Select1 {
    Select1::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectEq;

impl SelectEq {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> MapSingle<(I1, I2), (I1, I2)> for SelectEq
where
    I1: PartialEq<I2>,
{
    fn map_to(&self, val: (I1, I2)) -> Result<(I1, I2), Error> {
        if val.0 == val.1 {
            Ok(val)
        } else {
            Err(Error::SelectEq)
        }
    }
}

pub fn select_eq() -> SelectEq {
    SelectEq::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStr<T>(PhantomData<T>);

impl<T> FromStr<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I, O> MapSingle<I, O> for FromStr<O>
where
    O: std::str::FromStr,
    I: AsRef<str>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        let val: &str = val.as_ref();

        val.parse::<O>().map_err(|_| Error::FromStr)
    }
}

pub fn from_str<T>() -> FromStr<T> {
    FromStr::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapInto<T>(PhantomData<T>);

impl<T> MapInto<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I, O> MapSingle<I, O> for MapInto<O>
where
    O: From<I>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        Ok(val.into())
    }
}

pub fn into<T>() -> MapInto<T> {
    MapInto::new()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapTryInto<T>(PhantomData<T>);

impl<T> MapTryInto<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I, O> MapSingle<I, O> for MapTryInto<O>
where
    O: TryFrom<I>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        val.try_into().map_err(|_| Error::TryInto)
    }
}

pub fn try_into<T>() -> MapTryInto<T> {
    MapTryInto::new()
}

pub trait TryFromStrRadix
where
    Self: Sized,
{
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
}

macro_rules! impl_from_str_radix {
    ($int:ty) => {
        impl $crate::re::map::TryFromStrRadix for $int {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$int>::from_str_radix(src, radix)
            }
        }
    };
}

impl_from_str_radix!(i8);
impl_from_str_radix!(i16);
impl_from_str_radix!(i32);
impl_from_str_radix!(i64);
impl_from_str_radix!(isize);
impl_from_str_radix!(u8);
impl_from_str_radix!(u16);
impl_from_str_radix!(u32);
impl_from_str_radix!(u64);
impl_from_str_radix!(usize);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStrRadix<T>((PhantomData<T>, u32));

impl<T> FromStrRadix<T>
where
    T: TryFromStrRadix,
{
    pub fn new(radix: u32) -> Self {
        Self((PhantomData, radix))
    }

    pub fn radix(&self) -> u32 {
        self.0 .1
    }
}

impl<I, O> MapSingle<I, O> for FromStrRadix<O>
where
    O: TryFromStrRadix,
    I: AsRef<str>,
{
    fn map_to(&self, val: I) -> Result<O, Error> {
        O::from_str_radix(val.as_ref(), self.radix()).map_err(|_| Error::FromStr)
    }
}

pub fn from_str_radix<T: TryFromStrRadix>(radix: u32) -> FromStrRadix<T> {
    FromStrRadix::new(radix)
}
