use crate::math::float_point::FloatPoint;
use core::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use nalgebra::Complex;
use num_traits::{One, Zero};

pub trait Number:
    Sized
    + Debug
    + Display
    + Default
    + Copy
    + Clone
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Sum
    + Zero
    + One
{
    type Alpha: FloatPoint;

    fn scale(self, alpha: Self::Alpha) -> Self;
    fn lerp(start: Self, end: Self, alpha: Self::Alpha) -> Self;
    fn absolute(self) -> Self;
    fn sinusoid(amplitude: Self, omega_t: Self::Alpha, phase: Self) -> Self;
    fn max_real() -> Self;
}

impl Number for f32 {
    type Alpha = f32;

    fn scale(self, alpha: Self::Alpha) -> Self {
        self * alpha
    }

    fn lerp(start: Self, end: Self, alpha: Self::Alpha) -> Self {
        start + (end - start) * alpha
    }

    fn absolute(self) -> Self {
        self.abs()
    }

    fn sinusoid(amplitude: Self, omega_t: Self::Alpha, phase: Self) -> Self {
        amplitude * libm::sinf(omega_t + phase)
    }

    fn max_real() -> Self {
        f32::MAX
    }
}

impl Number for f64 {
    type Alpha = f64;

    fn scale(self, alpha: Self::Alpha) -> Self {
        self * alpha
    }

    fn lerp(start: Self, end: Self, alpha: Self::Alpha) -> Self {
        start + (end - start) * alpha
    }

    fn absolute(self) -> Self {
        self.abs()
    }

    fn sinusoid(amplitude: Self, omega_t: Self::Alpha, phase: Self) -> Self {
        amplitude * libm::sin(omega_t + phase)
    }

    fn max_real() -> Self {
        f64::MAX
    }
}

impl Number for Complex<f32> {
    type Alpha = f32;

    fn scale(self, alpha: Self::Alpha) -> Self {
        self * alpha
    }

    fn lerp(start: Self, end: Self, alpha: Self::Alpha) -> Self {
        start + (end - start) * alpha
    }

    fn absolute(self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }

    fn sinusoid(amplitude: Self, omega_t: Self::Alpha, phase: Self) -> Self {
        let arg = phase + Complex::new(omega_t, 0.0);
        amplitude
            * Complex::new(
                libm::sinf(arg.re) * libm::coshf(arg.im),
                libm::cosf(arg.re) * libm::sinhf(arg.im),
            )
    }

    fn max_real() -> Self {
        Complex {
            re: f32::MAX,
            im: 0.0,
        }
    }
}

impl Number for Complex<f64> {
    type Alpha = f64;

    fn scale(self, alpha: Self::Alpha) -> Self {
        self * alpha
    }

    fn lerp(start: Self, end: Self, alpha: Self::Alpha) -> Self {
        start + (end - start) * alpha
    }

    fn absolute(self) -> Self {
        Complex {
            re: self.re.abs(),
            im: self.im.abs(),
        }
    }

    fn sinusoid(amplitude: Self, omega_t: Self::Alpha, phase: Self) -> Self {
        let arg = phase + Complex::new(omega_t, 0.0);
        amplitude
            * Complex::new(
                libm::sin(arg.re) * libm::cosh(arg.im),
                libm::cos(arg.re) * libm::sinh(arg.im),
            )
    }

    fn max_real() -> Self {
        Complex {
            re: f64::MAX,
            im: 0.0,
        }
    }
}
