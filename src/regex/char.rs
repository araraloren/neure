use super::Regex;

use crate::trace_log;

#[derive(Debug, Clone, Default, Copy)]
pub struct Space;

impl Space {
    pub fn new() -> Self {
        Self {}
    }
}

impl Regex<char> for Space {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match space with value ({})(in)", other);
        other.is_whitespace()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiSpace;

impl AsciiSpace {
    pub fn new() -> Self {
        Self {}
    }
}

impl Regex<char> for AsciiSpace {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii space with value ({})(in)", other);
        other.is_ascii_whitespace()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Digit;

impl Digit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Regex<char> for Digit {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match digit with value ({})(in)", other);
        other.is_ascii_digit()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct HexDigit;

impl HexDigit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Regex<char> for HexDigit {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match hex digit with value ({})(in)", other);
        other.is_ascii_hexdigit()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Wild;

impl Wild {
    pub fn new() -> Self {
        Self {}
    }
}

impl Regex<char> for Wild {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match wild(.) with value ({})(in)", other);
        other != &'\n'
    }
}

pub fn space() -> Space {
    Space::default()
}

pub fn ascii_space() -> AsciiSpace {
    AsciiSpace::default()
}

pub fn digit() -> Digit {
    Digit::default()
}
