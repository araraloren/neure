pub mod _macro;
pub mod ctx;
pub mod err;
pub mod parser;
pub mod regex;

pub use self::ctx::CharsCtx;
pub use self::ctx::Context;
pub use self::ctx::Span;
pub use self::parser::Count;
pub use self::parser::End;
pub use self::parser::Parser;
pub use self::parser::Start;
pub use self::regex::Digit;
pub use self::regex::Dot;
pub use self::regex::Not;
pub use self::regex::Regex;
pub use self::regex::Space;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        let mut ctx = CharsCtx::new("cd", 1);
        let ch = neure!([^'a' - 'b']);

        dbg!(ctx.capture(0, ch));
        dbg!(ctx);
    }
}
