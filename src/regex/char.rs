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

/// 
/// Reference [`is_whitespace`](std::primitive::char::is_whitespace).
/// 
/// # Example
/// 
/// ```
/// use neure::prelude::*;
/// use neure::*;
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let space = regex::space();
///     let space1 = space.repeat(1);
///     let space3 = space.repeat(3);
///     let mut ctx = CharsCtx::new("   \u{A0}abcd");
///
///     assert_eq!(ctx.try_mat(&space1)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&space3)?, Span::new(1, 4));
///     assert!(ctx.try_mat(&space1).is_err());
///     Ok(())
/// }
/// ```
pub fn space() -> Space {
    Space::default()
}

pub fn ascii_space() -> AsciiSpace {
    AsciiSpace::default()
}

pub fn digit() -> Digit {
    Digit::default()
}

pub fn hex_digit() -> HexDigit {
    HexDigit::default()
}

pub fn wild() -> Wild {
    Wild::default()
}
