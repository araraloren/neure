mod collect;
mod r#dyn;
mod r#if;
mod ltm;
mod map;
mod or;
mod pad;
mod pat;
mod quote;
mod repeat;
mod sep;
mod then;

pub use self::collect::Collect;
pub use self::ltm::LongestTokenMatch;
pub use self::map::Map;
pub use self::or::Or;
pub use self::pad::PadUnit;
pub use self::pad::PaddedUnit;
pub use self::pat::Pattern;
pub use self::quote::Quote;
pub use self::r#dyn::DynamicRegex;
pub use self::r#dyn::DynamicRegexHandler;
pub use self::r#if::branch;
pub use self::r#if::IfRegex;
pub use self::repeat::Repeat;
pub use self::sep::Separate;
pub use self::sep::SeparateCollect;
pub use self::sep::SeparateOnce;
pub use self::then::Then;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::err::Error;
use crate::neu::AsciiWhiteSpace;
use crate::neu::CRange;
use crate::neu::Neu;
use crate::neu::NeureOneMore;
use crate::neu::NullCond;
use crate::re::Regex;

pub trait RegexOp<'a, C>
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn map<F, O>(self, f: F) -> Map<C, Self, F, O>;

    fn pat(self) -> Pattern<C, Self>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R>;

    fn sep<S>(self, sep: S) -> Separate<C, Self, S>;

    fn sep_once<S, R>(self, sep: S, right: R) -> SeparateOnce<C, Self, S, R>;

    fn sep_collect<S, O, T>(self, sep: S) -> SeparateCollect<C, Self, S, O, T>;

    fn or<P>(self, pat: P) -> Or<C, Self, P>;

    fn ltm<P>(self, pat: P) -> LongestTokenMatch<C, Self, P>;

    fn then<T>(self, then: T) -> Then<C, Self, T>;

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self>;

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>;

    fn pad<N: Neu<U>, U>(self, unit: N) -> PadUnit<C, Self, N, U>;

    fn padded<N: Neu<U>, U>(self, unit: N) -> PaddedUnit<C, Self, N, U>;

    fn ws(self) -> PadUnit<C, Self, AsciiWhiteSpace, char>;
}

