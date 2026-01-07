use crate::block::Block;
use crate::time::Delta;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Signal<T> {
    pub value: T,
    pub delta: Delta,
}

impl<T> Copy for Signal<T> where T: Copy {}

impl<T> Signal<T> {
    pub fn replace(self, value: T) -> Self {
        Signal {
            value,
            delta: self.delta,
        }
    }

    pub fn map<U, F>(self, f: F) -> Signal<U>
    where
        F: FnOnce(T) -> U,
    {
        Signal {
            value: f(self.value),
            delta: self.delta,
        }
    }

    pub fn filter<P>(self, predicate: P) -> Option<Self>
    where
        P: FnOnce(&T, &Delta) -> bool,
    {
        if predicate(&self.value, &self.delta) {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> Signal<Signal<T>> {
    pub fn flatten(self) -> Signal<T> {
        Signal {
            value: self.value.value,
            delta: self.value.delta.merge(self.delta),
        }
    }
}

impl<T> From<Delta> for Signal<T>
where
    T: Default,
{
    fn from(delta: Delta) -> Self {
        Signal {
            value: T::default(),
            delta,
        }
    }
}

impl<T> From<(T, Delta)> for Signal<T> {
    fn from((value, delta): (T, Delta)) -> Self {
        Signal { value, delta }
    }
}

impl<T> Neg for Signal<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            delta: self.delta,
        }
    }
}

impl<T> Sub for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T> Sub<T> for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            delta: self.delta,
        }
    }
}

impl<T> Sub<Option<T>> for Signal<T>
where
    T: Sub<Output = T> + Default,
{
    type Output = Self;

    fn sub(self, rhs: Option<T>) -> Self::Output {
        Signal {
            value: self.value - rhs.unwrap_or_default(),
            delta: self.delta,
        }
    }
}

impl<T> Sub<Option<Signal<T>>> for Signal<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl<T> Add for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T> Add<T> for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            delta: self.delta,
        }
    }
}

impl<T> Add<Option<Signal<T>>> for Signal<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl<T> Div for Signal<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T> Div<T> for Signal<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            delta: self.delta,
        }
    }
}

impl<T> Mul for Signal<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T> Mul<T> for Signal<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value * rhs,
            delta: self.delta,
        }
    }
}

impl<I, O> Mul<&mut dyn Block<Input = I, Output = O>> for Signal<I> {
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = I, Output = O>) -> Self::Output {
        block.output(self)
    }
}

impl<I, O> Mul<&mut dyn Block<Input = [I; 1], Output = [O; 1]>> for Signal<I>
where
    O: Clone,
{
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = [I; 1], Output = [O; 1]>) -> Self::Output {
        block
            .output(self.map(|value| [value]))
            .map(|arr| arr[0].clone())
    }
}

pub trait Pack<P> {
    fn pack(self) -> Signal<P>;
}

impl<T, const N: usize> Pack<[T; N]> for [Signal<T>; N]
where
    T: Copy,
{
    fn pack(self) -> Signal<[T; N]> {
        let values = self.map(|signal| signal.value);
        let deltas = self.map(|signal| signal.delta);
        let merged_delta = deltas
            .into_iter()
            .fold(deltas[0], |acc, delta| acc.merge(delta));

        Signal {
            value: values,
            delta: merged_delta,
        }
    }
}

impl<T> Pack<(T, T)> for (Signal<T>, Signal<T>)
where
    T: Copy,
{
    fn pack(self) -> Signal<(T, T)> {
        let (signal_a, signal_b) = self;
        let packed_value = (signal_a.value, signal_b.value);
        let packed_delta = signal_a.delta.merge(signal_b.delta);

        Signal {
            value: packed_value,
            delta: packed_delta,
        }
    }
}

impl<T> Pack<(T, T, T)> for (Signal<T>, Signal<T>, Signal<T>)
where
    T: Copy,
{
    fn pack(self) -> Signal<(T, T, T)> {
        let (signal_a, signal_b, signal_c) = self;
        let packed_value = (signal_a.value, signal_b.value, signal_c.value);
        let packed_delta = signal_a.delta.merge(signal_b.delta).merge(signal_c.delta);

        Signal {
            value: packed_value,
            delta: packed_delta,
        }
    }
}

pub trait Unpack<U> {
    fn unpack(self) -> U;
}

impl<T, U> Unpack<(Signal<T>, Signal<U>)> for Signal<(T, U)>
where
    T: Copy,
    U: Copy,
{
    fn unpack(self) -> (Signal<T>, Signal<U>) {
        let Signal { value, delta } = self;
        let (v1, v2) = value;
        (Signal { value: v1, delta }, Signal { value: v2, delta })
    }
}

impl<T> Unpack<(Signal<T>, Signal<T>, Signal<T>)> for Signal<(T, T, T)>
where
    T: Copy,
{
    fn unpack(self) -> (Signal<T>, Signal<T>, Signal<T>) {
        let Signal { value, delta } = self;
        let (v1, v2, v3) = value;
        (
            Signal { value: v1, delta },
            Signal { value: v2, delta },
            Signal { value: v3, delta },
        )
    }
}

impl<T, const N: usize> Unpack<[Signal<T>; N]> for Signal<[T; N]>
where
    T: Copy,
{
    fn unpack(self) -> [Signal<T>; N] {
        let Signal { value, delta } = self;
        value.map(|v| Signal { value: v, delta })
    }
}
