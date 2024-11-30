use std::str::CharIndices;

use super::re_policy;
use super::BPolicy;
use super::Context;
use super::PolicyCtx;
use super::PolicyMatch;
use super::RePolicy;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::iter::BytesIndices;
use crate::map::MapSingle;
use crate::re::Ctor;
use crate::re::Extract;
use crate::re::Handler;
use crate::re::Pass;
use crate::span::SimpleStorer;
use crate::trace_log;

#[derive(Debug)]
pub struct RegexCtx<'a, T>
where
    T: ?Sized,
{
    dat: &'a T,
    offset: usize,
}

impl<T> Clone for RegexCtx<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RegexCtx<'_, T> where T: ?Sized {}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
{
    pub fn new(dat: &'a T) -> Self {
        Self { dat, offset: 0 }
    }

    pub fn dat(&self) -> &'a T {
        self.dat
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn with_dat(mut self, dat: &'a T) -> Self {
        self.dat = dat;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn reset_with(&mut self, dat: &'a T) -> &mut Self {
        self.dat = dat;
        self.offset = 0;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.offset = 0;
        self
    }

    pub fn span_storer(&self, capacity: usize) -> SimpleStorer {
        SimpleStorer::new(capacity)
    }

    ///
    /// Setting a policy(which implemented [`BPolicy`]) will invoked before any match occurs.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::ctx::CtxGuard;
    /// # use neure::prelude::*;
    /// # use neure::re::Extract;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #   color_eyre::install()?;
    ///
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub struct Dat<'a> {
    ///         span: Span,
    ///
    ///         dat: &'a str,
    ///     }
    ///
    ///     impl<'a, C: Context<'a, Orig = str>> Extract<'a, C, Span> for Dat<'a> {
    ///         type Out<'b> = Dat<'b>;
    ///
    ///         type Error = neure::err::Error;
    ///
    ///         fn extract(ctx: &C, ret: &Span) -> std::result::Result<Self::Out<'a>, Self::Error> {
    ///             Ok(Dat {
    ///                 span: *ret,
    ///                 dat: ctx.orig_sub(ret.beg, ret.len)?,
    ///             })
    ///         }
    ///     }
    ///
    ///     // a sample data from https://adventofcode.com/2015/day/7
    ///     const DATA: &str = r#"
    /// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    /// XXXOOOOOXXXOXXXXOXXXXOOOOOXXXOOOOOXXX
    /// XXXOXXXOXXXOXXXXOXXXXOXXXXXXXXXOXXXXX
    /// XXXOOOOOXXXOXXXXOXXXXOOOOOXXXXXOXXXXX
    /// XXXOXOXXXXXOXXXXOXXXXXXXXOXXXXXOXXXXX
    /// XXXOXXOXXXXOXXXXOXXXXXXXXOXXXXXOXXXXX
    /// XXXOXXXOXXXOOOOOOXXXXOOOOOXXXXXOXXXXX
    /// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    /// "#;
    ///
    ///     // match "\n" or anything not 'X'
    ///     let text = "\n".or('X'.not().repeat_one_more());
    ///     let mut ctx = CharsCtx::new(DATA).with_policy(|ctx: &mut CharsCtx| {
    ///         let mut g = CtxGuard::new(ctx);
    ///         let ret = g.try_mat(&'X'.repeat_full());
    ///
    ///         g.process_ret(ret)?;
    ///         Ok(())
    ///     });
    ///
    ///     let texts: Vec<Dat> = ctx.ctor_with(&text.repeat(1..), &mut Ok)?;
    ///     let mut off = 0;
    ///
    ///     // output:
    ///     //    OOOOO   O    O    OOOOO   OOOOO
    ///     //    O   O   O    O    O         O
    ///     //    OOOOO   O    O    OOOOO     O
    ///     //    O O     O    O        O     O
    ///     //    O  O    O    O        O     O
    ///     //    O   O   OOOOOO    OOOOO     O
    ///     for text in texts {
    ///         let Span { beg, len } = text.span;
    ///         let dat = text.dat;
    ///
    ///         if off < beg {
    ///             print!("{}", " ".repeat(beg - off));
    ///             off = beg;
    ///         }
    ///         print!("{}", dat);
    ///         off += len;
    ///     }
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn with_policy<O>(self, before_policy: O) -> PolicyCtx<Self, O>
    where
        O: BPolicy<Self>,
    {
        PolicyCtx {
            inner: self,
            b_policy: before_policy,
        }
    }
}

impl<T> RegexCtx<'_, T>
where
    T: ?Sized,
{
    ///
    /// Match the given `regex` before any match.
    ///
    /// # Example
    ///
    /// A simple example ignore all the whitespace before any match.
    ///
    /// ```
    /// # use neure::prelude::*;
    /// # use neure::re::Extract;
    /// #
    /// # fn main() -> color_eyre::Result<()> {
    /// #   color_eyre::install()?;
    /// #
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub struct Dat<'a> {
    ///         span: Span,
    ///
    ///         dat: &'a str,
    ///     }
    ///
    ///     impl<'a, C: Context<'a, Orig = str>> Extract<'a, C, Span> for Dat<'a> {
    ///         type Out<'b> = Dat<'b>;
    ///
    ///         type Error = neure::err::Error;
    ///
    ///         fn extract(ctx: &C, ret: &Span) -> std::result::Result<Self::Out<'a>, Self::Error> {
    ///             Ok(Dat {
    ///                 span: *ret,
    ///                 dat: ctx.orig_sub(ret.beg, ret.len)?,
    ///             })
    ///         }
    ///     }
    ///
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub enum Op<'a> {
    ///         Sig(Dat<'a>),
    ///
    ///         Wire(Dat<'a>),
    ///     }
    ///
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub enum Inst<'a> {
    ///         Store(Op<'a>),
    ///
    ///         And((Op<'a>, Op<'a>)),
    ///
    ///         Or((Op<'a>, Op<'a>)),
    ///
    ///         LShift((Op<'a>, Op<'a>)),
    ///
    ///         RShift((Op<'a>, Op<'a>)),
    ///
    ///         Not(Op<'a>),
    ///     }
    ///
    ///     // a sample data from https://adventofcode.com/2015/day/7
    ///     const DATA: &str = r#"
    /// 123 -> x
    /// 456 -> y
    /// x AND y -> d
    /// x OR y -> e
    /// x LSHIFT 2 -> f
    /// y RSHIFT 2 -> g
    /// NOT x -> h
    /// NOT y -> i
    /// "#;
    ///
    ///     let sig = neu::digit(10).repeat_one_more().map(|v| Ok(Op::Sig(v)));
    ///     let wire = neu::ascii_lowercase().repeat_one_more();
    ///     let op = sig.or(wire.map(|v| Ok(Op::Wire(v))));
    ///     let and = op.sep_once("AND", op).map(|v| Ok(Inst::And(v)));
    ///     let or = op.sep_once("OR", op).map(|v| Ok(Inst::Or(v)));
    ///     let lshift = op.sep_once("LSHIFT", op).map(|v| Ok(Inst::LShift(v)));
    ///     let rshift = op.sep_once("RSHIFT", op).map(|v| Ok(Inst::RShift(v)));
    ///     let not = op.padded("NOT").map(|v| Ok(Inst::Not(v)));
    ///     let store = sig.map(|v| Ok(Inst::Store(v)));
    ///     let src = and
    ///         .or(or.or(lshift.or(rshift.or(not.or(store)))))
    ///         .into_box();
    ///     let parser = src.sep_once("->", wire).collect::<_, Vec<_>>();
    ///
    ///     // ignore white space using re_policy
    ///     let mut ctx = CharsCtx::new(DATA).ignore(neu::whitespace().repeat_full());
    ///
    ///     let insts: Vec<_> = ctx.ctor_with(&parser, &mut Ok)?;
    ///
    ///     assert_eq!(insts.len(), 8);
    ///     assert_eq!(
    ///         insts[0],
    ///         (
    ///             Inst::Store(Op::Sig(Dat {
    ///                 span: Span::new(1, 3),
    ///                 dat: "123"
    ///             })),
    ///             Dat {
    ///                 span: Span::new(8, 1),
    ///                 dat: "x"
    ///             }
    ///         )
    ///     );
    ///     assert_eq!(
    ///         insts[6],
    ///         (
    ///             Inst::Not(Op::Wire(Dat {
    ///                 span: Span::new(80, 1),
    ///                 dat: "x"
    ///             })),
    ///             Dat {
    ///                 span: Span::new(85, 1),
    ///                 dat: "h"
    ///             }
    ///         )
    ///     );
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn ignore<R>(self, regex: R) -> PolicyCtx<Self, RePolicy<Self, R>> {
        PolicyCtx {
            inner: self,
            b_policy: re_policy(regex),
        }
    }
}

impl<'a> Context<'a> for RegexCtx<'a, [u8]> {
    type Orig = [u8];

