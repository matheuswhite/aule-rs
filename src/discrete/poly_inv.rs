use crate::discrete::{DTf, z_inv};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct PolynomialInverse<T>(crate::poly::Polynomial<T>)
where
    T: Float + Default + AddAssign<T>;

impl<T> PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    pub fn new(coeff: &[T]) -> Self {
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

    pub fn coeff(&self) -> &[T] {
        self.0.coeff()
    }

    pub fn lead_coeff(&self) -> T {
        self.0.lead_coeff()
    }

    pub fn inner(&self) -> &crate::poly::Polynomial<T> {
        &self.0
    }
}

impl<T> Add for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> Sub for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T> Mul for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> Div for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = DTf<T>;

    fn div(self, rhs: Self) -> Self::Output {
        DTf::new(self.coeff(), rhs.coeff())
    }
}

impl<T> Neg for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T> Add<T> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn add(self, rhs: T) -> Self::Output {
        self + PolynomialInverse::from(rhs)
    }
}

impl<T> Sub<T> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn sub(self, rhs: T) -> Self::Output {
        self - PolynomialInverse::from(rhs)
    }
}

impl<T> Mul<T> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self * PolynomialInverse::from(rhs)
    }
}

impl<T> Div<T> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = DTf<T>;

    fn div(self, rhs: T) -> Self::Output {
        self / PolynomialInverse::from(rhs)
    }
}

impl<T> Add<z_inv> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn add(self, rhs: z_inv) -> Self::Output {
        self + PolynomialInverse::from(rhs)
    }
}

impl<T> Sub<z_inv> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn sub(self, rhs: z_inv) -> Self::Output {
        self - PolynomialInverse::from(rhs)
    }
}

impl<T> Mul<z_inv> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = PolynomialInverse<T>;

    fn mul(self, rhs: z_inv) -> Self::Output {
        self * PolynomialInverse::from(rhs)
    }
}

impl<T> Div<z_inv> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = DTf<T>;

    fn div(self, rhs: z_inv) -> Self::Output {
        self / PolynomialInverse::from(rhs)
    }
}

macro_rules! impl_poly_ops {
    ($type:ty, $poly_type:ty) => {
        impl Add<PolynomialInverse<$poly_type>> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn add(self, rhs: PolynomialInverse<$poly_type>) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) + rhs
            }
        }

        impl Sub<PolynomialInverse<$poly_type>> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn sub(self, rhs: PolynomialInverse<$poly_type>) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) - rhs
            }
        }

        impl Mul<PolynomialInverse<$poly_type>> for $type {
            type Output = PolynomialInverse<$poly_type>;

            fn mul(self, rhs: PolynomialInverse<$poly_type>) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) * rhs
            }
        }

        impl Div<PolynomialInverse<$poly_type>> for $type {
            type Output = DTf<$poly_type>;

            fn div(self, rhs: PolynomialInverse<$poly_type>) -> Self::Output {
                PolynomialInverse::from(self as $poly_type) / rhs
            }
        }
    };
}

impl_poly_ops!(f32, f32);
impl_poly_ops!(f64, f64);

impl_poly_ops!(u8, f64);
impl_poly_ops!(u16, f64);
impl_poly_ops!(u32, f64);
impl_poly_ops!(u64, f64);
impl_poly_ops!(usize, f64);
impl_poly_ops!(u128, f64);

impl_poly_ops!(i8, f64);
impl_poly_ops!(i16, f64);
impl_poly_ops!(i32, f64);
impl_poly_ops!(i64, f64);
impl_poly_ops!(isize, f64);
impl_poly_ops!(i128, f64);

impl<T> From<T> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    fn from(value: T) -> Self {
        PolynomialInverse(crate::poly::Polynomial::new(&[value]))
    }
}

impl<T> From<z_inv> for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T>,
{
    fn from(_value: z_inv) -> Self {
        PolynomialInverse(crate::poly::Polynomial::new(&[T::one(), T::zero()]))
    }
}

impl<T> Display for PolynomialInverse<T>
where
    T: Float + Default + AddAssign<T> + Display,
{
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
        let result: PolynomialInverse<f64> = p1 + p2;
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
        let result: PolynomialInverse<f64> = p1 - p2;
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
        let result: PolynomialInverse<f64> = p1 * p2;
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
