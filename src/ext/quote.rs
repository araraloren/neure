pub struct Quote<'a, C, R> {
    ctx: &'a mut C,
    right: R,
}

impl<'a, C, R> Quote<'a, C, R> {
    pub fn new(ctx: &'a mut C, right: R) -> Self {
        Self { ctx, right }
    }
}
