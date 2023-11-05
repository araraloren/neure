mod r#box;
mod r#dyn;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::r#box::BoxedInvoke;
pub use self::r#dyn::DynamicInvoke;
pub use self::r#dyn::DynamicInvokeHandler;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub trait Invoke<'a, C, M, O>
where
    C: Context<'a>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

impl<'a, C, O, F> Invoke<'a, C, O, O> for F
where
    C: Context<'a> + Policy<C>,
    F: Fn(&mut C) -> Result<Span, Error>,
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

impl<'a, C, O> Invoke<'a, C, O, O> for Box<dyn Regex<C, Ret = Span>>
where
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

impl<'a, C, O> Invoke<'a, C, O, O> for &str
where
    C: Context<'a, Orig = str> + Policy<C>,
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

impl<'a, C, O> Invoke<'a, C, O, O> for &[u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
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

impl<'a, const N: usize, C, O> Invoke<'a, C, O, O> for &[u8; N]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
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

impl<'a, C, M, O, I> Invoke<'a, C, M, O> for RefCell<I>
where
    I: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Invoke::invoke(&*self.borrow(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Invoke<'a, C, M, O> for Cell<I>
where
    I: Invoke<'a, C, M, O> + Copy,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Invoke::invoke(&self.get(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Invoke<'a, C, M, O> for Mutex<I>
where
    I: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = self.lock().expect("Oops ?! Can not unwrap mutex ...");

        Invoke::invoke(&*ret, ctx, handler)
    }
}

impl<'a, C, M, O, I> Invoke<'a, C, M, O> for Arc<I>
where
    I: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Invoke::invoke(self.as_ref(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Invoke<'a, C, M, O> for Rc<I>
where
    I: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn invoke<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Invoke::invoke(self.as_ref(), ctx, handler)
    }
}

pub fn dynamic_invoke<'a, 'b, C, O>(
    invoke: impl Fn(&mut C) -> Result<O, Error> + 'b,
) -> DynamicInvoke<'b, C, O>
where
    C: Context<'a>,
{
    DynamicInvoke::new(Box::new(invoke))
}

pub fn boxed_invoke<'a, C, M, O, I>(invoke: I) -> BoxedInvoke<C, I>
where
    I: Invoke<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    BoxedInvoke::new(Box::new(invoke))
}
