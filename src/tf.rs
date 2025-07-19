use crate::{s::s, ss::StateSpace};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Tf {
    numerator: Vec<f32>,
    denominator: Vec<f32>,
}

impl Tf {
    pub fn new(numerator: &[f32], denominator: &[f32]) -> Self {
        Tf {
            numerator: numerator.to_vec(),
            denominator: denominator.to_vec(),
        }
    }

    pub fn as_ss(self) -> StateSpace {
        StateSpace::new([[1.0; 2]; 2], [1.0; 2], [1.0; 2], 1.0)
        // todo!()
    }
}

impl From<s> for Tf {
    fn from(_s: s) -> Self {
        Tf::new(&[1.0], &[1.0])
    }
}

impl From<f32> for Tf {
    fn from(value: f32) -> Self {
        Tf::new(&[value], &[1.0])
    }
}

impl Add for Tf {
    type Output = Tf;

    fn add(self, rhs: Tf) -> Self::Output {
        self
        // todo!()
    }
}

impl Sub for Tf {
    type Output = Tf;

    fn sub(self, rhs: Tf) -> Self::Output {
        self
        // todo!()
    }
}

impl Mul for Tf {
    type Output = Tf;

    fn mul(self, rhs: Tf) -> Self::Output {
        Tf::new(
            &self
                .numerator
                .iter()
                .zip(rhs.numerator.iter())
                .map(|(a, b)| a * b)
                .collect::<Vec<_>>(),
            &self
                .denominator
                .iter()
                .zip(rhs.denominator.iter())
                .map(|(a, b)| a * b)
                .collect::<Vec<_>>(),
        )
    }
}

impl Div for Tf {
    type Output = Tf;

    fn div(self, rhs: Tf) -> Self::Output {
        Tf::new(
            &self
                .numerator
                .iter()
                .zip(rhs.denominator.iter())
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>(),
            &self
                .denominator
                .iter()
                .zip(rhs.numerator.iter())
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>(),
        )
    }
}

impl Neg for Tf {
    type Output = Tf;

    fn neg(self) -> Self::Output {
        Tf::new(
            &self.numerator.into_iter().map(|x| -x).collect::<Vec<_>>(),
            &self.denominator,
        )
    }
}

macro_rules! impl_ops_for_tf {
    ($type_:ty) => {
        impl Add<$type_> for Tf {
            type Output = Tf;

            fn add(self, rhs: $type_) -> Self::Output {
                self + Tf::from(rhs)
            }
        }

        impl Add<Tf> for $type_ {
            type Output = Tf;

            fn add(self, rhs: Tf) -> Self::Output {
                Tf::from(self) + rhs
            }
        }

        impl Sub<$type_> for Tf {
            type Output = Tf;

            fn sub(self, rhs: $type_) -> Self::Output {
                self - Tf::from(rhs)
            }
        }

        impl Sub<Tf> for $type_ {
            type Output = Tf;

            fn sub(self, rhs: Tf) -> Self::Output {
                Tf::from(self) - rhs
            }
        }

        impl Mul<$type_> for Tf {
            type Output = Tf;

            fn mul(self, rhs: $type_) -> Self::Output {
                self * Tf::from(rhs)
            }
        }

        impl Mul<Tf> for $type_ {
            type Output = Tf;

            fn mul(self, rhs: Tf) -> Self::Output {
                Tf::from(self) * rhs
            }
        }

        impl Div<$type_> for Tf {
            type Output = Tf;

            fn div(self, rhs: $type_) -> Self::Output {
                self / Tf::from(rhs)
            }
        }

        impl Div<Tf> for $type_ {
            type Output = Tf;

            fn div(self, rhs: Tf) -> Self::Output {
                Tf::from(self) / rhs
            }
        }
    };
}

impl_ops_for_tf!(f32);
impl_ops_for_tf!(s);
