pub mod _macro;
pub mod ctx;
pub mod err;
pub mod parser;
pub mod regex;

pub use self::ctx::CharsCtx;
pub use self::ctx::Context;
pub use self::ctx::Span;
pub use self::parser::*;
pub use self::regex::*;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        let mut ctx = CharsCtx::new("cd", 1);
        let ch = neure!([^'a' - 'b']);

        ctx.cap(0, &ch);
        ctx.cap(0, &ch);
        assert_eq!(
            ctx.spans(0),
            Some(&vec![Span { beg: 0, len: 1 }, Span { beg: 1, len: 1 }])
        );
        let ch = neure!([^'a' - 'b']+);

        ctx.reset();
        ctx.cap(0, ch);
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 2 }]));
        let ch = neure!('.'{2});
    }
}
