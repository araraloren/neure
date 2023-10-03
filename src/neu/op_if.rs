use std::marker::PhantomData;

use super::Neu;

#[derive(Debug, Default, Copy)]
pub struct IfUnit<U, I, O, T>
where
    U: Neu<T>,
    I: Neu<T>,
    O: Neu<T>,
{
    r#if: I,
    unit: U,
    other: O,
    marker: PhantomData<T>,
}

impl<U, I, O, T> Clone for IfUnit<U, I, O, T>
where
    U: Neu<T> + Clone,
    I: Neu<T> + Clone,
    O: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            r#if: self.r#if.clone(),
            unit: self.unit.clone(),
            other: self.other.clone(),
            marker: self.marker,
        }
    }
}

impl<U, I, O, T> IfUnit<U, I, O, T>
where
    U: Neu<T>,
    I: Neu<T>,
    O: Neu<T>,
{
    pub fn new(unit: U, r#if: I, other: O) -> Self {
        Self {
            r#if,
            unit,
            other,
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn r#if(&self) -> &I {
        &self.r#if
    }

    pub fn other(&self) -> &O {
        &self.other
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn r#if_mut(&mut self) -> &mut I {
        &mut self.r#if
    }

    pub fn other_mut(&mut self) -> &mut O {
        &mut self.other
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn with_if(&mut self, r#if: I) -> &mut Self {
        self.r#if = r#if;
        self
    }

    pub fn set_other(&mut self, other: O) -> &mut Self {
        self.other = other;
        self
    }
}

impl<U, I, O, T> Neu<T> for IfUnit<U, I, O, T>
where
    U: Neu<T>,
    I: Neu<T>,
    O: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        if self.r#if.is_match(other) {
            self.unit.is_match(other)
        } else {
            self.other.is_match(other)
        }
    }
}
