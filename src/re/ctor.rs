mod boxed;
mod collect;
mod dthen;
mod dynamic;
mod r#if;
mod ltm;
mod map;
mod opt;
mod or;
mod pad;
mod pat;
mod quote;
mod repeat;
mod sep;
mod then;
mod vec;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::boxed::into_boxed_ctor;
pub use self::boxed::BoxedCtor;
pub use self::boxed::BoxedCtorHelper;
pub use self::collect::Collect;
pub use self::dthen::DynamicCreateCtorThen;
pub use self::dthen::DynamicCreateCtorThenHelper;
pub use self::dynamic::into_dyn_ctor;
pub use self::dynamic::DynamicCtor;
pub use self::dynamic::DynamicCtorHandler;
pub use self::dynamic::DynamicCtorHelper;
pub use self::ltm::LongestTokenMatch;
pub use self::map::Map;
pub use self::opt::OptionPat;
pub use self::or::Or;
pub use self::pad::PadUnit;
pub use self::pad::PaddedUnit;
pub use self::pat::Pattern;
pub use self::quote::Quote;
pub use self::r#if::branch;
pub use self::r#if::IfRegex;
pub use self::repeat::Repeat;
pub use self::sep::SepCollect;
pub use self::sep::SepOnce;
pub use self::sep::Separate;
pub use self::then::IfThen;
pub use self::then::Then;
pub use self::vec::PairVector;
pub use self::vec::Vector;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::err::Error;
use crate::neu::AsciiWhiteSpace;
use crate::neu::CRange;
use crate::neu::NeureZeroMore;
use crate::neu::NullCond;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub trait Ctor<'a, C, M, O>
where
    C: Context<'a>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

impl<'a, C, O, F> Ctor<'a, C, O, O> for F
where
    C: Context<'a> + Match<C>,
    F: Fn(&mut C) -> Result<Span, Error>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for Box<dyn Regex<C, Ret = Span>>
where
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &str
where
    C: Context<'a, Orig = str> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for String
where
    C: Context<'a, Orig = str> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.as_str())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &String
where
    C: Context<'a, Orig = str> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(&self.as_str())?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &[u8]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, const N: usize, C, O> Ctor<'a, C, O, O> for &[u8; N]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, const N: usize, C, O> Ctor<'a, C, O, O> for [u8; N]
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for Vec<u8>
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &Vec<u8>
where
    C: Context<'a, Orig = [u8]> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Option<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref().ok_or(Error::Option)?, ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for RefCell<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(&*self.borrow(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Cell<I>
where
    I: Ctor<'a, C, M, O> + Copy,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(&self.get(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Mutex<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = self.lock().map_err(|_| Error::LockMutex)?;

        Ctor::constrct(&*ret, ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Arc<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Rc<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Match<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref(), ctx, handler)
    }
}

pub type RecursiveCtor<'a, C, O> = Rc<RefCell<Option<DynamicCtor<'a, C, O>>>>;

pub type RecursiveCtorSync<'a, C, O> = Arc<Mutex<Option<DynamicCtor<'a, C, O>>>>;

