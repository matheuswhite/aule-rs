use crate::discrete::{DTf, Polynomial};
use core::ops::{Add, Div, Mul, Sub};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct z;

impl From<&[f32]> for z {
    fn from(_value: &[f32]) -> Self {
        z
    }
}

impl Add<z> for f32 {
    type Output = Polynomial;

    fn add(self, rhs: z) -> Self::Output {
        self + Polynomial::from(rhs)
    }
}

impl Add<f32> for z {
    type Output = Polynomial;

    fn add(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl Sub<z> for f32 {
    type Output = Polynomial;

    fn sub(self, rhs: z) -> Self::Output {
        self - Polynomial::from(rhs)
    }
}

impl Sub<f32> for z {
    type Output = Polynomial;

    fn sub(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl Mul<z> for f32 {
    type Output = Polynomial;

    fn mul(self, rhs: z) -> Self::Output {
        self * Polynomial::from(rhs)
    }
}

impl Mul<f32> for z {
    type Output = Polynomial;

    fn mul(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl Mul<z> for z {
    type Output = Polynomial;

    fn mul(self, rhs: z) -> Self::Output {
        Polynomial::from(self) * Polynomial::from(rhs)
    }
}

impl Div<z> for f32 {
    type Output = DTf;

    fn div(self, rhs: z) -> Self::Output {
        self / Polynomial::from(rhs)
    }
}
