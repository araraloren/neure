use crate::ctx::Context;
use crate::ctx::Match;
use crate::re::Regex;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

// use super::ctor::BoxedCtor;
// use super::ctor::DynamicArcCtor;
// use super::ctor::DynamicBoxedCtor;
// use super::ctor::DynamicRcCtor;
use super::Ctor;

use crate::re::regex::Wrap;

pub trait RegexIntoOp
where
    Self: Sized,
{
    // fn into_box_regex(self) -> Wrap<BoxedRegex<Self>>;

    // fn into_rc_regex(self) -> Wrap<Rc<Self>>;

    // fn into_arc_regex(self) -> Wrap<Arc<Self>>;

    // fn into_cell_regex(self) -> Wrap<Cell<Self>>;

    // fn into_refcell_regex(self) -> Wrap<RefCell<Self>>;

    // fn into_mutex_regex(self) -> Wrap<Mutex<Self>>;

    // fn into_dyn_regex<'a, 'b, C>(self) -> Wrap<DynamicBoxedRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b;

    // fn into_dyn_arc_regex<'a, 'b, C>(self) -> Wrap<DynamicArcRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b;

    // fn into_dyn_rc_regex<'a, 'b, C>(self) -> Wrap<DynamicRcRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b;
}

impl<T> RegexIntoOp for T {
    // fn into_box_regex(self) -> Wrap<BoxedRegex<Self>> {
    //     Wrap {
    //         value: BoxedRegex::new(self),
    //     }
    // }

    // fn into_rc_regex(self) -> Wrap<Rc<Self>> {
    //     Wrap::new(Rc::new(self))
    // }

    // fn into_arc_regex(self) -> Wrap<Arc<Self>> {
    //     Wrap::new(Arc::new(self))
    // }

    // fn into_cell_regex(self) -> Wrap<Cell<Self>> {
    //     Wrap::new(Cell::new(self))
    // }

    // fn into_refcell_regex(self) -> Wrap<RefCell<Self>> {
    //     Wrap::new(RefCell::new(self))
    // }

    // fn into_mutex_regex(self) -> Wrap<Mutex<Self>> {
    //     Wrap::new(Mutex::new(self))
    // }

    // fn into_dyn_regex<'a, 'b, C>(self) -> Wrap<DynamicBoxedRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b,
    // {
    //     Wrap {
    //         value: DynamicBoxedRegex::new(self),
    //     }
    // }

    // fn into_dyn_arc_regex<'a, 'b, C>(self) -> Wrap<DynamicArcRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b,
    // {
    //     Wrap {
    //         value: DynamicArcRegex::new(self),
    //     }
    // }

    // fn into_dyn_rc_regex<'a, 'b, C>(self) -> Wrap<DynamicRcRegex<'b, C>>
    // where
    //     C: Context<'a>,
    //     Self: Regex<C> + 'b,
    // {
    //     Wrap {
    //         value: DynamicRcRegex::new(self),
    //     }
    // }
}

pub trait ConstructIntoOp
where
    Self: Sized,
{
    // fn into_box(self) -> Wrap<BoxedCtor<Self>>;

    // fn into_rc(self) -> Wrap<Rc<Self>>;

    // fn into_arc(self) -> Wrap<Arc<Self>>;

    // fn into_cell(self) -> Wrap<Cell<Self>>;

    // fn into_refcell(self) -> Wrap<RefCell<Self>>;

    // fn into_mutex(self) -> Wrap<Mutex<Self>>;

    // fn into_dyn<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b;

    // fn into_dyn_sync<'a, 'b, C, M, O, H, A>(
    //     self,
    // ) -> Wrap<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + Send + 'b;

    // fn into_dyn_arc<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b;

    // fn into_dyn_rc<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b;
}

impl<T> ConstructIntoOp for T
where
    Self: Sized,
{
    // fn into_box(self) -> Wrap<BoxedCtor<Self>> {
    //     Wrap::new(Box::new(self))
    // }

    // fn into_rc(self) -> Wrap<Rc<Self>> {
    //     Wrap::new(Rc::new(self))
    // }

    // fn into_arc(self) -> Wrap<Arc<Self>> {
    //     Wrap::new(Arc::new(self))
    // }

    // fn into_cell(self) -> Wrap<Cell<Self>> {
    //     Wrap::new(Cell::new(self))
    // }

    // fn into_refcell(self) -> Wrap<RefCell<Self>> {
    //     Wrap::new(RefCell::new(self))
    // }

    // fn into_mutex(self) -> Wrap<Mutex<Self>> {
    //     Wrap::new(Mutex::new(self))
    // }

    // fn into_dyn<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicBoxedCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b,
    // {
    //     Wrap {
    //         value: DynamicBoxedCtor::new(self),
    //     }
    // }

    // fn into_dyn_sync<'a, 'b, C, M, O, H, A>(
    //     self,
    // ) -> Wrap<DynamicBoxedCtorSync<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + Send + 'b,
    // {
    //     Wrap {
    //         value: DynamicBoxedCtorSync::new(self),
    //     }
    // }

    // fn into_dyn_arc<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicArcCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b,
    // {
    //     Wrap {
    //         value: DynamicArcCtor::new(self),
    //     }
    // }

    // fn into_dyn_rc<'a, 'b, C, M, O, H, A>(self) -> Wrap<DynamicRcCtor<'a, 'b, C, M, O, H, A>>
    // where
    //     C: Context<'a> + Match<C>,
    //     Self: Ctor<'a, C, M, O, H, A> + 'b,
    // {
    //     Wrap {
    //         value: DynamicRcCtor::new(self),
    //     }
    // }
}