    type Item = u8;

    type Iter<'b>
        = BytesIndices<'b, u8>
    where
        Self: 'b;

    fn len(&self) -> usize {
        self.dat.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        trace_log!("set {offset} -> ctx -> {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        trace_log!("inc {offset} -> ctx -> {}", self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        trace_log!("dec {offset} -> ctx -> {}", self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OriginOutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OriginOutOfBound)
    }

    fn clone_with(&self, orig: &'a Self::Orig) -> Self {
        RegexCtx::new(orig)
    }
}

impl<'a> Context<'a> for RegexCtx<'a, str> {
    type Orig = str;

    type Item = char;

    type Iter<'b>
        = CharIndices<'b>
    where
        Self: 'b;

    fn len(&self) -> usize {
        self.dat.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        trace_log!("set {offset} -> ctx -> {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        trace_log!("inc {offset} -> ctx -> {}", self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        trace_log!("dec {offset} -> ctx -> {}", self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<&'a Self::Orig, Error> {
        self.dat.get(offset..).ok_or(Error::OriginOutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<&'a Self::Orig, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OriginOutOfBound)
    }

    fn clone_with(&self, orig: &'a Self::Orig) -> Self {
        RegexCtx::new(orig)
    }
}

impl<'a, T> Match<RegexCtx<'a, T>> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat_t<Pat: Regex<RegexCtx<'a, T>> + ?Sized>(
        &mut self,
        pat: &Pat,
    ) -> Result<Pat::Ret, Error> {
        self.try_mat_policy(pat, &|_: &mut Self| Ok(()))
    }
}

