use crate::block::Block;
use crate::time::Delta;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Signal<T> {
    pub value: T,
    pub delta: Delta,
}

impl<T: Copy> Copy for Signal<T> {}

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

    pub fn zip<U>(self, other: Signal<U>) -> Signal<(T, U)> {
        Signal {
            value: (self.value, other.value),
            delta: self.delta.merge(other.delta),
        }
    }

    pub fn unzip<U, V>(self) -> (Signal<U>, Signal<V>)
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

impl<T> Signal<Signal<T>> {
    pub fn flatten(self) -> Signal<T> {
        Signal {
            value: self.value.value,
            delta: self.value.delta.merge(self.delta),
        }
    }
}

impl<T: Default> From<Delta> for Signal<T> {
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

impl<T: Neg<Output = T>> Neg for Signal<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            delta: self.delta,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T: Sub<Output = T>> Sub<T> for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            delta: self.delta,
        }
    }
}

impl<T: Sub<Output = T> + Default> Sub<Option<T>> for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: Option<T>) -> Self::Output {
        Signal {
            value: self.value - rhs.unwrap_or_default(),
            delta: self.delta,
        }
    }
}

impl<T: Sub<Output = T>> Sub<Option<Signal<T>>> for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl<T: Add<Output = T>> Add for Signal<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Signal<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            delta: self.delta,
        }
    }
}

impl<T: Add<Output = T>> Add<Option<Signal<T>>> for Signal<T> {
    type Output = Self;

    fn add(self, rhs: Option<Signal<T>>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl<T: Div<Output = T>> Div for Signal<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T: Div<Output = T>> Div<T> for Signal<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            delta: self.delta,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Signal<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            delta: self.delta.merge(rhs.delta),
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for Signal<T> {
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

impl<I, O: Clone> Mul<&mut dyn Block<Input = [I; 1], Output = [O; 1]>> for Signal<I> {
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = [I; 1], Output = [O; 1]>) -> Self::Output {
        block
            .output(self.map(|value| [value]))
            .map(|arr| arr[0].clone())
    }
}
