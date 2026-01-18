#[cfg(feature = "alloc")]
use crate::ctx::Context;

use crate::regex::Adapter;
use crate::regex::Regex;

#[cfg(feature = "alloc")]
use crate::regex::adapter::BoxAdapter;

use core::cell::Cell;
use core::cell::RefCell;

#[cfg(feature = "alloc")]
use crate::alloc;

pub trait RegexIntoHelper<C>
where
    Self: Sized + Regex<C>,
{
    fn into_ctor(self) -> Adapter<C, Self>;

    #[cfg(feature = "alloc")]
    fn into_box_regex(self) -> Adapter<C, BoxAdapter<C, Self>>;

    #[cfg(feature = "alloc")]
    fn into_rc_regex(self) -> Adapter<C, alloc::Rc<Self>>;

    #[cfg(feature = "alloc")]
    fn into_arc_regex(self) -> Adapter<C, alloc::Arc<Self>>;

    fn into_cell_regex(self) -> Adapter<C, Cell<Self>>;

    fn into_refcell_regex(self) -> Adapter<C, RefCell<Self>>;

    #[cfg(feature = "std")]
    fn into_mutex_regex(self) -> Adapter<C, crate::std::Mutex<Self>>;

    #[cfg(feature = "alloc")]
    fn into_dyn_regex<'a, 'b>(self) -> Adapter<C, alloc::Box<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b;

    #[cfg(feature = "alloc")]
    fn into_dyn_arc_regex<'a, 'b>(self) -> Adapter<C, alloc::Arc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b;

    #[cfg(feature = "alloc")]
    fn into_dyn_rc_regex<'a, 'b>(self) -> Adapter<C, alloc::Rc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b;
}

impl<C, T> RegexIntoHelper<C> for T
where
    T: Regex<C>,
{
    fn into_ctor(self) -> Adapter<C, Self> {
        Adapter::new(self)
    }

    #[cfg(feature = "alloc")]
    fn into_box_regex(self) -> Adapter<C, BoxAdapter<C, Self>> {
        Adapter::r#box(self)
    }

    #[cfg(feature = "alloc")]
    fn into_rc_regex(self) -> Adapter<C, alloc::Rc<Self>> {
        Adapter::rc(self)
    }

    #[cfg(feature = "alloc")]
    fn into_arc_regex(self) -> Adapter<C, alloc::Arc<Self>> {
        Adapter::arc(self)
    }

    fn into_cell_regex(self) -> Adapter<C, Cell<Self>> {
        Adapter::cell(self)
    }

    fn into_refcell_regex(self) -> Adapter<C, RefCell<Self>> {
        Adapter::refcell(self)
    }

    #[cfg(feature = "std")]
    fn into_mutex_regex(self) -> Adapter<C, crate::std::Mutex<Self>> {
        Adapter::mutex(self)
    }

    #[cfg(feature = "alloc")]
    fn into_dyn_regex<'a, 'b>(self) -> Adapter<C, alloc::Box<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_box(self)
    }

    #[cfg(feature = "alloc")]
    fn into_dyn_arc_regex<'a, 'b>(self) -> Adapter<C, alloc::Arc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_arc(self)
    }

    #[cfg(feature = "alloc")]
    fn into_dyn_rc_regex<'a, 'b>(self) -> Adapter<C, alloc::Rc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_rc(self)
    }
}
