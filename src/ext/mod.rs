mod collect;
mod extract;
mod guard;
mod invoke;
mod map;
mod or;
mod ormap;
mod parse;
mod pat;
mod quote;
mod repeat;
mod term;
mod then;

pub use self::collect::Collect;
pub use self::extract::*;
pub use self::guard::CtxGuard;
pub use self::invoke::*;
pub use self::map::Map;
pub use self::or::Or;
pub use self::ormap::OrMap;
pub use self::parse::ParseExtension;
pub use self::pat::Pattern;
pub use self::quote::Quote;
pub use self::repeat::Repeat;
pub use self::term::Terminated;
pub use self::then::Then;

use crate::ctx::Parse;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub trait IntoNonDynamic {
    fn into_box<C>(self) -> Box<Self>
    where
        Self: Parse<C> + 'static;

    fn into_arc<C>(self) -> Arc<Self>
    where
        Self: Parse<C> + 'static;

    fn into_rc<C>(self) -> Rc<Self>
    where
        Self: Parse<C> + 'static;

    fn into_cell<C>(self) -> Cell<Self>
    where
        Self: Parse<C> + 'static;

    fn into_refcell<C>(self) -> RefCell<Self>
    where
        Self: Parse<C> + 'static;

    fn into_mutex<C>(self) -> Mutex<Self>
    where
        Self: Parse<C> + 'static;
}

impl<P> IntoNonDynamic for P {
    fn into_box<C>(self) -> Box<Self>
    where
        Self: Parse<C> + 'static,
    {
        Box::new(self)
    }

    fn into_arc<C>(self) -> Arc<Self>
    where
        Self: Parse<C> + 'static,
    {
        Arc::new(self)
    }

    fn into_rc<C>(self) -> Rc<Self>
    where
        Self: Parse<C> + 'static,
    {
        Rc::new(self)
    }

    fn into_cell<C>(self) -> Cell<Self>
    where
        Self: Parse<C> + 'static,
    {
        Cell::new(self)
    }

    fn into_refcell<C>(self) -> RefCell<Self>
    where
        Self: Parse<C> + 'static,
    {
        RefCell::new(self)
    }

    fn into_mutex<C>(self) -> Mutex<Self>
    where
        Self: Parse<C> + 'static,
    {
        Mutex::new(self)
    }
}

pub trait IntoDynamic {
    fn into_dyn_box<C>(self) -> Box<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static;

    fn into_dyn_arc<C>(self) -> Arc<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static;

    fn into_dyn_rc<C>(self) -> Rc<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static;
}

impl<P> IntoDynamic for P {
    fn into_dyn_box<C>(self) -> Box<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static,
    {
        Box::new(self)
    }

    fn into_dyn_arc<C>(self) -> Arc<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static,
    {
        Arc::new(self)
    }

    fn into_dyn_rc<C>(self) -> Rc<dyn Parse<C, Ret = <Self as Parse<C>>::Ret>>
    where
        Self: Parse<C> + 'static,
    {
        todo!()
    }
}
