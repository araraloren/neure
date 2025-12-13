use crate::trace_retval;

macro_rules! setup_unit_ty {
    ($name:ident, $debug:literal, $func:ident) => {
        #[derive(Debug, Clone, Default, Copy)]
        pub struct $name;

        impl std::ops::Not for $name {
            type Output = crate::neu::Not<Self, char>;

            fn not(self) -> Self::Output {
                crate::neu::not(self)
            }
        }

        impl $name {
            pub const fn new() -> Self {
                Self {}
            }
        }

        impl Neu<char> for $name {
            #[inline(always)]
            fn is_match(&self, other: &char) -> bool {
                trace_retval!($debug, other, other.$func())
            }
        }
    };
}

macro_rules! setup_unit_ty2 {
    ($name:ident, $debug:literal, $func:ident) => {
        #[derive(Debug, Clone, Default, Copy)]
        pub struct $name<T>(std::marker::PhantomData<T>);

        impl<T> std::ops::Not for $name<T>
        where
            Self: Neu<T>,
        {
            type Output = crate::neu::Not<Self, T>;

            fn not(self) -> Self::Output {
                crate::neu::not(self)
            }
        }

        impl<T> $name<T> {
            pub const fn new() -> Self {
                Self(std::marker::PhantomData)
            }
        }

        impl Neu<char> for $name<char> {
            #[inline(always)]
            fn is_match(&self, other: &char) -> bool {
                trace_retval!($debug, other, other.$func())
            }
        }

        impl Neu<u8> for $name<u8> {
            #[inline(always)]
            fn is_match(&self, other: &u8) -> bool {
                trace_retval!($debug, other, other.$func())
            }
        }
    };
}

use super::Neu;

setup_unit_ty!(Alphabetic, "Alphabetic", is_alphabetic);

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

setup_unit_ty!(Alphanumeric, "Alphanumeric", is_alphanumeric);

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

setup_unit_ty2!(Ascii, "Ascii", is_ascii);

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
pub const fn ascii<T>() -> Ascii<T> {
    Ascii::new()
}

setup_unit_ty2!(AsciiAlphabetic, "AsciiAlphabetic", is_ascii_alphabetic);

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
pub const fn ascii_alphabetic<T>() -> AsciiAlphabetic<T> {
    AsciiAlphabetic::new()
}

setup_unit_ty2!(
    AsciiAlphanumeric,
    "AsciiAlphanumeric",
    is_ascii_alphanumeric
);

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
pub const fn ascii_alphanumeric<T>() -> AsciiAlphanumeric<T> {
    AsciiAlphanumeric::new()
}

setup_unit_ty2!(AsciiControl, "AsciiControl", is_ascii_control);

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
pub const fn ascii_control<T>() -> AsciiControl<T> {
    AsciiControl::new()
}

setup_unit_ty2!(AsciiDigit, "AsciiDigit", is_ascii_digit);

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
pub const fn ascii_digit<T>() -> AsciiDigit<T> {
    AsciiDigit::new()
}

setup_unit_ty2!(AsciiGraphic, "AsciiGraphic", is_ascii_graphic);

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
pub const fn ascii_graphic<T>() -> AsciiGraphic<T> {
    AsciiGraphic::new()
}

setup_unit_ty2!(AsciiHexDigit, "AsciiHexDigit", is_ascii_hexdigit);

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
pub const fn ascii_hexdigit<T>() -> AsciiHexDigit<T> {
    AsciiHexDigit::new()
}

setup_unit_ty2!(AsciiLowercase, "AsciiLowercase", is_ascii_lowercase);

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
pub const fn ascii_lowercase<T>() -> AsciiLowercase<T> {
    AsciiLowercase::new()
}

setup_unit_ty2!(AsciiPunctuation, "AsciiPunctuation", is_ascii_punctuation);

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
pub const fn ascii_punctuation<T>() -> AsciiPunctuation<T> {
    AsciiPunctuation::new()
}

setup_unit_ty2!(AsciiUppercase, "AsciiUppercase", is_ascii_uppercase);

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
pub const fn ascii_uppercase<T>() -> AsciiUppercase<T> {
    AsciiUppercase::new()
}

setup_unit_ty2!(AsciiWhiteSpace, "AsciiWhiteSpace", is_ascii_whitespace);

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
pub const fn ascii_whitespace<T>() -> AsciiWhiteSpace<T> {
    AsciiWhiteSpace::new()
}

setup_unit_ty!(Control, "Control", is_control);

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

impl std::ops::Not for Digit {
    type Output = crate::neu::Not<Self, char>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl Digit {
    pub const fn new(radix: u32) -> Self {
        Self(radix)
    }
}

impl Neu<char> for Digit {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_retval!("Digit", other, other.is_digit(self.0))
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

setup_unit_ty!(Lowercase, "Lowercase", is_lowercase);

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

setup_unit_ty!(Numeric, "Numeric", is_numeric);

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

setup_unit_ty!(Uppercase, "Uppercase", is_uppercase);

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

setup_unit_ty!(WhiteSpace, "WhiteSpace", is_whitespace);

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

impl std::ops::Not for Wild {
    type Output = crate::neu::Not<Self, char>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl Wild {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Neu<char> for Wild {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_retval!("Wild", other, other != &'\n')
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

#[derive(Debug, Clone, Default, Copy)]
pub struct Word<T>(std::marker::PhantomData<T>);

impl<T> std::ops::Not for Word<T>
where
    Self: Neu<T>,
{
    type Output = crate::neu::Not<Self, T>;

    fn not(self) -> Self::Output {
        crate::neu::not(self)
    }
}

impl<T> Word<T> {
    pub const fn new() -> Self {
        Self(std::marker::PhantomData)
    }

    pub fn contain_ch(other: &char) -> bool {
        let lower = 'a'..='z';
        let upper = 'A'..='Z';
        let digit = '0'..='9';

        lower.contains(other) || upper.contains(other) || digit.contains(other) || *other == '_'
    }

    pub fn contain_u8(other: &u8) -> bool {
        let lower = b'a'..=b'z';
        let upper = b'A'..=b'Z';
        let digit = b'0'..=b'9';

        lower.contains(other) || upper.contains(other) || digit.contains(other) || *other == b'_'
    }
}

impl Neu<char> for Word<char> {
    #[inline(always)]
    fn is_match(&self, other: &char) -> bool {
        trace_retval!("Wild", other, Self::contain_ch(other))
    }
}

impl Neu<u8> for Word<u8> {
    #[inline(always)]
    fn is_match(&self, other: &u8) -> bool {
        trace_retval!("Wild", other, Self::contain_u8(other))
    }
}

///
/// Match the character 'a' ..= 'z', 'A' ..= 'Z', '0' ..= '9' or '_'.
///
/// # Example
///
/// ```
/// use neure::prelude::*;
/// use neu::*;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let word = word();
///     let word = word.repeat_one_more();
///     let mut ctx = CharsCtx::new("TheLolipop_1\r\n");
///
///     assert_eq!(ctx.try_mat(&word)?, Span::new(0, 12));
///     assert!(ctx.try_mat(&word).is_err());
///     Ok(())
/// }
/// ```
pub const fn word<T>() -> Word<T> {
    Word::new()
}
