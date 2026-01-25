use core::fmt::Debug;
use core::marker::PhantomData;

use crate::ctor::Ctor;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;
use crate::regex::impl_not_for_regex;
use crate::span::Span;

#[cfg(feature = "alloc")]
use crate::alloc;

///
/// Transparent adapter that elevates Regex combinators to Ctor-enabled combinators.
///
/// [`Adapter`] serves as a zero-cost abstraction layer that allows any [`Regex`] combinator to
/// participate in constructor-based parsing chains. It preserves the exact matching behavior of
/// the inner combinator while enabling value construction through handler functions. This adapter
/// is essential when composing parsers that mix pure matching combinators with value-producing
/// combinators in sequence chains.
///
/// # Regex
///
/// Forwards parsing directly to inner combinator
///
/// # Ctor
///
/// Uses identical matching logic as regex mode, then constructs a value from the result.
#[derive(Copy)]
pub struct Adapter<C, I> {
    inner: I,
    marker: PhantomData<C>,
}

impl_not_for_regex!(Adapter<C, I>);

impl<I: Debug, C> Debug for Adapter<C, I> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Adapter")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<I: Default, C> Default for Adapter<C, I> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            marker: Default::default(),
        }
    }
}

impl<I: Clone, C> Clone for Adapter<C, I> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<I, C> From<I> for Adapter<C, I> {
    fn from(value: I) -> Self {
        Self {
            inner: value,
            marker: PhantomData,
        }
    }
}

impl<I, C> Adapter<C, I> {
    pub const fn new(inner: I) -> Self {
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

    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: I) -> &mut Self {
        self.inner = inner;
        self
    }

    pub fn into_inner(self) -> I {
        self.inner
    }
}

///
/// Return a type that wrap `Regex` with [`Box`](crate::alloc::Box).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&"who");
///     let regex = regex::Adapter::r#box(regex);
///
///     assert_eq!(CharsCtx::new("who are you?").ctor(&regex)?, "who");
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<T, C> Adapter<C, BoxAdapter<C, T>>
where
    T: Regex<C>,
{
    pub fn r#box(regex: T) -> Self {
        Self::new(BoxAdapter::new(regex))
    }
}

///
/// Return a type that wrap `Regex` with [`alloc::Rc`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&"@");
///     let regex = regex::Adapter::rc(regex);
///     let snd = regex.clone();
///     let fst = regex;
///
///     assert_eq!(CharsCtx::new("@@").ctor(&fst.then(snd))?, ("@", "@"));
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<T, C> Adapter<C, alloc::Rc<T>>
where
    T: Regex<C>,
{
    pub fn rc(regex: T) -> Self {
        Self::new(alloc::Rc::new(regex))
    }
}

///
/// Return a type that wrap `Regex` with [`alloc::Arc`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&"@");
///     let regex = regex::Adapter::arc(regex);
///     let snd = regex.clone();
///     let fst = regex;
///
///     assert_eq!(CharsCtx::new("@@").ctor(&fst.then(snd))?, ("@", "@"));
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<T, C> Adapter<C, alloc::Arc<T>>
where
    T: Regex<C>,
{
    pub fn arc(regex: T) -> Self {
        Self::new(alloc::Arc::new(regex))
    }
}

///
/// Return a type that wrap `Regex` with [`core::cell::Cell`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = move |ctx: &mut CharsCtx| ctx.try_mat(&"@");
///     let regex = regex::Adapter::cell(regex);
///     let snd = regex.clone();
///     let fst = regex;
///
///     assert_eq!(CharsCtx::new("@@").ctor(&fst.then(snd))?, ("@", "@"));
///
/// #   Ok(())
/// # }
/// ```
impl<T, C> Adapter<C, core::cell::Cell<T>>
where
    T: Regex<C>,
{
    pub fn cell(regex: T) -> Self {
        Self::new(core::cell::Cell::new(regex))
    }
}

