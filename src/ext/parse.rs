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
    fn pattern<M, O>(self) -> Pattern<Self, M, O>;

    fn map<F, M, O, V>(self, f: F) -> Map<Self, F, M, O, V>;

    fn quote<L, R, M, O>(self, left: L, right: R) -> Quote<Self, L, R, M, O>;

    fn terminated<S, M, O>(self, sep: S) -> Terminated<Self, S, M, O>;

    fn or<P2, M, O>(self, pat: P2) -> Or<Self, P2, M, O>;

    fn or_map<P2, F, M, O, V>(self, pat: P2, func: F) -> OrMap<Self, P2, F, M, O, V>;

    fn then<T, M, O>(self, then: T) -> Then<Self, T, M, O>;
}

impl<'a, C, P> ParseExtension<'a, C> for P
where
    P: Parse<C>,
    C: Context<'a>,
{
    fn pattern<M, O>(self) -> Pattern<Self, M, O> {
        Pattern::new(self)
    }

    fn map<F, M, O, V>(self, func: F) -> Map<Self, F, M, O, V> {
        Map::new(self, func)
    }

    fn quote<L, R, M, O>(self, left: L, right: R) -> Quote<Self, L, R, M, O> {
        Quote::new(self, left, right)
    }

    fn terminated<S, M, O>(self, sep: S) -> Terminated<Self, S, M, O> {
        Terminated::new(self, sep)
    }

    fn or<P2, M, O>(self, pat: P2) -> Or<Self, P2, M, O> {
        Or::new(self, pat)
    }

    fn or_map<P2, F, M, O, V>(self, pat: P2, func: F) -> OrMap<Self, P2, F, M, O, V> {
        OrMap::new(self, pat, func)
    }

    fn then<T, M, O>(self, then: T) -> Then<Self, T, M, O> {
        Then::new(self, then)
    }
}
