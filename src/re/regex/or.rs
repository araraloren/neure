use std::marker::PhantomData;

use crate::ctx::Context;
use crate::ctx::CtxGuard;
use crate::ctx::Policy;
use crate::err::Error;
use crate::re::Regex;
use crate::trace_log;

///
/// Match `L` or `R`.
///
/// # Ctor
///
/// When using with [`ctor`](crate::ctx::RegexCtx::ctor),
/// it will return result of either `L` or `R`.
///
/// # Example
///
/// ```
/// # use neure::{prelude::*, re::map::from_str_radix};
/// #
/// # fn main() -> color_eyre::Result<()> {
///     color_eyre::install()?;
///
///     macro_rules! num {
///         ($s:literal) => {
///             neu::digit($s).repeat_one_more().map(from_str_radix($s))
///         };
///     }
///
///     let (bin, oct, dec, hex) = (num!(2), num!(8), num!(10), num!(16));
///     let num = dec.ltm(hex);
///     let dec = "0d".then(dec)._1();
///     let oct = "0o".then(oct)._1();
///     let hex = "0x".then(hex)._1();
///     let bin = "0b".then(bin)._1();
///     let pos = "+".map(|_| Ok(1));
///     let neg = "-".map(|_| Ok(-1));
///     let sign = pos.or(neg.or(re::null().map(|_| Ok(1))));
///     let num = bin.or(oct.or(dec.or(hex))).or(num);
///     let num = sign.then(num).map(|(s, v): (_, i64)| Ok(s * v));
///     let val = num.sep(",".ws()).quote("[", "]");
///     let mut ctx = CharsCtx::new(r#"[0d18, 0o17, 0x18, 0b1010, 18, 1E]"#);
///
///     assert_eq!(ctx.ctor(&val)?, [18, 15, 24, 10, 18, 30]);
///     Ok(())
/// # }
/// ```
#[derive(Debug, Default, Copy)]
pub struct RegexOr<C, L, R> {
    left: L,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, R> Clone for RegexOr<C, L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, R> RegexOr<C, L, R> {
    pub fn new(pat1: L, pat2: R) -> Self {
        Self {
            left: pat1,
            right: pat2,
            marker: PhantomData,
        }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.right
    }

    pub fn set_left(&mut self, left: L) -> &mut Self {
        self.left = left;
        self
    }

    pub fn set_right(&mut self, right: R) -> &mut Self {
        self.right = right;
        self
    }
}

impl<'a, C, L, R, O> Regex<C> for RegexOr<C, L, R>
where
    L: Regex<C, Ret = O>,
    R: Regex<C, Ret = O>,
    C: Context<'a> + Policy<C>,
{
    type Ret = L::Ret;

    fn try_parse(&self, ctx: &mut C) -> Result<Self::Ret, Error> {
        let mut g = CtxGuard::new(ctx);

        trace_log!("(`or`: @{}) => try match left", g.beg());
        g.try_mat(&self.left).or_else(|_| {
            trace_log!("(`or`: @{}) => try match right", g.beg());
            g.try_mat(&self.right)
        })
    }
}