///
/// Return a type that wrap `Regex` with [`crate::std::Mutex`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = core::cell::RefCell::new("where");
///     let regex = regex::Adapter::mutex(regex);
///
///     std::thread::scope(|scope| {
///         let handler1 = scope.spawn(|| CharsCtx::new("where are you from?").ctor(&regex));
///         let handler2 = scope.spawn(|| CharsCtx::new("where are you from?").ctor(&regex));
///
///         assert_eq!(handler1.join().unwrap()?, handler2.join().unwrap()?);
///         Ok::<_, neure::err::Error>(())
///     })?;
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "std")]
impl<T, C> Adapter<C, crate::std::Mutex<T>>
where
    T: Regex<C>,
{
    pub fn mutex(regex: T) -> Self {
        Self::new(crate::std::Mutex::new(regex))
    }
}

///
/// Return a type that wrap `Regex` with [`core::cell::RefCell`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = " are you from?".prefix("what");
///     let regex = regex::Adapter::refcell(regex);
///
///     // replace prefix
///     regex.inner().borrow_mut().set_prefix("where");
///     assert_eq!(
///         CharsCtx::new("where are you from?").ctor(&regex)?,
///         "where are you from?"
///     );
///
/// #   Ok(())
/// # }
/// ```
impl<T, C> Adapter<C, core::cell::RefCell<T>>
where
    T: Regex<C>,
{
    pub fn refcell(regex: T) -> Self {
        Self::new(core::cell::RefCell::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex` with [`alloc::Arc`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&core::cell::RefCell::new("rust"));
///     let regex1 = regex::Adapter::dyn_arc(regex);
///     let regex2 = regex1.clone();
///
///     assert_eq!(
///         CharsCtx::new("rust rust 2024?").ctor(&regex1.sep_once(" ", regex2))?,
///         ("rust", "rust")
///     );
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Arc<dyn Regex<C> + 'a>> {
    pub fn dyn_arc(regex: impl Regex<C> + 'a) -> Self {
        Self::new(alloc::Arc::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex + Send` with [`alloc::Arc`].
///
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Arc<dyn Regex<C> + Send + 'a>> {
    pub fn dyn_arc_send(regex: impl Regex<C> + Send + 'a) -> Self {
        Self::new(alloc::Arc::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex + Send + Sync` with [`alloc::Arc`].
///
/// # Example
/// ```
/// # use std::sync::mpsc::channel;
/// #
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&"where");
///     let regex = regex::Adapter::dyn_arc_sync(regex);
///     let (send, recv) = channel();
///
///     std::thread::spawn(move || {
///         if let Ok(regex) = recv.recv() {
///             assert_eq!(
///                 CharsCtx::new("where are you from?").ctor(&regex).unwrap(),
///                 "where"
///             );
///         }
///     });
///
///     send.send(regex)?;
///
/// #    Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Arc<dyn Regex<C> + Send + Sync + 'a>> {
    pub fn dyn_arc_sync(regex: impl Regex<C> + Send + Sync + 'a) -> Self {
        Self::new(alloc::Arc::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex` with [`Box`](crate::alloc::Box).
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&core::cell::RefCell::new("rust"));
///     let regex = regex::Adapter::dyn_box(regex);
///
///     assert_eq!(CharsCtx::new("rust 2024?").ctor(&regex)?, "rust");
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Box<dyn Regex<C> + 'a>> {
    pub fn dyn_box(regex: impl Regex<C> + 'a) -> Self {
        Self::new(alloc::Box::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex + Send` with [`Box`](crate::alloc::Box).
///
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Box<dyn Regex<C> + Send + 'a>> {
    pub fn dyn_box_send(regex: impl Regex<C> + Send + 'a) -> Self {
        Self::new(alloc::Box::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex + Send + Sync` with [`Box`](crate::alloc::Box).
///
/// # Example
/// ```
/// # use std::sync::mpsc::channel;
/// #
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&"where");
///     let regex = regex::Adapter::dyn_box_sync(regex);
///     let (send, recv) = channel();
///
///     std::thread::spawn(move || {
///         if let Ok(regex) = recv.recv() {
///             assert_eq!(
///                 CharsCtx::new("where are you from?").ctor(&regex).unwrap(),
///                 "where"
///             );
///         }
///     });
///
///     send.send(regex)?;
///
/// #    Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Box<dyn Regex<C> + Send + Sync + 'a>> {
    pub fn dyn_box_sync(regex: impl Regex<C> + Send + Sync + 'a) -> Self {
        Self::new(alloc::Box::new(regex))
    }
}

///
/// Return a type that wrap `dyn Regex` with [`alloc::Rc`].
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let regex = |ctx: &mut CharsCtx| ctx.try_mat(&core::cell::RefCell::new("rust"));
///     let regex = regex::Adapter::rc(regex);
///
///     assert_eq!(CharsCtx::new("rust 2024?").ctor(&regex)?, "rust");
///
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Rc<dyn Regex<C> + 'a>> {
    pub fn dyn_rc(regex: impl Regex<C> + 'a) -> Self {
        Self::new(alloc::Rc::new(regex))
    }
}

#[cfg(feature = "alloc")]
impl<'a, C> Adapter<C, alloc::Rc<dyn Regex<C> + Send + 'a>> {
    pub fn dyn_rc_send(regex: impl Regex<C> + Send + 'a) -> Self {
        Self::new(alloc::Rc::new(regex))
    }
}

impl<C, I> Regex<C> for Adapter<C, I>
where
    I: Regex<C>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner().try_parse(ctx)
    }
}

impl<'a, C, O, H, I> Ctor<'a, C, O, H> for Adapter<C, I>
where
    I: Regex<C>,
    C: Match<'a>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

/// [`RefAdapter`] implement [`Ctor`] for reference of [`Regex`]
#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RefAdapter<'a, C, T: ?Sized> {
    inner: &'a T,
    marker: PhantomData<C>,
}

impl<'a, C, T: ?Sized> Clone for RefAdapter<'a, C, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            marker: self.marker,
        }
    }
}

