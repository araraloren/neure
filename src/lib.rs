#![doc = include_str!("../README.md")]
pub mod ctx;
pub mod err;
pub mod iter;
pub mod r#macro;
pub mod neu;
pub mod re;
pub mod span;

#[cfg(feature = "log")]
pub(crate) use tracing::trace as trace_log;
#[cfg(not(feature = "log"))]
#[macro_use]
pub(crate) mod log {
    #[macro_export]
    macro_rules! trace_log {
        ($($arg:tt)*) => {
            ();
        };
    }
}

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
    pub use crate::ctx::BytesCtx;
    pub use crate::ctx::CharsCtx;
    pub use crate::ctx::Context;
    pub use crate::ctx::Policy;
    pub use crate::ctx::RegexCtx;
    pub use crate::ctx::Ret;
    pub use crate::ctx::Span;
    pub use crate::neu;
    pub use crate::neu::Neu;
    pub use crate::neu::Neu2Re;
    pub use crate::neu::NeuCond;
    pub use crate::neu::NeuOp;
    pub use crate::re;
    pub use crate::re::ConstructOp;
    pub use crate::re::Regex;
    pub use crate::re::RegexIntoOp;
    pub use crate::span::SimpleStorer;
}

#[cfg(test)]
mod test {

    use crate::prelude::*;
    use crate::re;

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

