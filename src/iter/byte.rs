#[derive(Debug, Clone)]
pub struct BytesIndices<'a, T> {
    offset: usize,

    bytes: &'a [T],
}

impl<'a, T> BytesIndices<'a, T> {
    pub const fn new(bytes: &'a [T]) -> Self {
        Self { offset: 0, bytes }
    }
}

impl<T> Iterator for BytesIndices<'_, T>
where
    T: Copy,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.bytes.len() {
            let offset = self.offset;

            self.offset += 1;
            Some((offset, self.bytes[offset]))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.bytes.len() - self.offset;

        (size, Some(size))
    }
}

impl<T> ExactSizeIterator for BytesIndices<'_, T> where T: Copy {}