impl<'a, C, T: ?Sized> RefAdapter<'a, C, T> {
    pub const fn new(inner: &'a T) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<'a, C, T: ?Sized> Regex<C> for RefAdapter<'a, C, T>
where
    T: Regex<C>,
{
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'c, 'a, C, O, T, H> Ctor<'c, C, O, H> for RefAdapter<'a, C, T>
where
    C: Match<'c>,
    T: Regex<C> + ?Sized,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        handler.invoke(ctx, &ret).map_err(Into::into)
    }
}

#[cfg(feature = "alloc")]
mod box_adapter {

    use crate::alloc::Box;
    use crate::ctor::Ctor;
    use crate::ctor::Handler;
    use crate::ctx::Match;
    use crate::err::Error;
    use crate::regex::Regex;
    use crate::span::Span;
    use core::marker::PhantomData;

    /// [`BoxAdapter`] implement [`Ctor`] for boxed [`Regex`]
    #[derive(Debug)]
    pub struct BoxAdapter<C, T> {
        inner: Box<T>,
        marker: PhantomData<C>,
    }

    impl<C, T> Clone for BoxAdapter<C, T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
                marker: self.marker,
            }
        }
    }

    impl<T: Default, C> Default for BoxAdapter<C, T> {
        fn default() -> Self {
            Self {
                inner: Default::default(),
                marker: Default::default(),
            }
        }
    }

    impl<C, T> BoxAdapter<C, T> {
        pub fn new(inner: T) -> Self {
            Self {
                inner: Box::new(inner),
                marker: PhantomData,
            }
        }
    }

    impl<C, T> Regex<C> for BoxAdapter<C, T>
    where
        T: Regex<C>,
    {
        fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
            self.inner.try_parse(ctx)
        }
    }

    impl<'a, C, O, T, H> Ctor<'a, C, O, H> for BoxAdapter<C, T>
    where
        T: Regex<C>,
        C: Match<'a>,
        H: Handler<C, Out = O>,
    {
        fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
            let ret = ctx.try_mat(self)?;

            handler.invoke(ctx, &ret).map_err(Into::into)
        }
    }
}

#[cfg(feature = "alloc")]
pub use box_adapter::*;
