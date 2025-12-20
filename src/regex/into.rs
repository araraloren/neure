use crate::ctx::Context;
use crate::regex::Adapter;
use crate::regex::Regex;
use crate::regex::adapter::BoxAdapter;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub trait RegexIntoHelper<C>
where
    Self: Sized + Regex<C>,
{
    fn into_ctor(self) -> Adapter<C, Self>;

    fn into_box_regex(self) -> Adapter<C, BoxAdapter<C, Self>>;

    fn into_rc_regex(self) -> Adapter<C, Rc<Self>>;

    fn into_arc_regex(self) -> Adapter<C, Arc<Self>>;

    fn into_cell_regex(self) -> Adapter<C, Cell<Self>>;

    fn into_refcell_regex(self) -> Adapter<C, RefCell<Self>>;

    fn into_mutex_regex(self) -> Adapter<C, Mutex<Self>>;

    fn into_dyn_regex<'a, 'b>(self) -> Adapter<C, Box<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b;

    fn into_dyn_arc_regex<'a, 'b>(self) -> Adapter<C, std::sync::Arc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b;

    fn into_dyn_rc_regex<'a, 'b>(self) -> Adapter<C, std::rc::Rc<dyn Regex<C> + 'b>>
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

    fn into_box_regex(self) -> Adapter<C, BoxAdapter<C, Self>> {
        Adapter::r#box(self)
    }

    fn into_rc_regex(self) -> Adapter<C, Rc<Self>> {
        Adapter::rc(self)
    }

    fn into_arc_regex(self) -> Adapter<C, Arc<Self>> {
        Adapter::arc(self)
    }

    fn into_cell_regex(self) -> Adapter<C, Cell<Self>> {
        Adapter::cell(self)
    }

    fn into_refcell_regex(self) -> Adapter<C, RefCell<Self>> {
        Adapter::refcell(self)
    }

    fn into_mutex_regex(self) -> Adapter<C, Mutex<Self>> {
        Adapter::mutex(self)
    }

    fn into_dyn_regex<'a, 'b>(self) -> Adapter<C, Box<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_box(self)
    }

    fn into_dyn_arc_regex<'a, 'b>(self) -> Adapter<C, std::sync::Arc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_arc(self)
    }

    fn into_dyn_rc_regex<'a, 'b>(self) -> Adapter<C, std::rc::Rc<dyn Regex<C> + 'b>>
    where
        C: Context<'a>,
        Self: 'b,
    {
        Adapter::dyn_rc(self)
    }
}
