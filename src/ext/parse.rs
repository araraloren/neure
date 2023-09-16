use super::Map;
use super::Or;
use super::OrMap;
use super::Pattern;
use super::Quote;
use super::Terminated;
use super::Then;

use crate::ctx::Context;
use crate::ctx::Parse;

pub trait ParseExtension<'a, C>
where
    Self: Sized,
    C: Context<'a>,
{
    fn pattern(self) -> Pattern<Self>;

    fn map<F, O>(self, f: F) -> Map<Self, F, O>;

    fn quote<L, R>(self, left: L, right: R) -> Quote<Self, L, R>;

    fn terminated<S>(self, sep: S) -> Terminated<Self, S>;

    fn or<P>(self, pat: P) -> Or<Self, P>;

    fn or_map<P, F, O>(self, pat: P, func: F) -> OrMap<Self, P, F, O>;

    fn then<T>(self, then: T) -> Then<Self, T>;
}

impl<'a, C, T> ParseExtension<'a, C> for T
where
    T: Parse<C>,
    C: Context<'a>,
{
    fn pattern(self) -> Pattern<Self> {
        Pattern::new(self)
    }

    fn map<F, O>(self, func: F) -> Map<Self, F, O> {
        Map::new(self, func)
    }

    fn quote<L, R>(self, left: L, right: R) -> Quote<Self, L, R> {
        Quote::new(self, left, right)
    }

    fn terminated<S>(self, sep: S) -> Terminated<Self, S> {
        Terminated::new(self, sep)
    }

    fn or<P>(self, pat: P) -> Or<Self, P> {
        Or::new(self, pat)
    }

    fn or_map<P, F, O>(self, pat: P, func: F) -> OrMap<Self, P, F, O> {
        OrMap::new(self, pat, func)
    }

    fn then<P>(self, then: P) -> Then<Self, P> {
        Then::new(self, then)
    }
}
