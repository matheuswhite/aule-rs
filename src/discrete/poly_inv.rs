use crate::discrete::{DTf, Polynomial, z_inv};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct PolynomialInverse(crate::poly::Polynomial);

impl PolynomialInverse {
    pub fn new(coeff: &[f32]) -> Self {
        Self(crate::poly::Polynomial::new(coeff))
    }

    pub fn empty() -> Self {
        Self(crate::poly::Polynomial::empty())
    }

    pub fn pow(self, exp: usize) -> Self {
        Self(self.0.pow(exp))
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

impl Add for PolynomialInverse {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for PolynomialInverse {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for PolynomialInverse {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for PolynomialInverse {
    type Output = DTf;

    fn div(self, rhs: Self) -> Self::Output {
        DTf::new(self.coeff(), rhs.coeff())
    }
}

impl Neg for PolynomialInverse {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

macro_rules! impl_poly_ops {
    ($type:ty) => {
        impl Add<$type> for PolynomialInverse {
            type Output = Self;

            fn add(self, rhs: $type) -> Self::Output {
                self + Self::from(rhs)
            }
        }

        impl Add<PolynomialInverse> for $type {
            type Output = PolynomialInverse;

            fn add(self, rhs: PolynomialInverse) -> Self::Output {
                PolynomialInverse::from(self) + rhs
            }
        }

        impl Sub<$type> for PolynomialInverse {
            type Output = Self;

            fn sub(self, rhs: $type) -> Self::Output {
                self - Self::from(rhs)
            }
        }

        impl Sub<PolynomialInverse> for $type {
            type Output = PolynomialInverse;

            fn sub(self, rhs: PolynomialInverse) -> Self::Output {
                PolynomialInverse::from(self) - rhs
            }
        }

        impl Mul<$type> for PolynomialInverse {
            type Output = Self;

            fn mul(self, rhs: $type) -> Self::Output {
                self * Self::from(rhs)
            }
        }

        impl Mul<PolynomialInverse> for $type {
            type Output = PolynomialInverse;

            fn mul(self, rhs: PolynomialInverse) -> Self::Output {
                PolynomialInverse::from(self) * rhs
            }
        }

        impl Div<$type> for PolynomialInverse {
            type Output = DTf;

            fn div(self, rhs: $type) -> Self::Output {
                self / Self::from(rhs)
            }
        }

        impl Div<PolynomialInverse> for $type {
            type Output = DTf;

            fn div(self, rhs: PolynomialInverse) -> Self::Output {
                PolynomialInverse::from(self) / rhs
            }
        }
    };
}

impl From<Polynomial> for PolynomialInverse {
    fn from(value: Polynomial) -> Self {
        let coeff = value.coeff().iter().rev().cloned().collect::<Vec<f32>>();
        PolynomialInverse(crate::poly::Polynomial::new(coeff.as_slice()))
    }
}

impl From<f32> for PolynomialInverse {
    fn from(value: f32) -> Self {
        Self(crate::poly::Polynomial::new(&[value]))
    }
}

impl_poly_ops!(f32);

impl From<z_inv> for PolynomialInverse {
    fn from(_value: z_inv) -> Self {
        Self(crate::poly::Polynomial::new(&[1.0, 0.0]))
    }
}

impl_poly_ops!(z_inv);

impl Display for PolynomialInverse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = self
            .coeff()
            .iter()
            .enumerate()
            .map(|(i, &coeff)| {
                if i == 0 {
                    format!("{}", coeff)
                } else {
                    format!("{}*z^-{}", coeff, i)
                }
            })
            .collect::<Vec<String>>()
            .join(" + ");

        write!(f, "{}", string)
    }
}

#[cfg(all(test, feature = "std"))]
mod test_f32_impl {
    use super::*;

    #[test]
    fn test_f32_add() {
        let p1 = 5.0;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_add_f32() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_sub() {
        let p1 = 5.0;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[-1.0, 3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_sub_f32() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[1.0, -3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_mul() {
        let p1 = 2.0;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[2.0, 4.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_mul_f32() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = 2.0;
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[2.0, 4.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_div() {
        let p1 = 6.0;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 / p2;
        assert_eq!(result, DTf::new(&[6.0], &[1.0, 2.0]));
    }

    #[test]
    #[should_panic(expected = "Denominator must have degree greater than or equal to numerator.")]
    fn test_div_f32() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = 6.0;
        let _result = p1 / p2;
    }
}

#[cfg(all(test, feature = "std"))]
mod test_s_impl {
    use super::*;

    #[test]
    fn test_s_add() {
        let p1 = z_inv;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_add_s() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = z_inv;
        let result = p1 + p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_s_sub() {
        let p1 = z_inv;
        let p2 = PolynomialInverse::new(&[3.0, 2.0]);
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[-2.0, -2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_sub_s() {
        let p1 = PolynomialInverse::new(&[3.0, 2.0]);
        let p2 = z_inv;
        let result = p1 - p2;
        assert_eq!(result.coeff(), &[2.0, 2.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_s_mul() {
        let p1 = z_inv;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[1.0, 2.0, 0.0]);
        assert_eq!(result.degree(), 2);
    }

    #[test]
    fn test_mul_s() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0]);
        let p2 = z_inv;
        let result = p1 * p2;
        assert_eq!(result.coeff(), &[1.0, 2.0, 0.0]);
        assert_eq!(result.degree(), 2);
    }

    #[test]
    fn test_s_div() {
        let p1 = z_inv;
        let p2 = PolynomialInverse::new(&[1.0, 2.0]);
        let result = p1 / p2;
        assert_eq!(result, DTf::new(&[1.0, 0.0], &[1.0, 2.0]));
    }

    #[test]
    #[should_panic(expected = "Denominator must have degree greater than or equal to numerator.")]
    fn test_div_s() {
        let p1 = PolynomialInverse::new(&[1.0, 2.0, 3.0]);
        let p2 = z_inv;
        let _result = p1 / p2;
    }
}
