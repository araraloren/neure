use core::cell::Cell;
use core::cell::RefCell;

use crate::ctor::Adapter;
use crate::regex::Regex;

#[cfg(feature = "alloc")]
use crate::ctor::adapter::BoxAdapter;

#[cfg(feature = "alloc")]
use crate::ctor::Ctor;

#[cfg(feature = "alloc")]
use crate::alloc;

pub trait CtorIntoHelper<C>
where
    Self: Sized,
{
    #[cfg(feature = "alloc")]
    fn into_box(self) -> Adapter<C, BoxAdapter<C, Self>>;

    #[cfg(feature = "alloc")]
    fn into_rc(self) -> Adapter<C, alloc::Rc<Self>>;

    #[cfg(feature = "alloc")]
    fn into_arc(self) -> Adapter<C, alloc::Arc<Self>>;

    fn into_cell(self) -> Adapter<C, Cell<Self>>;

    fn into_refcell(self) -> Adapter<C, RefCell<Self>>;

    #[cfg(feature = "std")]
    fn into_mutex(self) -> Adapter<C, crate::std::Mutex<Self>>;

    #[cfg(feature = "alloc")]
    fn into_dyn<'a, 'b, O, H>(self) -> Adapter<C, alloc::Box<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b;

    #[cfg(feature = "alloc")]
    fn into_dyn_arc<'a, 'b, O, H>(self) -> Adapter<C, alloc::Arc<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b;

    #[cfg(feature = "alloc")]
    fn into_dyn_rc<'a, 'b, O, H>(self) -> Adapter<C, alloc::Rc<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b;
}

impl<C, T> CtorIntoHelper<C> for T
where
    Self: Sized + Regex<C>,
{
    ///
    /// Return a type that wraps `Ctor` with Box.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #   color_eyre::install()?;
    ///     let re = b'+'
    ///         .or(b'-')
    ///         .then(u8::is_ascii_hexdigit)
    ///         .then(u8::is_ascii_hexdigit.count::<3>())
    ///         .pat()
    ///         .try_map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .into_box();
    ///
    ///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, "+AE00");
    ///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
    ///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, "-83FD");
    ///     Ok(())
    /// # }
    /// ```
    #[cfg(feature = "alloc")]
    fn into_box(self) -> Adapter<C, BoxAdapter<C, Self>> {
        Adapter::r#box(self)
    }

    ///
    /// Return a type that wrap `Ctor` with `Rc`.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let year = char::is_ascii_digit.count::<4>();
    ///     let num = char::is_ascii_digit.count::<2>();
    ///     let date = year.sep_once("-", num.sep_once("-", num)).into_rc();
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
    #[cfg(feature = "alloc")]
    fn into_rc(self) -> Adapter<C, alloc::Rc<Self>> {
        Adapter::rc(self)
    }

    #[cfg(feature = "alloc")]
    fn into_arc(self) -> Adapter<C, alloc::Arc<Self>> {
        Adapter::arc(self)
    }

    fn into_cell(self) -> Adapter<C, Cell<Self>> {
        Adapter::cell(self)
    }

    fn into_refcell(self) -> Adapter<C, RefCell<Self>> {
        Adapter::refcell(self)
    }

    #[cfg(feature = "std")]
    fn into_mutex(self) -> Adapter<C, crate::std::Mutex<Self>> {
        Adapter::mutex(self)
    }

    /// # Example 2
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let num = u8::is_ascii_digit
    ///         .once()
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
    #[cfg(feature = "alloc")]
    fn into_dyn<'a, 'b, O, H>(self) -> Adapter<C, alloc::Box<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b,
    {
        Adapter::dyn_box(self)
    }

    #[cfg(feature = "alloc")]
    fn into_dyn_arc<'a, 'b, O, H>(self) -> Adapter<C, alloc::Arc<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b,
    {
        Adapter::dyn_arc(self)
    }

    #[cfg(feature = "alloc")]
    fn into_dyn_rc<'a, 'b, O, H>(self) -> Adapter<C, alloc::Rc<dyn Ctor<'a, C, O, H> + 'b>>
    where
        Self: Ctor<'a, C, O, H> + 'b,
    {
        Adapter::dyn_rc(self)
    }
}
