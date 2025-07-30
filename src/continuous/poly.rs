use crate::continuous::{Tf, s};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

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
