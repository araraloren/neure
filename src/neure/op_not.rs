use std::marker::PhantomData;

use super::Neure;

#[derive(Debug, Default, Copy)]
pub struct Not<U, T>
where
    U: Neure<T>,
{
    unit: U,
    marker: PhantomData<T>,
}

impl<U, T> Clone for Not<U, T>
where
    U: Neure<T> + Clone,
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
    U: Neure<T>,
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

impl<U, T> Neure<T> for Not<U, T>
where
    U: Neure<T>,
{
    fn is_match(&self, other: &T) -> bool {
        !self.unit.is_match(other)
    }
}
