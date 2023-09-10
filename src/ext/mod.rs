mod guard;
mod handler;
mod quote;
mod term;
mod then;

pub use self::guard::CtxGuard;
pub use self::handler::*;
pub use self::quote::LazyQuote;
pub use self::quote::NonLazyQuote;
pub use self::term::LazyTerm;
pub use self::term::NonLazyTerm;
pub use self::term::NonLazyTermIter;
pub use self::then::LazyPattern;
pub use self::then::LazyPatternValue;
pub use self::then::NonLazyPattern;
pub use self::then::NonLazyPatternValue;

use crate::ctx::Context;
use crate::ctx::Pattern;
use crate::ctx::Policy;
use crate::ctx::True;
use crate::err::Error;

pub trait LazyCtxExtension<'a, C: Context<'a> + Policy<C>> {
    fn quote<L, R>(&mut self, left: L, right: R) -> LazyQuote<'_, C, L, R>
    where
        L: Pattern<C, Ret = <C as Policy<C>>::Ret>,
        R: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn pat<P>(&mut self, pattern: P) -> LazyPattern<'_, C, P, True<C>, True<C>>
    where
        P: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn term<S>(&mut self, sep: S) -> LazyTerm<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    fn term_opt<S>(&mut self, sep: S, optional: bool) -> LazyTerm<'_, C, S, True<C>, True<C>>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone;
}

pub trait NonLazyCtxExtension<'a, C: Context<'a> + Policy<C>> {
    fn quote<L, R>(&mut self, left: L, right: R) -> Result<NonLazyQuote<'_, C, R>, Error>
    where
        L: Pattern<C, Ret = <C as Policy<C>>::Ret>,
        R: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn pat<P>(&mut self, pattern: P) -> Result<NonLazyPattern<'_, C, True<C>>, Error>
    where
        P: Pattern<C, Ret = <C as Policy<C>>::Ret>;

    fn term<S>(&mut self, sep: S) -> Result<NonLazyTerm<'_, C, S, True<C>>, Error>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone,
    {
        self.term_opt(sep, true)
    }

    fn term_opt<S>(
        &mut self,
        sep: S,
        optional: bool,
    ) -> Result<NonLazyTerm<'_, C, S, True<C>>, Error>
    where
        S: Pattern<C, Ret = <C as Policy<C>>::Ret> + Clone;
}
