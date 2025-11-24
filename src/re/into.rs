use crate::ctx::Context;
use crate::ctx::Match;
use crate::re::BoxedRegex;
use crate::re::Regex;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use super::ctor::BoxedCtor;
use super::ctor::DynamicArcCtor;
use super::ctor::DynamicBoxedCtor;
use super::ctor::DynamicRcCtor;
use super::Ctor;
use super::DynamicArcRegex;
use super::DynamicBoxedCtorSync;
use super::DynamicBoxedRegex;
use super::DynamicRcRegex;
use super::Wrapped;

pub trait RegexIntoOp
where
    Self: Sized,
{
    fn into_box_regex(self) -> Wrapped<BoxedRegex<Self>>;

    fn into_rc_regex(self) -> Wrapped<Rc<Self>>;

    fn into_arc_regex(self) -> Wrapped<Arc<Self>>;

    fn into_cell_regex(self) -> Wrapped<Cell<Self>>;

    fn into_refcell_regex(self) -> Wrapped<RefCell<Self>>;

    fn into_mutex_regex(self) -> Wrapped<Mutex<Self>>;

    fn into_dyn_regex<'a, 'b, C>(self) -> Wrapped<DynamicBoxedRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b;

    fn into_dyn_arc_regex<'a, 'b, C>(self) -> Wrapped<DynamicArcRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b;

    fn into_dyn_rc_regex<'a, 'b, C>(self) -> Wrapped<DynamicRcRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b;
}

impl<T> RegexIntoOp for T {
    fn into_box_regex(self) -> Wrapped<BoxedRegex<Self>> {
        Wrapped {
            value: BoxedRegex::new(self),
        }
    }

    fn into_rc_regex(self) -> Wrapped<Rc<Self>> {
        Wrapped::new(Rc::new(self))
    }

    fn into_arc_regex(self) -> Wrapped<Arc<Self>> {
        Wrapped::new(Arc::new(self))
    }

    fn into_cell_regex(self) -> Wrapped<Cell<Self>> {
        Wrapped::new(Cell::new(self))
    }

    fn into_refcell_regex(self) -> Wrapped<RefCell<Self>> {
        Wrapped::new(RefCell::new(self))
    }

    fn into_mutex_regex(self) -> Wrapped<Mutex<Self>> {
        Wrapped::new(Mutex::new(self))
    }

    fn into_dyn_regex<'a, 'b, C>(self) -> Wrapped<DynamicBoxedRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b,
    {
        Wrapped {
            value: DynamicBoxedRegex::new(self),
        }
    }

    fn into_dyn_arc_regex<'a, 'b, C>(self) -> Wrapped<DynamicArcRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b,
    {
        Wrapped {
            value: DynamicArcRegex::new(self),
        }
    }

    fn into_dyn_rc_regex<'a, 'b, C>(self) -> Wrapped<DynamicRcRegex<'b, C>>
    where
        C: Context<'a>,
        Self: Regex<C> + 'b,
    {
        Wrapped {
            value: DynamicRcRegex::new(self),
        }
    }
}

pub trait ConstructIntoOp
where
    Self: Sized,
{
    fn into_box(self) -> Wrapped<BoxedCtor<Self>>;

    fn into_rc(self) -> Wrapped<Rc<Self>>;

    fn into_arc(self) -> Wrapped<Arc<Self>>;

    fn into_cell(self) -> Wrapped<Cell<Self>>;

    fn into_refcell(self) -> Wrapped<RefCell<Self>>;

    fn into_mutex(self) -> Wrapped<Mutex<Self>>;

    fn into_dyn<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b;

    fn into_dyn_sync<'a, 'b, C, M, O, H, A>(
        self,
    ) -> Wrapped<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + Send + 'b;

    fn into_dyn_arc<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b;

    fn into_dyn_rc<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b;
}

impl<T> ConstructIntoOp for T
where
    Self: Sized,
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
    ///         .then(u8::is_ascii_hexdigit.repeat_times::<3>())
    ///         .pat()
    ///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .into_box();
    ///
    ///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, "+AE00");
    ///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
    ///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, "-83FD");
    ///     Ok(())
    /// # }
    /// ```
    fn into_box(self) -> Wrapped<BoxedCtor<Self>> {
        Wrapped::new(Box::new(self))
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
    ///     let year = char::is_ascii_digit.repeat_times::<4>();
    ///     let num = char::is_ascii_digit.repeat_times::<2>();
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
    fn into_rc(self) -> Wrapped<Rc<Self>> {
        Wrapped::new(Rc::new(self))
    }

    fn into_arc(self) -> Wrapped<Arc<Self>> {
        Wrapped::new(Arc::new(self))
    }

    fn into_cell(self) -> Wrapped<Cell<Self>> {
        Wrapped::new(Cell::new(self))
    }

    fn into_refcell(self) -> Wrapped<RefCell<Self>> {
        Wrapped::new(RefCell::new(self))
    }

    fn into_mutex(self) -> Wrapped<Mutex<Self>> {
        Wrapped::new(Mutex::new(self))
    }

    /// # Example 2
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    ///     color_eyre::install()?;
    ///     let num = u8::is_ascii_digit
    ///         .repeat_one()
    ///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .map(map::from_str::<usize>());
    ///     let num = num.clone().sep_once(b",", num);
    ///     let re = num.into_dyn();
    ///
    ///     assert_eq!(BytesCtx::new(b"3,0").ctor(&re)?, (3, 0));
    ///     assert_eq!(BytesCtx::new(b"2,1").ctor(&re)?, (2, 1));
    ///     assert_eq!(BytesCtx::new(b"0,3").ctor(&re)?, (0, 3));
    ///     Ok(())
    /// # }
    /// ```
    fn into_dyn<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b,
    {
        Wrapped {
            value: DynamicBoxedCtor::new(self),
        }
    }

    fn into_dyn_sync<'a, 'b, C, M, O, H, A>(
        self,
    ) -> Wrapped<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + Send + 'b,
    {
        Wrapped {
            value: DynamicBoxedCtorSync::new(self),
        }
    }

    fn into_dyn_arc<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b,
    {
        Wrapped {
            value: DynamicArcCtor::new(self),
        }
    }

    fn into_dyn_rc<'a, 'b, C, M, O, H, A>(self) -> Wrapped<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    where
        C: Context<'a> + Match<C>,
        Self: Ctor<'a, C, M, O, H, A> + 'b,
    {
        Wrapped {
            value: DynamicRcCtor::new(self),
        }
    }
}
