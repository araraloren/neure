pub mod _macro;
pub mod ctx;
pub mod err;
pub mod iter;
pub mod parser;
pub mod peek;
pub mod policy;
pub mod regex;

pub use self::ctx::CharsCtx;
pub use self::parser::*;
pub use self::peek::CharPeek;
pub use self::peek::Span;
pub use self::peek::StrPeek;
pub use self::policy::MatchPolicy;
pub use self::policy::Ret;
pub use self::regex::*;
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
        ($ctx:ident, $str:literal, $id:literal, $parser:expr) => {
            let space_parser = $parser;

            $ctx.reset_with($str);
            assert!($ctx.try_cap($id, space_parser).is_err());
        };
        ($ctx:ident, $str:literal, $id:literal, $parser:expr, $($span:expr)*) => {
            let space_parser = $parser;

            $ctx.reset_with($str);
            $ctx.try_cap($id, space_parser)?;
            assert_eq!($ctx.spans($id)?, &vec![$($span)*])
        };
    }

    fn test_other() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "abedf", 0, neure!(.), Span { beg: 0, len: 1 });
        test_t!(ctx, "abedf", 0, neure!(.?), Span { beg: 0, len: 1 });
        test_t!(ctx, "\nabedf", 0, neure!(.?), Span { beg: 0, len: 0 });
        test_t!(ctx, "abedf", 0, neure!(.*), Span { beg: 0, len: 5 });
        test_t!(ctx, "\nabedf", 0, neure!(.*), Span { beg: 0, len: 0 });
        test_t!(ctx, "abedf", 0, neure!(.+), Span { beg: 0, len: 5 });
        test_t!(ctx, "\nabedf", 0, neure!(.+));
        test_t!(ctx, "abedf", 0, neure!(.{2}), Span { beg: 0, len: 2 });
        test_t!(ctx, "ab\nedf", 0, neure!(.{3}));
        test_t!(ctx, "abedf", 0, neure!(.{2,}), Span { beg: 0, len: 5 });
        test_t!(ctx, "abedf", 0, neure!(.{4,6}), Span { beg: 0, len: 5 });
        test_t!(ctx, "abe\ndf", 0, neure!(.{4,6}));
        test_t!(ctx, "c\nabedf", 0, neure!(^), Span { beg: 0, len: 1 });

        Ok(())
    }

    fn test_range_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "Raefc", 0, neure!([^a - z]), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "你aefc",
            0,
            neure!([^a - z A - Z]),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "aefc", 0, neure!([^a - z]?), Span { beg: 0, len: 0 });
        test_t!(ctx, "AEUF", 0, neure!([^a - z]?), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "&AEUF",
            0,
            neure!([^a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "AEUF", 0, neure!([^a-z]*), Span { beg: 0, len: 4 });
        test_t!(ctx, "aefc", 0, neure!([^a-z]*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            "@#$%",
            0,
            neure!([^a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, neure!([^a-z]+), Span { beg: 0, len: 4 });
        test_t!(ctx, "aefc", 0, neure!([^a-z]+));
        test_t!(ctx, "AEUF", 0, neure!([^a-z]{3}), Span { beg: 0, len: 3 });
        test_t!(ctx, "aefc", 0, neure!([^a-z]{3}));
        test_t!(ctx, "AEUF", 0, neure!([^a-z]{3,6}), Span { beg: 0, len: 4 });
        test_t!(ctx, "aefc", 0, neure!([^a-z]{3,6}));
        test_t!(ctx, "AEUF", 0, neure!([^a-z]{3,}), Span { beg: 0, len: 4 });
        test_t!(ctx, "aefc", 0, neure!([^a-z]{3,}));

        test_t!(
            ctx,
            "@#$%",
            0,
            neure!([^'a' - 'z' 'A' - 'Z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, neure!([^'a'-'z']+), Span { beg: 0, len: 4 });
        test_t!(ctx, "aefc", 0, neure!([^'a'-'z']+));
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!([^'a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "aefc", 0, neure!([^'a'-'z']{3}));
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!([^'a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aefc", 0, neure!([^'a'-'z']{3,6}));
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!([^'a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aefc", 0, neure!([^'a'-'z']{3,}));
        Ok(())
    }

    fn test_range() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "aefc", 0, neure!([a - z]), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            neure!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "aefc", 0, neure!([a - z]?), Span { beg: 0, len: 1 });
        test_t!(ctx, "AEUF", 0, neure!([a - z]?), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "aefc", 0, neure!([a-z]*), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!([a-z]*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!([a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aefc", 0, neure!([a-z]+), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!([a-z]+));
        test_t!(ctx, "aefc", 0, neure!([a-z]{3}), Span { beg: 0, len: 3 });
        test_t!(ctx, "AEUF", 0, neure!([a-z]{3}));
        test_t!(ctx, "aefc", 0, neure!([a-z]{3,6}), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!([a-z]{3,6}));
        test_t!(ctx, "aefc", 0, neure!([a-z]{3,}), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!([a-z]{3,}));

        test_t!(ctx, "aefc", 0, neure!(['a' - 'z']), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            neure!(['a' - 'z']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!(['a' - 'z']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, "aefc", 0, neure!(['a'-'z']*), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!(['a'-'z']*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, neure!(['a'-'z']+), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, neure!(['a'-'z']+));
        test_t!(
            ctx,
            "AEUF",
            0,
            neure!(['a'-'z''A'-'Z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aefc",
            0,
            neure!(['a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "AEUF", 0, neure!(['a'-'z']{3}));
        test_t!(
            ctx,
            "aefc",
            0,
            neure!(['a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, neure!(['a'-'z']{3,6}));
        test_t!(
            ctx,
            "aefc",
            0,
            neure!(['a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, neure!(['a'-'z']{3,}));

        Ok(())
    }

    fn test_chars_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "aefc", 0, neure!([^b c d]), Span { beg: 0, len: 1 });
        test_t!(ctx, "aefc", 0, neure!([^b c d]?), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            neure!([^'b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "aefc",
            0,
            neure!([^'b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "daefc", 0, neure!([^b c d]?), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, neure!([^b c d]*), Span { beg: 0, len: 3 });
        test_t!(ctx, "daefc", 0, neure!([^b c d]*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, neure!([^b c d]+), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            "aefcddd",
            0,
            neure!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "baefcddd", 0, neure!([^b c d]+));
        test_t!(ctx, "aefh", 0, neure!([^b c d]{4}), Span { beg: 0, len: 4 });
        test_t!(
            ctx,
            "aefhcc",
            0,
            neure!([^b c d]{4,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aefhccd",
            0,
            neure!([^b c d]{4,7}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aecfhccd", 0, neure!([^b c d]{4,7}));
        Ok(())
    }

    fn test_chars() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "dabcd", 0, neure!([b c d]), Span { beg: 0, len: 1 });
        test_t!(ctx, "dabcd", 0, neure!([b c d]?), Span { beg: 0, len: 1 });
        test_t!(ctx, "edabcd", 0, neure!([b c d]?), Span { beg: 0, len: 0 });
        test_t!(ctx, "dbcd", 0, neure!([b c d]*), Span { beg: 0, len: 4 });
        test_t!(ctx, "aeuyf", 0, neure!([b c d]*), Span { beg: 0, len: 0 });
        test_t!(ctx, "dabcd", 0, neure!([b c d]+), Span { beg: 0, len: 1 });
        test_t!(ctx, "dbcdfff", 0, neure!([b c d]+), Span { beg: 0, len: 4 });
        test_t!(ctx, "abcd", 0, neure!([b c d]+));
        test_t!(ctx, "dbcd", 0, neure!([b c d]{4}), Span { beg: 0, len: 4 });
        test_t!(
            ctx,
            "dbcdccc",
            0,
            neure!([b c d]{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            "dbcdbb",
            0,
            neure!([b c d]{4,7}),
            Span { beg: 0, len: 6 }
        );

        test_t!(
            ctx,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "edabcd",
            0,
            neure!(['b' 'c' 'd']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            "dbcd",
            0,
            neure!(['b' 'c' 'd']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aeuyf",
            0,
            neure!(['b' 'c' 'd']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            "dabcd",
            0,
            neure!(['b' 'c' 'd']+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "dbcdfff",
            0,
            neure!(['b' 'c' 'd']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "abcd", 0, neure!(['b' 'c' 'd']+));
        test_t!(
            ctx,
            "dbcd",
            0,
            neure!(['b' 'c' 'd']{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "dbcdccc",
            0,
            neure!(['b' 'c' 'd']{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            "dbcdbb",
            0,
            neure!(['b' 'c' 'd']{4,7}),
            Span { beg: 0, len: 6 }
        );
        Ok(())
    }

    fn test_char() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("a", 1);

        test_t!(ctx, "a", 0, neure!(a), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, neure!('a'), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, neure!(a?), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, neure!('a'?), Span { beg: 0, len: 1 });
        test_t!(ctx, "你", 0, neure!(你), Span { beg: 0, len: 3 });
        test_t!(ctx, "你you", 0, neure!('你'), Span { beg: 0, len: 3 });
        test_t!(ctx, "@", 0, neure!('@'), Span { beg: 0, len: 1 });
        test_t!(ctx, "der", 0, neure!(a?), Span { beg: 0, len: 0 });
        test_t!(ctx, "", 0, neure!('a'?), Span { beg: 0, len: 0 });
        test_t!(ctx, "a", 0, neure!(a*), Span { beg: 0, len: 1 });
        test_t!(ctx, "aaaaaee", 0, neure!('a'*), Span { beg: 0, len: 5 });
        test_t!(ctx, "cde", 0, neure!(a*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aaaaee", 0, neure!('a'+), Span { beg: 0, len: 4 });
        test_t!(ctx, "你你你", 0, neure!(你+), Span { beg: 0, len: 9 });
        test_t!(ctx, "我你你你", 0, neure!(你+));
        test_t!(ctx, "aaaaee", 0, neure!('a'{2}), Span { beg: 0, len: 2 });
        test_t!(ctx, "你你你", 0, neure!(你{2}), Span { beg: 0, len: 6 });
        test_t!(ctx, "你", 0, neure!(你{2}));
        test_t!(ctx, "aaaaee", 0, neure!('a'{2,}), Span { beg: 0, len: 4 });
        test_t!(ctx, "你你你", 0, neure!(你{2,}), Span { beg: 0, len: 9 });
        test_t!(ctx, "你", 0, neure!(你{2,}));
        test_t!(ctx, "aaaaee", 0, neure!('a'{2,3}), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            "你你你你你啊",
            0,
            neure!(你{2,4}),
            Span { beg: 0, len: 12 }
        );
        test_t!(ctx, "你啊", 0, neure!(你{2,4}));

        Ok(())
    }

    fn test_space() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = CharsCtx::new("\tcd", 1);

        test_t!(ctx, "\tcd", 0, neure!(), Span { beg: 0, len: 1 });
        test_t!(ctx, "\tdwq", 0, neure!(?), Span { beg: 0, len: 1 });
        test_t!(ctx, "dwq", 0, neure!(?), Span { beg: 0, len: 0 });
        test_t!(ctx, "\t\n\rdda", 0, neure!(*), Span { beg: 0, len: 3 });
        test_t!(ctx, "dda", 0, neure!(*), Span { beg: 0, len: 0 });
        test_t!(ctx, "\t\n\rdda", 0, neure!(+), Span { beg: 0, len: 3 });
        test_t!(ctx, "\tdda", 0, neure!(+), Span { beg: 0, len: 1 });
        test_t!(ctx, "dda", 0, neure!(+));
        test_t!(ctx, " \u{A0}dda", 0, neure!({ 2 }), Span { beg: 0, len: 3 });
        test_t!(ctx, "\u{A0}dda", 0, neure!({ 2 }));
        test_t!(ctx, "\t\rdda", 0, neure!({2,}), Span { beg: 0, len: 2 });
        test_t!(
            ctx,
            "\t\r\u{A0}dda",
            0,
            neure!({2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "dda", 0, neure!());
        test_t!(ctx, "\t\ndda", 0, neure!({2,3}), Span { beg: 0, len: 2 });
        test_t!(
            ctx,
            "\t\r\u{A0}dda",
            0,
            neure!({2,3}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "\t\r \u{A0}dda",
            0,
            neure!({2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, " dda", 0, neure!({2,3}));

        Ok(())
    }
}
