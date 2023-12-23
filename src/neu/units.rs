use super::trace_u;
use super::Neu;

#[derive(Debug, Clone, Default, Copy)]
pub struct Alphabetic;

impl Alphabetic {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Alphabetic {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("alphabetic", self, other, other.is_alphabetic())
    }
}

///
/// Reference [`is_alphabetic`](std::primitive::char::is_alphabetic).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let alpha = alphabetic();
///     let alpha = alpha.repeat_times::<1>();
///     let mut ctx = CharsCtx::new("a💝abcd");
///
///     assert_eq!(ctx.try_mat(&alpha)?, Span::new(0, 1));
///     assert!(ctx.try_mat(&alpha).is_err());
///     Ok(())
/// }
/// ```
pub const fn alphabetic() -> Alphabetic {
    Alphabetic
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Alphanumeric;

impl Alphanumeric {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Alphanumeric {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("alphanumeric", self, other, other.is_alphanumeric())
    }
}

///
/// Reference [`is_alphanumeric`](std::primitive::char::is_alphanumeric).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let alphanumeric = alphanumeric();
///     let alphanumeric = alphanumeric.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("①7Kوf");
///
///     assert_eq!(ctx.try_mat(&alphanumeric)?, Span::new(0, 4));
///     assert_eq!(ctx.try_mat(&alphanumeric)?, Span::new(4, 3));
///     assert!(ctx.try_mat(&alphanumeric).is_err());
///     Ok(())
/// }
/// ```
pub const fn alphanumeric() -> Alphanumeric {
    Alphanumeric
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Ascii;

impl Ascii {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Ascii {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii", self, other, other.is_ascii())
    }
}

impl Neu<u8> for Ascii {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii", self, other, other.is_ascii())
    }
}

///
/// Reference [`is_ascii`](std::primitive::char::is_ascii) or [`is_ascii`](std::primitive::u8::is_ascii).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii = ascii();
///     let ascii = ascii.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("ab❤e");
///
///     assert_eq!(ctx.try_mat(&ascii)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii() -> Ascii {
    Ascii
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiAlphabetic;

impl AsciiAlphabetic {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiAlphabetic {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_alphabetic", self, other, other.is_ascii_alphabetic())
    }
}

impl Neu<u8> for AsciiAlphabetic {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_alphabetic", self, other, other.is_ascii_alphabetic())
    }
}

///
/// Reference [`is_ascii_alphabetic`](std::primitive::char::is_ascii_alphabetic) or [`is_ascii_alphabetic`](std::primitive::u8::is_ascii_alphabetic).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_alphabetic = ascii_alphabetic();
///     let ascii_alphabetic = ascii_alphabetic.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("ab%e");
///
///     assert_eq!(ctx.try_mat(&ascii_alphabetic)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_alphabetic).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_alphabetic() -> AsciiAlphabetic {
    AsciiAlphabetic
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiAlphanumeric;

impl AsciiAlphanumeric {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiAlphanumeric {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!(
            "ascii_alphanumeric",
            self,
            other,
            other.is_ascii_alphanumeric()
        )
    }
}

impl Neu<u8> for AsciiAlphanumeric {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!(
            "ascii_alphanumeric",
            self,
            other,
            other.is_ascii_alphanumeric()
        )
    }
}

///
/// Reference [`is_ascii_alphanumeric`](std::primitive::char::is_ascii_alphanumeric) or [`is_ascii_alphanumeric`](std::primitive::u8::is_ascii_alphanumeric).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_alphanumeric = ascii_alphanumeric();
///     let ascii_alphanumeric = ascii_alphanumeric.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("8a%e");
///
///     assert_eq!(ctx.try_mat(&ascii_alphanumeric)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_alphanumeric).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_alphanumeric() -> AsciiAlphanumeric {
    AsciiAlphanumeric
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiControl;

impl AsciiControl {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiControl {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_control", self, other, other.is_ascii_control())
    }
}

impl Neu<u8> for AsciiControl {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_control", self, other, other.is_ascii_control())
    }
}

///
/// Reference [`is_ascii_control`](std::primitive::char::is_ascii_control) or [`is_ascii_control`](std::primitive::u8::is_ascii_control).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_control = ascii_control();
///     let ascii_control = ascii_control.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("\r\n%e");
///
///     assert_eq!(ctx.try_mat(&ascii_control)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_control).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_control() -> AsciiControl {
    AsciiControl
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiDigit;

impl AsciiDigit {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiDigit {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_digit", self, other, other.is_ascii_digit())
    }
}

impl Neu<u8> for AsciiDigit {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_digit", self, other, other.is_ascii_digit())
    }
}

