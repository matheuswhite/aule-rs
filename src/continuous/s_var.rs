use crate::{continuous::Polynomial, prelude::Tf};
use core::ops::{Add, AddAssign, Div, Mul, Sub};
use num_traits::Float;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct s;

impl<T> Add<T> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: T) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl<T> Add<Polynomial<T>> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl<T> Sub<T> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl<T> Sub<Polynomial<T>> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl<T> Mul<T> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl<T> Mul<Polynomial<T>> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl Mul<s> for s {
    type Output = Polynomial<f64>;

    fn mul(self, rhs: s) -> Self::Output {
        Polynomial::from(self) * Polynomial::from(rhs)
    }
}

impl<T> Div<Polynomial<T>> for s
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Tf<T>;

    fn div(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) / rhs
    }
}

macro_rules! impl_s_ops {
    ($type:ty, $poly_type:ty) => {
        impl Add<s> for $type {
            type Output = Polynomial<$poly_type>;

            fn add(self, rhs: s) -> Self::Output {
                Polynomial::from(self as $poly_type) + Polynomial::from(rhs)
            }
        }

        impl Sub<s> for $type {
            type Output = Polynomial<$poly_type>;

            fn sub(self, rhs: s) -> Self::Output {
                Polynomial::from(self as $poly_type) - Polynomial::from(rhs)
            }
        }

        impl Mul<s> for $type {
            type Output = Polynomial<$poly_type>;

            fn mul(self, rhs: s) -> Self::Output {
                Polynomial::from(self as $poly_type) * Polynomial::from(rhs)
            }
        }

        impl Div<s> for $type {
            type Output = Tf<$poly_type>;

            fn div(self, rhs: s) -> Self::Output {
                Polynomial::from(self as $poly_type) / Polynomial::from(rhs)
            }
        }
    };
}

impl_s_ops!(f32, f32);
impl_s_ops!(f64, f64);

impl_s_ops!(u8, f64);
impl_s_ops!(u16, f64);
impl_s_ops!(u32, f64);
impl_s_ops!(u64, f64);
impl_s_ops!(u128, f64);
impl_s_ops!(usize, f64);

impl_s_ops!(i8, f64);
impl_s_ops!(i16, f64);
impl_s_ops!(i32, f64);
impl_s_ops!(i64, f64);
impl_s_ops!(i128, f64);
impl_s_ops!(isize, f64);