impl<'a, T, B> PolicyMatch<RegexCtx<'a, T>, B> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
    B: BPolicy<RegexCtx<'a, T>>,
{
    fn try_mat_policy<Pat>(&mut self, pat: &Pat, b_policy: &B) -> Result<Pat::Ret, Error>
    where
        Pat: Regex<RegexCtx<'a, T>> + ?Sized,
    {
        b_policy.invoke_policy(self)?;
        pat.try_parse(self)
    }
}

impl<'a, T, R> Extract<'a, Self, R> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    type Out<'b> = RegexCtx<'a, T>;

    type Error = Error;

    fn extract(ctx: &Self, _: &R) -> Result<Self::Out<'a>, Self::Error> {
        Ok(Clone::clone(ctx))
    }
}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    pub fn ctor_with<H, A, P, M, O>(&mut self, pat: &P, handler: &mut H) -> Result<O, Error>
    where
        P: Ctor<'a, Self, M, O, H, A>,
        H: Handler<A, Out = M, Error = Error>,
        A: Extract<'a, Self, Span, Out<'a> = A, Error = Error>,
    {
        pat.construct(self, handler)
    }

    pub fn map_with<H, A, P, O>(&mut self, pat: &P, mut handler: H) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        H: Handler<A, Out = O, Error = Error>,
        A: Extract<'a, Self, P::Ret, Out<'a> = A, Error = Error>,
    {
        let ret = self.try_mat(pat)?;

        handler.invoke(A::extract(self, &ret)?)
    }

    pub fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<
            'a,
            Self,
            &'a <Self as Context<'a>>::Orig,
            O,
            Pass,
            &'a <Self as Context<'a>>::Orig,
        >,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, Span, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error> + 'a,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map<P, O>(
        &mut self,
        pat: &P,
        mapper: impl MapSingle<&'a <Self as Context<'a>>::Orig, O>,
    ) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        &'a <Self as Context<'a>>::Orig:
            Extract<'a, Self, P::Ret, Out<'a> = &'a <Self as Context<'a>>::Orig, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }

    pub fn ctor_span<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, Span, O, Pass, Span>,
        Span: Extract<'a, Self, Span, Out<'a> = Span, Error = Error>,
    {
        self.ctor_with(pat, &mut Pass)
    }

    pub fn map_span<P, O>(&mut self, pat: &P, mapper: impl MapSingle<Span, O>) -> Result<O, Error>
    where
        P: Regex<Self, Ret = Span>,
        Span: Extract<'a, Self, P::Ret, Out<'a> = Span, Error = Error>,
    {
        mapper.map_to(self.map_with(pat, Ok)?)
    }
}
