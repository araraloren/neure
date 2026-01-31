#![doc = include_str!("../README.md")]
#![no_std]
pub mod ctor;
pub mod ctx;
pub mod err;
pub mod iter;
pub mod map;
pub mod neu;
pub mod regex;
pub mod span;

pub(crate) mod log;
pub(crate) mod r#macro;
pub(crate) use log::*;

#[cfg(feature = "alloc")]
pub(crate) mod alloc {
    extern crate alloc;

    pub use alloc::borrow::Cow;
    pub use alloc::boxed::Box;
    pub use alloc::rc::Rc;
    pub use alloc::string::String;
    pub use alloc::sync::Arc;
    pub use alloc::vec;
    pub use alloc::vec::Vec;
}

#[cfg(feature = "std")]
pub(crate) mod std {
    extern crate std;

    #[allow(unused)]
    pub use std::collections::BTreeSet;
    #[allow(unused)]
    pub use std::collections::HashSet;
    pub use std::sync::Mutex;
}

#[cfg(any(feature = "log", feature = "tracing"))]
pub trait MayDebug: core::fmt::Debug {}

#[cfg(any(feature = "log", feature = "tracing"))]
impl<T> MayDebug for T where T: core::fmt::Debug {}

#[cfg(not(any(feature = "log", feature = "tracing")))]
pub trait MayDebug {}

#[cfg(not(any(feature = "log", feature = "tracing")))]
impl<T> MayDebug for T {}

pub use charize::charize;