        test_t!(ctx, storer, "abedf", 0, re!(.), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "abedf", 0, re!(.?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "\nabedf", 0, re!(.?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "abedf", 0, re!(.*), Span { beg: 0, len: 5 });
        test_t!(ctx, storer, "\nabedf", 0, re!(.*), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "abedf", 0, re!(.+), Span { beg: 0, len: 5 });
        test_t!(ctx, storer, "\nabedf", 0, re!(.+));
        test_t!(ctx, storer, "abedf", 0, re!(.{2}), Span { beg: 0, len: 2 });
        test_t!(ctx, storer, "ab\nedf", 0, re!(.{3}));
        test_t!(ctx, storer, "abedf", 0, re!(.{2,}), Span { beg: 0, len: 5 });
        test_t!(
            ctx,
            storer,
            "abedf",
            0,
            re!(.{4,6}),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "abe\ndf", 0, re!(.{4,6}));
        test_t!(ctx, storer, "c\nabedf", 0, re!(^), Span { beg: 0, len: 1 });

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
            re!([^a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "你aefc",
            0,
            re!([^a - z A - Z]),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "&AEUF",
            0,
            re!([^a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            re!([^a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^a-z]+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^a-z]{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "@#$%",
            0,
            re!([^'a' - 'z' 'A' - 'Z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^'a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^'a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^'a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^'a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^'a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^'a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([^'a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([^'a'-'z']{3,}));
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
            re!([a - z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([a-z]*), Span { beg: 0, len: 4 });
        test_t!(ctx, storer, "AEUF", 0, re!([a-z]*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!([a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aefc", 0, re!([a-z]+), Span { beg: 0, len: 4 });
        test_t!(ctx, storer, "AEUF", 0, re!([a-z]+));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!([a-z]{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!([a-z]{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!([a-z]{3,}));

        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a' - 'z']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a' - 'z']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!(['a' - 'z']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a'-'z']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!(['a'-'z']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!(['a'-'z']+));
        test_t!(
            ctx,
            storer,
            "AEUF",
            0,
            re!(['a'-'z''A'-'Z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!(['a'-'z']{3}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!(['a'-'z']{3,6}));
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!(['a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "AEUF", 0, re!(['a'-'z']{3,}));

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
            re!([^b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^'b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^'b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            re!([^b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^b c d]*),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "daefc",
            0,
            re!([^b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "aefc",
            0,
            re!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "aefcddd",
            0,
            re!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "baefcddd", 0, re!([^b c d]+));
        test_t!(
            ctx,
            storer,
            "aefh",
            0,
            re!([^b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhcc",
            0,
            re!([^b c d]{4,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aefhccd",
            0,
            re!([^b c d]{4,7}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "aecfhccd", 0, re!([^b c d]{4,7}));
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
            re!([b c d]),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            re!([b c d]?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            re!([b c d]?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            re!([b c d]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            re!([b c d]*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            re!([b c d]+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            re!([b c d]+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, re!([b c d]+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            re!([b c d]{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            re!([b c d]{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            re!([b c d]{4,7}),
            Span { beg: 0, len: 6 }
        );

        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            re!(['b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            re!(['b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "edabcd",
            0,
            re!(['b' 'c' 'd']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            re!(['b' 'c' 'd']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "aeuyf",
            0,
            re!(['b' 'c' 'd']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            storer,
            "dabcd",
            0,
            re!(['b' 'c' 'd']+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdfff",
            0,
            re!(['b' 'c' 'd']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "abcd", 0, re!(['b' 'c' 'd']+));
        test_t!(
            ctx,
            storer,
            "dbcd",
            0,
            re!(['b' 'c' 'd']{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdccc",
            0,
            re!(['b' 'c' 'd']{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            storer,
            "dbcdbb",
            0,
            re!(['b' 'c' 'd']{4,7}),
            Span { beg: 0, len: 6 }
        );
        Ok(())
    }

    fn test_char() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(ctx, storer, "a", 0, re!(a), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, re!('a'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, re!(a?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "a", 0, re!('a'?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "你", 0, re!(你), Span { beg: 0, len: 3 });
        test_t!(ctx, storer, "你you", 0, re!('你'), Span { beg: 0, len: 3 });
        test_t!(ctx, storer, "@", 0, re!('@'), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "der", 0, re!(a?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "", 0, re!('a'?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "a", 0, re!(a*), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            storer,
            "aaaaaee",
            0,
            re!('a'*),
            Span { beg: 0, len: 5 }
        );
        test_t!(ctx, storer, "cde", 0, re!(a*), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "aaaaee", 0, re!('a'+), Span { beg: 0, len: 4 });
        test_t!(ctx, storer, "你你你", 0, re!(你+), Span { beg: 0, len: 9 });
        test_t!(ctx, storer, "我你你你", 0, re!(你+));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            re!('a'{2}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            re!(你{2}),
            Span { beg: 0, len: 6 }
        );
        test_t!(ctx, storer, "你", 0, re!(你{2}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            re!('a'{2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "你你你",
            0,
            re!(你{2,}),
            Span { beg: 0, len: 9 }
        );
        test_t!(ctx, storer, "你", 0, re!(你{2,}));
        test_t!(
            ctx,
            storer,
            "aaaaee",
            0,
            re!('a'{2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(
            ctx,
            storer,
            "你你你你你啊",
            0,
            re!(你{2,4}),
            Span { beg: 0, len: 12 }
        );
        test_t!(ctx, storer, "你啊", 0, re!(你{2,4}));

        Ok(())
    }

    fn test_space() -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = RegexCtx::new("");
        let mut storer = SimpleStorer::new(1);

        test_t!(ctx, storer, "\tcd", 0, re!(), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "\tdwq", 0, re!(?), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dwq", 0, re!(?), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "\t\n\rdda", 0, re!(*), Span { beg: 0, len: 3 });
        test_t!(ctx, storer, "dda", 0, re!(*), Span { beg: 0, len: 0 });
        test_t!(ctx, storer, "\t\n\rdda", 0, re!(+), Span { beg: 0, len: 3 });
        test_t!(ctx, storer, "\tdda", 0, re!(+), Span { beg: 0, len: 1 });
        test_t!(ctx, storer, "dda", 0, re!(+));
        test_t!(
            ctx,
            storer,
            " \u{A0}dda",
            0,
            re!({ 2 }),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, "\u{A0}dda", 0, re!({ 2 }));
        test_t!(
            ctx,
            storer,
            "\t\rdda",
            0,
            re!({2,}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            re!({2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, storer, "dda", 0, re!());
        test_t!(
            ctx,
            storer,
            "\t\ndda",
            0,
            re!({2,3}),
            Span { beg: 0, len: 2 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r\u{A0}dda",
            0,
            re!({2,3}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            storer,
            "\t\r \u{A0}dda",
            0,
            re!({2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, storer, " dda", 0, re!({2,3}));

        Ok(())
    }
}
