#[cfg(feature = "std")]
pub mod inner_rec_sync {

    use crate::alloc::Arc;
    use crate::alloc::Box;
    use crate::ctor::Adapter;
    use crate::ctor::Ctor;
    use crate::ctor::Handler;
    use crate::ctx::Context;
    use crate::ctx::Match;
    use crate::std::Mutex;

    pub type RecursiveCtorSync<'a, 'b, C, O, H> =
        Arc<Mutex<Option<Adapter<C, Box<dyn Ctor<'a, C, O, H> + Send + Sync + 'b>>>>>;

    ///
    /// This function will construct a [RecursiveCtorSync] for recursive parsing,
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
    ///         let re = u8::is_ascii_lowercase.once();
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
    pub fn rec_parser_sync<'a, 'b, C, O, I, H>(
        mut handler: impl FnMut(RecursiveCtorSync<'a, 'b, C, O, H>) -> I,
    ) -> RecursiveCtorSync<'a, 'b, C, O, H>
    where
        C: Match<'a>,
        H: Handler<C>,
        I: Ctor<'a, C, O, H> + Send + Sync + 'b,
    {
        let r_ctor: RecursiveCtorSync<'a, 'b, C, O, H> = Arc::new(Mutex::new(None));
        let r_ctor_clone = r_ctor.clone();
        let ctor = handler(r_ctor_clone);

        *r_ctor.lock().unwrap() = Some(Adapter::dyn_box_sync(ctor));
        r_ctor
    }

    pub trait RecursiveParserSync<'c, C>
    where
        C: Context<'c>,
    {
        type Ctor<'a, 'b, Ctx, O, H>;

        fn build_with<'b, O, I, H, F>(func: F) -> Self::Ctor<'c, 'b, C, O, H>
        where
            H: Handler<C>,
            I: Ctor<'c, C, O, H> + Send + Sync + 'b,
            F: FnMut(Self::Ctor<'c, 'b, C, O, H>) -> I;

        fn build<O, I, H, F>(func: F) -> Self::Ctor<'c, 'static, C, O, H>
        where
            H: Handler<C>,
            I: Ctor<'c, C, O, H> + Send + Sync + 'static,
            F: FnMut(Self::Ctor<'c, 'static, C, O, H>) -> I,
        {
            Self::build_with(func)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RecParserSync;

    impl<'c, Ctx> RecursiveParserSync<'c, Ctx> for RecParserSync
    where
        Ctx: Context<'c> + Match<'c>,
    {
        type Ctor<'a, 'b, C, O, H> = RecursiveCtorSync<'a, 'b, C, O, H>;

        fn build_with<'b, O, I, H, F>(func: F) -> Self::Ctor<'c, 'b, Ctx, O, H>
        where
            H: Handler<Ctx>,
            I: Ctor<'c, Ctx, O, H> + Send + Sync + 'b,
            F: FnMut(Self::Ctor<'c, 'b, Ctx, O, H>) -> I,
        {
            rec_parser_sync(func)
        }
    }
}

#[cfg(feature = "alloc")]
pub mod inner_rec_unsync {
    use crate::alloc::Box;
    use crate::alloc::Rc;
    use crate::ctor::Adapter;
    use crate::ctor::Ctor;
    use crate::ctor::Handler;
    use crate::ctx::Context;
    use crate::ctx::Match;
    use core::cell::RefCell;

    pub type RecursiveCtor<'a, 'b, C, O, H> =
        Rc<RefCell<Option<Adapter<C, Box<dyn Ctor<'a, C, O, H> + 'b>>>>>;

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
    ///             .many0()
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
    pub fn rec_parser<'a, 'b, C, O, H, I>(
        mut handler: impl FnMut(RecursiveCtor<'a, 'b, C, O, H>) -> I,
    ) -> RecursiveCtor<'a, 'b, C, O, H>
    where
        C: Match<'a>,
        H: Handler<C>,
        I: Ctor<'a, C, O, H> + 'b,
    {
        let r_ctor: RecursiveCtor<'a, 'b, C, O, H> = Rc::new(RefCell::new(None));
        let r_ctor_clone = r_ctor.clone();
        let ctor = handler(r_ctor_clone);

        *r_ctor.borrow_mut() = Some(Adapter::dyn_box(ctor));
        r_ctor
    }

    pub trait RecursiveParser<'c, C>
    where
        C: Match<'c>,
    {
        type Ctor<'a, 'b, Ctx, O, H>;

        fn build<'b, O, I, H, F>(func: F) -> Self::Ctor<'c, 'b, C, O, H>
        where
            H: Handler<C>,
            I: Ctor<'c, C, O, H> + 'b,
            F: FnMut(Self::Ctor<'c, 'b, C, O, H>) -> I;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RecParser;

    impl<'c, C> RecursiveParser<'c, C> for RecParser
    where
        C: Context<'c> + Match<'c>,
    {
        type Ctor<'a, 'b, Ctx, O, H> = RecursiveCtor<'a, 'b, Ctx, O, H>;

        fn build<'b, O, I, H, F>(func: F) -> Self::Ctor<'c, 'b, C, O, H>
        where
            H: Handler<C>,
            I: Ctor<'c, C, O, H> + 'b,
            F: FnMut(Self::Ctor<'c, 'b, C, O, H>) -> I,
        {
            rec_parser(func)
        }
    }
}

#[cfg(feature = "std")]
pub use inner_rec_sync::*;

#[cfg(feature = "alloc")]
pub use inner_rec_unsync::*;
