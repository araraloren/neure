use std::{borrow::Cow, marker::PhantomData, mem::size_of, num::ParseIntError};

use crate::err::Error;

pub trait FallibleMap<I, O> {
    fn out_size(&self) -> usize {
        size_of::<O>()
    }

    /// Attempts to map a value from type `I` to type `O`.
    fn try_map(&self, val: I) -> Result<O, Error>;
}

impl<I, O, F> FallibleMap<I, O> for F
where
    F: Fn(I) -> Result<O, Error>,
{
    fn try_map(&self, val: I) -> Result<O, Error> {
        (self)(val)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FuncMapper<F> {
    func: F,
}

impl<F> FuncMapper<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F, I, O> FallibleMap<I, O> for FuncMapper<F>
where
    F: Fn(I) -> O,
{
    fn try_map(&self, val: I) -> Result<O, Error> {
        Ok((self.func)(val))
    }
}

/// Adapts infallible functions to the [`FallibleMap`] trait system.
pub fn mapper<F>(func: F) -> FuncMapper<F> {
    FuncMapper::new(func)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Select0;

impl Select0 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> FallibleMap<(I1, I2), I1> for Select0 {
    fn try_map(&self, val: (I1, I2)) -> Result<I1, Error> {
        Ok(val.0)
    }
}

/// Selects the first element (index 0) from a tuple.
pub fn select0() -> Select0 {
    Select0::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Select1;

impl Select1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> FallibleMap<(I1, I2), I2> for Select1 {
    fn try_map(&self, val: (I1, I2)) -> Result<I2, Error> {
        Ok(val.1)
    }
}

/// Selects the second element (index 1) from a tuple.
pub fn select1() -> Select1 {
    Select1::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SelectEq;

impl SelectEq {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> FallibleMap<(I1, I2), (I1, I2)> for SelectEq
where
    I1: PartialEq<I2>,
{
    fn try_map(&self, val: (I1, I2)) -> Result<(I1, I2), Error> {
        if val.0 == val.1 {
            Ok(val)
        } else {
            Err(Error::SelectEq)
        }
    }
}

/// Validates that both elements of a tuple are equal.
pub fn select_eq() -> SelectEq {
    SelectEq::new()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SelectNeq;

impl SelectNeq {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I1, I2> FallibleMap<(I1, I2), (I1, I2)> for SelectNeq
where
    I1: PartialEq<I2>,
{
    fn try_map(&self, val: (I1, I2)) -> Result<(I1, I2), Error> {
        if val.0 != val.1 {
            Ok(val)
        } else {
            Err(Error::SelectNeq)
        }
    }
}

/// Validates that both elements of a tuple are not equal.
pub fn select_neq() -> SelectNeq {
    SelectNeq::new()
}

#[derive(Debug, Copy)]
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

impl<I, O> FallibleMap<I, O> for FromStr<O>
where
    O: std::str::FromStr,
    I: AsRef<str>,
{
    fn try_map(&self, val: I) -> Result<O, Error> {
        let val: &str = val.as_ref();

        val.parse::<O>().map_err(|_| Error::FromStr)
    }
}

/// Converts strings to typed values using [`FromStr`](std::str::FromStr).
///
/// [`FromStr`] is a zero-cost adapter that safely parses strings into strongly-typed
/// values. It wraps the standard library's [`FromStr`](std::str::FromStr) trait implementation to provide
/// a consistent interface for transformation pipelines and parser combinators.
pub fn from_str<T>() -> FromStr<T> {
    FromStr::new()
}

#[derive(Debug, Copy)]
pub struct IntoMapper<T>(PhantomData<T>);

impl<T> Clone for IntoMapper<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> IntoMapper<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for IntoMapper<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I, O> FallibleMap<I, O> for IntoMapper<O>
where
    O: From<I>,
{
    fn try_map(&self, val: I) -> Result<O, Error> {
        Ok(val.into())
    }
}

/// A zero-cost adapter that converts type using the [`into`](std::convert::Into::into) method.
pub fn into<T>() -> IntoMapper<T> {
    IntoMapper::new()
}

#[derive(Debug, Copy)]
pub struct TryIntoMapper<T>(PhantomData<T>);

impl<T> Clone for TryIntoMapper<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> TryIntoMapper<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for TryIntoMapper<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I, O> FallibleMap<I, O> for TryIntoMapper<O>
where
    O: TryFrom<I>,
{
    fn try_map(&self, val: I) -> Result<O, Error> {
        val.try_into().map_err(|_| Error::TryInto)
    }
}

/// A zero-cost adapter that converts type using the [`try_into`](std::convert::TryInto::try_into) method.
pub fn try_into<T>() -> TryIntoMapper<T> {
    TryIntoMapper::new()
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

#[derive(Debug, Copy)]
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

impl<I, O> FallibleMap<I, O> for FromStrRadix<O>
where
    O: TryFromStrRadix,
    I: AsRef<str>,
{
    #[inline(always)]
    fn try_map(&self, val: I) -> Result<O, Error> {
        O::from_str_radix(val.as_ref(), self.radix()).map_err(|_| Error::FromStr)
    }
}

/// A trait that abstracts over integer types' `from_str_radix` functionality.
///
/// This trait is implemented for all standard integer types and provides a consistent
/// interface for parsing integers from strings with a specified radix (base).
#[inline(always)]
pub fn from_str_radix<T: TryFromStrRadix>(radix: u32) -> FromStrRadix<T> {
    FromStrRadix::new(radix)
}

#[derive(Debug, Copy)]
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

impl<'a> FallibleMap<&'a [u8], &'a str> for FromUtf8<&'a str> {
    fn try_map(&self, val: &'a [u8]) -> Result<&'a str, Error> {
        std::str::from_utf8(val).map_err(|_| Error::Utf8Error)
    }
}

impl<'a> FallibleMap<&'a [u8], String> for FromUtf8<String> {
    fn try_map(&self, val: &'a [u8]) -> Result<String, Error> {
        String::from_utf8(val.to_vec()).map_err(|_| Error::Utf8Error)
    }
}

/// A mapper that converts byte slices to UTF-8 [`String`].
#[inline(always)]
pub fn from_utf8<T>() -> FromUtf8<T> {
    FromUtf8::default()
}

#[derive(Debug, Copy)]
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

impl<'a> FallibleMap<&'a [u8], Cow<'a, str>> for FromUtf8Lossy<Cow<'a, str>> {
    fn try_map(&self, val: &'a [u8]) -> Result<Cow<'a, str>, Error> {
        Ok(String::from_utf8_lossy(val))
    }
}

/// A mapper that converts byte slices to UTF-8 [`String`] with lossy conversion.
#[inline(always)]
pub fn from_utf8_lossy<T>() -> FromUtf8Lossy<T> {
    FromUtf8Lossy::default()
}

#[derive(Debug, Copy)]
pub struct FromLeBytes<T>(PhantomData<T>);

impl<T> FromLeBytes<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub const fn size(&self) -> usize {
        size_of::<T>()
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

#[derive(Debug, Copy)]
pub struct FromBeBytes<T>(PhantomData<T>);

impl<T> FromBeBytes<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub const fn size(&self) -> usize {
        size_of::<T>()
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

#[derive(Debug, Copy)]
pub struct FromNeBytes<T>(PhantomData<T>);

impl<T> FromNeBytes<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub const fn size(&self) -> usize {
        size_of::<T>()
    }
}

impl<T> Clone for FromNeBytes<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Default for FromNeBytes<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

macro_rules! impl_from_bytes {
    (le $ty:ty, $size:literal) => {
        impl<'a> FallibleMap<&'a [u8], $ty> for FromLeBytes<$ty> {
            fn try_map(&self, val: &'a [u8]) -> Result<$ty, $crate::err::Error> {
                debug_assert_eq!($size, self.size());
                let bytes = val
                    .chunks_exact($size)
                    .next()
                    .ok_or_else(|| $crate::err::Error::FromLeBytes)
                    .map(|v| {
                        <&[u8; $size]>::try_from(v).map_err(|_| $crate::err::Error::FromLeBytes)
                    })??;
                Ok(<$ty>::from_le_bytes(*bytes))
            }
        }
    };
    (be $ty:ty, $size:literal) => {
        impl<'a> FallibleMap<&'a [u8], $ty> for FromBeBytes<$ty> {
            fn try_map(&self, val: &'a [u8]) -> Result<$ty, $crate::err::Error> {
                debug_assert_eq!($size, self.size());
                let bytes = val
                    .chunks_exact($size)
                    .next()
                    .ok_or_else(|| $crate::err::Error::FromBeBytes)
                    .map(|v| {
                        <&[u8; $size]>::try_from(v).map_err(|_| $crate::err::Error::FromBeBytes)
                    })??;
                Ok(<$ty>::from_be_bytes(*bytes))
            }
        }
    };
    (ne $ty:ty, $size:literal) => {
        impl<'a> FallibleMap<&'a [u8], $ty> for FromNeBytes<$ty> {
            fn try_map(&self, val: &'a [u8]) -> Result<$ty, $crate::err::Error> {
                debug_assert_eq!($size, self.size());
                let bytes = val
                    .chunks_exact($size)
                    .next()
                    .ok_or_else(|| $crate::err::Error::FromNeBytes)
                    .map(|v| {
                        <&[u8; $size]>::try_from(v).map_err(|_| $crate::err::Error::FromNeBytes)
                    })??;
                Ok(<$ty>::from_ne_bytes(*bytes))
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
impl_from_bytes!(ne i8, 1);
impl_from_bytes!(ne u8, 1);
impl_from_bytes!(ne i16, 2);
impl_from_bytes!(ne u16, 2);
impl_from_bytes!(ne i32, 4);
impl_from_bytes!(ne u32, 4);
impl_from_bytes!(ne i64, 8);
impl_from_bytes!(ne u64, 8);
impl_from_bytes!(ne f32, 4);
impl_from_bytes!(ne f64, 8);
impl_from_bytes!(ne i128, 16);
impl_from_bytes!(ne u128, 16);
impl_from_bytes!(ne isize, 8);
impl_from_bytes!(ne usize, 8);

///
/// Map an integer value from its memory representation as a byte array in little endianness.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let data = [0x01, 0x02, 0x03, 0x04];
///     let parser = regex::consume(4).try_map(map::from_le_bytes::<i32>());
///
///     assert_eq!(BytesCtx::new(&data).ctor(&parser)?, 0x04030201);
///
///     Ok(())
/// # }
/// ```
#[inline(always)]
pub fn from_le_bytes<T>() -> FromLeBytes<T> {
    FromLeBytes::default()
}

///
/// Map an integer value from its memory representation as a byte array in bigger endianness.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let data = [0x01, 0x02, 0x03, 0x04];
///     let parser = regex::consume(4).try_map(map::from_be_bytes::<i32>());
///
///     assert_eq!(BytesCtx::new(&data).ctor(&parser)?, 0x01020304);
///
///     Ok(())
/// # }
/// ```
#[inline(always)]
pub fn from_be_bytes<T>() -> FromBeBytes<T> {
    FromBeBytes::default()
}

///
/// Map an integer value from its memory representation as a byte array in native endianness.
///
#[inline(always)]
pub fn from_ne_bytes<T>() -> FromNeBytes<T> {
    FromNeBytes::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bounded<T> {
    min: T,
    max: T,
}

impl<T> Bounded<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T> FallibleMap<T, T> for Bounded<T>
where
    T: PartialOrd,
{
    fn try_map(&self, val: T) -> Result<T, Error> {
        if self.min <= val && val < self.max {
            Ok(val)
        } else {
            Err(Error::SelectEq)
        }
    }
}

/// A mapper that validates values against a specified range.
///
/// This struct checks if a value falls within the range `[min, max)` -
/// inclusive of the minimum value and exclusive of the maximum value.
///
/// # Example
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut ctx = CharsCtx::new("1,3,5,7,9");
///
///     let parser = neu::digit(10)
///         .many1()
///         .try_map(map::from_str::<i32>())
///         .try_map(map::bounded(1, 8))
///         .sep(",");
///
///     assert_eq!(ctx.ctor(&parser)?, vec![1, 3, 5, 7]);
///
/// #   Ok(())
/// # }
/// ```
#[inline(always)]
pub fn bounded<T: PartialOrd>(min: T, max: T) -> Bounded<T> {
    Bounded::new(min, max)
}

#[derive(Debug, Clone, Copy)]
pub struct WithDefault<T, F, M> {
    func: F,
    mapper: M,
    marker: PhantomData<T>,
}

impl<T, F, M> WithDefault<T, F, M>
where
    F: Fn() -> T,
{
    pub fn new(func: F, mapper: M) -> Self {
        Self {
            func,
            mapper,
            marker: PhantomData,
        }
    }
}

impl<T, F, M> FallibleMap<T, T> for WithDefault<T, F, M>
where
    F: Fn() -> T,
    M: FallibleMap<T, T>,
{
    fn try_map(&self, val: T) -> Result<T, Error> {
        if let Ok(val) = self.mapper.try_map(val) {
            Ok(val)
        } else {
            Ok((self.func)())
        }
    }
}

pub trait WithDefaultHelper<T>: Sized {
    fn with_default<F>(self, func: F) -> WithDefault<T, F, Self>
    where
        F: Fn() -> T;
}

impl<T, K: Sized> WithDefaultHelper<T> for K {
    fn with_default<F>(self, func: F) -> WithDefault<T, F, Self>
    where
        F: Fn() -> T,
    {
        with_default(func, self)
    }
}

/// A mapper that provides a fallback default value when the primary mapping fails.
///
/// This struct wraps another mapper and a default value provider function. When mapping,
/// it first attempts to use the inner mapper. If that fails, it invokes the default
/// function to produce a fallback value.
///
/// # Example
/// ```
/// # use neure::{map::WithDefaultHelper, prelude::*};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut ctx = CharsCtx::new("1,3,5,7,9");
///
///     let parser = regex!(['0' - '9']+)
///         .try_map(map::from_str::<i32>())
///         .try_map(map::bounded(1, 5).with_default(|| 0))
///         .sep(",");
///
///     assert_eq!(ctx.ctor(&parser)?, vec![1, 3, 0, 0, 0]);
///
/// #   Ok(())
/// # }
/// ```
#[inline(always)]
pub fn with_default<T, F, M>(func: F, mapper: M) -> WithDefault<T, F, M>
where
    F: Fn() -> T,
{
    WithDefault::new(func, mapper)
}
