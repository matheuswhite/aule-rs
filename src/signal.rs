use crate::time::Delta;
use crate::{block::Block, time::TimeType};
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Signal<T, D>
where
    D: TimeType,
{
    pub value: T,
    pub delta: Delta<D>,
}

impl<T, D> Copy for Signal<T, D>
where
    T: Copy,
    D: TimeType,
{
}

impl<T, D> Signal<T, D>
where
    D: TimeType,
{
    pub fn replace(self, value: T) -> Self {
        Signal {
            value,
            delta: self.delta,
        }
    }

    pub fn map<U, F>(self, f: F) -> Signal<U, D>
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
        P: FnOnce(&T, &Delta<D>) -> bool,
    {
        if predicate(&self.value, &self.delta) {
            Some(self)
        } else {
            None
        }
    }

    pub fn zip<U>(self, other: Signal<U, D>) -> Signal<(T, U), D> {
        Signal {
            value: (self.value, other.value),
            delta: self.delta.merge(other.delta),
        }
    }

    pub fn unzip<U, V>(self) -> (Signal<U, D>, Signal<V, D>)
    where
        T: Into<(U, V)>,
    {
        let (u, v) = self.value.into();
        (
            Signal {
                value: u,
                delta: self.delta,
            },
            Signal {
                value: v,
                delta: self.delta,
            },
        )
    }
}

impl<T, D> Signal<Signal<T, D>, D>
where
    D: TimeType,
{
    pub fn flatten(self) -> Signal<T, D> {
        Signal {
            value: self.value.value,
            delta: self.value.delta.merge(self.delta),
        }
    }
}

impl<T, D> From<Delta<D>> for Signal<T, D>
where
    T: Default,
    D: TimeType,
{
    fn from(delta: Delta<D>) -> Self {
        Signal {
            value: T::default(),
            delta,
        }
    }
}

impl<T, D> From<(T, Delta<D>)> for Signal<T, D>
where
    D: TimeType,
{
    fn from((value, delta): (T, Delta<D>)) -> Self {
        Signal { value, delta }
    }
}

impl<T, D> Neg for Signal<T, D>
where
    T: Neg<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            delta: self.delta,
        }
    }
}

impl<T, D> Sub for Signal<T, D>
where
    T: Sub<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, D> Sub<T> for Signal<T, D>
where
    T: Sub<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            delta: self.delta,
        }
    }
}

impl<T, D> Sub<Option<T>> for Signal<T, D>
where
    T: Sub<Output = T> + Default,
    D: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Option<T>) -> Self::Output {
        Signal {
            value: self.value - rhs.unwrap_or_default(),
            delta: self.delta,
        }
    }
}

impl<T, D> Sub<Option<Signal<T, D>>> for Signal<T, D>
where
    T: Sub<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn sub(self, rhs: Option<Signal<T, D>>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl<T, D> Add for Signal<T, D>
where
    T: Add<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, D> Add<T> for Signal<T, D>
where
    T: Add<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            delta: self.delta,
        }
    }
}

impl<T, D> Add<Option<Signal<T, D>>> for Signal<T, D>
where
    T: Add<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn add(self, rhs: Option<Signal<T, D>>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl<T, D> Div for Signal<T, D>
where
    T: Div<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, D> Div<T> for Signal<T, D>
where
    T: Div<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            delta: self.delta,
        }
    }
}

impl<T, D> Mul for Signal<T, D>
where
    T: Mul<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T, D> Mul<T> for Signal<T, D>
where
    T: Mul<Output = T>,
    D: TimeType,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value * rhs,
            delta: self.delta,
        }
    }
}

impl<I, O, D> Mul<&mut dyn Block<Input = I, Output = O, TimeType = D>> for Signal<I, D>
where
    D: TimeType,
{
    type Output = Signal<O, D>;

    fn mul(self, block: &mut dyn Block<Input = I, Output = O, TimeType = D>) -> Self::Output {
        block.output(self)
    }
}

impl<I, O, D> Mul<&mut dyn Block<Input = [I; 1], Output = [O; 1], TimeType = D>> for Signal<I, D>
where
    O: Clone,
    D: TimeType,
{
    type Output = Signal<O, D>;

    fn mul(
        self,
        block: &mut dyn Block<Input = [I; 1], Output = [O; 1], TimeType = D>,
    ) -> Self::Output {
        block
            .output(self.map(|value| [value]))
            .map(|arr| arr[0].clone())
    }
}
