use core::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::Match;
use crate::err::Error;
use crate::regex::Regex;
use crate::span::Span;

/// A guard that temporarily holds a mutable reference to a [`Context`] and automatically
/// rolls back its internal offset on error.
#[derive(Debug)]
pub struct CtxGuard<'a, 'b, C>
where
    C: Context<'b>,
{
    ctx: &'a mut C,

    offset: usize,

    reset: bool,

    marker: PhantomData<&'b ()>,
}

impl<'a, 'b, C> CtxGuard<'a, 'b, C>
where
    C: Context<'b>,
{
    pub fn new(ctx: &'a mut C) -> Self {
        let offset = ctx.offset();

        Self {
            ctx,
            offset,
            reset: false,
            marker: PhantomData,
        }
    }

    /// Returns the offset at which this guard was created (the backtrack point).
    pub fn beg(&self) -> usize {
        self.offset
    }

    /// Returns the current offset of the underlying context (may have advanced).
    pub fn end(&self) -> usize {
        self.ctx.offset()
    }

    /// Returns the total length of the context (equivalent to `ctx.len()`).
    pub fn len(&self) -> usize {
        self.ctx.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ctx.is_empty()
    }

    pub fn ctx(&mut self) -> &mut C {
        self.ctx
    }

    /// Advances the context by `len` bytes and returns the corresponding [`Span`].
    pub fn inc(&mut self, len: usize) -> Span {
        let span = Span::new(self.beg(), len);

        self.ctx.inc(len);
        span
    }

    /// Peeks at the current position without consuming input.
    pub fn peek(&self) -> Result<<C as Context<'b>>::Iter<'b>, Error> {
        self.ctx.peek()
    }

    /// Peeks at a specific offset without consuming input.
    pub fn peek_at(&self, offset: usize) -> Result<<C as Context<'b>>::Iter<'b>, Error> {
        self.ctx.peek_at(offset)
    }

    /// Manually triggers a rollback: resets the context's offset to the guard's initial position.
    pub fn reset(&mut self) -> &mut Self {
        self.ctx.set_offset(self.offset);
        self
    }

    /// Processes a [`Result`]: if it is an `Err`, marks the guard for rollback.
    pub fn process_ret<R>(&mut self, ret: Result<R, Error>) -> Result<R, Error> {
        if ret.is_err() {
            self.reset = true;
        }
        ret
    }
}

impl<'a, C> CtxGuard<'_, 'a, C>
where
    C: Match<'a>,
{
    /// Attempts to match the given regex pattern against the context.
    ///
    /// On failure, automatically sets the `reset` flag so the context is rolled back on drop.
    pub fn try_mat<P: Regex<C> + ?Sized>(&mut self, pattern: &P) -> Result<Span, Error> {
        self.ctx.try_mat(pattern).inspect_err(|_| {
            self.reset = true;
        })
    }
}

impl<'b, C> Drop for CtxGuard<'_, 'b, C>
where
    C: Context<'b>,
{
    /// On drop, if an error occurred (`reset == true`), restores the context's offset
    /// to the value recorded at guard creation—enabling transparent backtracking.
    fn drop(&mut self) {
        if self.reset {
            self.ctx.set_offset(self.offset);
        }
    }
}
