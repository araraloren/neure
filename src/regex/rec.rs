use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctor::Wrap;
use crate::ctx::Context;
use crate::ctx::Match;

pub type RecursiveCtorWith<'a, 'b, C, M, O, H> =
    Rc<RefCell<Option<Wrap<Box<dyn Ctor<'a, C, M, O, H> + 'b>, C>>>>;

pub type RecursiveCtorWithSync<'a, 'b, C, M, O, H> =
    Arc<Mutex<Option<Wrap<Box<dyn Ctor<'a, C, M, O, H> + Send + 'b>, C>>>>;

pub type RecursiveCtor<'a, 'b, C, M, O> = RecursiveCtorWith<'a, 'b, C, M, O, Extract<M>>;

pub type RecursiveCtorSync<'a, 'b, C, M, O> =
    Arc<Mutex<Option<Wrap<Box<dyn Ctor<'a, C, M, O, Extract<M>> + Send + 'b>, C>>>>;

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
///         Enclose(String),
///     }
///
///     let xml = regex::rec_parser(|ctor| {
///         let alpha = neu::alphabetic()
///             .repeat_full()
///             .map(|v: &str| v.to_string());
///         let s = alpha.enclose("<", ">");
///         let e = alpha.enclose("</", ">");
///         let c = alpha.enclose("<", "/>").map(Xml::Enclose);
///         let m = |((l, c), r): ((String, Vec<Xml>), String)| {
///             if l != r {
///                 Err(Error::Uid(0))
///             } else {
///                 Ok(Xml::Element { name: l, child: c })
///             }
///         };
///
///         s.then(ctor.clone()).then(e).try_map(m).or(c).repeat(1..)
///     });
///     let ret = CharsCtx::new("<language><rust><linux/></rust><cpp><windows/></cpp></language>")
///         .ctor(&xml)?;
///     let chk = vec![Xml::Element {
///         name: "language".to_owned(),
///         child: vec![
///             Xml::Element {
///                 name: "rust".to_owned(),
///                 child: vec![Xml::Enclose("linux".to_owned())],
///             },
///             Xml::Element {
///                 name: "cpp".to_owned(),
///                 child: vec![Xml::Enclose("windows".to_owned())],
///             },
///         ],
///     }];
///
///     assert_eq!(ret, chk);
///     Ok(())
/// # }
/// ```
pub fn rec_parser_with<'a, 'b, C, M, O, H, I>(
    mut handler: impl FnMut(RecursiveCtorWith<'a, 'b, C, M, O, H>) -> I,
) -> RecursiveCtorWith<'a, 'b, C, M, O, H>
where
    C: Match<'a>,
    H: Handler<C, Out = M>,
    I: Ctor<'a, C, M, O, H> + 'b,
{
    let r_ctor: RecursiveCtorWith<'a, 'b, C, M, O, H> = Rc::new(RefCell::new(None));
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
///             .enclose(b"{", b"}")
///             .or(ctor.clone().or(re).enclose(b"[", b"]"))
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
    C: Match<'a>,
    Extract<M>: Handler<C, Out = M>,
    I: Ctor<'a, C, M, O, Extract<M>> + 'b,
{
    let r_ctor: RecursiveCtor<'a, 'b, C, M, O> = Rc::new(RefCell::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.borrow_mut() = Some(Wrap::dyn_box(ctor));
    r_ctor
}

pub fn rec_parser_with_sync<'a, 'b, C, M, O, I, H>(
    mut handler: impl FnMut(RecursiveCtorWithSync<'a, 'b, C, M, O, H>) -> I,
) -> RecursiveCtorWithSync<'a, 'b, C, M, O, H>
where
    C: Match<'a>,
    H: Handler<C, Out = M>,
    I: Ctor<'a, C, M, O, H> + Send + 'b,
{
    let r_ctor: RecursiveCtorWithSync<'a, 'b, C, M, O, H> = Arc::new(Mutex::new(None));
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
    C: Match<'a>,
    Extract<M>: Handler<C, Out = M> + Send,
    I: Ctor<'a, C, M, O, Extract<M>> + Send + 'b,
{
    let r_ctor: RecursiveCtorSync<'a, 'b, C, M, O> = Arc::new(Mutex::new(None));
    let r_ctor_clone = r_ctor.clone();
    let ctor = handler(r_ctor_clone);

    *r_ctor.lock().unwrap() = Some(Wrap::dyn_box_sync(ctor));
    r_ctor
}

pub trait RecursiveParser<'c, Ctx>
where
    Ctx: Match<'c>,
{
    type H: Handler<Ctx>;

    type Ctor<'a, 'b, C, M, O, H>;

    fn build<'b, O, I, F>(
        func: F,
    ) -> Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>
    where
        F: FnMut(Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>) -> I,
        I: Ctor<'c, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H> + 'b;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecParser;

impl<'c, Ctx> RecursiveParser<'c, Ctx> for RecParser
where
    Ctx: Context<'c> + Match<'c>,
    Extract<Ctx::Orig<'c>>: Handler<Ctx>,
{
    type H = Extract<Ctx::Orig<'c>>;

    type Ctor<'a, 'b, C, M, O, H> = RecursiveCtorWith<'a, 'b, C, M, O, H>;

    fn build<'b, O, I, F>(
        func: F,
    ) -> Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>
    where
        F: FnMut(Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>) -> I,
        I: Ctor<'c, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H> + 'b,
    {
        rec_parser_with(func)
    }
}

pub trait RecursiveParserSync<'c, Ctx>
where
    Ctx: Context<'c>,
{
    type H: Handler<Ctx>;

    type Ctor<'a, 'b, C, M, O, H>;

    fn build_with<'b, O, I, F>(
        func: F,
    ) -> Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>
    where
        F: FnMut(Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>) -> I,
        I: Ctor<'c, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H> + Send + 'b;

    fn build<O, I, F>(
        func: F,
    ) -> Self::Ctor<'c, 'static, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>
    where
        F: FnMut(Self::Ctor<'c, 'static, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>) -> I,
        I: Ctor<'c, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H> + Send + 'static,
    {
        Self::build_with(func)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecParserSync;

impl<'c, Ctx> RecursiveParserSync<'c, Ctx> for RecParserSync
where
    Ctx: Context<'c> + Match<'c>,
    Extract<Ctx::Orig<'c>>: Handler<Ctx>,
{
    type H = Extract<Ctx::Orig<'c>>;

    type Ctor<'a, 'b, C, M, O, H> = RecursiveCtorWithSync<'a, 'b, C, M, O, H>;

    fn build_with<'b, O, I, F>(
        func: F,
    ) -> Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>
    where
        F: FnMut(Self::Ctor<'c, 'b, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H>) -> I,
        I: Ctor<'c, Ctx, <Self::H as Handler<Ctx>>::Out, O, Self::H> + Send + 'b,
    {
        rec_parser_with_sync(func)
    }
}
