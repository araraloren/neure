use std::{borrow::Cow, marker::PhantomData, num::ParseIntError};

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

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStr<T>(PhantomData<T>);

impl<T> Clone for FromStr<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromStr<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

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

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapInto<T>(PhantomData<T>);

impl<T> Clone for MapInto<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> MapInto<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for MapInto<T> {
    fn default() -> Self {
        Self(Default::default())
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

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapTryInto<T>(PhantomData<T>);

impl<T> Clone for MapTryInto<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> MapTryInto<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for MapTryInto<T> {
    fn default() -> Self {
        Self(Default::default())
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
        impl $crate::map::TryFromStrRadix for $int {
            #[inline(always)]
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

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStrRadix<T> {
    radix: u32,
    marker: PhantomData<T>,
}

impl<T> Clone for FromStrRadix<T> {
    fn clone(&self) -> Self {
        Self {
            radix: self.radix,
            marker: self.marker,
        }
    }
}

impl<T> Default for FromStrRadix<T> {
    fn default() -> Self {
        Self {
            radix: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<T> FromStrRadix<T>
where
    T: TryFromStrRadix,
{
    pub fn new(radix: u32) -> Self {
        Self {
            radix,
            marker: PhantomData,
        }
    }

    pub fn radix(&self) -> u32 {
        self.radix
    }
}

impl<I, O> MapSingle<I, O> for FromStrRadix<O>
where
    O: TryFromStrRadix,
    I: AsRef<str>,
{
    #[inline(always)]
    fn map_to(&self, val: I) -> Result<O, Error> {
        O::from_str_radix(val.as_ref(), self.radix()).map_err(|_| Error::FromStr)
    }
}

#[inline(always)]
pub fn from_str_radix<T: TryFromStrRadix>(radix: u32) -> FromStrRadix<T> {
    FromStrRadix::new(radix)
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromUtf8<T>(PhantomData<T>);

impl<T> FromUtf8<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for FromUtf8<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromUtf8<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a> MapSingle<&'a [u8], &'a str> for FromUtf8<&'a str> {
    fn map_to(&self, val: &'a [u8]) -> Result<&'a str, Error> {
        std::str::from_utf8(val).map_err(|_| Error::Utf8Error)
    }
}

impl<'a> MapSingle<&'a [u8], String> for FromUtf8<String> {
    fn map_to(&self, val: &'a [u8]) -> Result<String, Error> {
        String::from_utf8(val.to_vec()).map_err(|_| Error::Utf8Error)
    }
}

#[inline(always)]
pub fn from_utf8<T>() -> FromUtf8<T> {
    FromUtf8::default()
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromUtf8Lossy<T>(PhantomData<T>);

impl<T> FromUtf8Lossy<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for FromUtf8Lossy<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromUtf8Lossy<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a> MapSingle<&'a [u8], Cow<'a, str>> for FromUtf8Lossy<Cow<'a, str>> {
    fn map_to(&self, val: &'a [u8]) -> Result<Cow<'a, str>, Error> {
        Ok(String::from_utf8_lossy(val))
    }
}

#[inline(always)]
pub fn from_utf8_lossy<T>() -> FromUtf8Lossy<T> {
    FromUtf8Lossy::default()
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromLeBytes<T>(PhantomData<T>);

impl<T> FromLeBytes<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for FromLeBytes<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromLeBytes<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromBeBytes<T>(PhantomData<T>);

impl<T> FromBeBytes<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for FromBeBytes<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromBeBytes<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

macro_rules! impl_from_bytes {
    (le $ty:ty, $size:literal) => {
        impl<'a> MapSingle<&'a [u8], $ty> for FromLeBytes<$ty> {
            fn map_to(&self, val: &'a [u8]) -> Result<$ty, Error> {
                let bytes = val
                    .chunks_exact($size)
                    .next()
                    .ok_or_else(|| Error::FromLeBytes)
                    .map(|v| <&[u8; $size]>::try_from(v).map_err(|_| Error::FromLeBytes))??;
                Ok(<$ty>::from_le_bytes(*bytes))
            }
        }
    };
    (be $ty:ty, $size:literal) => {
        impl<'a> MapSingle<&'a [u8], $ty> for FromBeBytes<$ty> {
            fn map_to(&self, val: &'a [u8]) -> Result<$ty, Error> {
                let bytes = val
                    .chunks_exact($size)
                    .next()
                    .ok_or_else(|| Error::FromBeBytes)
                    .map(|v| <&[u8; $size]>::try_from(v).map_err(|_| Error::FromBeBytes))??;
                Ok(<$ty>::from_be_bytes(*bytes))
            }
        }
    };
}

impl_from_bytes!(le i8, 1);
impl_from_bytes!(le u8, 1);
impl_from_bytes!(le i16, 2);
impl_from_bytes!(le u16, 2);
impl_from_bytes!(le i32, 4);
impl_from_bytes!(le u32, 4);
impl_from_bytes!(le i64, 8);
impl_from_bytes!(le u64, 8);
impl_from_bytes!(le f32, 4);
impl_from_bytes!(le f64, 8);
impl_from_bytes!(le i128, 16);
impl_from_bytes!(le u128, 16);
impl_from_bytes!(le isize, 8);
impl_from_bytes!(le usize, 8);
impl_from_bytes!(be i8, 1);
impl_from_bytes!(be u8, 1);
impl_from_bytes!(be i16, 2);
impl_from_bytes!(be u16, 2);
impl_from_bytes!(be i32, 4);
impl_from_bytes!(be u32, 4);
impl_from_bytes!(be i64, 8);
impl_from_bytes!(be u64, 8);
impl_from_bytes!(be f32, 4);
impl_from_bytes!(be f64, 8);
impl_from_bytes!(be i128, 16);
impl_from_bytes!(be u128, 16);
impl_from_bytes!(be isize, 8);
impl_from_bytes!(be usize, 8);

#[inline(always)]
pub fn from_le_bytes<T>() -> FromLeBytes<T> {
    FromLeBytes::default()
}

#[inline(always)]
pub fn from_be_bytes<T>() -> FromBeBytes<T> {
    FromBeBytes::default()
}
