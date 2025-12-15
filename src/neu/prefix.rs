use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

use super::Neu;

use crate::MayDebug;

#[derive(Debug, Default)]
pub struct Prefix<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    prefix: P,
    unit: U,
    count: Cell<usize>,
    value: Cell<bool>,
    marker: PhantomData<T>,
}

impl<U, P, T> Clone for Prefix<U, P, T>
where
    U: Neu<T> + Clone,
    P: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            unit: self.unit.clone(),
            value: self.value.clone(),
            count: self.count.clone(),
            marker: self.marker,
        }
    }
}

impl<U, P, T> Prefix<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    pub fn new(prefix: P, count: usize, unit: U) -> Self {
        Self {
            prefix,
            unit,
            count: Cell::new(count),
            value: Cell::new(true),
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn prefix(&self) -> &P {
        &self.prefix
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn prefix_mut(&mut self) -> &mut P {
        &mut self.prefix
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn set_prefix(&mut self, prefix: P) -> &mut Self {
        self.prefix = prefix;
        self
    }
}

impl<U, I, T> Neu<T> for Prefix<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
    T: MayDebug,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let mut value = self.value.get();

        if value {
            let count = self.count.get();

            if count == 0 {
                value = self.unit.is_match(other) && value;

                crate::trace_retval!("Prefix", other, value);
            } else {
                value = self.prefix.is_match(other) && value;

                crate::trace_retval!("Prefix", count, other, value);
                self.value.set(value);
                self.count.set(count - 1);
            }
        }

        value
    }
}

/// [`Prefix`] is a type that maintains internal state.
/// Initially, it attempts to match the `prefix` `count` times.
/// If all attempts succeed, subsequent invocations will then attempt to match the `unit`.
/// If any match fails, the [`Prefix`] stops matching altogether.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let dec = neu::digit(10);
///     let hex = neu::digit(16);
///     let oct = neu::digit(8);
///     let bin = neu::digit(2);
///
///     let tests = [
///         ("99EF", Some(Span::new(0, 2))),     // dec
///         ("0x99EF", Some(Span::new(0, 6))),   // hex
///         ("0o7389EF", Some(Span::new(0, 4))), // oct
///         ("9899", Some(Span::new(0, 4))),     // dec
///         ("0b01102", Some(Span::new(0, 6))),  // bin
///         ("x99EF", None),
///     ];
///
///     for test in tests {
///         // `prefix` is not reuseable, make new one in every match
///         let oct = neu::prefix('o', oct);
///         let bin = neu::prefix('b', bin);
///         let hex = neu::prefix('x', hex);
///         let num = neu::prefix('0', oct.or(hex).or(bin));
///         let num = num.many1().or(dec.many1());
///         let mut ctx = CharsCtx::new(test.0);
///
///         if let Some(span) = test.1 {
///             assert_eq!(ctx.try_mat(&num)?, span, "at {}", test.0);
///         } else {
///             assert!(ctx.try_mat(&num).is_err());
///         }
///     }
///
///     Ok(())
/// # }
/// ```
pub fn prefix<T, P, U>(prefix: P, unit: U) -> Prefix<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    prefix_cnt(prefix, 1, unit)
}

/// [`Prefix`] is a type that maintains internal state.
/// Initially, it attempts to match the `prefix` `count` times.
/// If all attempts succeed, subsequent invocations will then attempt to match the `unit`.
/// If any match fails, the [`Prefix`] stops matching altogether.
///
/// # Example
///
/// ```
/// # use neure::prelude::*;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     tracing_subscriber::fmt::fmt()
///         .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
///         .init();
///
///     let replys = [
///         "Let fate decide!",
///         "Don't expect too much.",
///         "Absolutely worth visiting!",
///         "Highly recommended!!",
///     ];
///
///     let tests = [
///         ("AAAAA Palace Museum", Some(3)),
///         ("AAAA Wulong Longshui Fissure Gorge", Some(3)),
///         ("AAA Wuliangshan Cherry Blossom Valley", Some(2)),
///         ("AA Shimei'an Scenic Area", Some(1)),
///         ("Terracotta Warriors", None),
///     ];
///
///     let new_prefix = |n: usize, i: usize| {
///         neu::prefix_cnt('A', n, neu::never())
///             .times(n..=n)
///             .map(move |_| replys[i])
///     };
///
///     for test in tests {
///         // `prefix` is not reuseable, make new one in every match
///         let name = neu::always().many1();
///         let ultra = new_prefix(5, 3);
///         let max = new_prefix(4, 3);
///         let plus = new_prefix(3, 2);
///         let pro = new_prefix(2, 1);
///         let scenics = ultra.or(max).or(plus).or(pro).sep_once(" ", name);
///         let ret = CharsCtx::new(test.0).ctor(&scenics);
///
///         match test.1 {
///             Some(i) => {
///                 assert_eq!(ret?.0, replys[i]);
///             }
///             None => {
///                 assert!(ret.is_err());
///             }
///         }
///     }
///
///     Ok(())
/// # }
/// ```
pub fn prefix_cnt<T, P, U>(prefix: P, n: usize, unit: U) -> Prefix<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    Prefix::new(prefix, n, unit)
}

#[derive(Debug, Default)]
pub struct PrefixSync<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    prefix: P,
    unit: U,
    count: AtomicUsize,
    value: AtomicBool,
    marker: PhantomData<T>,
}

impl<U, P, T> Clone for PrefixSync<U, P, T>
where
    U: Neu<T> + Clone,
    P: Neu<T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            unit: self.unit.clone(),
            value: AtomicBool::new(self.value.load(Relaxed)),
            count: AtomicUsize::new(self.count.load(Relaxed)),
            marker: self.marker,
        }
    }
}

impl<U, P, T> PrefixSync<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    pub fn new(prefix: P, count: usize, unit: U) -> Self {
        Self {
            prefix,
            unit,
            count: AtomicUsize::new(count),
            value: AtomicBool::new(true),
            marker: PhantomData,
        }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    pub fn prefix(&self) -> &P {
        &self.prefix
    }

    pub fn unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    pub fn prefix_mut(&mut self) -> &mut P {
        &mut self.prefix
    }

    pub fn set_unit(&mut self, unit: U) -> &mut Self {
        self.unit = unit;
        self
    }

    pub fn set_prefix(&mut self, prefix: P) -> &mut Self {
        self.prefix = prefix;
        self
    }
}

impl<U, I, T> Neu<T> for PrefixSync<U, I, T>
where
    U: Neu<T>,
    I: Neu<T>,
    T: MayDebug,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        let mut value = self.value.load(Relaxed);

        if value {
            let count = self.count.load(Relaxed);

            if count == 0 {
                value = self.unit.is_match(other) && value;

                crate::trace_retval!("PrefixSync", other, value);
            } else {
                value = self.prefix.is_match(other) && value;

                crate::trace_retval!("PrefixSync", count, other, value);
                self.value.store(value, Relaxed);
                self.count.store(count - 1, Relaxed);
            }
        }

        value
    }
}

pub fn prefix_sync<T, P, U>(prefix: P, unit: U) -> PrefixSync<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    prefix_sync_cnt(prefix, 1, unit)
}

pub fn prefix_sync_cnt<T, P, U>(prefix: P, count: usize, unit: U) -> PrefixSync<U, P, T>
where
    U: Neu<T>,
    P: Neu<T>,
{
    PrefixSync::new(prefix, count, unit)
}
