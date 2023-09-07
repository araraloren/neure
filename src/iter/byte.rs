#[derive(Debug, Clone)]
pub struct BytesIndices<'a, T> {
    offset: usize,

    bytes: &'a [T],
}

impl<'a, T> BytesIndices<'a, T> {
    pub fn new(bytes: &'a [T]) -> Self {
        Self { offset: 0, bytes }
    }
}

impl<'a, T> Iterator for BytesIndices<'a, T>
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
        (self.bytes.len(), Some(self.bytes.len()))
    }
}

impl<'a, T> ExactSizeIterator for BytesIndices<'a, T> where T: Copy {}
