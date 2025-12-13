use std::fmt::Debug;
use std::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctx::Span;
use crate::err::Error;
use crate::regex::impl_not_for_regex;
use crate::regex::Regex;

///
/// Adapter that restricts a type to constructor-only contexts while satisfying compiler trait requirements.
///
/// [`Wrap`] solves a specific compiler limitation when working with generic combinators that require both [`Regex`]
/// and [`Ctor`] trait implementations, but your type only meaningfully implements [`Ctor`].
///
/// # Regex
///
/// Explicitly unsupported - any attempt to use regex operations will panic at runtime.
///
/// # Ctor
///
/// Delegates all operations to the inner value (fully functional).
#[derive(Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrap<I, C> {
    inner: I,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Wrap<I, C>);

impl<I: Debug, C> Debug for Wrap<I, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wrap").field("inner", &self.inner).finish()
    }
}

impl<I: Clone, C> Clone for Wrap<I, C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<I, C> From<I> for Wrap<I, C> {
    fn from(value: I) -> Self {
        Self {
            inner: value,
            marker: PhantomData,
        }
    }
}

impl<I, C> Wrap<I, C> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: I) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &I {
        &self.inner
    }

    pub fn set_inner(&mut self, inner: I) -> &mut Self {
        self.inner = inner;
        self
    }

    pub fn into_inner(self) -> I {
        self.inner
    }
}

impl<T, C> Wrap<T, C> {
    pub fn ctor(ctor: T) -> Self {
        Self::new(ctor)
    }
}

#[derive(Debug, Clone)]
pub struct BoxedCtor<I> {
    inner: Box<I>,
}

impl<I, C> Regex<C> for BoxedCtor<I> {
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("BoxedCtor not support Regex trait")
    }
}

impl<'a, C, M, O, I, H> Ctor<'a, C, M, O, H> for BoxedCtor<I>
where
    I: Ctor<'a, C, M, O, H>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(self.inner.as_ref(), ctx, handler)
    }
}

///
/// Return a type that wraps `Ctor` with [`Box`].
///
/// # Example
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let re = b'+'
///         .or(b'-')
///         .then(u8::is_ascii_hexdigit)
///         .then(u8::is_ascii_hexdigit.repeat_times::<3>())
///         .pat()
///         .try_map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)));
///    let re = ctor::Wrap::r#box(re);
///
///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, "+AE00");
///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, "-83FD");
///     Ok(())
/// # }
/// ```
impl<T, C> Wrap<BoxedCtor<T>, C> {
    pub fn r#box(ctor: T) -> Self {
        Self::new(BoxedCtor {
            inner: Box::new(ctor),
        })
    }
}

///
/// Return a type that wrap `Ctor` with [`Rc`](std::rc::Rc).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let year = char::is_ascii_digit.repeat_times::<4>();
///     let num = char::is_ascii_digit.repeat_times::<2>();
///     let date = ctor::Wrap::rc(year.sep_once("-", num.sep_once("-", num)));
///     let time = num.sep_once(":", num.sep_once(":", num));
///     let datetime = date.clone().sep_once(" ", time);
///
///     assert_eq!(
///         CharsCtx::new("2024-01-08").ctor(&date)?,
///         ("2024", ("01", "08"))
///     );
///     assert_eq!(
///         CharsCtx::new("2024-01-08 10:01:13").ctor(&datetime)?,
///         (("2024", ("01", "08")), ("10", ("01", "13")))
///     );
///     Ok(())
/// # }
/// ```
impl<T, C> Wrap<std::rc::Rc<T>, C> {
    pub fn rc(ctor: T) -> Self {
        Self::new(std::rc::Rc::new(ctor))
    }
}

///
/// Return a type that wrap `Ctor` with [`Arc`](std::sync::Arc).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let re = u8::is_ascii_hexdigit.then(u8::is_ascii_hexdigit);
///     let re = ctor::Wrap::arc(re);
///
///     assert_eq!(BytesCtx::new(b"AE").ctor(&re)?, b"AE");
///     assert!(BytesCtx::new(b"GH").ctor(&re).is_err());
///
///     Ok(())
/// # }
/// ```
impl<T, C> Wrap<std::sync::Arc<T>, C> {
    pub fn arc(ctor: T) -> Self {
        Self::new(std::sync::Arc::new(ctor))
    }
}

