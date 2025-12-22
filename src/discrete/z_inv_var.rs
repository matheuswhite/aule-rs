use crate::discrete::{DTf, PolynomialInverse};
use core::ops::{Add, AddAssign, Div, Mul, Sub};
use num_traits::Float;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct z_inv;

impl<T> Add<T> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn add(self, rhs: T) -> Self::Output {
        PolynomialInverse::from(self) + rhs
    }
}

impl<T> Add<PolynomialInverse<T>> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn add(self, rhs: PolynomialInverse<T>) -> Self::Output {
        PolynomialInverse::from(self) + rhs
    }
}

impl<T> Sub<T> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn sub(self, rhs: T) -> Self::Output {
        PolynomialInverse::from(self) - rhs
    }
}

impl<T> Sub<PolynomialInverse<T>> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn sub(self, rhs: PolynomialInverse<T>) -> Self::Output {
        PolynomialInverse::from(self) - rhs
    }
}

impl<T> Mul<T> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn mul(self, rhs: T) -> Self::Output {
        PolynomialInverse::from(self) * rhs
    }
}

impl<T> Mul<PolynomialInverse<T>> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn mul(self, rhs: PolynomialInverse<T>) -> Self::Output {
        PolynomialInverse::from(self) * rhs
    }
}

impl Mul<z_inv> for z_inv {
    type Output = PolynomialInverse<f64>;

    fn mul(self, rhs: z_inv) -> Self::Output {
        PolynomialInverse::from(self) * PolynomialInverse::from(rhs)
    }
}

impl<T> Div<PolynomialInverse<T>> for z_inv
where
    T: Float + Default + AddAssign<T>,
{
    type Output = DTf<T>;

    fn div(self, rhs: PolynomialInverse<T>) -> Self::Output {
        PolynomialInverse::from(self) / rhs
    }
}

macro_rules! impl_z_inv_ops {
    ($type:ty, $poly_type:ty) => {
        impl Add<z_inv> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn add(self, rhs: z_inv) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) + PolynomialInverse::from(rhs)
            }
        }

        impl Sub<z_inv> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn sub(self, rhs: z_inv) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) - PolynomialInverse::from(rhs)
            }
        }

        impl Mul<z_inv> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn mul(self, rhs: z_inv) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) * PolynomialInverse::from(rhs)
            }
        }

        impl Div<z_inv> for $type {
            type Output = DTf<$poly_type>;

            fn div(self, rhs: z_inv) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) / PolynomialInverse::from(rhs)
            }
        }
    };
}

impl_z_inv_ops!(f32, f32);
impl_z_inv_ops!(f64, f64);

impl_z_inv_ops!(u8, f64);
impl_z_inv_ops!(u16, f64);
impl_z_inv_ops!(u32, f64);
impl_z_inv_ops!(u64, f64);
impl_z_inv_ops!(u128, f64);
impl_z_inv_ops!(usize, f64);

impl_z_inv_ops!(i8, f64);
impl_z_inv_ops!(i16, f64);
impl_z_inv_ops!(i32, f64);
impl_z_inv_ops!(i64, f64);
impl_z_inv_ops!(i128, f64);
impl_z_inv_ops!(isize, f64);
