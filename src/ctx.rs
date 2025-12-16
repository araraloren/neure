mod guard;
mod policy;
#[allow(clippy::module_inception)]
mod regex;
mod span;

use crate::MayDebug;
use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctor::extract;
use crate::err::Error;
use crate::map::MapSingle;
use crate::regex::Regex;

pub use self::guard::CtxGuard;
pub use self::policy::PolicyCtx;
pub use self::regex::RegexCtx;
pub use self::span::Span;

pub type BytesCtx<'a> = RegexCtx<'a, [u8]>;
pub type CharsCtx<'a> = RegexCtx<'a, str>;

pub trait Context<'a> {
    type Orig<'b>;

    type Item: MayDebug;

    type Iter<'b>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'b;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    fn orig(&self) -> Result<Self::Orig<'a>, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error>;

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error>;

    fn clone_at(&self, offset: usize) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Match<'a>: Context<'a>
where
    Self: Sized,
{
    fn is_mat<Pat>(&mut self, pat: &Pat) -> bool
    where
        Pat: Regex<Self> + ?Sized,
    {
        self.try_mat(pat).is_ok()
    }

    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

pub trait PolicyMatch<'a>
where
    Self: Sized,
{
    fn try_mat_before<P, B>(&mut self, pat: &P, before: &B) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, before, &|_: &mut Self| Ok(Span::default()))
    }

    fn try_mat_after<P, A>(&mut self, pat: &P, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized,
    {
        self.try_mat_policy(pat, &|_: &mut Self| Ok(Span::default()), after)
    }

    fn try_mat_policy<P, B, A>(&mut self, pat: &P, before: &B, after: &A) -> Result<Span, Error>
    where
        P: Regex<Self> + ?Sized,
        B: Regex<Self> + ?Sized,
        A: Regex<Self> + ?Sized;
}

pub trait Assert<'a>
where
    Self: Sized,
{
    /// Silent assertion that swallows all errors and returns `false` on failure
    /// without altering context position.
    fn assert<Pat>(&mut self, pat: &Pat) -> bool
    where
        Pat: Regex<Self> + ?Sized,
    {
        self.try_assert(pat).unwrap_or_default()
    }

    /// Precise assertion preserving error details without altering context position.
    fn try_assert<Pat>(&mut self, pat: &Pat) -> Result<bool, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

impl<'a, T> Assert<'a> for T
where
    T: Match<'a>,
{
    fn try_assert<Pat>(&mut self, pat: &Pat) -> Result<bool, Error>
    where
        Pat: Regex<Self> + ?Sized,
    {
        let mut ctx = CtxGuard::new(self);
        let ret = ctx.try_mat(pat);

        ctx.reset();
        Ok(ret.is_ok())
    }
}

pub trait MatchExt<'a>
where
    Self: Context<'a> + Sized,
{
    fn ctor_handler<H, P, M, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O, H>,
        H: Handler<Self, Out = M>,
    {
        pat.construct(self, &mut handler)
    }

    fn ctor_with<H, P, M, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O, H>,
        H: FnMut(&Self, &Span) -> Result<M, Error>,
    {
        self.ctor_handler(pat, handler)
    }

    fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O, Extract<Span>>,
        Extract<Span>: Handler<Self, Out = Span>,
    {
        self.ctor_handler(pat, extract())
    }

    fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Self::Orig<'a>, O, Extract<Self::Orig<'a>>>,
        Extract<Self::Orig<'a>>: Handler<Self, Out = Self::Orig<'a>>,
    {
        self.ctor_handler(pat, extract())
    }

    fn map_handler<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<Self, Out = O>;

    fn map_with<H, P, O>(&mut self, pat: &P, handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: FnMut(&Self, &Span) -> Result<O, Error>,
    {
        self.map_handler(pat, handler)
    }

    fn map_span<P, O, M>(&mut self, pat: &P, mapper: M) -> Result<O, Error>
    where
        P: Regex<Self>,
        M: MapSingle<Span, O>,
    {
        mapper.map_to(self.map_handler(pat, extract::<Span>())?)
    }

    fn map<P, O, M>(&mut self, pat: &P, mapper: M) -> Result<O, Error>
    where
        P: Regex<Self>,
        M: MapSingle<Self::Orig<'a>, O>,
    {
        mapper.map_to(self.map_handler(pat, |ctx: &Self, span: &Span| {
            ctx.orig_sub(span.beg, span.len)
        })?)
    }
}

impl<'a, C> MatchExt<'a> for C
where
    C: Sized + Match<'a>,
{
    fn map_handler<H, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self>,
        H: Handler<Self, Out = O>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(self, &ret).map_err(Into::into)
    }
}

pub trait MatchMulti<'a>: Sized + Match<'a> {
    fn find_all<P>(&mut self, pat: P) -> impl Iterator<Item = Span>
    where
        P: Regex<Self>,
    {
        self.find_all_with(pat, |ctx, _| {
            ctx.inc(1);
        })
    }

    fn match_seq<P>(&mut self, pat: P) -> impl Iterator<Item = Span>
    where
        P: Regex<Self>,
    {
        self.find_all_with(pat, |ctx, _| {
            ctx.set_offset(ctx.len());
        })
    }

    fn find_all_with<P, F>(&mut self, pat: P, mut err_handler: F) -> impl Iterator<Item = Span>
    where
        P: Regex<Self>,
        F: FnMut(&mut Self, Error),
    {
        std::iter::from_fn(move || {
            let mut next = None;

            while self.offset() < self.len() {
                match self.try_mat(&pat) {
                    Ok(span) => {
                        next = Some(span);
                        break;
                    }
                    Err(e) => {
                        err_handler(self, e);
                    }
                }
            }
            next
        })
    }
}

impl<'a, T> MatchMulti<'a> for T where Self: Sized + Match<'a> {}