///
/// Return a type that wrap `Ctor` with [`Cell`](std::cell::Cell).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let num = char::is_ascii_digit.repeat_one_more();
///     let float = num.sep_once(".", num).pat();
///     let float = float.try_map(map::from_str::<f64>());
///     let ctor = ctor::Wrap::cell(float);
///
///     assert_eq!(CharsCtx::new("999.88").ctor(&ctor)?, 999.88);
///     assert!(CharsCtx::new("99A/100").ctor(&ctor).is_err());
///
///     Ok(())
/// # }
/// ```
impl<T, C> Wrap<std::cell::Cell<T>, C> {
    pub fn cell(ctor: T) -> Self {
        Self::new(std::cell::Cell::new(ctor))
    }
}

///
/// Return a type that wrap `Ctor` with [`Mutex`](std::sync::Mutex).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let cnt = char::is_ascii_digit.repeat_from::<3>();
///     let prog = cnt.sep_once("/", cnt);
///     let ctor = ctor::Wrap::cell(prog);
///
///     assert_eq!(CharsCtx::new("999/1000").ctor(&ctor)?, ("999", "1000"));
///     assert!(CharsCtx::new("99A/100").ctor(&ctor).is_err());
///
///     Ok(())
/// # }
/// ```
impl<T, C> Wrap<std::sync::Mutex<T>, C> {
    pub fn mutex(ctor: T) -> Self {
        Self::new(std::sync::Mutex::new(ctor))
    }
}

