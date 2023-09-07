mod guard;
mod quote;
mod term;
mod then;

use crate::ctx::Context;
use crate::ctx::Parser;
use crate::ctx::Pattern;
use crate::ctx::True;
use crate::policy::Policy;

pub use self::guard::CtxGuard;
pub use self::quote::Quote;
pub use self::term::Term;
pub use self::then::MatThenValue;
pub use self::then::Then;

pub trait Extension<'a, C: Context<'a> + Policy<C>> {
    fn quote<L, R>(&mut self, left: L, right: R) -> Quote<'_, C, L, R>
    where
        L: Pattern<C, Ret = C::Ret>,
        R: Pattern<C, Ret = C::Ret>;

    fn mat<P>(&mut self, parser: P) -> Then<'_, C, P, True<C>, True<C>>
    where
        P: Pattern<C, Ret = C::Ret>;

    fn term<S>(&mut self, sep: S) -> Term<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = C::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    fn term_opt<S>(&mut self, sep: S, optional: bool) -> Term<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = C::Ret> + Clone;
}

impl<'a, T> Extension<'a, Self> for Parser<'a, T>
where
    Self: Context<'a>,
{
    fn quote<L, R>(&mut self, left: L, right: R) -> Quote<'_, Self, L, R>
    where
        L: Pattern<Self, Ret = <Self as Policy<Self>>::Ret>,
        R: Pattern<Self, Ret = <Self as Policy<Self>>::Ret>,
    {
        Quote::new(self, left, right)
    }

    fn mat<P>(&mut self, parser: P) -> Then<'_, Self, P, True<Self>, True<Self>>
    where
        P: Pattern<Self, Ret = <Self as Policy<Self>>::Ret>,
    {
        Then::new(self, True::default(), True::default(), parser)
    }

    fn term_opt<S>(&mut self, sep: S, optional: bool) -> Term<'_, Self, S, True<Self>, True<Self>>
    where
        S: Pattern<Self, Ret = <Self as Policy<Self>>::Ret> + Clone,
    {
        Term::new(
            self,
            Some(True::default()),
            Some(True::default()),
            sep,
            optional,
        )
    }
}

// todo! Clean the code of extension

// add support multi arguments closure to map
