#![doc = include_str!("../README.md")]
pub mod _macro;
pub mod bytes;
pub mod chars;
pub mod err;
pub mod etx2;
pub mod index;
pub mod iter;
pub mod parser;
pub mod policy;
pub mod regex;
pub mod span;

pub use self::bytes::BytesCtx;
pub use self::chars::CharsCtx;
pub use self::index::IndexBySpan;
pub use self::iter::IteratorBySpan;
pub use self::parser::*;
pub use self::policy::Context;
pub use self::policy::Length;
pub use self::policy::MatchPolicy;
pub use self::regex::*;
pub use self::span::Span;
pub use self::span::SpanStore;
pub use self::span::SpanStorer;
pub use charize::charize;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_all() {
        assert!(test_space().is_ok());
        assert!(test_char().is_ok());
        assert!(test_chars().is_ok());
        assert!(test_chars_negative().is_ok());
        assert!(test_range().is_ok());
        assert!(test_range_negative().is_ok());
        assert!(test_other().is_ok());
    }

    macro_rules! test_t {
        ($ctx:ident, $storer:ident, $str:literal, $id:literal, $parser:expr) => {
            let space_parser = $parser;

            $ctx.reset_with($str);
            $storer.reset();
            assert!($ctx.try_cap($id, &mut $storer, space_parser).is_err());
        };
        ($ctx:ident, $storer:ident, $str:literal, $id:literal, $parser:expr, $($span:expr)*) => {
            let space_parser = $parser;

            $ctx.reset_with($str);
            $storer.reset();
            $ctx.try_cap($id, &mut $storer, space_parser)?;
            assert_eq!($storer.spans($id)?.collect::<Vec<_>>(), vec![$($span)*])
        };
    }

    fn test_other() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(ctx, storer, "abedf", 0, neure!(.), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "abedf", 0, neure!(.?), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            storer,
            "\nabedf",
            0,
            neure!(.?),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, storer, "abedf", 0, neure!(.*), Span { beg: 0, len: 5 });
        test_t!(
            ctx,
            storer,
            "\nabedf",
            0,
            neure!(.*),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, storer, "abedf", 0, neure!(.+), Span { beg: 0, len: 5 });
        test_t!(ctx, storer, "\nabedf", 0, neure!(.+));
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            neure!(.{2}),
            Span { beg: 0, len: 2 }
        );
        test_t!(ctx, storer, "ab\nedf", 0, neure!(.{3}));
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            neure!(.{2,}),
            Span { beg: 0, len: 5 }
        );
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            neure!(.{4,6}),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "abe\ndf", 0, neure!(.{4,6}));
        test_t!(
            ctx,
            storer,
            "c\nabedf",
            0,
            neure!(^),
            Span { beg: 0, len: 1 }
        );

        Ok(())
    }

    fn test_range_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(
            ctx,
            storer,
            "Raefc",
            0,
            neure!([^a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "你aefc",
            0,
            neure!([^a - z A - Z]),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "&AEUF",
            0,
            neure!([^a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            neure!([^a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^a-z]+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^a-z]{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            neure!([^'a' - 'z' 'A' - 'Z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^'a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^'a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^'a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^'a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^'a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^'a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([^'a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, neure!([^'a'-'z']{3,}));
        Ok(())
    }

    fn test_range() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!([a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!([a-z]+));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!([a-z]{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!([a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!([a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a' - 'z']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a' - 'z']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!(['a' - 'z']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a'-'z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!(['a'-'z']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!(['a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            neure!(['a'-'z''A'-'Z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!(['a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!(['a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!(['a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, neure!(['a'-'z']{3,}));

        Ok(())
    }

    fn test_chars_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^'b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^'b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            neure!([^b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^b c d]*),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            neure!([^b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            neure!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefcddd",
            0,
            neure!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "baefcddd", 0, neure!([^b c d]+));
        test_t!(
            ctx,
            storer,
            "aefh",
            0,
            neure!([^b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhcc",
            0,
            neure!([^b c d]{4,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhccd",
            0,
            neure!([^b c d]{4,7}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aecfhccd", 0, neure!([^b c d]{4,7}));
        Ok(())
    }

    fn test_chars() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!([b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!([b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            neure!([b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            neure!([b c d]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            neure!([b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!([b c d]+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            neure!([b c d]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, neure!([b c d]+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            neure!([b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            neure!([b c d]{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            neure!([b c d]{4,7}),
            Span { beg: 0, len: 6 }
        );

        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            neure!(['b' 'c' 'd']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            neure!(['b' 'c' 'd']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            neure!(['b' 'c' 'd']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            neure!(['b' 'c' 'd']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, neure!(['b' 'c' 'd']+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            neure!(['b' 'c' 'd']{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            neure!(['b' 'c' 'd']{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            neure!(['b' 'c' 'd']{4,7}),
            Span { beg: 0, len: 6 }
        );
        Ok(())
    }

    fn test_char() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(ctx, storer, "a", 0, neure!(a), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, neure!('a'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, neure!(a?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, neure!('a'?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "你", 0, neure!(你), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            storer,
            "你you",
            0,
            neure!('你'),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "@", 0, neure!('@'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "der", 0, neure!(a?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "", 0, neure!('a'?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "a", 0, neure!(a*), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            storer,
            "aaaaaee",
            0,
            neure!('a'*),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "cde", 0, neure!(a*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            neure!('a'+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            neure!(你+),
            Span { beg: 0, len: 9 }
        );
        test_t!(ctx, storer, "我你你你", 0, neure!(你+));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            neure!('a'{2}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            neure!(你{2}),
            Span { beg: 0, len: 6 }
        );
        test_t!(ctx, storer, "你", 0, neure!(你{2}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            neure!('a'{2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            neure!(你{2,}),
            Span { beg: 0, len: 9 }
        );
        test_t!(ctx, storer, "你", 0, neure!(你{2,}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            neure!('a'{2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "你你你你你啊",
            0,
            neure!(你{2,4}),
            Span { beg: 0, len: 12 }
        );
        test_t!(ctx, storer, "你啊", 0, neure!(你{2,4}));

        Ok(())
    }

    fn test_space() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("");
        let mut storer = SpanStorer::new(1);

        test_t!(ctx, storer, "\tcd", 0, neure!(), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "\tdwq", 0, neure!(?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dwq", 0, neure!(?), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "\t\n\rdda",
            0,
            neure!(*),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "dda", 0, neure!(*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "\t\n\rdda",
            0,
            neure!(+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "\tdda", 0, neure!(+), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dda", 0, neure!(+));
        test_t!(
            ctx,
            storer,
            " \u{A0}dda",
            0,
            neure!({ 2 }),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "\u{A0}dda", 0, neure!({ 2 }));
        test_t!(
            ctx,
            storer,
            "\t\rdda",
            0,
            neure!({2,}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            neure!({2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "dda", 0, neure!());
        test_t!(
            ctx,
            storer,
            "\t\ndda",
            0,
            neure!({2,3}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            neure!({2,3}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r \u{A0}dda",
            0,
            neure!({2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, " dda", 0, neure!({2,3}));

        Ok(())
    }
}
