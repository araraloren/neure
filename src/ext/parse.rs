use super::MapValue;
use super::Pattern;
use super::Quote;
use super::Terminated;

use crate::ctx::Context;
use crate::ctx::Parse;

pub trait ParseExtension<'a, C>
where
    Self: Sized,
    C: Context<'a>,
{
    fn pattern<M, O>(self) -> Pattern<Self, M, O>;

    fn map_value<F, M, O, V>(self, f: F) -> MapValue<Self, F, M, O, V>;

    fn quote<L, R, M, O>(self, left: L, right: R) -> Quote<Self, L, R, M, O>;

    fn terminated<S, M, O>(self, sep: S) -> Terminated<Self, S, M, O>;
}

impl<'a, C, P> ParseExtension<'a, C> for P
where
    P: Parse<C>,
    C: Context<'a>,
{
    fn pattern<M, O>(self) -> Pattern<Self, M, O> {
        Pattern::new(self)
    }

    fn map_value<F, M, O, V>(self, func: F) -> MapValue<Self, F, M, O, V> {
        MapValue::new(self, func)
    }

    fn quote<L, R, M, O>(self, left: L, right: R) -> Quote<Self, L, R, M, O> {
        Quote::new(self, left, right)
    }

    fn terminated<S, M, O>(self, sep: S) -> Terminated<Self, S, M, O> {
        Terminated::new(self, sep)
    }
}
