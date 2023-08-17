use crate::{Context, MatchPolicy, Parser, SpanStore};

use super::mat::MatThen;

#[derive(Debug)]
pub struct CapThen<'a, C, R> {
    start: usize,
    ret: R,
    ctx: &'a mut C,
}

impl<'a, C, R> CapThen<'a, C, R> {
    pub fn new(ctx: &'a mut C, start: usize, ret: R) -> Self {
        Self { ctx, start, ret }
    }
}