pub mod prelude {
    pub use crate::ctor;
    pub use crate::ctor::CtorIntoHelper;
    pub use crate::ctor::CtorOps;
    pub use crate::ctor::CtorRefAsDynCtor;
    pub use crate::ctx::Assert;
    pub use crate::ctx::BytesCtx;
    pub use crate::ctx::CharsCtx;
    pub use crate::ctx::Context;
    pub use crate::ctx::Match;
    pub use crate::ctx::MatchExt;
    pub use crate::ctx::MatchMulti;
    pub use crate::ctx::RegexCtx;
    pub use crate::map;
    pub use crate::neu;
    pub use crate::neu::Condition;
    pub use crate::neu::Neu;
    pub use crate::neu::NeuCond;
    pub use crate::neu::NeuIntoRegexOps;
    pub use crate::neu::NeuOp;
    pub use crate::regex;
    pub use crate::regex::Regex;
    pub use crate::regex::RegexIntoHelper;
    pub use crate::regex::RegexRefAsCtor;
    pub use crate::regex::RegexRefAsDynRegex;
    pub use crate::span::ArrayStorer;
    pub use crate::span::Span;
    #[cfg(feature = "alloc")]
    pub use crate::span::VecStorer;
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
        #[cfg(feature = "alloc")]
        assert!(test_range_negative().is_ok());
        #[cfg(feature = "alloc")]
        assert!(test_other().is_ok());
    }

    macro_rules! test_t {
        ($ctx:ident, $str:literal, $id:literal, $parser:expr) => {
            let parser = $parser;

            $ctx.reset_with($str);
            assert!($ctx.try_mat(&parser).is_err());
        };
        ($ctx:ident, $str:literal, $id:literal, $parser:expr, $span:expr) => {
            let parser = $parser;

            $ctx.reset_with($str);
            assert_eq!($span, $ctx.try_mat(&parser)?);
        };
    }

    #[cfg(feature = "alloc")]
    macro_rules! test_st {
        ($ctx:ident, $storer:ident, $str:literal, $id:literal, $parser:expr) => {
            let parser = $parser;

            $ctx.reset_with($str);
            $storer.reset();
            assert!($storer.try_cap($id, &mut $ctx, &parser).is_err());
        };
        ($ctx:ident, $storer:ident, $str:literal, $id:literal, $parser:expr, $($span:expr)*) => {
            let parser = $parser;
            let spans = [$($span)*];

            $ctx.reset_with($str);
            $storer.reset();
            $storer.try_cap($id, &mut $ctx, &parser)?;
            for (i, span) in $storer.spans_iter($id).unwrap().enumerate() {
                assert_eq!(span, spans[i]);
            }
        };
    }

    #[cfg(feature = "alloc")]
    fn test_other() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");
        let mut storer = VecStorer::new(1);

        test_st!(ctx, storer, "abedf", 0, regex!(.), Span { beg: 0, len: 1 });
        test_st!(ctx, storer, "abedf", 0, regex!(.?), Span { beg: 0, len: 1 });
        test_st!(
            ctx,
            storer,
            "\nabedf",
            0,
            regex!(.?),
            Span { beg: 0, len: 0 }
        );
        test_st!(ctx, storer, "abedf", 0, regex!(.*), Span { beg: 0, len: 5 });
        test_st!(
            ctx,
            storer,
            "\nabedf",
            0,
            regex!(.*),
            Span { beg: 0, len: 0 }
        );
        test_st!(ctx, storer, "abedf", 0, regex!(.+), Span { beg: 0, len: 5 });
        test_st!(ctx, storer, "\nabedf", 0, regex!(.+));
        test_st!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{2}),
            Span { beg: 0, len: 2 }
        );
        test_st!(ctx, storer, "ab\nedf", 0, regex!(.{3}));
        test_st!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{2,}),
            Span { beg: 0, len: 5 }
        );
        test_st!(
            ctx,
            storer,
            "abedf",
            0,
            regex!(.{4,6}),
            Span { beg: 0, len: 5 }
        );
        test_st!(ctx, storer, "abe\ndf", 0, regex!(.{4,6}));
        test_st!(
            ctx,
            storer,
            "c\nabedf",
            0,
            regex!(^),
            Span { beg: 0, len: 1 }
        );

        Ok(())
    }

    #[cfg(feature = "alloc")]
    fn test_range_negative() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");
        let mut storer = VecStorer::new(1);

        test_st!(
            ctx,
            storer,
            "Raefc",
            0,
            regex!([^a - z]),
            Span { beg: 0, len: 1 }
        );
        test_st!(
            ctx,
            storer,
            "你aefc",
            0,
            regex!([^a - z A - Z]),
            Span { beg: 0, len: 3 }
        );
        test_st!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^a - z]?),
            Span { beg: 0, len: 0 }
        );
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a - z]?),
            Span { beg: 0, len: 1 }
        );
        test_st!(
            ctx,
            storer,
            "&AEUF",
            0,
            regex!([^a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]*),
            Span { beg: 0, len: 4 }
        );
        test_st!(
            ctx,
            storer,
            "aefc",
            0,
            regex!([^a-z]*),
            Span { beg: 0, len: 0 }
        );
        test_st!(
            ctx,
            storer,
            "@#$%",
            0,
            regex!([^a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]+),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^a-z]+));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3}),
            Span { beg: 0, len: 3 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^a-z]{3}));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^a-z]{3,6}));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^a-z]{3,}),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^a-z]{3,}));

        test_st!(
            ctx,
            storer,
            "@#$%",
            0,
            regex!([^'a' - 'z' 'A' - 'Z']*),
            Span { beg: 0, len: 4 }
        );
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']+),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^'a'-'z']+));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3}));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3,6}));
        test_st!(
            ctx,
            storer,
            "AEUF",
            0,
            regex!([^'a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_st!(ctx, storer, "aefc", 0, regex!([^'a'-'z']{3,}));
        Ok(())
    }

    fn test_range() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");

        test_t!(ctx, "aefc", 0, regex!([a - z]), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            regex!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "aefc", 0, regex!([a - z]?), Span { beg: 0, len: 1 });
        test_t!(ctx, "AEUF", 0, regex!([a - z]?), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            "AEUF",
            0,
            regex!([a - z A - Z]),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "aefc", 0, regex!([a-z]*), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!([a-z]*), Span { beg: 0, len: 0 });
        test_t!(
            ctx,
            "AEUF",
            0,
            regex!([a - z A - Z]*),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aefc", 0, regex!([a-z]+), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!([a-z]+));
        test_t!(ctx, "aefc", 0, regex!([a-z]{3}), Span { beg: 0, len: 3 });
        test_t!(ctx, "AEUF", 0, regex!([a-z]{3}));
        test_t!(ctx, "aefc", 0, regex!([a-z]{3,6}), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!([a-z]{3,6}));
        test_t!(ctx, "aefc", 0, regex!([a-z]{3,}), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!([a-z]{3,}));

        test_t!(ctx, "aefc", 0, regex!(['a' - 'z']), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            regex!(['a' - 'z']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "AEUF",
            0,
            regex!(['a' - 'z']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(ctx, "aefc", 0, regex!(['a'-'z']*), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!(['a'-'z']*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, regex!(['a'-'z']+), Span { beg: 0, len: 4 });
        test_t!(ctx, "AEUF", 0, regex!(['a'-'z']+));
        test_t!(
            ctx,
            "AEUF",
            0,
            regex!(['a'-'z''A'-'Z']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aefc",
            0,
            regex!(['a'-'z']{3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "AEUF", 0, regex!(['a'-'z']{3}));
        test_t!(
            ctx,
            "aefc",
            0,
            regex!(['a'-'z']{3,6}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, regex!(['a'-'z']{3,6}));
        test_t!(
            ctx,
            "aefc",
            0,
            regex!(['a'-'z']{3,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "AEUF", 0, regex!(['a'-'z']{3,}));

        Ok(())
    }

    fn test_chars_negative() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");

        test_t!(ctx, "aefc", 0, regex!([^b c d]), Span { beg: 0, len: 1 });
        test_t!(ctx, "aefc", 0, regex!([^b c d]?), Span { beg: 0, len: 1 });
        test_t!(
            ctx,
            "aefc",
            0,
            regex!([^'b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "aefc",
            0,
            regex!([^'b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(ctx, "daefc", 0, regex!([^b c d]?), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, regex!([^b c d]*), Span { beg: 0, len: 3 });
        test_t!(ctx, "daefc", 0, regex!([^b c d]*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aefc", 0, regex!([^b c d]+), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            "aefcddd",
            0,
            regex!([^b c d]+),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, "baefcddd", 0, regex!([^b c d]+));
        test_t!(ctx, "aefh", 0, regex!([^b c d]{4}), Span { beg: 0, len: 4 });
        test_t!(
            ctx,
            "aefhcc",
            0,
            regex!([^b c d]{4,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aefhccd",
            0,
            regex!([^b c d]{4,7}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "aecfhccd", 0, regex!([^b c d]{4,7}));
        Ok(())
    }

    fn test_chars() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");

        test_t!(ctx, "dabcd", 0, regex!([b c d]), Span { beg: 0, len: 1 });
        test_t!(ctx, "dabcd", 0, regex!([b c d]?), Span { beg: 0, len: 1 });
        test_t!(ctx, "edabcd", 0, regex!([b c d]?), Span { beg: 0, len: 0 });
        test_t!(ctx, "dbcd", 0, regex!([b c d]*), Span { beg: 0, len: 4 });
        test_t!(ctx, "aeuyf", 0, regex!([b c d]*), Span { beg: 0, len: 0 });
        test_t!(ctx, "dabcd", 0, regex!([b c d]+), Span { beg: 0, len: 1 });
        test_t!(ctx, "dbcdfff", 0, regex!([b c d]+), Span { beg: 0, len: 4 });
        test_t!(ctx, "abcd", 0, regex!([b c d]+));
        test_t!(ctx, "dbcd", 0, regex!([b c d]{4}), Span { beg: 0, len: 4 });
        test_t!(
            ctx,
            "dbcdccc",
            0,
            regex!([b c d]{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            "dbcdbb",
            0,
            regex!([b c d]{4,7}),
            Span { beg: 0, len: 6 }
        );

        test_t!(
            ctx,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']?),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "edabcd",
            0,
            regex!(['b' 'c' 'd']?),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            "dbcd",
            0,
            regex!(['b' 'c' 'd']*),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "aeuyf",
            0,
            regex!(['b' 'c' 'd']*),
            Span { beg: 0, len: 0 }
        );
        test_t!(
            ctx,
            "dabcd",
            0,
            regex!(['b' 'c' 'd']+),
            Span { beg: 0, len: 1 }
        );
        test_t!(
            ctx,
            "dbcdfff",
            0,
            regex!(['b' 'c' 'd']+),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "abcd", 0, regex!(['b' 'c' 'd']+));
        test_t!(
            ctx,
            "dbcd",
            0,
            regex!(['b' 'c' 'd']{4}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "dbcdccc",
            0,
            regex!(['b' 'c' 'd']{4,}),
            Span { beg: 0, len: 7 }
        );
        test_t!(
            ctx,
            "dbcdbb",
            0,
            regex!(['b' 'c' 'd']{4,7}),
            Span { beg: 0, len: 6 }
        );
        Ok(())
    }

    fn test_char() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");

        test_t!(ctx, "a", 0, regex!(a), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, regex!('a'), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, regex!(a?), Span { beg: 0, len: 1 });
        test_t!(ctx, "a", 0, regex!('a'?), Span { beg: 0, len: 1 });
        test_t!(ctx, "你", 0, regex!(你), Span { beg: 0, len: 3 });
        test_t!(ctx, "你you", 0, regex!('你'), Span { beg: 0, len: 3 });
        test_t!(ctx, "@", 0, regex!('@'), Span { beg: 0, len: 1 });
        test_t!(ctx, "der", 0, regex!(a?), Span { beg: 0, len: 0 });
        test_t!(ctx, "", 0, regex!('a'?), Span { beg: 0, len: 0 });
        test_t!(ctx, "a", 0, regex!(a*), Span { beg: 0, len: 1 });
        test_t!(ctx, "aaaaaee", 0, regex!('a'*), Span { beg: 0, len: 5 });
        test_t!(ctx, "cde", 0, regex!(a*), Span { beg: 0, len: 0 });
        test_t!(ctx, "aaaaee", 0, regex!('a'+), Span { beg: 0, len: 4 });
        test_t!(ctx, "你你你", 0, regex!(你+), Span { beg: 0, len: 9 });
        test_t!(ctx, "我你你你", 0, regex!(你+));
        test_t!(ctx, "aaaaee", 0, regex!('a'{2}), Span { beg: 0, len: 2 });
        test_t!(ctx, "你你你", 0, regex!(你{2}), Span { beg: 0, len: 6 });
        test_t!(ctx, "你", 0, regex!(你{2}));
        test_t!(ctx, "aaaaee", 0, regex!('a'{2,}), Span { beg: 0, len: 4 });
        test_t!(ctx, "你你你", 0, regex!(你{2,}), Span { beg: 0, len: 9 });
        test_t!(ctx, "你", 0, regex!(你{2,}));
        test_t!(ctx, "aaaaee", 0, regex!('a'{2,3}), Span { beg: 0, len: 3 });
        test_t!(
            ctx,
            "你你你你你啊",
            0,
            regex!(你{2,4}),
            Span { beg: 0, len: 12 }
        );
        test_t!(ctx, "你啊", 0, regex!(你{2,4}));

        Ok(())
    }

    fn test_space() -> Result<(), crate::err::Error> {
        let mut ctx = RegexCtx::new("");

        test_t!(ctx, "\tcd", 0, regex!(), Span { beg: 0, len: 1 });
        test_t!(ctx, "\tdwq", 0, regex!(?), Span { beg: 0, len: 1 });
        test_t!(ctx, "dwq", 0, regex!(?), Span { beg: 0, len: 0 });
        test_t!(ctx, "\t\n\rdda", 0, regex!(*), Span { beg: 0, len: 3 });
        test_t!(ctx, "dda", 0, regex!(*), Span { beg: 0, len: 0 });
        test_t!(ctx, "\t\n\rdda", 0, regex!(+), Span { beg: 0, len: 3 });
        test_t!(ctx, "\tdda", 0, regex!(+), Span { beg: 0, len: 1 });
        test_t!(ctx, "dda", 0, regex!(+));
        test_t!(ctx, " \u{A0}dda", 0, regex!({ 2 }), Span { beg: 0, len: 3 });
        test_t!(ctx, "\u{A0}dda", 0, regex!({ 2 }));
        test_t!(ctx, "\t\rdda", 0, regex!({2,}), Span { beg: 0, len: 2 });
        test_t!(
            ctx,
            "\t\r\u{A0}dda",
            0,
            regex!({2,}),
            Span { beg: 0, len: 4 }
        );
        test_t!(ctx, "dda", 0, regex!());
        test_t!(ctx, "\t\ndda", 0, regex!({2,3}), Span { beg: 0, len: 2 });
        test_t!(
            ctx,
            "\t\r\u{A0}dda",
            0,
            regex!({2,3}),
            Span { beg: 0, len: 4 }
        );
        test_t!(
            ctx,
            "\t\r \u{A0}dda",
            0,
            regex!({2,3}),
            Span { beg: 0, len: 3 }
        );
        test_t!(ctx, " dda", 0, regex!({2,3}));

        Ok(())
    }
}
