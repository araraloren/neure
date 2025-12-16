///
/// Construct [`Regex`](crate::regex::Regex) element
///
/// # Match whitespace
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ws1 = regex!();
///     let ws2 = regex!(+);
///     let ws3 = regex!(*);
///
///     assert_eq!(CharsCtx::new("  !").ctor(&ws1)?, " ");
///     assert_eq!(CharsCtx::new("  !").ctor(&ws2)?, "  ");
///     assert_eq!(CharsCtx::new("!").ctor(&ws3)?, "");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Match single character
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ch1 = regex!(a?);
///     let ch2 = regex!('a'*);
///     let ch3 = regex!(a+);
///
///     assert_eq!(CharsCtx::new("assert!").ctor(&ch1)?, "a");
///     assert_eq!(CharsCtx::new("ssert!").ctor(&ch2)?, "");
///     assert_eq!(CharsCtx::new("aaaassert!").ctor(&ch3)?, "aaaa");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Match wild character(except '\n')
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let wild1 = regex!(.+);
///     let wild2 = regex!(^*);
///
///     assert_eq!(CharsCtx::new("hello world!").ctor(&wild1)?, "hello world!");
///     assert_eq!(CharsCtx::new("hello world!").ctor(&wild2)?, "hello");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Match character in the set
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let set1 = regex!([h e l o]);
///     let set2 = regex!([h e l o]+);
///
///     assert_eq!(CharsCtx::new("hello world!").ctor(&set1)?, "h");
///     assert_eq!(CharsCtx::new("hello world!").ctor(&set2)?, "hello");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Match character in the range
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let set1 = regex!([a - z]*);
///     let set2 = regex!([^ a - g]+);
///
///     assert_eq!(CharsCtx::new("hello world!").ctor(&set1)?, "hello");
///     assert_eq!(CharsCtx::new("hello world!").ctor(&set2)?, "h");
///
/// #   Ok(())
/// # }
/// ```
///
/// # Match multiple units
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let lower = 'a'..='z';
///     let regex = regex!((lower, 'A' ..= 'W', ' ')+);
///
///     assert_eq!(CharsCtx::new("hello World!").ctor(&regex)?, "hello World");
///
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! regex {
    (@q * $($res:tt)*) => {
        $crate::regex::many0($($res)*)
    };
    (@q ? $($res:tt)*) => {
        $crate::regex::opt($($res)*)
    };
    (@q + $($res:tt)*) => {
        $crate::regex::many1($($res)*)
    };
    (@q {$st:literal} $($res:tt)*) => {
        $crate::regex::count::<$st, $st, _, _>($($res)*)
    };
    (@q {$st:literal,} $($res:tt)*) => {
        $crate::regex::count::<$st, {usize::MAX}, _, _>($($res)*)
    };
    (@q {$st:literal, $ed:literal} $($res:tt)*) => {
        $crate::regex::count::<$st, $ed, _, _>($($res)*)
    };
    (@q $($res:tt)*) => {
        $crate::regex::once($($res)*)
    };

    (@r ^ $($res:tt)*) => { // \S
        regex!(@q $($res)* $crate::neu!(^))
    };
    (@r . $($res:tt)*) => { // .
        regex!(@q $($res)* $crate::neu!(.))
    };
    (@r [ $($range:tt)+ ] $($res:tt)*) => {
        regex!(@q $($res)* $crate::neu!([$($range)+]))
    };
    (@r $ch:ident $($res:tt)*) => {
        regex!(@q $($res)* $crate::neu::equal($crate::charize!($ch)))
    };
    (@r $ch:literal $($res:tt)*) => {
        regex!(@q $($res)* $crate::neu::equal($ch))
    };
    (@r ($($regex:expr),+) $($res:tt)*) => {
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($regex);
            )+
            regex!(@q $($res)* re)
        }
    };
    (@r $($res:tt)*) => {
        regex!(@q $($res)* $crate::neu::whitespace())
    };
    ($($res:tt)*) => {
        regex!(@r $($res)*)
    };
}

#[macro_export]
macro_rules! neu {
    (^) => { // \S
        $crate::neu::whitespace().not()
    };
    (.) => { // .
        $crate::neu::wild()
    };
    ([^$l:literal - $r:literal] ) => {
        $crate::neu::range($l..=$r).not()
    };
    ([^$l:ident - $r:ident] ) => {
        $crate::neu::range($crate::charize!($l)..(char::from_u32($crate::charize!($r) as u32 + 1).unwrap())).not()
    };
    ([$l:literal - $r:literal] ) => {
        $crate::neu::range($l..=$r)
    };
    ([$l:ident - $r:ident] ) => {
        $crate::neu::range($crate::charize!($l)..(char::from_u32($crate::charize!($r) as u32 + 1).unwrap()))
    };

    ([^$($l:literal - $r:literal)+] ) => {// [ ^ 'a'-'z' 'A'-'Z' ]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::range($l..=$r));
            )+
            re.not()
        }
    };
    ([^$($l:ident - $r:ident)+] ) => { // [ ^ a-z A-Z ]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re.not()
        }
    };
    ([$($l:literal - $r:literal)+] ) => { // [ 'a'-'Z' 'A'-'Z' ]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::range($l..=$r));
            )+
            re
        }
    };
    ([$($l:ident - $r:ident)+] ) => { // [ a-Z A-Z ]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::range($crate::charize!($l)..=$crate::charize!($r)));
            )+
            re
        }
    };


    ([ ^ $($ch:literal)+ ] ) => { // [ ^ 'a' 'b' 'c']
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::equal($ch));
            )+
            re.not()
        }
    };
    ([ ^ $($ch:ident)+ ] ) => { // [^ a b c]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::equal($crate::charize!($ch)));
            )+
            re.not()
        }
    };
    ([ $($ch:literal)+ ] ) => { // ['a' 'b' 'c']
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::equal($ch));
            )+
            re
        }
    };
    ([ $($ch:ident)+ ] ) => { // [a b c]
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($crate::neu::equal($crate::charize!($ch)));
            )+
            re
        }
    };
    (($($regex:expr),+)) => {
        {
            let re = $crate::neu::never();
            $(
                let re = re.or($regex);
            )+
            re
        }
    };

    ($ch:ident ) => {
        $crate::neu::equal($crate::charize!($ch))
    };
    ($ch:literal ) => {
        $crate::neu::equal($ch)
    };
    () => {
        $crate::neu::space()
    };
}

///
/// Construct a string value regex pattern.
///
/// ```
/// #
/// # use neure::escape_strval;
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let mut ctx = BytesCtx::new(b"\"abc\t\"");
///     let pat = escape_strval!(b'\"', b'\\', [b'\\', b'\"', b'\t']);
///     let pat = pat.enclose(b"\"", b"\"");
///
///     assert_eq!(ctx.ctor(&pat)?, b"abc\t");
///     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! escape_strval {
    ($quote:literal, $prefix:literal, $escape:expr) => {{
        let escape = $escape;
        let escape = $prefix.then(escape);
        let cond = $crate::neu::regex_cond($crate::regex::not(escape.clone()));

        $quote
            .not()
            .many1()
            .set_cond(cond)
            .or(escape)
            .repeat(0..)
            .pat()
    }};
}
