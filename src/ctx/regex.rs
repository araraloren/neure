use std::str::CharIndices;

use super::Context;
use super::PolicyCtx;
use super::PolicyMatch;
use super::Regex;
use super::Span;

use crate::ctx::Match;
use crate::err::Error;
use crate::iter::BytesIndices;
use crate::neu::Neu2Re;
use crate::span::SimpleStorer;

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

    pub fn with<F, R>(dat: &'a T, mut func: F) -> R
    where
        F: FnMut(Self) -> R,
    {
        let ctx = Self::new(dat);

        func(ctx)
    }
}

impl<T> RegexCtx<'_, T>
where
    T: ?Sized,
{
    ///
    /// Setting a policy(which implemented [`BPolicy`]) will invoked before any match occurs.
    ///
    /// # Example
    ///
    /// ```
    /// # use neure::ctx::CtxGuard;
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub struct Dat<'a> {
    ///         span: Span,
    ///
    ///         dat: &'a str,
    ///     }
    ///
    ///     // a sample data from https://adventofcode.com/2015/day/7
    ///     const DATA: &str = r#"
    ///      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    ///      XXXOOOOOXXXOXXXXOXXXXOOOOOXXXOOOOOXXX
    ///      XXXOXXXOXXXOXXXXOXXXXOXXXXXXXXXOXXXXX
    ///      XXXOOOOOXXXOXXXXOXXXXOOOOOXXXXXOXXXXX
    ///      XXXOXOXXXXXOXXXXOXXXXXXXXOXXXXXOXXXXX
    ///      XXXOXXOXXXXOXXXXOXXXXXXXXOXXXXXOXXXXX
    ///      XXXOXXXOXXXOOOOOOXXXXOOOOOXXXXXOXXXXX
    ///      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
    ///      "#;
    ///
    ///     // match "\n" or anything not 'X'
    ///     let text = "\n".or('X'.not().repeat_one_more());
    ///     let mut ctx = CharsCtx::new(DATA).skip_before(|ctx: &mut CharsCtx| {
    ///         let mut g = CtxGuard::new(ctx);
    ///         let ret = g.try_mat(&'X'.repeat_full());
    ///
    ///         g.process_ret(ret)
    ///     });
    ///
    ///     let texts: Vec<Dat> = ctx.ctor_with(&text.repeat(1..), |ctx, span| {
    ///         Ok(Dat {
    ///             span: *span,
    ///             dat: span.orig(ctx)?,
    ///         })
    ///     })?;
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
    ///
    /// #   Ok(())
    /// # }
    /// ```
    pub fn skip_before<R>(self, regex: R) -> PolicyCtx<Self, R> {
        PolicyCtx { inner: self, regex }
    }
}

impl<'a> RegexCtx<'a, [u8]> {
    pub fn skip_ascii_whitespace(
        self,
    ) -> PolicyCtx<
        Self,
        crate::neu::NeureRepeat<
            0,
            { usize::MAX },
            Self,
            crate::neu::AsciiWhiteSpace<u8>,
            crate::neu::NullCond,
        >,
    > {
        self.skip_before(crate::neu::ascii_whitespace().repeat_full())
    }
}

