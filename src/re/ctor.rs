mod r#box;
mod r#dyn;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub use self::r#box::BoxedCtor;
pub use self::r#dyn::DynamicCtor;
pub use self::r#dyn::DynamicCtorHandler;

use crate::ctx::Context;
use crate::ctx::Policy;
use crate::ctx::Span;
use crate::err::Error;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Regex;

pub trait Ctor<'a, C, M, O>
where
    C: Context<'a>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>;
}

impl<'a, C, O, F> Ctor<'a, C, O, O> for F
where
    C: Context<'a> + Policy<C>,
    F: Fn(&mut C) -> Result<Span, Error>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for Box<dyn Regex<C, Ret = Span>>
where
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &str
where
    C: Context<'a, Orig = str> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, O> Ctor<'a, C, O, O> for &[u8]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, const N: usize, C, O> Ctor<'a, C, O, O> for &[u8; N]
where
    C: Context<'a, Orig = [u8]> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = ctx.try_mat(self)?;

        handler.invoke(A::extract(ctx, &ret)?)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Option<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref().ok_or(Error::RegexOption)?, ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for RefCell<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(&*self.borrow(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Cell<I>
where
    I: Ctor<'a, C, M, O> + Copy,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(&self.get(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Mutex<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        let ret = self.lock().expect("Oops ?! Can not unwrap mutex ...");

        Ctor::constrct(&*ret, ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Arc<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref(), ctx, handler)
    }
}

impl<'a, C, M, O, I> Ctor<'a, C, M, O> for Rc<I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    fn constrct<H, A>(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>
    where
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, C, Span, Out<'a> = A, Error = Error>,
    {
        Ctor::constrct(self.as_ref(), ctx, handler)
    }
}

pub fn into_dyn_ctor<'a, 'b, C, O>(
    invoke: impl Fn(&mut C) -> Result<O, Error> + 'b,
) -> DynamicCtor<'b, C, O>
where
    C: Context<'a>,
{
    DynamicCtor::new(Box::new(invoke))
}

pub fn into_box_ctor<'a, C, M, O, I>(invoke: I) -> BoxedCtor<C, I>
where
    I: Ctor<'a, C, M, O>,
    C: Context<'a> + Policy<C>,
{
    BoxedCtor::new(Box::new(invoke))
}

pub type RecursiveRegex<'a, C, O> = Rc<RefCell<Option<DynamicCtor<'a, C, O>>>>;

pub type RecursiveRegexSync<'a, C, O> = Arc<Mutex<Option<DynamicCtor<'a, C, O>>>>;

pub fn rec_parser<'a, 'b, C, O, I>(
    handler: impl Fn(RecursiveRegex<'b, C, O>) -> I,
) -> RecursiveRegex<'b, C, O>
where
    C: Context<'a>,
    I: Fn(&mut C) -> Result<O, Error> + 'b,
{
    let r_regex: RecursiveRegex<'b, C, O> = Rc::new(RefCell::new(None));
    let r_regex_clone = r_regex.clone();
    let regex = handler(r_regex_clone);

    *r_regex.borrow_mut() = Some(into_dyn_ctor(regex));
    r_regex
}

pub fn rec_parser_sync<'a, 'b, C, O, I>(
    handler: impl Fn(RecursiveRegexSync<'b, C, O>) -> I,
) -> RecursiveRegexSync<'b, C, O>
where
    C: Context<'a>,
    I: Fn(&mut C) -> Result<O, Error> + 'b,
{
    let r_regex: RecursiveRegexSync<'b, C, O> = Arc::new(Mutex::new(None));
    let r_regex_clone = r_regex.clone();
    let regex = handler(r_regex_clone);

    *r_regex.lock().unwrap() = Some(into_dyn_ctor(regex));
    r_regex
}