///
/// Reference [`is_ascii_digit`](std::primitive::char::is_ascii_digit) or [`is_ascii_digit`](std::primitive::u8::is_ascii_digit).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_digit = ascii_digit();
///     let ascii_digit = ascii_digit.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("90fe");
///
///     assert_eq!(ctx.try_mat(&ascii_digit)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_digit).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_digit() -> AsciiDigit {
    AsciiDigit
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiGraphic;

impl AsciiGraphic {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiGraphic {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_graphic", self, other, other.is_ascii_graphic())
    }
}

impl Neu<u8> for AsciiGraphic {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_graphic", self, other, other.is_ascii_graphic())
    }
}

///
/// Reference [`is_ascii_graphic`](std::primitive::char::is_ascii_graphic) or [`is_ascii_graphic`](std::primitive::u8::is_ascii_graphic).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_graphic = ascii_graphic();
///     let ascii_graphic = ascii_graphic.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("a%\r\n");
///
///     assert_eq!(ctx.try_mat(&ascii_graphic)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_graphic).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_graphic() -> AsciiGraphic {
    AsciiGraphic
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiHexDigit;

impl AsciiHexDigit {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiHexDigit {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_hexdigit", self, other, other.is_ascii_hexdigit())
    }
}

impl Neu<u8> for AsciiHexDigit {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_hexdigit", self, other, other.is_ascii_hexdigit())
    }
}

///
/// Reference [`is_ascii_hexdigit`](std::primitive::char::is_ascii_hexdigit) or [`is_ascii_hexdigit`](std::primitive::u8::is_ascii_hexdigit).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_hexdigit = ascii_hexdigit();
///     let ascii_hexdigit = ascii_hexdigit.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("9fg0");
///
///     assert_eq!(ctx.try_mat(&ascii_hexdigit)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_hexdigit).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_hexdigit() -> AsciiHexDigit {
    AsciiHexDigit
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiLowercase;

impl AsciiLowercase {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiLowercase {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_lowercase", self, other, other.is_ascii_lowercase())
    }
}

impl Neu<u8> for AsciiLowercase {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_lowercase", self, other, other.is_ascii_lowercase())
    }
}

///
/// Reference [`is_ascii_lowercase`](std::primitive::char::is_ascii_lowercase) or [`is_ascii_lowercase`](std::primitive::u8::is_ascii_lowercase).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_lowercase = ascii_lowercase();
///     let ascii_lowercase = ascii_lowercase.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("efAE");
///
///     assert_eq!(ctx.try_mat(&ascii_lowercase)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_lowercase).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_lowercase() -> AsciiLowercase {
    AsciiLowercase
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiPunctuation;

impl AsciiPunctuation {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiPunctuation {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!(
            "ascii_punctuation",
            self,
            other,
            other.is_ascii_punctuation()
        )
    }
}

impl Neu<u8> for AsciiPunctuation {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!(
            "ascii_punctuation",
            self,
            other,
            other.is_ascii_punctuation()
        )
    }
}

///
/// Reference [`is_ascii_punctuation`](std::primitive::char::is_ascii_punctuation) or [`is_ascii_punctuation`](std::primitive::u8::is_ascii_punctuation).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_punctuation = ascii_punctuation();
///     let ascii_punctuation = ascii_punctuation.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("%%\nE");
///
///     assert_eq!(ctx.try_mat(&ascii_punctuation)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_punctuation).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_punctuation() -> AsciiPunctuation {
    AsciiPunctuation
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiUppercase;

impl AsciiUppercase {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiUppercase {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_uppercase", self, other, other.is_ascii_uppercase())
    }
}

impl Neu<u8> for AsciiUppercase {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_uppercase", self, other, other.is_ascii_uppercase())
    }
}

///
/// Reference [`is_ascii_uppercase`](std::primitive::char::is_ascii_uppercase) or [`is_ascii_uppercase`](std::primitive::u8::is_ascii_uppercase).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ascii_uppercase = ascii_uppercase();
///     let ascii_uppercase = ascii_uppercase.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("EFef");
///
///     assert_eq!(ctx.try_mat(&ascii_uppercase)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&ascii_uppercase).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_uppercase() -> AsciiUppercase {
    AsciiUppercase
}

#[derive(Debug, Clone, Default, Copy)]
pub struct AsciiWhiteSpace;

impl AsciiWhiteSpace {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for AsciiWhiteSpace {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("ascii_whitespace", self, other, other.is_ascii_whitespace())
    }
}

impl Neu<u8> for AsciiWhiteSpace {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_u!("ascii_whitespace", self, other, other.is_ascii_whitespace())
    }
}

