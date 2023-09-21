use super::Unit;

use crate::trace_log;

#[derive(Debug, Clone, Default, Copy)]
pub struct Alphabetic;

impl Alphabetic {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Alphabetic {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match alphabetic with value ({})(in)", other);
        other.is_alphabetic()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Alphanumeric;

impl Alphanumeric {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Alphanumeric {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match alphanumeric with value ({})(in)", other);
        other.is_alphanumeric()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Ascii;

impl Ascii {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Ascii {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii with value ({})(in)", other);
        other.is_ascii()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiAlphabetic;

impl AsciiAlphabetic {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiAlphabetic {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii alphabetic with value ({})(in)", other);
        other.is_ascii_alphabetic()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiAlphanumeric;

impl AsciiAlphanumeric {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiAlphanumeric {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii alphanumeric with value ({})(in)", other);
        other.is_ascii_alphanumeric()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiControl;

impl AsciiControl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiControl {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii control with value ({})(in)", other);
        other.is_ascii_control()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiDigit;

impl AsciiDigit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiDigit {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii digit with value ({})(in)", other);
        other.is_ascii_digit()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiGraphic;

impl AsciiGraphic {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiGraphic {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii graphics with value ({})(in)", other);
        other.is_ascii_graphic()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiHexDigit;

impl AsciiHexDigit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiHexDigit {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii hex digit with value ({})(in)", other);
        other.is_ascii_hexdigit()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiLowercase;

impl AsciiLowercase {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiLowercase {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii lowercase with value ({})(in)", other);
        other.is_ascii_lowercase()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiPunctuation;

impl AsciiPunctuation {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiPunctuation {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii punctuation with value ({})(in)", other);
        other.is_ascii_punctuation()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiUppercase;

impl AsciiUppercase {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiUppercase {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii uppercase with value ({})(in)", other);
        other.is_ascii_uppercase()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiWhiteSpace;

impl AsciiWhiteSpace {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for AsciiWhiteSpace {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match ascii white space with value ({})(in)", other);
        other.is_ascii_whitespace()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Control;

impl Control {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Control {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match control with value ({})(in)", other);
        other.is_control()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Digit(u32);

impl Digit {
    pub fn new(radix: u32) -> Self {
        Self(radix)
    }
}

impl Unit<char> for Digit {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match digit(radix = {}) with value ({})(in)", self.0, other);
        other.is_digit(self.0)
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Lowercase;

impl Lowercase {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Lowercase {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match lowercase with value ({})(in)", other);
        other.is_lowercase()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Numeric;

impl Numeric {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Numeric {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match numeric with value ({})(in)", other);
        other.is_numeric()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Uppercase;

impl Uppercase {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Uppercase {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match uppercase with value ({})(in)", other);
        other.is_uppercase()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct WhiteSpace;

impl WhiteSpace {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for WhiteSpace {
    fn is_match(&self, other: &char) -> bool {
        trace_log!("match space with value ({})(in)", other);
        other.is_whitespace()
    }
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Wild;

impl Wild {
    pub fn new() -> Self {
        Self {}
    }
}

impl Unit<char> for Wild {
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
///     let space = regex::whitespace();
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
pub fn whitespace() -> WhiteSpace {
    WhiteSpace::default()
}

///
/// Reference [`is_ascii_whitespace`](std::primitive::char::is_ascii_whitespace).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neure::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let space = regex::ascii_whitespace();
///     let space1 = space.repeat(1);
///     let space3 = space.repeat(3);
///     let mut ctx = CharsCtx::new("    \u{A0}abcd");
///
///     assert_eq!(ctx.try_mat(&space1)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&space3)?, Span::new(1, 3));
///     assert!(ctx.try_mat(&space1).is_err());
///     Ok(())
/// }
/// ```
pub fn ascii_whitespace() -> AsciiWhiteSpace {
    AsciiWhiteSpace::default()
}

pub fn digit() -> Digit {
    Digit::default()
}

pub fn hex_digit() -> AsciiHexDigit {
    AsciiHexDigit::default()
}

pub fn wild() -> Wild {
    Wild::default()
}
