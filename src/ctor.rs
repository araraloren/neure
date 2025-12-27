mod adapter;
mod affix;
mod array;
mod branch;
mod collect;
mod dthen;
mod enclose;
mod extract;
mod into;
mod longest;
mod map;
mod opt;
mod or;
mod pat;
mod repeat;
mod sep;
mod slice;
mod then;
mod vec;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::adapter::Adapter;
pub use self::affix::Prefix;
pub use self::affix::Suffix;
pub use self::array::Array;
pub use self::array::PairArray;
pub use self::branch::Branch;
pub use self::branch::branch;
pub use self::collect::Collect;
pub use self::dthen::DynamicCtorThenBuilder;
pub use self::dthen::DynamicCtorThenBuilderHelper;
pub use self::enclose::Enclose;
pub use self::extract::Extract;
pub use self::extract::extract;
pub use self::into::CtorIntoHelper;
pub use self::longest::LongestTokenMatch;
pub use self::map::Map;
pub use self::opt::OptionPat;
pub use self::or::Or;
pub use self::pat::Pattern;
pub use self::repeat::Repeat;
pub use self::sep::SepCollect;
pub use self::sep::SepOnce;
pub use self::sep::Separate;
pub use self::slice::PairSlice;
pub use self::slice::Slice;
pub use self::then::IfThen;
pub use self::then::Then;
pub use self::vec::PairVector;
pub use self::vec::Vector;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::err::Error;
use crate::map::FuncMapper;
use crate::map::mapper;
use crate::neu::AsciiWhiteSpace;
use crate::neu::CRange;
use crate::neu::EmptyCond;
use crate::neu::Many0;
use crate::regex::AsCtor;
use crate::regex::Regex;
use crate::span::Span;

pub trait Ctor<'a, C, O, H>: Regex<C> {
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>;
}

impl<'a, C, O, H, F> Ctor<'a, C, O, H> for F
where
    C: Match<'a>,
    H: Handler<C, Out = O>,
    F: Fn(&mut C) -> Result<Span, Error>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.as_ctor().construct(ctx, handler)
    }
}

pub trait Handler<C> {
    type Out;
    type Error: Into<Error>;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error>;
}

impl<Func, Out, C> Handler<C> for Func
where
    Func: FnMut(&C, &Span) -> Result<Out, Error>,
{
    type Out = Out;
    type Error = Error;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        (self)(ctx, span)
    }
}

macro_rules! impl_as_ctor {
    ($self:ident, $regex:expr, $type:ty) => {
        impl<'a, 'b, C, O, H> Ctor<'a, C, O, H> for $type
        where
            C: Match<'a>,
            H: Handler<C, Out = O>,
        {
            fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
                let $self = self;

                Ctor::construct(&$regex.as_ctor(), ctx, handler)
            }
        }
    };
}

impl_as_ctor!(self_, self_.as_ref(), Box<dyn Regex<C> + 'b>);

impl_as_ctor!(self_, self_.as_ref(), Box<dyn Regex<C> + Send + 'b>);

impl_as_ctor!(self_, self_.as_ref(), Box<dyn Regex<C> + Send + Sync + 'b>);

impl_as_ctor!(self_, self_.as_ref(), Rc<dyn Regex<C> + 'b>);

impl_as_ctor!(self_, self_.as_ref(), Rc<dyn Regex<C> + Send + 'b>);

macro_rules! impl_orig_ctor {
    ($type:ty, $orig:ty) => {
        impl<'a, 'b, C, O, H> Ctor<'a, C, O, H> for $type
        where
            C: Context<'a, Orig<'a> = &'a $orig> + Match<'a>,
            H: Handler<C, Out = O>,
        {
            fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
                Ctor::construct(&self.as_ctor(), ctx, handler)
            }
        }
    };
}

impl_orig_ctor!(&'_ str, str);

impl_orig_ctor!(String, str);

impl_orig_ctor!(&'_ String, str);

impl_orig_ctor!(&'_ [u8], [u8]);

impl_orig_ctor!(Vec<u8>, [u8]);

impl_orig_ctor!(&'_ Vec<u8>, [u8]);

impl<'a, const N: usize, C, O, H> Ctor<'a, C, O, H> for &[u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<'a>,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.as_ctor().construct(ctx, handler)
    }
}

impl<'a, const N: usize, C, O, H> Ctor<'a, C, O, H> for [u8; N]
where
    C: Context<'a, Orig<'a> = &'a [u8]> + Match<'a>,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        self.as_ctor().construct(ctx, handler)
    }
}

macro_rules! impl_forward_ctor {
    ($self:ident, $regex:expr, $type:ty) => {
        impl<'a, 'b, C, O, I, H> Ctor<'a, C, O, H> for $type
        where
            I: Ctor<'a, C, O, H>,
        {
            fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
                let $self = self;

                Ctor::construct($regex, ctx, handler)
            }
        }
    };
}

impl_forward_ctor!(self_, self_.as_ref().ok_or(Error::Option)?, Option<I>);

impl_forward_ctor!(self_, &*self_.borrow(), RefCell<I>);

impl_forward_ctor!(self_, &*self_.lock().map_err(|_| Error::Mutex)?, Mutex<I>);

impl_forward_ctor!(self_, self_.as_ref(), Arc<I>);

impl_forward_ctor!(self_, self_.as_ref(), Rc<I>);

impl<'a, C, O, I, H> Ctor<'a, C, O, H> for Cell<I>
where
    I: Ctor<'a, C, O, H> + Copy,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(&self.get(), ctx, handler)
    }
}

macro_rules! impl_dyn_ctor {
    ( $type:ty) => {
        impl<'a, 'b, C, O, H> Ctor<'a, C, O, H> for $type {
            fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
                Ctor::construct(self.as_ref(), ctx, handler)
            }
        }
    };
}

impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + 'b>);

impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + Send + 'b>);

impl_dyn_ctor!(Box<dyn Ctor<'a, C, O, H> + Send + Sync + 'b>);

impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + 'b>);

impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + Send + 'b>);

impl_dyn_ctor!(Arc<dyn Ctor<'a, C, O, H> + Send + Sync + 'b>);

impl_dyn_ctor!(Rc<dyn Ctor<'a, C, O, H> + 'b>);

impl_dyn_ctor!(Rc<dyn Ctor<'a, C, O, H> + Send + 'b>);

pub trait CtorOps<'a, C>: Sized
where
    C: Context<'a>,
{
    fn try_map<F, O>(self, f: F) -> Map<C, Self, F, O>;

    fn map<F, O>(self, f: F) -> Map<C, Self, FuncMapper<F>, O> {
        self.try_map(mapper(f))
    }

    fn pat(self) -> Pattern<C, Self>;

    fn opt(self) -> OptionPat<C, Self>;

    fn enclose<L, R>(self, open: L, close: R) -> Enclose<C, Self, L, R>;

    fn sep<S>(self, sep: S) -> Separate<C, Self, S>;

    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R>;

    fn sep_collect<S, O, V>(self, sep: S) -> SepCollect<C, Self, S, O, V>;

    fn or<P>(self, pat: P) -> Or<C, Self, P>;

    fn longest<P>(self, pat: P) -> LongestTokenMatch<C, Self, P>;

    fn then<T>(self, then: T) -> Then<C, Self, T>;

    fn if_then<I, T>(self, test: I, then: T) -> IfThen<C, Self, I, T>;

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self>;

    fn collect<O, T>(self) -> Collect<C, Self, O, T>;

    fn if_else<I, E>(self, test: I, other: E) -> Branch<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>;

    fn suffix<T>(self, suffix: T) -> Suffix<C, Self, T>;

    fn prefix<T>(self, prefix: T) -> Prefix<C, Self, T>;

    #[allow(clippy::type_complexity)]
    fn skip_ws(self) -> Suffix<C, Self, Many0<C, AsciiWhiteSpace<char>, C::Item>>
    where
        C: Context<'a, Item = char>;

    #[allow(clippy::type_complexity)]
    fn skip_ascii_ws(self) -> Suffix<C, Self, Many0<C, AsciiWhiteSpace<u8>, C::Item>>
    where
        C: Context<'a, Item = u8>;
}

