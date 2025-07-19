use crate::tf::Tf;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[allow(non_camel_case_types)]
pub struct s;

impl Add<s> for f32 {
    type Output = Tf;

    fn add(self, _rhs: s) -> Self::Output {
        Tf::new(&[1.0, self], &[1.0])
    }
}

impl Add<f32> for s {
    type Output = Tf;

    fn add(self, rhs: f32) -> Self::Output {
        Tf::new(&[1.0, rhs], &[1.0])
    }
}

impl Sub<s> for f32 {
    type Output = Tf;

    fn sub(self, _rhs: s) -> Self::Output {
        Tf::new(&[-1.0, self], &[1.0])
    }
}

impl Sub<f32> for s {
    type Output = Tf;

    fn sub(self, rhs: f32) -> Self::Output {
        Tf::new(&[1.0, -rhs], &[1.0])
    }
}

impl Mul<s> for f32 {
    type Output = Tf;

    fn mul(self, _rhs: s) -> Self::Output {
        Tf::new(&[self, 0.0], &[1.0])
    }
}

impl Mul<f32> for s {
    type Output = Tf;

    fn mul(self, rhs: f32) -> Self::Output {
        Tf::new(&[rhs, 0.0], &[1.0])
    }
}

impl Div<s> for f32 {
    type Output = Tf;

    fn div(self, _rhs: s) -> Self::Output {
        Tf::new(&[self], &[1.0, 0.0])
    }
}

impl Div<f32> for s {
    type Output = Tf;

    fn div(self, rhs: f32) -> Self::Output {
        Tf::new(&[1.0, 0.0], &[rhs])
    }
}

impl Neg for s {
    type Output = Tf;

    fn neg(self) -> Self::Output {
        Tf::new(&[-1.0, 0.0], &[1.0])
    }
}
