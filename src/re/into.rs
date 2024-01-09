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
use super::WrappedTy;

pub trait RegexIntoOp<'a, C>
where
    C: Context<'a>,
    Self: Sized,
{
    fn into_box_regex(self) -> WrappedTy<BoxedRegex<C, Self>>;

    fn into_rc_regex(self) -> WrappedTy<Rc<Self>>;

    fn into_arc_regex(self) -> WrappedTy<Arc<Self>>;

    fn into_cell_regex(self) -> WrappedTy<Cell<Self>>;

    fn into_refcell_regex(self) -> WrappedTy<RefCell<Self>>;

    fn into_mutex_regex(self) -> WrappedTy<Mutex<Self>>;

    fn into_dyn_box_regex<'b>(self) -> WrappedTy<DynamicBoxedRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b;

    fn into_dyn_arc_regex<'b>(self) -> WrappedTy<DynamicArcRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b;

    fn into_dyn_rc_regex<'b>(self) -> WrappedTy<DynamicRcRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b;
}

impl<'a, C, T> RegexIntoOp<'a, C> for T
where
    T: Regex<C>,
    C: Context<'a>,
{
    fn into_box_regex(self) -> WrappedTy<BoxedRegex<C, Self>> {
        WrappedTy::new(self)
    }

    fn into_rc_regex(self) -> WrappedTy<Rc<Self>> {
        WrappedTy::new(Rc::new(self))
    }

    fn into_arc_regex(self) -> WrappedTy<Arc<Self>> {
        WrappedTy::new(Arc::new(self))
    }

    fn into_cell_regex(self) -> WrappedTy<Cell<Self>> {
        WrappedTy::new(Cell::new(self))
    }

    fn into_refcell_regex(self) -> WrappedTy<RefCell<Self>> {
        WrappedTy::new(RefCell::new(self))
    }

    fn into_mutex_regex(self) -> WrappedTy<Mutex<Self>> {
        WrappedTy::new(Mutex::new(self))
    }

    fn into_dyn_box_regex<'b>(self) -> WrappedTy<DynamicBoxedRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b,
    {
        WrappedTy {
            value: DynamicBoxedRegex::new(self),
        }
    }

    fn into_dyn_arc_regex<'b>(self) -> WrappedTy<DynamicArcRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b,
    {
        WrappedTy {
            value: DynamicArcRegex::new(self),
        }
    }

    fn into_dyn_rc_regex<'b>(self) -> WrappedTy<DynamicRcRegex<'b, C, <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'b,
    {
        WrappedTy {
            value: DynamicRcRegex::new(self),
        }
    }
}

pub trait ConstructIntoOp<'a, C, M, O, H, A>
where
    C: Context<'a> + Match<C>,
    Self: Sized,
{
    fn into_box(self) -> WrappedTy<BoxedCtor<C, Self>>;

    fn into_rc(self) -> WrappedTy<Rc<Self>>;

    fn into_arc(self) -> WrappedTy<Arc<Self>>;

    fn into_cell(self) -> WrappedTy<Cell<Self>>;

    fn into_refcell(self) -> WrappedTy<RefCell<Self>>;

    fn into_mutex(self) -> WrappedTy<Mutex<Self>>;

    fn into_dyn_box<'b>(self) -> WrappedTy<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b;

    fn into_dyn_box_sync<'b>(self) -> WrappedTy<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    where
        Self: Send + 'b;

    fn into_dyn_arc<'b>(self) -> WrappedTy<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b;

    fn into_dyn_rc<'b>(self) -> WrappedTy<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b;
}

impl<'a, C, M, O, H, A, T> ConstructIntoOp<'a, C, M, O, H, A> for T
where
    C: Context<'a> + Match<C>,
    T: Ctor<'a, C, M, O, H, A>,
{
    ///
    /// Return a type that wraps `Ctor` with Box.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::{err::Error, prelude::*, re::BoxedCtorHelper};
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #     color_eyre::install()?;
    ///     let re = b'+'
    ///         .or(b'-')
    ///         .then(u8::is_ascii_hexdigit)
    ///         .then(u8::is_ascii_hexdigit.repeat_times::<3>())
    ///         .pat()
    ///         .map(|v: &[u8]| String::from_utf8(v.to_vec()).map_err(|_| Error::Uid(0)))
    ///         .into_boxed_ctor();
    ///
    ///     assert_eq!(BytesCtx::new(b"+AE00").ctor(&re)?, "+AE00");
    ///     assert!(BytesCtx::new(b"-GH66").ctor(&re).is_err());
    ///     assert_eq!(BytesCtx::new(b"-83FD").ctor(&re)?, "-83FD");
    ///     Ok(())
    /// # }
    /// ```
    fn into_box(self) -> WrappedTy<BoxedCtor<C, Self>> {
        WrappedTy::new(self)
    }

    fn into_rc(self) -> WrappedTy<Rc<Self>> {
        WrappedTy::new(Rc::new(self))
    }

    fn into_arc(self) -> WrappedTy<Arc<Self>> {
        WrappedTy::new(Arc::new(self))
    }

    fn into_cell(self) -> WrappedTy<Cell<Self>> {
        WrappedTy::new(Cell::new(self))
    }

    fn into_refcell(self) -> WrappedTy<RefCell<Self>> {
        WrappedTy::new(RefCell::new(self))
    }

    fn into_mutex(self) -> WrappedTy<Mutex<Self>> {
        WrappedTy::new(Mutex::new(self))
    }

    fn into_dyn_box<'b>(self) -> WrappedTy<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b,
    {
        WrappedTy {
            value: DynamicBoxedCtor::new(self),
        }
    }

    fn into_dyn_box_sync<'b>(self) -> WrappedTy<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    where
        Self: Send + 'b,
    {
        WrappedTy {
            value: DynamicBoxedCtorSync::new(self),
        }
    }

    fn into_dyn_arc<'b>(self) -> WrappedTy<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b,
    {
        WrappedTy {
            value: DynamicArcCtor::new(self),
        }
    }

    fn into_dyn_rc<'b>(self) -> WrappedTy<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    where
        Self: 'b,
    {
        WrappedTy {
            value: DynamicRcCtor::new(self),
        }
    }
}
