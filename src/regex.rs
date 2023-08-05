use std::ops::RangeBounds;

pub fn char(ch: char) -> impl Fn(&char) -> bool {
    move |dat: &char| dat == &ch
}

pub fn array<const N: usize>(chars: [char; N]) -> impl Fn(&char) -> bool {
    move |dat: &char| chars.contains(dat)
}

pub fn vector(chars: Vec<char>) -> impl Fn(&char) -> bool {
    move |dat: &char| chars.contains(dat)
}

pub fn range(bound: impl RangeBounds<char>) -> impl Fn(&char) -> bool {
    move |dat: &char| bound.contains(dat)
}

pub fn always_t() -> impl Fn(&char) -> bool {
    |_: &char| true
}

pub fn always_f() -> impl Fn(&char) -> bool {
    |_: &char| false
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

pub fn not(func: impl Fn(&char) -> bool) -> impl Fn(&char) -> bool {
    move |dat: &char| !func(dat)
}

pub fn and(func1: impl Fn(&char) -> bool, func2: impl Fn(&char) -> bool) -> impl Fn(&char) -> bool {
    move |dat: &char| func1(dat) && func2(dat)
}

pub fn or(func1: impl Fn(&char) -> bool, func2: impl Fn(&char) -> bool) -> impl Fn(&char) -> bool {
    move |dat: &char| func1(dat) || func2(dat)
}
