use core::marker::PhantomData;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::err::Error;
use crate::span::Span;

#[derive(Debug, Copy)]
pub struct Extract<T> {
    marker: PhantomData<T>,
}

impl<T> Default for Extract<T> {
    fn default() -> Self {
        Self {
            marker: Default::default(),
        }
    }
}

impl<T> Clone for Extract<T> {
    fn clone(&self) -> Self {
        Self {
            marker: self.marker,
        }
    }
}

impl<T> Extract<T> {
    pub const fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

pub const fn extract<T>() -> Extract<T> {
    Extract::new()
}

impl<'a, C: Context<'a, Orig<'a> = &'a str>> Handler<C> for Extract<&'a str> {
    type Out = &'a str;

    type Error = Error;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        ctx.orig_sub(span.beg(), span.len())
    }
}

impl<'a, C: Context<'a, Orig<'a> = &'a [u8]>> Handler<C> for Extract<&'a [u8]> {
    type Out = &'a [u8];

    type Error = Error;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        ctx.orig_sub(span.beg(), span.len())
    }
}

#[cfg(feature = "alloc")]
impl<'a, C: Context<'a, Orig<'a> = &'a str>> Handler<C> for Extract<crate::alloc::String> {
    type Out = crate::alloc::String;

    type Error = Error;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        Ok(crate::alloc::String::from(
            ctx.orig_sub(span.beg(), span.len())?,
        ))
    }
}

impl<'a, C: Context<'a>> Handler<C> for Extract<Span> {
    type Out = Span;

    type Error = Error;

    fn invoke(&mut self, _: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        Ok(*span)
    }
}

impl<'a, C: Context<'a>> Handler<C> for Extract<()> {
    type Out = ();

    type Error = Error;

    fn invoke(&mut self, _: &C, _: &Span) -> Result<Self::Out, Self::Error> {
        Ok(())
    }
}
