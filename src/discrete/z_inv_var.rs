use crate::discrete::{DTf, PolynomialInverse};
use core::ops::{Add, Div, Mul, Sub};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct z_inv;

impl From<&[f32]> for z_inv {
    fn from(_value: &[f32]) -> Self {
        z_inv
    }
}

impl Add<z_inv> for f32 {
    type Output = PolynomialInverse;

    fn add(self, rhs: z_inv) -> Self::Output {
        self + PolynomialInverse::from(rhs)
    }
}

impl Add<f32> for z_inv {
    type Output = PolynomialInverse;

    fn add(self, rhs: f32) -> Self::Output {
        PolynomialInverse::from(self) + rhs
    }
}

impl Sub<z_inv> for f32 {
    type Output = PolynomialInverse;

    fn sub(self, rhs: z_inv) -> Self::Output {
        self - PolynomialInverse::from(rhs)
    }
}

impl Sub<f32> for z_inv {
    type Output = PolynomialInverse;

    fn sub(self, rhs: f32) -> Self::Output {
        PolynomialInverse::from(self) - rhs
    }
}

impl Mul<z_inv> for f32 {
    type Output = PolynomialInverse;

    fn mul(self, rhs: z_inv) -> Self::Output {
        self * PolynomialInverse::from(rhs)
    }
}

impl Mul<f32> for z_inv {
    type Output = PolynomialInverse;

    fn mul(self, rhs: f32) -> Self::Output {
        PolynomialInverse::from(self) * rhs
    }
}

impl Mul<z_inv> for z_inv {
    type Output = PolynomialInverse;

    fn mul(self, rhs: z_inv) -> Self::Output {
        PolynomialInverse::from(self) * PolynomialInverse::from(rhs)
    }
}

impl Div<z_inv> for f32 {
    type Output = DTf;

    fn div(self, rhs: z_inv) -> Self::Output {
        self / PolynomialInverse::from(rhs)
    }
}
