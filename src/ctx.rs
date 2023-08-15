use crate::err::Error;

pub trait Context {
    type Orig: ?Sized;

    type Item;

    type Iter<'a>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'a;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn dec(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'_>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'_>, Error>;

    fn orig(&self) -> Result<&Self::Orig, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<&Self::Orig, Error>;
}
