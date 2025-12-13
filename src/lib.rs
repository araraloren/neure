#![doc = include_str!("../README.md")]
pub mod ctor;
pub mod ctx;
pub mod err;
pub mod iter;
pub mod r#macro;
pub mod map;
pub mod neu;
pub mod regex;
pub mod span;

#[macro_use]
pub(crate) mod log;

pub(crate) use log::*;

#[cfg(feature = "log")]
pub trait MayDebug: std::fmt::Debug {}

#[cfg(feature = "log")]
impl<T> MayDebug for T where T: std::fmt::Debug {}

#[cfg(not(feature = "log"))]
pub trait MayDebug {}

#[cfg(not(feature = "log"))]
impl<T> MayDebug for T {}

pub use charize::charize;

pub mod prelude {
    pub use crate::ctor;
    pub use crate::ctor::ConstructIntoOp;
    pub use crate::ctor::ConstructOp;
    pub use crate::ctx::BytesCtx;
    pub use crate::ctx::CharsCtx;
    pub use crate::ctx::Context;
    pub use crate::ctx::ContextHelper;
    pub use crate::ctx::Match;
    pub use crate::ctx::RegexCtx;
    pub use crate::ctx::Span;
    pub use crate::map;
    pub use crate::neu;
    pub use crate::neu::Condition;
    pub use crate::neu::Neu;
    pub use crate::neu::NeuCond;
    pub use crate::neu::NeuIntoRegexOps;
    pub use crate::neu::NeuOp;
    pub use crate::regex;
    pub use crate::regex::Regex;
    pub use crate::regex::RegexIntoOp;
    pub use crate::span::SimpleStorer;
}

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use crate::regex;

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
            assert!($storer.try_cap($id, &mut $ctx, &space_parser).is_err());
        };
        ($ctx:ident, $storer:ident, $str:literal, $id:literal, $parser:expr, $($span:expr)*) => {
            let space_parser = $parser;

            $ctx.reset_with($str);
            $storer.reset();
            $storer.try_cap($id, &mut $ctx, &space_parser)?;
            assert_eq!($storer.spans_iter($id).unwrap().collect::<Vec<_>>(), vec![$($span)*])
        };
    }

    fn test_other() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(ctx, storer, "abedf", 0, regex!(.), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "abedf", 0, regex!(.?), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            storer,
            "\nabedf",
            0,
            regex!(.?),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, storer, "abedf", 0, regex!(.*), Span { beg: 0, len: 5 });
        test_t!(
            ctx,
            storer,
            "\nabedf",
            0,
            regex!(.*),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, storer, "abedf", 0, regex!(.+), Span { beg: 0, len: 5 });
        test_t!(ctx, storer, "\nabedf", 0, regex!(.+));
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{2}),
            Span { beg: 0, len: 2 }
        );
        test_t!(ctx, storer, "ab\nedf", 0, regex!(.{3}));
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{2,}),
            Span { beg: 0, len: 5 }
        );
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{4,6}),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "abe\ndf", 0, regex!(.{4,6}));
        test_t!(
            ctx,
            storer,
            "c\nabedf",
            0,
            regex!(^),
            Span { beg: 0, len: 1 }
        );

        Ok(())
    }

    fn test_range_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(
            ctx,
            storer,
            "Raefc",
            0,
            regex!([^a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "你aefc",
            0,
            regex!([^a - z A - Z]),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "&AEUF",
            0,
            regex!([^a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            regex!([^a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^a-z]+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^a-z]{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            regex!([^'a' - 'z' 'A' - 'Z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^'a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3,}));
        Ok(())
    }

    fn test_range() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!([a-z]+));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!([a-z]{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!([a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!([a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a' - 'z']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a' - 'z']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!(['a' - 'z']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a'-'z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!(['a'-'z']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!(['a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!(['a'-'z''A'-'Z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!(['a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!(['a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!(['a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, regex!(['a'-'z']{3,}));

        Ok(())
    }

    fn test_chars_negative() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^'b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^'b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            regex!([^b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^b c d]*),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            regex!([^b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefcddd",
            0,
            regex!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "baefcddd", 0, regex!([^b c d]+));
        test_t!(
            ctx,
            storer,
            "aefh",
            0,
            regex!([^b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhcc",
            0,
            regex!([^b c d]{4,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhccd",
            0,
            regex!([^b c d]{4,7}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aecfhccd", 0, regex!([^b c d]{4,7}));
        Ok(())
    }

    fn test_chars() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!([b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!([b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            regex!([b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            regex!([b c d]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            regex!([b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!([b c d]+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            regex!([b c d]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, regex!([b c d]+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            regex!([b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            regex!([b c d]{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            regex!([b c d]{4,7}),
            Span { beg: 0, len: 6 }
        );

        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            regex!(['b' 'c' 'd']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            regex!(['b' 'c' 'd']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            regex!(['b' 'c' 'd']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            regex!(['b' 'c' 'd']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, regex!(['b' 'c' 'd']+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            regex!(['b' 'c' 'd']{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            regex!(['b' 'c' 'd']{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            regex!(['b' 'c' 'd']{4,7}),
            Span { beg: 0, len: 6 }
        );
        Ok(())
    }

    fn test_char() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(ctx, storer, "a", 0, regex!(a), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, regex!('a'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, regex!(a?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, regex!('a'?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "你", 0, regex!(你), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            storer,
            "你you",
            0,
            regex!('你'),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "@", 0, regex!('@'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "der", 0, regex!(a?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "", 0, regex!('a'?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "a", 0, regex!(a*), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            storer,
            "aaaaaee",
            0,
            regex!('a'*),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "cde", 0, regex!(a*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            regex!('a'+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            regex!(你+),
            Span { beg: 0, len: 9 }
        );
        test_t!(ctx, storer, "我你你你", 0, regex!(你+));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            regex!('a'{2}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            regex!(你{2}),
            Span { beg: 0, len: 6 }
        );
        test_t!(ctx, storer, "你", 0, regex!(你{2}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            regex!('a'{2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            regex!(你{2,}),
            Span { beg: 0, len: 9 }
        );
        test_t!(ctx, storer, "你", 0, regex!(你{2,}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            regex!('a'{2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "你你你你你啊",
            0,
            regex!(你{2,4}),
            Span { beg: 0, len: 12 }
        );
        test_t!(ctx, storer, "你啊", 0, regex!(你{2,4}));

        Ok(())
    }

    fn test_space() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(ctx, storer, "\tcd", 0, regex!(), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "\tdwq", 0, regex!(?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dwq", 0, regex!(?), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "\t\n\rdda",
            0,
            regex!(*),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "dda", 0, regex!(*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "\t\n\rdda",
            0,
            regex!(+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "\tdda", 0, regex!(+), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dda", 0, regex!(+));
        test_t!(
            ctx,
            storer,
            " \u{A0}dda",
            0,
            regex!({ 2 }),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "\u{A0}dda", 0, regex!({ 2 }));
        test_t!(
            ctx,
            storer,
            "\t\rdda",
            0,
            regex!({2,}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            regex!({2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "dda", 0, regex!());
        test_t!(
            ctx,
            storer,
            "\t\ndda",
            0,
            regex!({2,3}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            regex!({2,3}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r \u{A0}dda",
            0,
            regex!({2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, " dda", 0, regex!({2,3}));

        Ok(())
    }
}