///
/// Reference [`is_ascii_whitespace`](std::primitive::char::is_ascii_whitespace) or [`is_ascii_whitespace`](std::primitive::u8::is_ascii_whitespace).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let space = ascii_whitespace();
///     let space1 = space.repeat_times::<1>();
///     let space3 = space.repeat_times::<3>();
///     let mut ctx = CharsCtx::new("    w\u{A0}abcd");
///
///     assert_eq!(ctx.try_mat(&space1)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&space3)?, Span::new(1, 3));
///     assert!(ctx.try_mat(&space1).is_err());
///     Ok(())
/// }
/// ```
pub const fn ascii_whitespace() -> AsciiWhiteSpace {
    AsciiWhiteSpace
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Control;

impl Control {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Control {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("control", self, other, other.is_control())
    }
}

///
/// Reference [`is_control`](std::primitive::char::is_control).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let control = control();
///     let control = control.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("ef");
///
///     assert_eq!(ctx.try_mat(&control)?, Span::new(0, 4));
///     assert!(ctx.try_mat(&control).is_err());
///     Ok(())
/// }
/// ```
pub const fn control() -> Control {
    Control
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Digit(u32);

impl Digit {
    pub const fn new(radix: u32) -> Self {
        Self(radix)
    }
}

impl Neu<char> for Digit {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("digit", self, other, other.is_digit(self.0))
    }
}

///
/// Reference [`is_digit`](std::primitive::char::is_digit).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let digit = digit(10);
///     let digit = digit.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("54aa");
///
///     assert_eq!(ctx.try_mat(&digit)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&digit).is_err());
///     Ok(())
/// }
/// ```
pub const fn digit(radix: u32) -> Digit {
    Digit::new(radix)
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Lowercase;

impl Lowercase {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Lowercase {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("lowercase", self, other, other.is_lowercase())
    }
}

///
/// Reference [`is_lowercase`](std::primitive::char::is_lowercase).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let lowercase = lowercase();
///     let lowercase = lowercase.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("aδΔA");
///
///     assert_eq!(ctx.try_mat(&lowercase)?, Span::new(0, 3));
///     assert!(ctx.try_mat(&lowercase).is_err());
///     Ok(())
/// }
/// ```
pub const fn lowercase() -> Lowercase {
    Lowercase
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Numeric;

impl Numeric {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Numeric {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("numeric", self, other, other.is_numeric())
    }
}

///
/// Reference [`is_numeric`](std::primitive::char::is_numeric).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let numeric = numeric();
///     let numeric = numeric.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("①¾Kو");
///
///     assert_eq!(ctx.try_mat(&numeric)?, Span::new(0, 5));
///     assert!(ctx.try_mat(&numeric).is_err());
///     Ok(())
/// }
/// ```
pub const fn numeric() -> Numeric {
    Numeric
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Uppercase;

impl Uppercase {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Uppercase {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("uppercase", self, other, other.is_uppercase())
    }
}

///
/// Reference [`is_uppercase`](std::primitive::char::is_uppercase).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let uppercase = uppercase();
///     let uppercase = uppercase.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("ΔAaΔ");
///
///     assert_eq!(ctx.try_mat(&uppercase)?, Span::new(0, 3));
///     assert!(ctx.try_mat(&uppercase).is_err());
///     Ok(())
/// }
/// ```
pub const fn uppercase() -> Uppercase {
    Uppercase
}

#[derive(Debug, Clone, Default, Copy)]
pub struct WhiteSpace;

impl WhiteSpace {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for WhiteSpace {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("whitespace", self, other, other.is_whitespace())
    }
}

///
/// Reference [`is_whitespace`](std::primitive::char::is_whitespace).
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let space = whitespace();
///     let space1 = space.repeat_times::<1>();
///     let space3 = space.repeat_times::<3>();
///     let mut ctx = CharsCtx::new("   \u{A0}abcd");
///
///     assert_eq!(ctx.try_mat(&space1)?, Span::new(0, 1));
///     assert_eq!(ctx.try_mat(&space3)?, Span::new(1, 4));
///     assert!(ctx.try_mat(&space1).is_err());
///     Ok(())
/// }
/// ```
pub const fn whitespace() -> WhiteSpace {
    WhiteSpace
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Wild;

impl Wild {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Wild {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_u!("wild", '\n', other, other != &'\n')
    }
}

///
/// Match all the characters except `\n`.
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let wild = wild();
///     let wild = wild.repeat_times::<2>();
///     let mut ctx = CharsCtx::new("aa\r\n");
///
///     assert_eq!(ctx.try_mat(&wild)?, Span::new(0, 2));
///     assert!(ctx.try_mat(&wild).is_err());
///     Ok(())
/// }
/// ```
pub const fn wild() -> Wild {
    Wild
}
