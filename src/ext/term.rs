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
