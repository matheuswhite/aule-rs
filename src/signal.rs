use crate::block::mimo::MIMO;
#[cfg(feature = "alloc")]
use crate::output::Output;
use crate::{block::siso::SISO, metrics::Metric};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::ops::{Add, Div, Mul, Neg, Shr, Sub};
use core::time::Duration;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Signal {
    pub value: f32,
    pub dt: Duration,
}

impl From<Duration> for Signal {
    fn from(dt: Duration) -> Self {
        Signal { value: 0.0, dt }
    }
}

impl From<(f32, Duration)> for Signal {
    fn from((value, dt): (f32, Duration)) -> Self {
        Signal { value, dt }
    }
}

impl From<(Duration, f32)> for Signal {
    fn from((dt, value): (Duration, f32)) -> Self {
        Signal { value, dt }
    }
}

impl From<(f32, f32)> for Signal {
    fn from((value, dt): (f32, f32)) -> Self {
        Signal {
            value,
            dt: Duration::from_secs_f32(dt),
        }
    }
}

impl Neg for Signal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Signal {
            value: -self.value,
            dt: self.dt,
        }
    }
}

impl Sub for Signal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Sub<f32> for Signal {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value - rhs,
            dt: self.dt,
        }
    }
}

impl Sub<Option<Signal>> for Signal {
    type Output = Self;

    fn sub(self, rhs: Option<Signal>) -> Self::Output {
        match rhs {
            Some(signal) => self - signal,
            None => self,
        }
    }
}

impl Add for Signal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Add<f32> for Signal {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value + rhs,
            dt: self.dt,
        }
    }
}

impl Add<Option<Signal>> for Signal {
    type Output = Self;

    fn add(self, rhs: Option<Signal>) -> Self::Output {
        match rhs {
            Some(signal) => self + signal,
            None => self,
        }
    }
}

impl Div for Signal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value / rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Div<f32> for Signal {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value / rhs,
            dt: self.dt,
        }
    }
}

impl Mul for Signal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            dt: Duration::from_secs_f32((self.dt.as_secs_f32() + rhs.dt.as_secs_f32()) / 2.0),
        }
    }
}

impl Mul<f32> for Signal {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value * rhs,
            dt: self.dt,
        }
    }
}

impl Mul<&mut dyn SISO> for Signal {
    type Output = Signal;

    fn mul(self, block: &mut dyn SISO) -> Self::Output {
        block.output(self)
    }
}

impl<const I: usize> Mul<&mut dyn MIMO> for [Signal; I] {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self.to_vec())
    }
}

impl Mul<&mut dyn MIMO> for &[Signal] {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self.to_vec())
    }
}

impl Mul<&mut dyn MIMO> for Signal {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self].to_vec())
    }
}

impl Mul<&mut dyn MIMO> for (Signal, Signal) {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self.0, self.1].to_vec())
    }
}

impl Mul<&mut dyn MIMO> for (Signal, Signal, Signal) {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output([self.0, self.1, self.2].to_vec())
    }
}

#[cfg(feature = "alloc")]
impl Mul<&mut dyn MIMO> for Vec<Signal> {
    type Output = Vec<Signal>;

    fn mul(self, rhs: &mut dyn MIMO) -> Self::Output {
        rhs.output(self)
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for Signal {
    type Output = Signal;

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self]);
        self
    }
}

impl Shr<&mut dyn Metric<1>> for Signal {
    type Output = Signal;

    fn shr(self, rhs: &mut dyn Metric<1>) -> Self::Output {
        let input = [self];
        let output = rhs.update(input);
        output[0]
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for (Signal, Signal) {
    type Output = (Signal, Signal);

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self.0, self.1]);
        self
    }
}

impl Shr<&mut dyn Metric<2>> for (Signal, Signal) {
    type Output = (Signal, Signal);

    fn shr(self, rhs: &mut dyn Metric<2>) -> Self::Output {
        let input = [self.0, self.1];
        let output = rhs.update(input);
        (output[0], output[1])
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for (Signal, Signal, Signal) {
    type Output = (Signal, Signal, Signal);

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&[self.0, self.1, self.2]);
        self
    }
}

#[cfg(feature = "alloc")]
impl<'a> Shr<&mut dyn Output> for &'a [Signal] {
    type Output = &'a [Signal];

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(self);
        self
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> Shr<&mut dyn Output> for [Signal; N] {
    type Output = [Signal; N];

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&self);
        self
    }
}

#[cfg(feature = "alloc")]
impl Shr<&mut dyn Output> for Vec<Signal> {
    type Output = Vec<Signal>;

    fn shr(self, monitor: &mut dyn Output) -> Self::Output {
        monitor.show(&self);
        self
    }
}
