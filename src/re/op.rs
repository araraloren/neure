mod collect;
mod r#dyn;
mod r#if;
mod map;
mod or;
mod pat;
mod quote;
mod repeat;
mod term;
mod then;
mod ws;

pub use self::collect::Collect;
pub use self::map::FromStr;
pub use self::map::Map;
pub use self::map::MapSingle;
pub use self::map::Select0;
pub use self::map::Select1;
pub use self::map::SelectEq;
pub use self::map::Single;
pub use self::or::Or;
pub use self::pat::Pattern;
pub use self::quote::Quote;
pub use self::r#dyn::DynamicRegex;
pub use self::r#dyn::DynamicRegexHandler;
pub use self::r#if::branch;
pub use self::r#if::IfRegex;
pub use self::repeat::Repeat;
pub use self::term::Terminated;
pub use self::then::Then;
pub use self::ws::PaddingUnit;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::err::Error;
use crate::neu::AsciiWhiteSpace;
use crate::neu::CRange;
use crate::neu::Neu;
use crate::neu::NeureOneMore;
use crate::neu::NullCond;
use crate::neu::WhiteSpace;
use crate::re::Regex;

pub trait RegexOp<'a, C>
where
    Self: Sized,
    C: Context<'a> + Policy<C>,
{
    fn map<F, O>(self, f: F) -> Map<C, Self, F, O>;

    fn pattern(self) -> Pattern<C, Self>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R>;

    fn terminated<S>(self, sep: S) -> Terminated<C, Self, S>;

    fn or<P>(self, pat: P) -> Or<C, Self, P>;

    fn then<T>(self, then: T) -> Then<C, Self, T>;

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self>;

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>;

    fn pad<N: Neu<U>, U>(self, unit: N) -> PaddingUnit<C, Self, N, U>;

    fn ws(self) -> PaddingUnit<C, Self, AsciiWhiteSpace, char>;

    fn ws_u(self) -> PaddingUnit<C, Self, WhiteSpace, char>;
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
    ///     let pat = "rust".pattern().map(|v: &str| Ok(v.to_string()));
    ///     let mut ctx = CharsCtx::new("rust");
    ///
    ///     assert_eq!(ctx.invoke(&pat)?, String::from("rust"));
    ///     Ok(())
    /// # }
    /// ```
    fn pattern(self) -> Pattern<C, Self> {
        Pattern::new(self)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// # use re::FromStr;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///
    ///     let num = neu::digit(10).repeat_full();
    ///     let num = num.quote("(", ")").map(FromStr::<i64>::new());
    ///     let mut ctx = CharsCtx::new(r#"(42)"#);
    ///
    ///     assert_eq!(ctx.invoke(&num)?, 42);
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
    ///     let str = '"'.not().repeat_full();
    ///     let str = str.quote("\"", "\"");
    ///     let arr = str.terminated(','.repeat_zero_one()).ws().try_repeat(1..);
    ///     let arr = arr.quote("[", "]");
    ///     let mut ctx = CharsCtx::new(r#"["c", "rust", "java", "c++"]"#);
    ///
    ///     assert_eq!(ctx.invoke(&arr)?, vec!["c", "rust", "java", "c++"]);
    ///     Ok(())
    /// # }
    /// ```
    fn terminated<S>(self, sep: S) -> Terminated<C, Self, S> {
        Terminated::new(self, sep)
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
    ///     let str = '"'.not().repeat_full();
    ///     let str = str.quote("\"", "\"");
    ///     let num = neu::digit(10).repeat_full();
    ///     let ele = str.or(num);
    ///     let mut ctx = CharsCtx::new(r#"42"#);
    ///
    ///     assert_eq!(ctx.invoke(&ele)?, "42");
    ///     let mut ctx = CharsCtx::new(r#""rust""#);
    ///
    ///     assert_eq!(ctx.invoke(&ele)?, "rust");
    ///     Ok(())
    /// # }
    /// ```
    fn or<P>(self, pat: P) -> Or<C, Self, P> {
        Or::new(self, pat)
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
    ///     let num = num.then(','.repeat_zero_one().ws()).select0();
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

    fn pad<N: Neu<U>, U>(self, unit: N) -> PaddingUnit<C, Self, N, U> {
        PaddingUnit::new(self, NeureOneMore::new(unit, NullCond))
    }

    fn ws(self) -> PaddingUnit<C, Self, AsciiWhiteSpace, char> {
        PaddingUnit::new(self, NeureOneMore::new(AsciiWhiteSpace, NullCond))
    }

    fn ws_u(self) -> PaddingUnit<C, Self, WhiteSpace, char> {
        PaddingUnit::new(self, NeureOneMore::new(WhiteSpace, NullCond))
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
