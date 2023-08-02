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
    fn test_all() {
        //assert!(test_space().is_ok());
        assert!(test_char().is_ok());
    }

    fn test_char() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 2);
        let ch = neure!('a');

        ctx.try_mat(start())?;
        ctx.try_cap(0, ch)?;
        ctx.try_mat(end())?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 1 }]));

        Ok(())
    }

    fn test_space() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("\tcd", 1);
        let space = neure!();

        ctx.try_cap(0, space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 1 }]));

        let space = neure!(?);

        ctx.reset_with("\tdwq");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 1 }]));

        ctx.reset_with("dwq");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 0 }]));

        let space = neure!(*);

        ctx.reset_with("\t\n\rdda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 3 }]));

        ctx.reset_with("dda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 0 }]));

        let space = neure!(+);

        ctx.reset_with("\t\n\rdda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 3 }]));

        ctx.reset_with("\tdda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 1 }]));

        ctx.reset_with("dda");
        assert!(ctx.try_cap(0, &space).is_err());

        let space = neure!({ 2 });

        ctx.reset_with(" \u{A0}dda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 3 }]));

        ctx.reset_with("\u{A0}dda");
        assert!(ctx.try_cap(0, &space).is_err());

        let space = neure!({2,});

        ctx.reset_with("\t\rdda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 2 }]));

        ctx.reset_with("\t\r\u{A0}dda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 4 }]));

        ctx.reset_with(" dda");
        assert!(ctx.try_cap(0, &space).is_err());

        let space = neure!({2, 3});

        ctx.reset_with("\t\rdda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 2 }]));

        ctx.reset_with("\t\r\u{A0}dda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 4 }]));

        ctx.reset_with("\t\r \u{A0}dda");
        ctx.try_cap(0, &space)?;
        assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 3 }]));

        ctx.reset_with(" dda");
        assert!(ctx.try_cap(0, &space).is_err());
        Ok(())
    }
}
