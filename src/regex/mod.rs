pub mod r#macro;

use std::ops::RangeBounds;

use crate::trace_log;

#[inline(always)]
pub fn char(ch: char) -> impl Fn(&char) -> bool {
    move |dat: &char| {
        trace_log!("match a char {ch} with {dat}(in)");
        dat == &ch
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn equal<T: PartialEq>(val: T) -> impl Fn(&T) -> bool {
    move |dat: &T| dat == &val
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn equal<T: PartialEq + std::fmt::Debug>(val: T) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("match a value {val:?} with {dat:?}(in)");
        dat == &val
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn array<const N: usize, T: PartialEq>(vals: [T; N]) -> impl Fn(&T) -> bool {
    move |dat: &T| vals.contains(dat)
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn array<const N: usize, T: PartialEq + std::fmt::Debug>(vals: [T; N]) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("match a array {vals:?} with {dat:?}(in)");
        vals.contains(dat)
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn vector<T: PartialEq>(vals: Vec<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| vals.contains(dat)
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn vector<T: PartialEq + std::fmt::Debug>(vals: Vec<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("match a vector {vals:?} with {dat:?}(in)");
        vals.contains(dat)
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn range<T: PartialOrd>(bound: impl RangeBounds<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| bound.contains(dat)
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn range<T: PartialOrd + std::fmt::Debug>(
    bound: impl RangeBounds<T> + std::fmt::Debug,
) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("match a range {bound:?} with {dat:?}(in)");
        bound.contains(dat)
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn always_t<T>() -> impl Fn(&T) -> bool {
    |_dat: &T| true
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn always_t<T: std::fmt::Debug>() -> impl Fn(&T) -> bool {
    |_dat: &T| {
        trace_log!("always true, consume {_dat:?}(in)");
        true
    }
}

#[cfg(not(feature = "log"))]
#[inline(always)]
pub fn always_f<T>() -> impl Fn(&T) -> bool {
    |_dat: &T| false
}

#[cfg(feature = "log")]
#[inline(always)]
pub fn always_f<T: std::fmt::Debug>() -> impl Fn(&T) -> bool {
    |_dat: &T| {
        trace_log!("always false, consume {_dat:?}(in)");
        false
    }
}

#[inline(always)]
pub fn space() -> impl Fn(&char) -> bool {
    |dat: &char| {
        trace_log!("match space with {dat:?}(in)");
        dat.is_whitespace()
    }
}

#[inline(always)]
pub fn digit() -> impl Fn(&char) -> bool {
    |dat: &char| {
        trace_log!("match ascii digit with {dat:?}(in)");
        dat.is_ascii_digit()
    }
}

#[inline(always)]
pub fn wild() -> impl Fn(&char) -> bool {
    |dat: &char| {
        trace_log!("match wild(.) with {dat:?}(in)");
        dat != &'\n'
    }
}

#[inline(always)]
pub fn not<T>(func: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("Change the match logical, not");
        !func(dat)
    }
}

#[inline(always)]
pub fn and<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("Change the match logical, and");
        func1(dat) && func2(dat)
    }
}

#[inline(always)]
pub fn or<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| {
        trace_log!("Change the match logical, or");
        func1(dat) || func2(dat)
    }
}

#[inline(always)]
pub fn byte(byte: u8) -> impl Fn(&u8) -> bool {
    move |dat: &u8| {
        trace_log!("match byte {byte} with {dat}(in)");
        byte == *dat
    }
}
