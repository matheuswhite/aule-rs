use crate::math::number::Number;
use core::{ops::Rem, time::Duration};
use std::{f32, f64};

trait Sealed {}

#[allow(private_bounds)]
pub trait FloatPoint: Number + Rem<Output = Self> + PartialOrd + Sealed {
    fn is_sign_negative(self) -> bool;
    fn infinity() -> Self;
    fn square_root(self) -> Self;
    fn arc_sin_h(self) -> Self;
    fn sin_h(self) -> Self;
    fn cos_h(self) -> Self;
    fn power(self, exp: Self) -> Self;
    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_usize(value: usize) -> Self;
    fn recip_of_count(count: usize) -> Self;
    fn from_duration(duration: Duration) -> Self;
    fn to_duration(self) -> Duration;
    fn half(self) -> Self;
    fn two_pi() -> Self;
    fn pi() -> Self;
    fn sqrt_2() -> Self;
    fn exp(self) -> Self;
    fn tangent(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl Sealed for f32 {}

impl FloatPoint for f32 {
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }

    fn infinity() -> Self {
        f32::INFINITY
    }

    fn arc_sin_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.asinh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asinh(self)
        }
    }

    fn sin_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.sinh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sinh(self)
        }
    }

    fn cos_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.cosh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cosh(self)
        }
    }

    fn power(self, exp: Self) -> Self {
        #[cfg(feature = "std")]
        {
            self.powf(exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::pow(self, exp)
        }
    }
    fn square_root(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.sqrt()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sqrt(self)
        }
    }

    fn from_f32(value: f32) -> Self {
        value
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn exp(self) -> Self {
        f32::exp(self)
    }

    fn from_usize(value: usize) -> Self {
        value as f32
    }

    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f32
    }

    fn from_duration(duration: Duration) -> Self {
        duration.as_secs_f32()
    }

    fn to_duration(self) -> Duration {
        Duration::from_secs_f32(self)
    }

    fn half(self) -> Self {
        self / 2.0
    }

    fn two_pi() -> Self {
        2.0 * f32::consts::PI
    }

    fn pi() -> Self {
        f32::consts::PI
    }

    fn sqrt_2() -> Self {
        f32::consts::SQRT_2
    }

    fn tangent(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.tan()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tan(self)
        }
    }
}

impl Sealed for f64 {}

impl FloatPoint for f64 {
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }

    fn infinity() -> Self {
        f64::INFINITY
    }

    fn arc_sin_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.asinh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::asinh(self)
        }
    }

    fn sin_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.sinh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sinh(self)
        }
    }

    fn cos_h(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.cosh()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cosh(self)
        }
    }

    fn power(self, exp: Self) -> Self {
        #[cfg(feature = "std")]
        {
            self.powf(exp)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::pow(self, exp)
        }
    }

    fn square_root(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.sqrt()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sqrt(self)
        }
    }

    fn from_f32(value: f32) -> Self {
        value as f64
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn exp(self) -> Self {
        f64::exp(self)
    }

    fn from_usize(value: usize) -> Self {
        value as f64
    }

    fn recip_of_count(count: usize) -> Self {
        1.0 / count as f64
    }

    fn from_duration(duration: Duration) -> Self {
        duration.as_secs_f64()
    }

    fn to_duration(self) -> Duration {
        Duration::from_secs_f64(self)
    }

    fn half(self) -> Self {
        self / 2.0
    }

    fn two_pi() -> Self {
        2.0 * f64::consts::PI
    }

    fn pi() -> Self {
        f64::consts::PI
    }

    fn sqrt_2() -> Self {
        f64::consts::SQRT_2
    }

    fn tangent(self) -> Self {
        #[cfg(feature = "std")]
        {
            self.tan()
        }
        #[cfg(not(feature = "std"))]
        {
            libm::tan(self)
        }
    }
}

pub trait AsFloatPoint<F> {
    fn as_fp(self) -> F;
}

impl<T> AsFloatPoint<T> for f32
where
    T: FloatPoint,
{
    fn as_fp(self) -> T {
        T::from_f32(self)
    }
}

impl<T> AsFloatPoint<T> for f64
where
    T: FloatPoint,
{
    fn as_fp(self) -> T {
        T::from_f64(self)
    }
}
