use crate::{Context, MatchPolicy, err::Error, CharsCtx, neure};

use super::mat::MatThen;

pub struct Term<'a, C, T, S> {
    ctx: &'a mut C,
    cont: T,
    sep: S,
}

impl<'a, C, T, S> Term<'a, C, T, S> {
    pub fn new(ctx: &'a mut C, cont: T, sep: S) -> Self {
        Self { ctx, cont, sep }
    }
}

impl<'a, C, T, S> Term<'a, C, T, S> 
where C: Context + MatchPolicy, for<'b> &'b T: FnMut(&mut C) -> Result<C::Ret, Error>, S: FnMut(&mut C) -> Result<C::Ret, Error> {
    fn map<R>(
        &mut self,
        mut func: impl FnMut(&C, usize, &<C as MatchPolicy>::Ret) -> R,
    ) -> Result<R, crate::err::Error> {
        let beg = self.ctx.offset();
        let ret = self.ctx.try_mat(&self.cont);

        ret.map(|ret| func(self.ctx, beg, &ret))
    }
}

fn create<C, T, S>(ctx: &mut C, parser: T, sep: S) -> Term<'_, C, T, S> where for<'b> &'b T: FnMut(&mut C) -> Result<C::Ret, Error>, for<'a> &'a S: FnMut(&mut C) -> Result<C::Ret, Error>, C: MatchPolicy {
    Term::new(ctx, parser, sep)
}

fn test() {
    let mut ctx = CharsCtx::default();
    let mut x = create(&mut ctx, neure!('a'), neure!('a'));

    x.map(|_, _, _| {
        1
    });
}