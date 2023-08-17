#[derive(Debug)]
pub struct MatThen<'a, C, R> {
    start: usize,
    ret: R,
    ctx: &'a mut C,
}

impl<'a, C, R> MatThen<'a, C, R> {
    pub fn new(ctx: &'a mut C, start: usize, ret: R) -> Self {
        Self { ctx, start, ret }
    }
}
