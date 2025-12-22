use crate::continuous::{Tf, s};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T>(crate::poly::Polynomial<T>)
where
    T: Float + Default + AddAssign<T>;

impl<T> Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    pub fn new(coeff: &[T]) -> Self {
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

impl<T> Add for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 + rhs.0)
    }
}

impl<T> Sub for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 - rhs.0)
    }
}

impl<T> Mul for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 * rhs.0)
    }
}

impl<T> Div for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Tf<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Tf::new(self.coeff(), rhs.coeff())
    }
}

impl<T> Neg for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn neg(self) -> Self::Output {
        Polynomial(-self.0)
    }
}

impl<T> Add<T> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: T) -> Self::Output {
        self + Polynomial::from(rhs)
    }
}

impl<T> Sub<T> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: T) -> Self::Output {
        self - Polynomial::from(rhs)
    }
}

impl<T> Mul<T> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self * Polynomial::from(rhs)
    }
}

impl<T> Div<T> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Tf<T>;

    fn div(self, rhs: T) -> Self::Output {
        self / Polynomial::from(rhs)
    }
}

impl<T> Add<s> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: s) -> Self::Output {
        self + Polynomial::from(rhs)
    }
}

impl<T> Sub<s> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: s) -> Self::Output {
        self - Polynomial::from(rhs)
    }
}

impl<T> Mul<s> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: s) -> Self::Output {
        self * Polynomial::from(rhs)
    }
}

impl<T> Div<s> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    type Output = Tf<T>;

    fn div(self, rhs: s) -> Self::Output {
        self / Polynomial::from(rhs)
    }
}

macro_rules! impl_poly_ops {
    ($type:ty, $poly_type:ty) => {
        impl Add<Polynomial<$poly_type>> for $type {
            type Output = Polynomial<$poly_type>;

            fn add(self, rhs: Polynomial<$poly_type>) -> Self::Output {
                Polynomial::from(self as $poly_type) + rhs
            }
        }

        impl Sub<Polynomial<$poly_type>> for $type {
            type Output = Polynomial<$poly_type>;

            fn sub(self, rhs: Polynomial<$poly_type>) -> Self::Output {
                Polynomial::from(self as $poly_type) - rhs
            }
        }

        impl Mul<Polynomial<$poly_type>> for $type {
            type Output = Polynomial<$poly_type>;

            fn mul(self, rhs: Polynomial<$poly_type>) -> Self::Output {
                Polynomial::from(self as $poly_type) * rhs
            }
        }

        impl Div<Polynomial<$poly_type>> for $type {
            type Output = Tf<$poly_type>;

            fn div(self, rhs: Polynomial<$poly_type>) -> Self::Output {
                Polynomial::from(self as $poly_type) / rhs
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

impl<T> From<T> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    fn from(value: T) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[value]))
    }
}

impl<T> From<s> for Polynomial<T>
where
    T: Float + Default + AddAssign<T>,
{
    fn from(_value: s) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[T::one(), T::zero()]))
    }
}

impl<T> Display for Polynomial<T>
where
    T: Float + Default + AddAssign<T> + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[cfg(all(test, feature = "std"))]
mod test_f32_impl {
    use super::*;

    #[test]
    fn test_f32_add() {
        let p1 = 5.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result: Polynomial<f64> = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_add_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result: Polynomial<f64> = p1 + p2;
        assert_eq!(result.coeff(), &[1.0, 7.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_sub() {
        let p1 = 5.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result: Polynomial<f64> = p1 - p2;
        assert_eq!(result.coeff(), &[-1.0, 3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_sub_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 5.0;
        let result: Polynomial<f64> = p1 - p2;
        assert_eq!(result.coeff(), &[1.0, -3.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_f32_mul() {
        let p1 = 2.0;
        let p2 = Polynomial::new(&[1.0, 2.0]);
        let result: Polynomial<f64> = p1 * p2;
        assert_eq!(result.coeff(), &[2.0, 4.0]);
        assert_eq!(result.degree(), 1);
    }

    #[test]
    fn test_mul_f32() {
        let p1 = Polynomial::new(&[1.0, 2.0]);
        let p2 = 2.0;
        let result: Polynomial<f64> = p1 * p2;
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

#[cfg(all(test, feature = "std"))]
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
