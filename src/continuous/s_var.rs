use crate::continuous::{Polynomial, Tf};
use std::ops::{Add, Div, Mul, Sub};

#[allow(non_camel_case_types)]
pub struct s;

impl Add<s> for f32 {
    type Output = Polynomial;

    fn add(self, rhs: s) -> Self::Output {
        self + Polynomial::from(rhs)
    }
}

impl Add<f32> for s {
    type Output = Polynomial;

    fn add(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl Sub<s> for f32 {
    type Output = Polynomial;

    fn sub(self, rhs: s) -> Self::Output {
        self - Polynomial::from(rhs)
    }
}

impl Sub<f32> for s {
    type Output = Polynomial;

    fn sub(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl Mul<s> for f32 {
    type Output = Polynomial;

    fn mul(self, rhs: s) -> Self::Output {
        self * Polynomial::from(rhs)
    }
}

impl Mul<f32> for s {
    type Output = Polynomial;

    fn mul(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl Div<s> for f32 {
    type Output = Tf;

    fn div(self, rhs: s) -> Self::Output {
        self / Polynomial::from(rhs)
    }
}

impl Div<f32> for s {
    type Output = Tf;

    fn div(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) / rhs
    }
}