impl<'a> RegexCtx<'a, str> {
    ///
    /// Match the given `regex` before any match.
    ///
    /// # Example
    ///
    /// A simple example ignore all the whitespace before any match.
    ///
    /// ```
    /// # use neure::prelude::*;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     tracing_subscriber::fmt::fmt()
    ///         .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    ///         .init();
    ///
    ///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    ///     pub struct Dat<'a> {
    ///         span: Span,
    ///
    ///         dat: &'a str,
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
    ///      "#;
    ///
    ///     let sig = neu::digit(10).repeat_one_more().map(Op::Sig);
    ///     let wire = neu::ascii_lowercase().repeat_one_more();
    ///     let op = sig.or(wire.map(Op::Wire));
    ///     let and = op.sep_once("AND", op).map(Inst::And);
    ///     let or = op.sep_once("OR", op).map(Inst::Or);
    ///     let lshift = op.sep_once("LSHIFT", op).map(Inst::LShift);
    ///     let rshift = op.sep_once("RSHIFT", op).map(Inst::RShift);
    ///     let not = op.prefix("NOT").map(Inst::Not);
    ///     let store = sig.map(Inst::Store);
    ///     let src = and
    ///         .or(or.or(lshift.or(rshift.or(not.or(store)))))
    ///         .into_box();
    ///     let parser = src.sep_once("->", wire).collect::<_, Vec<_>>();
    ///
    ///     // ignore white space using re_policy
    ///     let mut ctx = CharsCtx::new(DATA).skip_ascii_whitespace();
    ///
    ///     let insts: Vec<_> = ctx.ctor_with(&parser, |ctx, span| {
    ///         Ok(Dat {
    ///             span: *span,
    ///             dat: span.orig(ctx)?,
    ///         })
    ///     })?;
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
    ///
    /// #   Ok(())
    /// # }
    /// ```
    pub fn skip_ascii_whitespace(
        self,
    ) -> PolicyCtx<
        Self,
        crate::neu::NeureRepeat<
            0,
            { usize::MAX },
            Self,
            crate::neu::AsciiWhiteSpace<char>,
            crate::neu::NullCond,
        >,
    > {
        self.skip_before(crate::neu::ascii_whitespace().repeat_full())
    }
}

impl<'a> RegexCtx<'a, str> {
    pub fn skip_whitespace(
        self,
    ) -> PolicyCtx<
        Self,
        crate::neu::NeureRepeat<
            0,
            { usize::MAX },
            Self,
            crate::neu::WhiteSpace,
            crate::neu::NullCond,
        >,
    > {
        self.skip_before(crate::neu::whitespace().repeat_full())
    }
}

impl<'a> Context<'a> for RegexCtx<'a, [u8]> {
    type Orig<'b> = &'b [u8];

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
        crate::neure_debug!("RegexCtx: set offset => {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        crate::neure_debug!("RegexCtx: + {} => {}", offset, self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        crate::neure_debug!("RegexCtx: - {} => {}", offset, self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat.get(offset..).ok_or(Error::OutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(BytesIndices::new(self.orig_at(offset)?))
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OutOfBound)
    }

    fn clone_at(&self, offset: usize) -> Result<Self, Error> {
        self.orig_at(offset).map(RegexCtx::new)
    }
}

impl<'a> Context<'a> for RegexCtx<'a, str> {
    type Orig<'b> = &'b str;

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
        crate::neure_debug!("RegexCtx: set offset = {}", self.offset);
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        crate::neure_debug!("RegexCtx: + {} => {}", offset, self.offset);
        self
    }

    fn dec(&mut self, offset: usize) -> &mut Self {
        self.offset -= offset;
        crate::neure_debug!("RegexCtx: - {} => {}", offset, self.offset);
        self
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat.get(offset..).ok_or(Error::OutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OutOfBound)
    }

    fn clone_at(&self, offset: usize) -> Result<Self, Error> {
        self.orig_at(offset).map(RegexCtx::new)
    }
}

impl<'a, T> Match<'a> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<RegexCtx<'a, T>> + ?Sized,
    {
        self.try_mat_before(pat, &|_: &mut Self| Ok(Span::default()))
    }
}

impl<'a, T> PolicyMatch<'a> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat_policy<P, B, A>(&mut self, pat: &P, before: &B, after: &A) -> Result<Span, Error>
    where
        P: Regex<RegexCtx<'a, T>> + ?Sized,
        B: Regex<RegexCtx<'a, T>> + ?Sized,
        A: Regex<RegexCtx<'a, T>> + ?Sized,
    {
        before.try_parse(self)?;
        let ret = pat.try_parse(self)?;

        after.try_parse(self)?;
        Ok(ret)
    }
}