impl<'a, C, T: Regex<C>> CtorOps<'a, C> for T
where
    C: Context<'a>,
{
    fn try_map<F, O>(self, func: F) -> Map<C, Self, F, O> {
        Map::new(self, func)
    }

    ///
    /// Adapts a [`Regex`]-only pattern to be usable in [`Ctor`] contexts by providing span-to-value conversion.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let digit = regex!(['0' - '9']+);
    ///     let digit = digit.map(|v: &str| v.parse::<i64>().unwrap());
    ///     let digits = digit.sep(",".skip_ws());
    ///     let array = digits.enclose("[", "]");
    ///     let mut ctx = CharsCtx::new("[2, 4, 8, 16, 42]");
    ///
    ///     assert_eq!(ctx.ctor(&array)?, vec![2, 4, 8, 16, 42]);
    ///     assert_eq!(ctx.reset().ctor(&array.pat())?, "[2, 4, 8, 16, 42]");
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn pat(self) -> Pattern<C, Self> {
        Pattern::new(self)
    }

    /// Makes a pattern optional, returning `None` (for [`Ctor`]) or an empty span
    /// (for [`Regex`]) when the pattern fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let num = neu::digit(10)
    ///         .many1()
    ///         .try_map(map::from_str::<usize>())
    ///         .opt();
    ///
    ///     assert_eq!(CharsCtx::new("foo").ctor(&num)?, None);
    ///     assert_eq!(CharsCtx::new("955").ctor(&num)?, Some(955));
    ///     Ok(())
    /// # }
    /// ```
    fn opt(self) -> OptionPat<C, Self> {
        OptionPat::new(self)
    }

    ///
    /// Wraps a pattern between opening and closing delimiters, forming a boundary-enclosed structure.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let ascii = neu::ascii().once();
    ///     let lit = ascii.enclose("'", "'");
    ///     let ele = lit.sep(",".skip_ws());
    ///     let arr = ele.enclose("[", "]");
    ///     let mut ctx = CharsCtx::new("['a', 'c', 'd', 'f']");
    ///
    ///     assert_eq!(ctx.ctor(&arr)?, ["a", "c", "d", "f"]);
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn enclose<L, R>(self, open: L, close: R) -> Enclose<C, Self, L, R> {
        Enclose::new(self, open, close)
    }

    ///
    /// Matches a pattern repeated with separators, supporting trailing separators and minimum counts.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let name = regex!([^ ',' ']' '[']+);
    ///     let sep = ','.once().skip_ws();
    ///     let arr = name.sep(sep);
    ///     let arr = arr.enclose("[", "]");
    ///     let mut ctx = CharsCtx::new(r#"[c, rust, java, c++]"#);
    ///
    ///     assert_eq!(ctx.ctor(&arr)?, vec!["c", "rust", "java", "c++"]);
    ///     Ok(())
    /// # }
    /// ```
    fn sep<S>(self, sep: S) -> Separate<C, Self, S> {
        Separate::new(self, sep)
    }

    ///
    /// Splits input into two parts at a single separator pattern, discarding the separator value.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let key = neu::alphabetic().many1().skip_ws();
    ///     let val = neu::whitespace().or(',').not().many1().skip_ws();
    ///     let sep = "=>".skip_ws();
    ///     let ele = key.sep_once(sep, val);
    ///     let hash = ele.sep(",".skip_ws()).enclose("{".skip_ws(), "}");
    ///     let mut ctx = CharsCtx::new(
    ///         r#"{
    ///         c => c11,
    ///         cpp => c++23,
    ///         rust => 2021,
    ///     }"#,
    ///     );
    ///
    ///     assert_eq!(
    ///         ctx.ctor(&hash)?,
    ///         [("c", "c11"), ("cpp", "c++23"), ("rust", "2021")]
    ///     );
    ///     Ok(())
    /// # }
    /// ```
    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R> {
        SepOnce::new(self, sep, right)
    }

    ///
    /// Collects separator-delimited patterns into any [`FromIterator`] collection type.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let key = neu::alphabetic().many1().skip_ws();
    ///     let val = neu::whitespace().or(',').not().many1().skip_ws();
    ///     let sep = "=>".skip_ws();
    ///     let ele = key.sep_once(sep, val);
    ///     let hash = ele.sep_collect(",".skip_ws()).enclose("{".skip_ws(), "}");
    ///     let mut ctx = CharsCtx::new(
    ///         r#"{
    ///         c => c11,
    ///         cpp => c++23,
    ///         rust => 2021,
    ///     }"#,
    ///     );
    ///
    ///     let hash: HashMap<&str, &str> = ctx.ctor(&hash)?;
    ///
    ///     assert_eq!(hash.get("c"), Some(&"c11"));
    ///     assert_eq!(hash.get("cpp"), Some(&"c++23"));
    ///     assert_eq!(hash.get("rust"), Some(&"2021"));
    ///     Ok(())
    /// # }
    /// ```
    fn sep_collect<S, O, V>(self, sep: S) -> SepCollect<C, Self, S, O, V> {
        SepCollect::new(self, sep)
    }

    ///
    /// Provides alternation between two patterns, trying the second only if the first fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     #[derive(Debug, PartialEq, Eq)]
    ///     pub enum V<'a> {
    ///         S(&'a str),
    ///     }
    ///
    ///     let cond = neu::regex_cond(regex::not("\\\""));
    ///     let str = regex!([^ '"' ]+).set_cond(cond).or("\\\"").repeat(1..).pat();
    ///     let str = str.enclose("\"", "\"");
    ///     let str = str.map(V::S);
    ///     let vals = str.sep(",".skip_ws());
    ///     let text = r#""lily\"", "lilei", "lucy""#;
    ///     let mut ctx = CharsCtx::new(text);
    ///
    ///     assert_eq!(
    ///         ctx.ctor(&vals)?,
    ///         [V::S("lily\\\""), V::S("lilei"), V::S("lucy")]
    ///     );
    ///     Ok(())
    /// # }
    /// ```
    fn or<P>(self, pat: P) -> Or<C, Self, P> {
        Or::new(self, pat)
    }

    ///
    /// Selects the longest matching pattern between two alternatives.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     #[derive(Debug, PartialEq, Eq)]
    ///     pub struct Val<'a>(&'a str);
    ///
    ///     let val = "v".longest("val".longest("value"));
    ///     let val = val.map(Val);
    ///     let val = val.sep(",".skip_ws());
    ///     let val = val.enclose("{", "}");
    ///     let mut ctx = CharsCtx::new(r#"{val, v, value}"#);
    ///
    ///     assert_eq!(ctx.ctor(&val)?, [Val("val"), Val("v"), Val("value")]);
    ///     Ok(())
    /// # }
    /// ```
    fn longest<P>(self, pat: P) -> LongestTokenMatch<C, Self, P> {
        LongestTokenMatch::new(self, pat)
    }

    ///
    /// Sequentially composes two expressions, requiring **both** to match in order.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let ws = neu::whitespace().many0();
    ///     let id = neu::ascii_alphabetic().many1();
    ///     let st = "struct".skip_ws().then(id)._1();
    ///     let en = "enum".skip_ws().then(id)._1();
    ///     let ty = st.or(en);
    ///     let ty = ty.skip_ws().then(ws.enclose("{", "}"))._0();
    ///     let mut ctx = CharsCtx::new(r#"struct widget { }"#);
    ///
    ///     assert_eq!(ctx.ctor(&ty)?, "widget");
    ///     Ok(())
    /// # }
    /// ```
    fn then<P>(self, then: P) -> Then<C, Self, P> {
        Then::new(self, then)
    }

    ///
    /// Conditionally extends a match with an optional suffix **only if** a test expression succeeds.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let sp = neu::whitespace().many0();
    ///     let using = "use"
    ///         .sep_once(
    ///             "",
    ///             neu::ascii_alphanumeric()
    ///                 .or('*')
    ///                 .or('_')
    ///                 .many1()
    ///                 .sep("::"),
    ///         )
    ///         ._1()
    ///         .if_then("as", neu::ascii_alphanumeric().many1());
    ///
    ///     for (str, res) in [
    ///         (
    ///             "use neure::prelude::*",
    ///             (vec!["neure", "prelude", "*"], None),
    ///         ),
    ///         ("use neure as regex", (vec!["neure"], Some("regex"))),
    ///     ] {
    ///         assert_eq!(CharsCtx::new(str).skip_before(sp).ctor(&using)?, res);
    ///     }
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn if_then<I, P>(self, test: I, then: P) -> IfThen<C, Self, I, P> {
        IfThen::new(self, test, then)
    }

    ///
    /// Repeats a pattern a specified number of times, collecting results or spans based on context.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let int = neu::digit(10).many1();
    ///     let int = int.try_map(map::from_str_radix::<i32>(10));
    ///     let num = int.skip_ws().repeat(3..5);
    ///     let mut ctx = CharsCtx::new(r#"1 2 3 4"#);
    ///
    ///     assert_eq!(ctx.ctor(&num)?, [1, 2, 3, 4]);
    ///     Ok(())
    /// # }
    /// ```
    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self> {
        Repeat::new(self, range)
    }

    ///
    /// Repeatedly match the regex `P` at least [`min`](crate::ctor::Collect#tymethod.min) times.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let val = regex::consume(2)
    ///         .try_map(map::from_le_bytes::<i16>())
    ///         .collect::<_, Vec<_>>();
    ///
    ///     assert_eq!(
    ///         BytesCtx::new(b"\x2f\0\x1f\0\x0f\0").ctor(&val)?,
    ///         vec![0x2f, 0x1f, 0x0f]
    ///     );
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn collect<O, V>(self) -> Collect<C, Self, O, V> {
        Collect::new(self)
    }

    ///
    /// Conditional branching combinator that selects between two patterns based on a test function.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let val = "file://".if_else(
    ///         // test if it is a file url
    ///         |ctx: &CharsCtx| Ok(ctx.orig()?.starts_with("file")),
    ///         "http://",
    ///     );
    ///
    ///     assert_eq!(CharsCtx::new("file://").ctor(&val)?, "file://");
    ///     assert_eq!(CharsCtx::new("http://").ctor(&val)?, "http://");
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn if_else<F, E>(self, test: F, other: E) -> Branch<C, Self, F, E>
    where
        F: Fn(&C) -> Result<bool, Error>,
    {
        Branch::new(test, self, other)
    }

    ///
    /// Matches a pattern followed by a mandatory suffix, returning only the main pattern's value during construction.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let sep = neu!(['，' ';']);
    ///     let end = neu!(['。' '？' '！']);
    ///     let word = sep.or(end).not().many1();
    ///     let sent = word.sep(sep.once().skip_ws()).suffix(end.once());
    ///     let sent = sent.repeat(1..);
    ///     let mut ctx = CharsCtx::new(
    ///         r#"暖日晴风初破冻。柳眼眉腮，已觉春心动。酒意诗情谁与共。泪融残粉花钿重。乍试夹衫金缕缝。山枕斜敧，枕损钗头凤。独抱浓愁无好梦。夜阑犹剪灯花弄。"#,
    ///     );
    ///
    ///     assert_eq!(ctx.ctor(&sent)?.len(), 8);
    ///     Ok(())
    /// # }
    /// ```
    ///
    fn suffix<P>(self, pat: P) -> Suffix<C, Self, P> {
        Suffix::new(self, pat)
    }

    ///
    /// Matches a mandatory prefix followed by a pattern, returning only the main
    /// pattern's value during construction.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let num = neu::digit(10).count::<2>();
    ///     let time = num.sep_once(":", num);
    ///     let time = time.enclose("[", "]").skip_ws();
    ///     let star = '*'.count::<3>().skip_ws();
    ///     let name = neu::whitespace().not().many1().skip_ws();
    ///     let status = "left".or("joined").skip_ws();
    ///     let record = name.prefix(star).then(status);
    ///     let record = time.then(record).repeat(1..);
    ///     let mut ctx = CharsCtx::new(
    ///         r#"[20:59] *** jpn left
    ///         [21:00] *** jpn joined
    ///         [21:06] *** guifa left
    ///         [21:07] *** guifa joined"#,
    ///     );
    ///     let records = ctx.ctor(&record)?;
    ///
    ///     assert_eq!(records[0], (("20", "59"), ("jpn", "left")));
    ///     assert_eq!(records[1], (("21", "00"), ("jpn", "joined")));
    ///     assert_eq!(records[2], (("21", "06"), ("guifa", "left")));
    ///     assert_eq!(records[3], (("21", "07"), ("guifa", "joined")));
    ///     Ok(())
    /// # }
    /// ```
    fn prefix<P>(self, pat: P) -> Prefix<C, Self, P> {
        Prefix::new(self, pat)
    }

    /// A shortcut for matching trailing whitespaces.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let str = "file://      ";
    ///     let val = "file://".skip_ws();
    ///
    ///     assert_eq!(CharsCtx::new(str).ctor(&val)?, "file://");
    ///     assert_eq!(CharsCtx::new(str).try_mat(&val)?, Span::new(0, 13));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn skip_ws(self) -> Suffix<C, Self, Many0<C, AsciiWhiteSpace<char>, C::Item>>
    where
        C: Context<'a, Item = char>,
    {
        Suffix::new(self, Many0::new(AsciiWhiteSpace::new(), EmptyCond))
    }

    /// A shortcut for matching trailing ascii whitespaces.
    fn skip_ascii_ws(self) -> Suffix<C, Self, Many0<C, AsciiWhiteSpace<u8>, C::Item>>
    where
        C: Context<'a, Item = u8>,
    {
        Suffix::new(self, Many0::new(AsciiWhiteSpace::new(), EmptyCond))
    }
}
