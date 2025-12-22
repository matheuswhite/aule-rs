use crate::time::Delta;
use crate::{block::Block, time::TimeType};
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Signal<T, K>
where
    K: TimeType,
{
    pub value: T,
    pub delta: Delta<K>,
}

impl<T, K> Copy for Signal<T, K>
where
    T: Copy,
    K: TimeType,
{
}

impl<T, K> Signal<T, K>
where
    K: TimeType,
{
    pub fn replace(self, value: T) -> Self {
        Signal {
            value,
            delta: self.delta,
        }
    }

    pub fn map<U, F>(self, f: F) -> Signal<U, K>
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
        P: FnOnce(&T, &Delta<K>) -> bool,
    {
        if predicate(&self.value, &self.delta) {
            Some(self)
        } else {
            None
        }
    }
}

impl<T, K> Signal<Signal<T, K>, K>
where
    K: TimeType,
{
    pub fn flatten(self) -> Signal<T, K> {
        Signal {
            value: self.value.value,
            delta: self.value.delta.merge(self.delta),
        }
    }
}

impl<T, K> From<Delta<K>> for Signal<T, K>
where
    T: Default,
    K: TimeType,
{
    fn from(delta: Delta<K>) -> Self {
        Signal {
            value: T::default(),
            delta,
        }
    }
}

impl<T, K> From<(T, Delta<K>)> for Signal<T, K>
where
    K: TimeType,
{
    fn from((value, delta): (T, Delta<K>)) -> Self {
        Signal { value, delta }
    }
}

impl<T, K> Neg for Signal<T, K>
where
    T: Neg<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            delta: self.delta,
        }
    }
}

impl<T, K> Sub for Signal<T, K>
where
    T: Sub<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, K> Sub<T> for Signal<T, K>
where
    T: Sub<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            delta: self.delta,
        }
    }
}

impl<T, K> Sub<Option<T>> for Signal<T, K>
where
    T: Sub<Output = T> + Default,
    K: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Option<T>) -> Self::Output {
        Signal {
            value: self.value - rhs.unwrap_or_default(),
            delta: self.delta,
        }
    }
}

impl<T, K> Sub<Option<Signal<T, K>>> for Signal<T, K>
where
    T: Sub<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Option<Signal<T, K>>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl<T, K> Add for Signal<T, K>
where
    T: Add<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, K> Add<T> for Signal<T, K>
where
    T: Add<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            delta: self.delta,
        }
    }
}

impl<T, K> Add<Option<Signal<T, K>>> for Signal<T, K>
where
    T: Add<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn add(self, rhs: Option<Signal<T, K>>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl<T, K> Div for Signal<T, K>
where
    T: Div<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, K> Div<T> for Signal<T, K>
where
    T: Div<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            delta: self.delta,
        }
    }
}

impl<T, K> Mul for Signal<T, K>
where
    T: Mul<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, K> Mul<T> for Signal<T, K>
where
    T: Mul<Output = T>,
    K: TimeType,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value * rhs,
            delta: self.delta,
        }
    }
}

impl<I, O, K> Mul<&mut dyn Block<Input = I, Output = O, TimeType = K>> for Signal<I, K>
where
    K: TimeType,
{
    type Output = Signal<O, K>;

    fn mul(self, block: &mut dyn Block<Input = I, Output = O, TimeType = K>) -> Self::Output {
        block.output(self)
    }
}

impl<I, O, K> Mul<&mut dyn Block<Input = [I; 1], Output = [O; 1], TimeType = K>> for Signal<I, K>
where
    O: Clone,
    K: TimeType,
{
    type Output = Signal<O, K>;

    fn mul(
        self,
        block: &mut dyn Block<Input = [I; 1], Output = [O; 1], TimeType = K>,
    ) -> Self::Output {
        block
            .output(self.map(|value| [value]))
            .map(|arr| arr[0].clone())
    }
}

pub trait Pack {
    type Packed;
    type TimeType: TimeType;

    fn pack(self) -> Signal<Self::Packed, Self::TimeType>;
}

impl<T, K, const N: usize> Pack for [Signal<T, K>; N]
where
    T: Copy,
    K: TimeType,
{
    type Packed = [T; N];
    type TimeType = K;

    fn pack(self) -> Signal<Self::Packed, Self::TimeType> {
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

impl<T, K> Pack for (Signal<T, K>, Signal<T, K>)
where
    T: Copy,
    K: TimeType,
{
    type Packed = (T, T);
    type TimeType = K;

    fn pack(self) -> Signal<Self::Packed, Self::TimeType> {
        let (signal_a, signal_b) = self;
        let packed_value = (signal_a.value, signal_b.value);
        let packed_delta = signal_a.delta.merge(signal_b.delta);

        Signal {
            value: packed_value,
            delta: packed_delta,
        }
    }
}

impl<T, K> Pack for (Signal<T, K>, Signal<T, K>, Signal<T, K>)
where
    T: Copy,
    K: TimeType,
{
    type Packed = (T, T, T);
    type TimeType = K;

    fn pack(self) -> Signal<Self::Packed, Self::TimeType> {
        let (signal_a, signal_b, signal_c) = self;
        let packed_value = (signal_a.value, signal_b.value, signal_c.value);
        let packed_delta = signal_a.delta.merge(signal_b.delta).merge(signal_c.delta);

        Signal {
            value: packed_value,
            delta: packed_delta,
        }
    }
}

pub trait Unpack {
    type Unpacked;
    type TimeType: TimeType;

    fn unpack(self) -> Self::Unpacked;
}

impl<T, K> Unpack for Signal<(T, T), K>
where
    T: Copy,
    K: TimeType,
{
    type Unpacked = (Signal<T, K>, Signal<T, K>);
    type TimeType = K;

    fn unpack(self) -> Self::Unpacked {
        let Signal { value, delta } = self;
        let (v1, v2) = value;
        (Signal { value: v1, delta }, Signal { value: v2, delta })
    }
}

impl<T, K> Unpack for Signal<(T, T, T), K>
where
    T: Copy,
    K: TimeType,
{
    type Unpacked = (Signal<T, K>, Signal<T, K>, Signal<T, K>);
    type TimeType = K;

    fn unpack(self) -> Self::Unpacked {
        let Signal { value, delta } = self;
        let (v1, v2, v3) = value;
        (
            Signal { value: v1, delta },
            Signal { value: v2, delta },
            Signal { value: v3, delta },
        )
    }
}

impl<T, K, const N: usize> Unpack for Signal<[T; N], K>
where
    T: Copy,
    K: TimeType,
{
    type Unpacked = [Signal<T, K>; N];
    type TimeType = K;

    fn unpack(self) -> Self::Unpacked {
        let Signal { value, delta } = self;
        value.map(|v| Signal { value: v, delta })
    }
}
