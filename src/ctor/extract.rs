use std::marker::PhantomData;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Span;
use crate::err::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct Extract<T> {
    marker: PhantomData<T>,
}

impl<T> Extract<T> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

pub fn extract<T>() -> Extract<T> {
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

impl<'a, C: Context<'a, Orig<'a> = &'a str>> Handler<C> for Extract<String> {
    type Out = String;

    type Error = Error;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        Ok(String::from(ctx.orig_sub(span.beg(), span.len())?))
    }
}

impl<'a, C: Context<'a>> Handler<C> for Extract<Span> {
    type Out = Span;

    type Error = Error;

    fn invoke(&mut self, _: &C, span: &Span) -> Result<Self::Out, Self::Error> {
        Ok(*span)
    }
}
