use crate::block::Block;
use crate::metrics::Metric;
use crate::output::Output;
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::time::Duration;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Signal<T> {
    pub value: T,
    pub dt: Duration,
}

impl<T: Copy> Copy for Signal<T> {}

impl<T: Default> From<Duration> for Signal<T> {
    fn from(dt: Duration) -> Self {
        Signal {
            value: T::default(),
            dt,
        }
    }
}

impl<T> From<(T, Duration)> for Signal<T> {
    fn from((value, dt): (T, Duration)) -> Self {
        Signal { value, dt }
    }
}

impl<T> From<(T, f32)> for Signal<T> {
    fn from((value, dt): (T, f32)) -> Self {
        Signal {
            value,
            dt: Duration::from_secs_f32(dt),
        }
    }
}

impl<T: Neg<Output = T>> Neg for Signal<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            dt: self.dt,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl<T: Sub<Output = T>> Sub<T> for Signal<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value - rhs,
            dt: self.dt,
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
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Signal<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value + rhs,
            dt: self.dt,
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
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl<T: Div<Output = T>> Div<T> for Signal<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value / rhs,
            dt: self.dt,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Signal<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for Signal<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Signal {
            value: self.value * rhs,
            dt: self.dt,
        }
    }
}

impl<I, O> Mul<&mut dyn Block<Input = I, Output = O>> for Signal<I> {
    type Output = Signal<O>;

    fn mul(self, block: &mut dyn Block<Input = I, Output = O>) -> Self::Output {
        block.output(self)
    }
}

impl<T: Clone> Mul<&mut dyn Output<T>> for Signal<T> {
    type Output = Signal<T>;

    fn mul(self, monitor: &mut dyn Output<T>) -> Self::Output {
        monitor.show(self.clone());
        self
    }
}

impl<T> Mul<&mut dyn Metric<Input = T>> for Signal<T> {
    type Output = Signal<T>;

    fn mul(self, rhs: &mut dyn Metric<Input = T>) -> Self::Output {
        let output = rhs.update(self);
        output
    }
}

#[macro_export]
macro_rules! merge {
    ($v:expr) => {
        Signal {
            value: $v.map(|x| x.value),
            dt: $v[0].dt,
        }
    };
    ($v:expr, $($x:expr),+) => {
        Signal {
            value: [$v.value, $($x.value),+],
            dt: $v.dt,
        }
    };
}

#[macro_export]
macro_rules! extract_struct {
    ($signal:expr, $field:tt) => {
        Signal {
            value: $signal.value.$field,
            dt: $signal.dt,
        }
    };
}

#[macro_export]
macro_rules! extract_array {
    ($signal:expr, $index:expr) => {
        Signal {
            value: $signal.value[$index],
            dt: $signal.dt,
        }
    };
}
