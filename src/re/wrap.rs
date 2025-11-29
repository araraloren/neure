use crate::re::{Ctor, Regex};

pub trait Wrappable
where
    Self: Sized,
{
    type Inner;

    fn wrap(inner: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;

    fn inner_mut(&mut self) -> &mut Self::Inner;
}

impl<T> Wrappable for Box<T> {
    type Inner = T;

    fn wrap(inner: Self::Inner) -> Self {
        Self::new(inner)
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut *self
    }
}

macro_rules! def_wrap_for {
    ($ty:path) => {
        impl<T> Wrappable for $ty {
            type Inner = $ty;

            fn wrap(inner: Self::Inner) -> Self {
                inner
            }

            fn inner(&self) -> &Self::Inner {
                self
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                self
            }
        }
    };
}

def_wrap_for!(std::rc::Rc<T>);

def_wrap_for!(std::cell::Cell<T>);

def_wrap_for!(std::cell::RefCell<T>);

def_wrap_for!(std::sync::Arc<T>);

def_wrap_for!(std::sync::Mutex<T>);

impl<'a, C> Wrappable for Box<dyn Regex<C> + 'a> {
    type Inner = Box<dyn Regex<C> + 'a>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, C> Wrappable for std::sync::Arc<dyn Regex<C> + 'a> {
    type Inner = std::sync::Arc<dyn Regex<C> + 'a>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, C> Wrappable for std::rc::Rc<dyn Regex<C> + 'a> {
    type Inner = std::rc::Rc<dyn Regex<C> + 'a>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for Box<dyn Ctor<'a, C, M, O, H, A> + 'b> {
    type Inner = Box<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b> {
    type Inner = Box<dyn Ctor<'a, C, M, O, H, A> + Send + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for std::rc::Rc<dyn Ctor<'a, C, M, O, H, A> + 'b> {
    type Inner = std::rc::Rc<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}

impl<'a, 'b, C, M, O, H, A> Wrappable for std::sync::Arc<dyn Ctor<'a, C, M, O, H, A> + 'b> {
    type Inner = std::sync::Arc<dyn Ctor<'a, C, M, O, H, A> + 'b>;

    fn wrap(inner: Self::Inner) -> Self {
        inner
    }

    fn inner(&self) -> &Self::Inner {
        self
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self
    }
}
