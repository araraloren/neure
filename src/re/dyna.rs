use crate::re::Regex;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub trait IntoNonDynamic {
    fn into_box<C>(self) -> Box<Self>
    where
        Self: Regex<C> + 'static;

    fn into_arc<C>(self) -> Arc<Self>
    where
        Self: Regex<C> + 'static;

    fn into_rc<C>(self) -> Rc<Self>
    where
        Self: Regex<C> + 'static;

    fn into_cell<C>(self) -> Cell<Self>
    where
        Self: Regex<C> + 'static;

    fn into_refcell<C>(self) -> RefCell<Self>
    where
        Self: Regex<C> + 'static;

    fn into_mutex<C>(self) -> Mutex<Self>
    where
        Self: Regex<C> + 'static;
}

impl<P> IntoNonDynamic for P {
    fn into_box<C>(self) -> Box<Self>
    where
        Self: Regex<C> + 'static,
    {
        Box::new(self)
    }

    fn into_arc<C>(self) -> Arc<Self>
    where
        Self: Regex<C> + 'static,
    {
        Arc::new(self)
    }

    fn into_rc<C>(self) -> Rc<Self>
    where
        Self: Regex<C> + 'static,
    {
        Rc::new(self)
    }

    fn into_cell<C>(self) -> Cell<Self>
    where
        Self: Regex<C> + 'static,
    {
        Cell::new(self)
    }

    fn into_refcell<C>(self) -> RefCell<Self>
    where
        Self: Regex<C> + 'static,
    {
        RefCell::new(self)
    }

    fn into_mutex<C>(self) -> Mutex<Self>
    where
        Self: Regex<C> + 'static,
    {
        Mutex::new(self)
    }
}

pub trait IntoDynamic {
    fn into_dyn_box<C>(self) -> Box<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static;

    fn into_dyn_arc<C>(self) -> Arc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static;

    fn into_dyn_rc<C>(self) -> Rc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static;
}

impl<P> IntoDynamic for P {
    fn into_dyn_box<C>(self) -> Box<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static,
    {
        Box::new(self)
    }

    fn into_dyn_arc<C>(self) -> Arc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static,
    {
        Arc::new(self)
    }

    fn into_dyn_rc<C>(self) -> Rc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret>>
    where
        Self: Regex<C> + 'static,
    {
        todo!()
    }
}
