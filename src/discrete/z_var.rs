use crate::discrete::{DTf, Polynomial};
use core::ops::{Add, AddAssign, Div, Mul, Sub};
use num_traits::Float;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct z;

impl<T> Add<T> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: T) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl<T> Sub<T> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl<T> Add<Polynomial<T>> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl<T> Sub<Polynomial<T>> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl<T> Mul<T> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl<T> Mul<Polynomial<T>> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl Mul<z> for z {
    type Output = Polynomial<f64>;

    fn mul(self, rhs: z) -> Self::Output {
        Polynomial::from(self) * Polynomial::from(rhs)
    }
}

impl<T> Div<Polynomial<T>> for z
where
    T: Float + Default + AddAssign<T>,
{
    type Output = DTf<T>;

    fn div(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial::from(self) / rhs
    }
}

macro_rules! impl_z_ops {
    ($type:ty, $poly_type:ty) => {
        impl Add<z> for $type {
            type Output = Polynomial<$poly_type>;

            fn add(self, rhs: z) -> Self::Output {
                Polynomial::from(self as $poly_type) + Polynomial::from(rhs)
            }
        }

        impl Sub<z> for $type {
            type Output = Polynomial<$poly_type>;

            fn sub(self, rhs: z) -> Self::Output {
                Polynomial::from(self as $poly_type) - Polynomial::from(rhs)
            }
        }

        impl Mul<z> for $type {
            type Output = Polynomial<$poly_type>;

            fn mul(self, rhs: z) -> Self::Output {
                Polynomial::from(self as $poly_type) * Polynomial::from(rhs)
            }
        }

        impl Div<z> for $type {
            type Output = DTf<$poly_type>;

            fn div(self, rhs: z) -> Self::Output {
                Polynomial::from(self as $poly_type) / Polynomial::from(rhs)
            }
        }
    };
}

impl_z_ops!(f32, f32);
impl_z_ops!(f64, f64);

impl_z_ops!(u8, f64);
impl_z_ops!(u16, f64);
impl_z_ops!(u32, f64);
impl_z_ops!(u64, f64);
impl_z_ops!(u128, f64);
impl_z_ops!(usize, f64);

impl_z_ops!(i8, f64);
impl_z_ops!(i16, f64);
impl_z_ops!(i32, f64);
impl_z_ops!(i64, f64);
impl_z_ops!(i128, f64);
impl_z_ops!(isize, f64);
