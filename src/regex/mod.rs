pub mod r#macro;

use std::ops::RangeBounds;

#[inline(always)]
pub fn char(ch: char) -> impl Fn(&char) -> bool {
    move |dat: &char| dat == &ch
}

#[inline(always)]
pub fn equal<T: PartialEq>(val: T) -> impl Fn(&T) -> bool {
    move |dat: &T| dat == &val
}

#[inline(always)]
pub fn array<const N: usize, T: PartialEq>(chars: [T; N]) -> impl Fn(&T) -> bool {
    move |dat: &T| chars.contains(dat)
}

#[inline(always)]
pub fn vector<T: PartialEq>(chars: Vec<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| chars.contains(dat)
}

#[inline(always)]
pub fn range<T: PartialOrd>(bound: impl RangeBounds<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| bound.contains(dat)
}

#[inline(always)]
pub fn always_t<T>() -> impl Fn(&T) -> bool {
    |_: &T| true
}

#[inline(always)]
pub fn always_f<T>() -> impl Fn(&T) -> bool {
    |_: &T| false
}

#[inline(always)]
pub fn space() -> impl Fn(&char) -> bool {
    |dat: &char| dat.is_whitespace()
}

#[inline(always)]
pub fn digit() -> impl Fn(&char) -> bool {
    |dat: &char| dat.is_ascii_digit()
}

#[inline(always)]
pub fn wild() -> impl Fn(&char) -> bool {
    |dat: &char| dat != &'\n'
}

#[inline(always)]
pub fn not<T>(func: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| !func(dat)
}

#[inline(always)]
pub fn and<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| func1(dat) && func2(dat)
}

#[inline(always)]
pub fn or<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| func1(dat) || func2(dat)
}

#[inline(always)]
pub fn byte(byte: u8) -> impl Fn(&u8) -> bool {
    move |dat: &u8| byte == *dat
}
