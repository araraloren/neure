use std::{cell::Cell, marker::PhantomData};

use super::Neu;

#[derive(Debug, Default)]
pub struct MayUnit<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
{
    r#if: I,
    unit: U,
    count: Cell<usize>,
    value: Cell<bool>,
    marker: PhantomData<T>,
}

impl<U, I, T> Clone for MayUnit<U, I, T>
where
    U: Neu<T> + Clone,
    I: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            r#if: self.r#if.clone(),
            unit: self.unit.clone(),
            value: self.value.clone(),
            count: self.count.clone(),
            marker: self.marker,
        }
    }
}

impl<U, I, T> MayUnit<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
{
    pub fn new(r#if: I, count: usize, unit: U) -> Self {
        Self {
            r#if,
            unit,
            count: Cell::new(count),
            value: Cell::new(true),
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn r#if(&self) -> &I {
        &self.r#if
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn r#if_mut(&mut self) -> &mut I {
        &mut self.r#if
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn and_if(&mut self, r#if: I) -> &mut Self {
        self.r#if = r#if;
        self
    }
}

impl<U, I, T> Neu<T> for MayUnit<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
{
    fn is_match(&self, other: &T) -> bool {
        let count = self.count.get();
        let value = self.value.get();

        if count == 0 {
            value && self.unit.is_match(other)
        } else {
            let ret = self.r#if.is_match(other);

            self.value.set(value && ret);
            self.count.set(count - 1);
            ret
        }
    }
}
