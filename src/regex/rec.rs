use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::ctor::Ctor;
use crate::ctor::Pass;
use crate::ctor::Wrap;
use crate::ctx::Context;
use crate::ctx::Match;

pub type RecursiveCtorWith<'a, 'b, C, M, O, H, A> =
    Rc<RefCell<Option<Wrap<Box<dyn Ctor<'a, C, M, O, H, A> + 'b>, C>>>>;

pub type RecursiveCtorWithSync<'a, 'b, C, M, O, H, A> =
    Arc<Mutex<Option<Wrap<Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b>, C>>>>;

pub type RecursiveCtor<'a, 'b, C, M, O> = RecursiveCtorWith<'a, 'b, C, M, O, Pass, M>;

pub type RecursiveCtorSync<'a, 'b, C, M, O> =
    Arc<Mutex<Option<Wrap<Box<dyn Ctor<'a, C, M, O, Pass, M> + Send + 'b>, C>>>>;

///
/// # Example
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #    color_eyre::install()?;
///     #[derive(Debug, PartialEq, Eq)]
///     enum Xml {
///         Element { name: String, child: Vec<Xml> },
///         Enclosed(String),
///     }
///
///     let xml = regex::rec_parser(|ctor| {
///         let alpha = neu::alphabetic()
///             .repeat_full()
///             .map(|v: &str| Ok(v.to_string()));
///         let s = alpha.quote("<", ">");
///         let e = alpha.quote("</", ">");
///         let c = alpha.quote("<", "/>").map(|v| Ok(Xml::Enclosed(v)));
///         let m = |((l, c), r): ((String, Vec<Xml>), String)| {
///             if l != r {
///                 Err(Error::Uid(0))
///             } else {
///                 Ok(Xml::Element { name: l, child: c })
///             }
///         };
///
///         s.then(ctor.clone()).then(e).map(m).or(c).repeat(1..)
///     });
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
pub fn rec_parser_with<'a, 'b, C, M, O, H, A, I>(
    mut handler: impl FnMut(RecursiveCtorWith<'a, 'b, C, M, O, H, A>) -> I,
) -> RecursiveCtorWith<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<'a>,
    I: Ctor<'a, C, M, O, H, A> + 'b,
{
    let r_ctor: RecursiveCtorWith<'a, 'b, C, M, O, H, A> = Rc::new(RefCell::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.borrow_mut() = Some(Wrap::dyn_box(ctor));
    r_ctor
}

///
/// This function will construct a [RecursiveCtor] for recursive parsing,
/// which accepts a closure as a parameter,
/// and the parameter of the closure is the return value of the function.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, regex::rec_parser};
/// #
/// # fn main() -> color_eyre::Result<()> {
/// #    color_eyre::install()?;
///     // into_dyn_ctor using by rec_parser
///     let parser = rec_parser(|ctor| {
///         let re = u8::is_ascii_lowercase.repeat_one();
///
///         ctor.clone()
///             .or(re)
///             .quote(b"{", b"}")
///             .or(ctor.clone().or(re).quote(b"[", b"]"))
///     });
///
///     assert_eq!(BytesCtx::new(b"[a]").ctor(&parser)?, b"a");
///     assert_eq!(BytesCtx::new(b"{{[[{[{b}]}]]}}").ctor(&parser)?, b"b");
///     assert_eq!(BytesCtx::new(b"[{{{[c]}}}]").ctor(&parser)?, b"c");
///     Ok(())
/// # }
/// ```
///
pub fn rec_parser<'a, 'b, C, M, O, I>(
    mut handler: impl FnMut(RecursiveCtor<'a, 'b, C, M, O>) -> I,
) -> RecursiveCtor<'a, 'b, C, M, O>
where
    C: Context<'a> + Match<'a>,
    I: Ctor<'a, C, M, O, Pass, M> + 'b,
{
    let r_ctor: RecursiveCtor<'a, 'b, C, M, O> = Rc::new(RefCell::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.borrow_mut() = Some(Wrap::dyn_box(ctor));
    r_ctor
}

pub fn rec_parser_with_sync<'a, 'b, C, M, O, I, H, A>(
    mut handler: impl FnMut(RecursiveCtorWithSync<'a, 'b, C, M, O, H, A>) -> I,
) -> RecursiveCtorWithSync<'a, 'b, C, M, O, H, A>
where
    C: Context<'a> + Match<'a>,
    I: Ctor<'a, C, M, O, H, A> + Send + 'b,
{
    let r_ctor: RecursiveCtorWithSync<'a, 'b, C, M, O, H, A> = Arc::new(Mutex::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.lock().unwrap() = Some(Wrap::dyn_box_sync(ctor));
    r_ctor
}

///
/// Same as [`rec_parser`], but need [`Send`] or [`Sync`] for some type.
///
pub fn rec_parser_sync<'a, 'b, C, M, O, I>(
    mut handler: impl FnMut(RecursiveCtorSync<'a, 'b, C, M, O>) -> I,
) -> RecursiveCtorSync<'a, 'b, C, M, O>
where
    C: Context<'a> + Match<'a>,
    I: Ctor<'a, C, M, O, Pass, M> + Send + 'b,
{
    let r_ctor: RecursiveCtorSync<'a, 'b, C, M, O> = Arc::new(Mutex::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.lock().unwrap() = Some(Wrap::dyn_box_sync(ctor));
    r_ctor
}

pub trait RecursiveParser<'ctx, Ctx>
where
    Ctx: Context<'ctx>,
{
    type H;

    type A;

    type Ctor<'a, 'b, C, M, O, H, A>;

    fn build<'b, M, O, I, F>(func: F) -> Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>
    where
        F: FnMut(Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>) -> I,
        I: Ctor<'ctx, Ctx, M, O, Self::H, Self::A> + 'b;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecParser;

impl<'ctx, Ctx> RecursiveParser<'ctx, Ctx> for RecParser
where
    Ctx: Context<'ctx> + Match<'ctx>,
{
    type H = Pass;

    type A = Ctx::Orig<'ctx>;

    type Ctor<'a, 'b, C, M, O, H, A> = RecursiveCtorWith<'a, 'b, C, M, O, H, A>;

    fn build<'b, M, O, I, F>(func: F) -> Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>
    where
        F: FnMut(Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>) -> I,
        I: Ctor<'ctx, Ctx, M, O, Self::H, Self::A> + 'b,
    {
        rec_parser_with(func)
    }
}

pub trait RecursiveParserSync<'ctx, Ctx>
where
    Ctx: Context<'ctx>,
{
    type H;

    type A;

    type Ctor<'a, 'b, C, M, O, H, A>;

    fn build_with<'b, M, O, I, F>(func: F) -> Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>
    where
        F: FnMut(Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>) -> I,
        I: Ctor<'ctx, Ctx, M, O, Self::H, Self::A> + Send + 'b;

    fn build<M, O, I, F>(func: F) -> Self::Ctor<'ctx, 'static, Ctx, M, O, Self::H, Self::A>
    where
        F: FnMut(Self::Ctor<'ctx, 'static, Ctx, M, O, Self::H, Self::A>) -> I,
        I: Ctor<'ctx, Ctx, M, O, Self::H, Self::A> + Send + 'static,
    {
        Self::build_with(func)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecParserSync;

impl<'ctx, Ctx> RecursiveParserSync<'ctx, Ctx> for RecParserSync
where
    Ctx: Context<'ctx> + Match<'ctx>,
{
    type H = Pass;

    type A = Ctx::Orig<'ctx>;

    type Ctor<'a, 'b, C, M, O, H, A> = RecursiveCtorWithSync<'a, 'b, C, M, O, H, A>;

    fn build_with<'b, M, O, I, F>(func: F) -> Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>
    where
        F: FnMut(Self::Ctor<'ctx, 'b, Ctx, M, O, Self::H, Self::A>) -> I,
        I: Ctor<'ctx, Ctx, M, O, Self::H, Self::A> + Send + 'b,
    {
        rec_parser_with_sync(func)
    }
}
