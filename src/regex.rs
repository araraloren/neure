use std::ops::RangeBounds;

pub fn char(ch: char) -> impl Fn(&char) -> bool {
    move |dat: &char| dat == &ch
}

pub fn equal<T: PartialEq>(val: T) -> impl Fn(&T) -> bool {
    move |dat: &T| dat == &val
}

pub fn array<const N: usize, T: PartialEq>(chars: [T; N]) -> impl Fn(&T) -> bool {
    move |dat: &T| chars.contains(dat)
}

pub fn vector<T: PartialEq>(chars: Vec<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| chars.contains(dat)
}

pub fn range<T: PartialOrd>(bound: impl RangeBounds<T>) -> impl Fn(&T) -> bool {
    move |dat: &T| bound.contains(dat)
}

pub fn always_t<T>() -> impl Fn(&T) -> bool {
    |_: &T| true
}

pub fn always_f<T>() -> impl Fn(&T) -> bool {
    |_: &T| false
}

pub fn space() -> impl Fn(&char) -> bool {
    |dat: &char| dat.is_whitespace()
}

pub fn digit() -> impl Fn(&char) -> bool {
    |dat: &char| dat.is_ascii_digit()
}

pub fn wild() -> impl Fn(&char) -> bool {
    |dat: &char| dat != &'\n'
}

pub fn not<T>(func: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| !func(dat)
}

pub fn and<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| func1(dat) && func2(dat)
}

pub fn or<T>(func1: impl Fn(&T) -> bool, func2: impl Fn(&T) -> bool) -> impl Fn(&T) -> bool {
    move |dat: &T| func1(dat) || func2(dat)
}

pub fn byte(byte: u8) -> impl Fn(&u8) -> bool {
    move |dat: &u8| byte == *dat
}
