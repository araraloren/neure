use std::marker::PhantomData;

use super::Neu;

#[derive(Debug, Default, Copy)]
pub struct Not<U, T>
where
    U: Neu<T>,
{
    unit: U,
    marker: PhantomData<T>,
}

impl<U, T> Clone for Not<U, T>
where
    U: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            marker: self.marker,
        }
    }
}

impl<U, T> Not<U, T>
where
    U: Neu<T>,
{
    pub fn new(unit: U) -> Self {
        Self {
            unit,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }
}

impl<U, T> Neu<T> for Not<U, T>
where
    U: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.unit.is_match(other)
    }
}