pub fn rec_parser<'a, 'b, C, O, I>(
    handler: impl Fn(RecursiveCtor<'b, C, O>) -> I,
) -> RecursiveCtor<'b, C, O>
where
    C: Context<'a>,
    I: Fn(&mut C) -> Result<O, Error> + 'b,
{
    let r_ctor: RecursiveCtor<'b, C, O> = Rc::new(RefCell::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.borrow_mut() = Some(into_dyn_ctor(ctor));
    r_ctor
}

pub fn rec_parser_sync<'a, 'b, C, O, I>(
    handler: impl Fn(RecursiveCtorSync<'b, C, O>) -> I,
) -> RecursiveCtorSync<'b, C, O>
where
    C: Context<'a>,
    I: Fn(&mut C) -> Result<O, Error> + 'b,
{
    let r_ctor: RecursiveCtorSync<'b, C, O> = Arc::new(Mutex::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.lock().unwrap() = Some(into_dyn_ctor(ctor));
    r_ctor
}

pub trait ConstructOp<'a, C>
where
    Self: Sized,
    C: Context<'a> + Match<C>,
{
    fn map<F, O>(self, f: F) -> Map<C, Self, F, O>;

    fn pat(self) -> Pattern<C, Self>;

    fn opt(self) -> OptionPat<C, Self>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<C, Self, L, R>;

    fn sep<S>(self, sep: S) -> Separate<C, Self, S>;

    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R>;

    fn sep_collect<S, O, T>(self, sep: S) -> SepCollect<C, Self, S, O, T>;

    fn or<P>(self, pat: P) -> Or<C, Self, P>;

    fn ltm<P>(self, pat: P) -> LongestTokenMatch<C, Self, P>;

    fn then<T>(self, then: T) -> Then<C, Self, T>;

    fn if_then<I, T>(self, r#if: I, then: T) -> IfThen<C, Self, I, T>;

    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self>;

    fn collect<O, T>(self) -> Collect<C, Self, O, T>;

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>;

    fn pad<T>(self, tail: T) -> PadUnit<C, Self, T>;

    fn padded<T>(self, tail: T) -> PaddedUnit<C, Self, T>;

    fn ws(self) -> PadUnit<C, Self, NeureZeroMore<C, AsciiWhiteSpace, C::Item, NullCond>>
    where
        C: Context<'a, Item = char>;
}

///
/// # Example
///
/// ```
/// # use neure::{
/// #     err::Error,
/// #     prelude::*,
/// #     re::{rec_parser, RecursiveCtor},
/// # };
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///     #[derive(Debug, PartialEq, Eq)]
///     enum Xml {
///         Element { name: String, child: Vec<Xml> },
///         Enclosed(String),
///     }
///
///     pub fn parser<'a: 'b, 'b>(
///         ctor: RecursiveCtor<'b, CharsCtx<'a>, Vec<Xml>>,
///     ) -> impl Fn(&mut CharsCtx<'a>) -> Result<Vec<Xml>, Error> + 'b {
///         move |ctx| {
///             let alpha = neu::alphabetic()
///                 .repeat_full()
///                 .map(|v: &str| Ok(v.to_string()));
///             let s = alpha.quote("<", ">");
///             let e = alpha.quote("</", ">");
///             let c = alpha.quote("<", "/>").map(|v| Ok(Xml::Enclosed(v)));
///             let m = |((l, c), r): ((String, Vec<Xml>), String)| {
///                 if l != r {
///                     Err(Error::Uid(0))
///                 } else {
///                     Ok(Xml::Element { name: l, child: c })
///                 }
///             };
///
///             ctx.ctor(&s.then(ctor.clone()).then(e).map(m).or(c).repeat(1..))
///         }
///     }
///     let xml = rec_parser(parser);
///     let ret = CharsCtx::new("<language><rust><linux/></rust><cpp><windows/></cpp></language>")
///         .ctor(&xml)?;
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
///     Ok(())
/// # }
/// ```
impl<'a, C, T> ConstructOp<'a, C> for T
where
    T: Regex<C>,
    C: Context<'a> + Match<C>,
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
    ///     let digit = re!(['0' - '9']+);
    ///     let digit = digit.map(|v: &str| Ok(v.parse::<i64>().unwrap()));
    ///     let digits = digit.sep(",".ws());
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

    fn opt(self) -> OptionPat<C, Self> {
        OptionPat::new(self)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let ascii = neu::ascii().repeat_one();
    ///     let lit = ascii.quote("'", "'");
    ///     let ele = lit.sep(",".ws());
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
    ///     let name = re!([^ ',' ']' '[']+);
    ///     let sep = ','.repeat_one().ws();
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
    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R> {
        SepOnce::new(self, sep, right)
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
    fn sep_collect<S, O, V>(self, sep: S) -> SepCollect<C, Self, S, O, V> {
        SepCollect::new(self, sep)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
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

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     #[derive(Debug, PartialEq, Eq)]
    ///     pub struct Val<'a>(&'a str);
    ///
    ///     let val = "v".ltm("val".ltm("value"));
    ///     let val = val.map(|v| Ok(Val(v)));
    ///     let val = val.sep(",".ws());
    ///     let val = val.quote("{", "}");
    ///     let mut ctx = CharsCtx::new(r#"{val, v, value}"#);
    ///
    ///     assert_eq!(ctx.ctor(&val)?, [Val("val"), Val("v"), Val("value")]);
    ///     Ok(())
    /// # }
    /// ```
    fn ltm<P>(self, pat: P) -> LongestTokenMatch<C, Self, P> {
        LongestTokenMatch::new(self, pat)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let ws = neu::whitespace().repeat_full();
    ///     let id = neu::ascii_alphabetic().repeat_one_more();
    ///     let st = "struct".ws().then(id)._1();
    ///     let en = "enum".ws().then(id)._1();
    ///     let ty = st.or(en);
    ///     let ty = ty.ws().then(ws.quote("{", "}"))._0();
    ///     let mut ctx = CharsCtx::new(r#"struct widget { }"#);
    ///
    ///     assert_eq!(ctx.ctor(&ty)?, "widget");
    ///     Ok(())
    /// # }
    /// ```
    fn then<P>(self, then: P) -> Then<C, Self, P> {
        Then::new(self, then)
    }

    fn if_then<I, P>(self, r#if: I, then: P) -> IfThen<C, Self, I, P> {
        IfThen::new(self, r#if, then)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let int = neu::digit(10).repeat_one_more();
    ///     let int = int.map(re::map::from_str_radix::<i32>(10));
    ///     let num = int.ws().repeat(3..5);
    ///     let mut ctx = CharsCtx::new(r#"1 2 3 4"#);
    ///
    ///     assert_eq!(ctx.ctor(&num)?, [1, 2, 3, 4]);
    ///     Ok(())
    /// # }
    /// ```
    fn repeat(self, range: impl Into<CRange<usize>>) -> Repeat<C, Self> {
        Repeat::new(self, range)
    }

    fn collect<O, V>(self) -> Collect<C, Self, O, V> {
        Collect::new(self)
    }

    fn r#if<I, E>(self, r#if: I, r#else: E) -> IfRegex<C, Self, I, E>
    where
        I: Fn(&C) -> Result<bool, Error>,
    {
        IfRegex::new(self, r#if, r#else)
    }

    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let sep = neu!(['，' ';']);
    ///     let end = neu!(['。' '？' '！']);
    ///     let word = sep.or(end).not().repeat_one_more();
    ///     let sent = word.sep(sep.repeat_one().ws()).pad(end.repeat_one());
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
    fn pad<P>(self, pat: P) -> PadUnit<C, Self, P> {
        PadUnit::new(self, pat)
    }

    ///  
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let num = neu::digit(10).repeat_times::<2>();
    ///     let time = num.sep_once(":", num);
    ///     let time = time.quote("[", "]").ws();
    ///     let star = '*'.repeat_times::<3>().ws();
    ///     let name = neu::whitespace().not().repeat_one_more().ws();
    ///     let status = "left".or("joined").ws();
    ///     let record = name.padded(star).then(status);
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
    fn padded<P>(self, pat: P) -> PaddedUnit<C, Self, P> {
        PaddedUnit::new(self, pat)
    }

    fn ws(self) -> PadUnit<C, Self, NeureZeroMore<C, AsciiWhiteSpace, C::Item, NullCond>>
    where
        C: Context<'a, Item = char>,
    {
        PadUnit::new(self, NeureZeroMore::new(AsciiWhiteSpace, NullCond))
    }
}