///
/// # Example
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     #[derive(Debug, PartialEq, Eq)]
///     enum Tag {
///         Start(String),
///         End(String),
///         Empty(String),
///     }
///
///     #[derive(Debug, PartialEq, Eq)]
///     enum Xml {
///         Element { name: String, child: Vec<Xml> },
///         Enclosed(String),
///     }
///
///     fn xml_parser(ctx: &mut CharsCtx) -> Result<Vec<Xml>, Error> {
///         let alpha = neu::alphabetic().repeat_full();
///         let start = alpha
///             .quote("<", ">")
///             .map(|v: &str| Ok(Tag::Start(v.to_string())));
///         let end = alpha
///             .quote("</", ">")
///             .map(|v: &str| Ok(Tag::End(v.to_string())));
///         let empty = alpha
///             .quote("<", "/>")
///             .map(|v: &str| Ok(Tag::Empty(v.to_string())));
///         let mut ret = vec![];
///
///         while let Ok(tag) = ctx.invoke(&start.or(empty)) {
///             match tag {
///                 Tag::Start(name) => {
///                     let child = xml_parser(ctx)?;
///                     let end = ctx.invoke(&end)?;
///
///                     if let Tag::End(end_name) = &end {
///                         debug_assert_eq!(&name, end_name);
///                         ret.push(Xml::Element { name, child });
///                         continue;
///                     }
///                     unreachable!("Can not find end tag of {:?}", name);
///                 }
///                 Tag::Empty(name) => {
///                     ret.push(Xml::Enclosed(name));
///                 }
///                 _ => {}
///             }
///         }
///         Ok(ret)
///     }
///
///     let ret = xml_parser(&mut CharsCtx::new(
///         "<language><rust><linux/></rust><cpp><windows/></cpp></language>",
///     ))?;
///     let chk = vec![Xml::Element {
///         name: "language".to_owned(),
///         child: vec![
///             Xml::Element {
///                 name: "rust".to_owned(),
///                 child: vec![Xml::Enclosed("linux".to_owned())],
///             },
///             Xml::Element {
///                 name: "cpp".to_owned(),
///                 child: vec![Xml::Enclosed("windows".to_owned())],
///             },
///         ],
///     }];
///
///     assert_eq!(ret, chk);
///
///     Ok(())
/// # }
/// ```
impl<'a, C, T> RegexOp<'a, C> for T
where
    T: Regex<C>,
    C: Context<'a> + Policy<C>,
{
    fn map<F, O>(self, func: F) -> Map<C, Self, F, O> {
        Map::new(self, func)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let digit = re!(['0' - '9']+);
    ///     let digit = digit.map(|v: &str| Ok(v.parse::<i64>().unwrap()));
    ///     let comma = ",".pad(' ');
    ///     let digits = digit.terminated(comma);
    ///     let array = digits.quote("[", "]");
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

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let ascii = neu::ascii().repeat_one();
    ///     let lit = ascii.quote("'", "'");
    ///     let ele = lit.sep(",".pad(' '));
    ///     let arr = ele.quote("[", "]");
    ///     let mut ctx = CharsCtx::new("['a', 'c', 'd', 'f']");
    ///
    ///     assert_eq!(ctx.ctor(&arr)?, ["a", "c", "d", "f"]);
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R> {
        Quote::new(self, left, right)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let name = re!([^ ',' ']' '[']+);
    ///     let sep = ','.repeat_one().pad(neu::whitespace());
    ///     let arr = name.sep(sep);
    ///     let arr = arr.quote("[", "]");
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
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let key = neu::alphabetic().repeat_one_more().ws();
    ///     let val = neu::whitespace().or(',').not().repeat_one_more().ws();
    ///     let sep = "=>".ws();
    ///     let ele = key.sep_once(sep, val);
    ///     let hash = ele.sep(",".ws()).quote("{".ws(), "}");
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
    fn sep_once<S, R>(self, sep: S, right: R) -> SeparateOnce<C, Self, S, R> {
        SeparateOnce::new(self, sep, right)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let key = neu::alphabetic().repeat_one_more().ws();
    ///     let val = neu::whitespace().or(',').not().repeat_one_more().ws();
    ///     let sep = "=>".ws();
    ///     let ele = key.sep_once(sep, val);
    ///     let hash = ele.sep_collect(",".ws()).quote("{".ws(), "}");
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
    fn sep_collect<S, O, V>(self, sep: S) -> SeparateCollect<C, Self, S, O, V> {
        SeparateCollect::new(self, sep)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     #[derive(Debug, PartialEq, Eq)]
    ///     pub enum V<'a> {
    ///         S(&'a str),
    ///     }
    ///
    ///     let cond = neu::re_cond(re::not("\\\""));
    ///     let str = re!([^ '"' ]+).set_cond(cond).or("\\\"").repeat(1..).pat();
    ///     let str = str.quote("\"", "\"");
    ///     let str = str.map(|v| Ok(V::S(v)));
    ///     let vals = str.sep(",".ws());
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

    fn ltm<P>(self, pat: P) -> LongestTokenMatch<C, Self, P> {
        LongestTokenMatch::new(self, pat)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{prelude::*, re::FromStr};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let str = '"'.not().repeat_full();
    ///     let str = str.quote("\"", "\"");
    ///     let num = neu::digit(10).repeat_full();
    ///     let ele = str.or(num).terminated(','.repeat_zero_one()).ws();
    ///     let tuple = ele.then(ele.map(FromStr::<i32>::new()));
    ///     let mut ctx = CharsCtx::new(r#""c", 42"#);
    ///
    ///     assert_eq!(ctx.invoke(&tuple)?, ("c", 42));
    ///
    ///     Ok(())
    /// # }
    /// ```
    fn then<P>(self, then: P) -> Then<C, Self, P> {
        Then::new(self, then)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{prelude::*, re::FromStr};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let num = neu::digit(10).repeat_full().map(FromStr::<i32>::new());
    ///     let num = num.then(','.repeat_zero_one().ws())._0();
    ///     let array = num.repeat(1..4);
    ///     let mut ctx = CharsCtx::new(r#"6, 8, 10"#);
    ///
    ///     assert_eq!(ctx.invoke(&array)?, vec![6, 8, 10]);
    ///     Ok(())
    /// # }
    /// ```
    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self> {
        Repeat::new(self, range)
    }

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>,
    {
        IfRegex::new(self, r#if, r#else)
    }

    fn pad<N: Neu<U>, U>(self, unit: N) -> PadUnit<C, Self, N, U> {
        PadUnit::new(self, NeureOneMore::new(unit, NullCond))
    }

    fn padded<N: Neu<U>, U>(self, unit: N) -> PaddedUnit<C, Self, N, U> {
        PaddedUnit::new(self, NeureOneMore::new(unit, NullCond))
    }

    fn ws(self) -> PadUnit<C, Self, AsciiWhiteSpace, char> {
        PadUnit::new(self, NeureOneMore::new(AsciiWhiteSpace, NullCond))
    }
}

pub fn into_dyn_regex<'a, 'b, C, R>(
    invoke: impl Fn(&mut C) -> Result<R, Error> + 'b,
) -> DynamicRegex<'b, C, R>
where
    C: Context<'a>,
{
    DynamicRegex::new(Box::new(invoke))
}
