use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Invoke;
use crate::re::Regex;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub trait RegexIntoOp<'a, C>
where
    C: Context<'a>,
    Self: Sized,
{
    fn into_box(self) -> BoxedRegex<C, Self>;

    fn into_rc(self) -> Rc<Self>;

    fn into_arc(self) -> Arc<Self>;

    fn into_cell(self) -> Cell<Self>;

    fn into_refcell(self) -> RefCell<Self>;

    fn into_mutex(self) -> Mutex<Self>;

    fn into_dyn_box<'b>(self) -> Box<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b;

    fn into_dyn_arc<'b>(self) -> Arc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b;

    fn into_dyn_rc<'b>(self) -> Rc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b;
}

impl<'a, C, T> RegexIntoOp<'a, C> for T
where
    T: Regex<C>,
    C: Context<'a>,
{
    fn into_box(self) -> BoxedRegex<C, Self> {
        BoxedRegex::new(Box::new(self))
    }

    fn into_rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    fn into_cell(self) -> Cell<Self> {
        Cell::new(self)
    }

    fn into_refcell(self) -> RefCell<Self> {
        RefCell::new(self)
    }

    fn into_mutex(self) -> Mutex<Self> {
        Mutex::new(self)
    }

    fn into_dyn_box<'b>(self) -> Box<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b,
    {
        Box::new(self)
    }

    fn into_dyn_arc<'b>(self) -> Arc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b,
    {
        Arc::new(self)
    }

    fn into_dyn_rc<'b>(self) -> Rc<dyn Regex<C, Ret = <Self as Regex<C>>::Ret> + 'b>
    where
        Self: Regex<C> + 'b,
    {
        Rc::new(self)
    }
}

use std::marker::PhantomData;

// into_box
#[derive(Debug, Clone)]
pub struct BoxedRegex<C, T> {
    inner: Box<T>,
    marker: PhantomData<C>,
}

impl<C, T> BoxedRegex<C, T> {
    pub fn new(inner: Box<T>) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn with_inner(mut self, inner: Box<T>) -> Self {
        self.inner = inner;
        self
    }

    pub fn inner(&self) -> &Box<T> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Box<T> {
        &mut self.inner
    }

    pub fn set_inner(&mut self, inner: Box<T>) -> &mut Self {
        self.inner = inner;
        self
    }
}

impl<'a, C, T> Regex<C> for BoxedRegex<C, T>
where
    T: Regex<C>,
{
    type Ret = <T as Regex<C>>::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        self.inner.try_parse(ctx)
    }
}

impl<'a, C, O, T> Invoke<'a, C, O, O> for BoxedRegex<C, T>
where
    T: Regex<C, Ret = Span>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}
