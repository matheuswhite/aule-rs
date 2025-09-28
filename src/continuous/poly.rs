use crate::continuous::{Tf, s};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial(crate::poly::Polynomial);

impl Polynomial {
    pub fn new(coeff: &[f32]) -> Self {
        Polynomial(crate::poly::Polynomial::new(coeff))
    }

    pub fn empty() -> Self {
        Polynomial(crate::poly::Polynomial::empty())
    }

    pub fn pow(self, exp: usize) -> Self {
        Polynomial(self.0.pow(exp))
    }

    pub fn degree(&self) -> isize {
        self.0.degree()
    }

    pub fn coeff(&self) -> &[f32] {
        self.0.coeff()
    }

    pub fn lead_coeff(&self) -> f32 {
        self.0.lead_coeff()
    }

    pub fn inner(&self) -> &crate::poly::Polynomial {
        &self.0
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 + rhs.0)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 - rhs.0)
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 * rhs.0)
    }
}

impl Div for Polynomial {
    type Output = Tf;

    fn div(self, rhs: Self) -> Self::Output {
        Tf::new(self.coeff(), rhs.coeff())
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial(-self.0)
    }
}

macro_rules! impl_poly_ops {
    ($type:ty) => {
        impl Add<$type> for Polynomial {
            type Output = Polynomial;

            fn add(self, rhs: $type) -> Self::Output {
                self + Polynomial::from(rhs)
            }
        }

        impl Add<Polynomial> for $type {
            type Output = Polynomial;

            fn add(self, rhs: Polynomial) -> Self::Output {
                Polynomial::from(self) + rhs
            }
        }

        impl Sub<$type> for Polynomial {
            type Output = Polynomial;

            fn sub(self, rhs: $type) -> Self::Output {
                self - Polynomial::from(rhs)
            }
        }

        impl Sub<Polynomial> for $type {
            type Output = Polynomial;

            fn sub(self, rhs: Polynomial) -> Self::Output {
                Polynomial::from(self) - rhs
            }
        }

        impl Mul<$type> for Polynomial {
            type Output = Polynomial;

            fn mul(self, rhs: $type) -> Self::Output {
                self * Polynomial::from(rhs)
            }
        }

        impl Mul<Polynomial> for $type {
            type Output = Polynomial;

            fn mul(self, rhs: Polynomial) -> Self::Output {
                Polynomial::from(self) * rhs
            }
        }

        impl Div<$type> for Polynomial {
            type Output = Tf;

            fn div(self, rhs: $type) -> Self::Output {
                self / Polynomial::from(rhs)
            }
        }

        impl Div<Polynomial> for $type {
            type Output = Tf;

            fn div(self, rhs: Polynomial) -> Self::Output {
                Polynomial::from(self) / rhs
            }
        }
    };
}

impl From<f32> for Polynomial {
    fn from(value: f32) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[value]))
    }
}

impl_poly_ops!(f32);

impl From<s> for Polynomial {
    fn from(_value: s) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[1.0, 0.0]))
    }
}

impl_poly_ops!(s);

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let degree = self.degree();
        let string = self
            .coeff()
            .iter()
            .enumerate()
            .map(|(i, &coeff)| {
                let i = degree - i as isize;
                if i == 0 {
                    format!("{}", coeff)
                } else if i == 1 {
                    format!("{}*s", coeff)
                } else {
                    format!("{}*s^{}", coeff, i)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test_f32_impl {
    use super::*;

    #[test]
    fn test_f32_add() {
        let p1 = 5.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_add_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_sub() {
        let p1 = 5.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[-1.0, 3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_sub_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[1.0, -3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_mul() {
        let p1 = 2.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[2.0, 4.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_mul_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 2.0;
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[2.0, 4.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_div() {
        let p1 = 6.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 / p2;
        assert_eq!(result, Tf::new(&[6.0], &[1.0, 2.0]));
    }

    #[test]
    #[should_panic(expected = "Denominator must have degree greater than or equal to numerator.")]
    fn test_div_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 6.0;
        let _result = p1 / p2;
    }
}

#[cfg(test)]
mod test_s_impl {
    use super::*;

    #[test]
    fn test_s_add() {
        let p1 = s;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_add_s() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = s;
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_s_sub() {
        let p1 = s;
        let p2 = Polynomial::new(&[3.0, 2.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[-2.0, -2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_sub_s() {
        let p1 = Polynomial::new(&[3.0, 2.0]);
        let p2 = s;
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_s_mul() {
        let p1 = s;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[1.0, 2.0, 0.0]);
        assert_eq!(result.degree(), 2);
    }

    #[test]
    fn test_mul_s() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = s;
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[1.0, 2.0, 0.0]);
        assert_eq!(result.degree(), 2);
    }

    #[test]
    fn test_s_div() {
        let p1 = s;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result = p1 / p2;
        assert_eq!(result, Tf::new(&[1.0, 0.0], &[1.0, 2.0]));
    }

    #[test]
    #[should_panic(expected = "Denominator must have degree greater than or equal to numerator.")]
    fn test_div_s() {
        let p1 = Polynomial::new(&[1.0, 2.0, 3.0]);
        let p2 = s;
        let _result = p1 / p2;
    }
}
