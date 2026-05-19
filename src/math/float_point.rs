use crate::math::number::Number;
use core::{ops::Rem, time::Duration};
use std::{f32, f64};

pub trait FloatPoint: Number + Rem<Output = Self> + PartialOrd {
    fn from_usize(value: usize) -> Self;
    fn recip_of_count(count: usize) -> Self;
    fn from_duration(duration: Duration) -> Self;
    fn half(self) -> Self;
    fn two_pi() -> Self;
}

impl FloatPoint for f32 {
    fn from_usize(value: usize) -> Self {
        value as f32
    }

    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f32
    }

    fn from_duration(duration: Duration) -> Self {
        duration.as_secs_f32()
    }

    fn half(self) -> Self {
        self / 2.0
    }

    fn two_pi() -> Self {
        2.0 * f32::consts::PI
    }
}

impl FloatPoint for f64 {
    fn from_usize(value: usize) -> Self {
        value as f64
    }

    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f64
    }

    fn from_duration(duration: Duration) -> Self {
        duration.as_secs_f64()
    }

    fn half(self) -> Self {
        self / 2.0
    }

    fn two_pi() -> Self {
        2.0 * f64::consts::PI
    }
}