///
/// Return a type that wrap `Ctor` with [`RefCell`](std::cell::RefCell).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let cjk = |ch: &char| ('\u{4e00}'..='\u{9fff}').contains(ch);
///     let scentence = cjk.repeat_one_more();
///     let parser = scentence.enclose("《", "》");
///     let ctor = ctor::Wrap::refcell(parser);
///
///     assert_eq!(CharsCtx::new("《东方力量》").ctor(&ctor)?, "东方力量");
///     assert!(CharsCtx::new("《Power》").ctor(&ctor).is_err());
///
///     Ok(())
/// # }
/// ```
impl<C, T> Wrap<std::cell::RefCell<T>, C> {
    pub fn refcell(ctor: T) -> Self {
        Self::new(std::cell::RefCell::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor` with [`Arc`](std::sync::Arc).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let name = char::is_ascii_alphabetic.repeat_one_more();
///     let time = char::is_ascii_digit.repeat_times::<10>();
///     let parser = name.sep_once("_", time).sep_once(".", "txt")._0();
///     let ctor = ctor::Wrap::refcell(parser);
///
///     assert_eq!(
///         CharsCtx::new("video_1135897722.txt").ctor(&ctor)?,
///         ("video", "1135897722")
///     );
///
///     Ok(())
/// # }
/// ```
impl<'a, 'b, C, M, O, H> Wrap<std::sync::Arc<dyn Ctor<'a, C, M, O, H> + 'b>, C> {
    pub fn dyn_arc(ctor: impl Ctor<'a, C, M, O, H> + 'b) -> Self {
        Self::new(std::sync::Arc::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor + Send` with [`std::sync::Arc`].
///
impl<'a, 'b, C, M, O, H> Wrap<std::sync::Arc<dyn Ctor<'a, C, M, O, H> + Send + 'b>, C> {
    pub fn dyn_arc_send(ctor: impl Ctor<'a, C, M, O, H> + Send + 'b) -> Self {
        Self::new(std::sync::Arc::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor + Send + Sync` with [`std::sync::Arc`].
///
impl<'a, 'b, C, M, O, H> Wrap<std::sync::Arc<dyn Ctor<'a, C, M, O, H> + Send + Sync + 'b>, C> {
    pub fn dyn_arc_sync(ctor: impl Ctor<'a, C, M, O, H> + Send + Sync + 'b) -> Self {
        Self::new(std::sync::Arc::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor` with [`Box`].
///
/// ```
/// # use neure::{err::Error, prelude::*};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let num = u8::is_ascii_digit
///         .repeat_one()
///         .try_map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
///         .try_map(map::from_str::<usize>());
///     let num = num.clone().sep_once(b",", num);
///     let re = num.into_dyn();
///
///     assert_eq!(BytesCtx::new(b"3,0").ctor(&re)?, (3, 0));
///     assert_eq!(BytesCtx::new(b"2,1").ctor(&re)?, (2, 1));
///     assert_eq!(BytesCtx::new(b"0,3").ctor(&re)?, (0, 3));
///     Ok(())
/// # }
/// ```
impl<'a, 'b, C, M, O, H> Wrap<Box<dyn Ctor<'a, C, M, O, H> + 'b>, C> {
    pub fn dyn_box(ctor: impl Ctor<'a, C, M, O, H> + 'b) -> Self {
        Self::new(Box::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor + Send` with [`Box`].
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
///     let (send, recv) = std::sync::mpsc::sync_channel(1);
///
///     let handler = std::thread::spawn(move || {
///         if let Ok(parser) = recv.recv() {
///             if let Ok(res) = CharsCtx::new("video_1135897722.txt").ctor(&parser) {
///                 assert_eq!(res, ("video", "1135897722"));
///             }
///         }
///     });
///
///     send.send({
///         let name = char::is_ascii_alphabetic.repeat_one_more();
///         let time = char::is_ascii_digit.repeat_one_more();
///         let parser = name.sep_once("_", time).sep_once(".", "txt")._0();
///         ctor::Wrap::dyn_box_send(parser)
///     })?;
///
///     handler.join().unwrap();
///
///     Ok(())
/// # }
/// ```
impl<'a, 'b, C, M, O, H> Wrap<Box<dyn Ctor<'a, C, M, O, H> + Send + 'b>, C> {
    pub fn dyn_box_send(ctor: impl Ctor<'a, C, M, O, H> + Send + 'b) -> Self {
        Self::new(Box::new(ctor))
    }
}

///
/// Return a type that wrap `dyn Ctor + Send + Sync` with [`Box`].
///
impl<'a, 'b, C, M, O, H> Wrap<Box<dyn Ctor<'a, C, M, O, H> + Send + Sync + 'b>, C> {
    pub fn dyn_box_sync(ctor: impl Ctor<'a, C, M, O, H> + Send + Sync + 'b) -> Self {
        Self::new(Box::new(ctor))
    }
}

///
/// Return a type that wrap `Ctor` with [`Rc`](std::rc::Rc).
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     tracing_subscriber::fmt::fmt()
///         .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
///         .init();
///
///     let root = "/".opt();
///     let name = '/'.not().repeat_one_more();
///     let path = name.sep("/").opt();
///     let path = root.then(path);
///     let parser = ctor::Wrap::rc(path);
///
///     assert_eq!(CharsCtx::new("/").ctor(&parser)?, (Some("/"), None));
///     assert_eq!(
///         CharsCtx::new("/foo").ctor(&parser)?,
///         (Some("/"), Some(vec!["foo"]))
///     );
///     assert_eq!(
///         CharsCtx::new("foo/bar").ctor(&parser)?,
///         (None, Some(vec!["foo", "bar"]))
///     );
///
///     Ok(())
/// # }
/// ```
impl<'a, 'b, C, M, O, H> Wrap<std::rc::Rc<dyn Ctor<'a, C, M, O, H> + 'b>, C> {
    pub fn dyn_rc(ctor: impl Ctor<'a, C, M, O, H> + 'b) -> Self {
        Self::new(std::rc::Rc::new(ctor))
    }
}

impl<C, I> Regex<C> for Wrap<I, C> {
    #[inline(always)]
    fn try_parse(&self, _: &mut C) -> Result<Span, Error> {
        unimplemented!("Wrap not support Regex trait")
    }
}

impl<'a, C, M, O, H, I> Ctor<'a, C, M, O, H> for Wrap<I, C>
where
    I: Ctor<'a, C, M, O, H>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        Ctor::construct(&self.inner, ctx, handler)
    }
}
