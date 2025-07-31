use crate::continuous::{Polynomial, Tf};
use std::ops::{Add, Div, Mul, Sub};

/// Represents the continuous variable 's' in transfer functions and polynomials.
/// This struct is used to facilitate operations involving the 's' variable in control systems.
/// It allows for operations like addition, subtraction, multiplication, and division with `f32` values,
/// which represent coefficients or constants in the context of transfer functions.
/// It is designed to work seamlessly with the `Polynomial` and `Tf` types.
/// This struct does not hold any data itself; it serves as a marker type for the 's' variable.
///
/// # Examples
/// ```
/// use aule::prelude::*;
///
/// let poly = 2.0 + s; // Creates a polynomial with 's' variable
/// assert_eq!(poly.coeff(), &[1.0, 2.0]);
/// let tf = 3.0 / s; // Creates a transfer function
/// assert_eq!(tf, Tf::new(&[3.0], &[1.0, 0.0]));
/// ```
#[allow(non_camel_case_types)]
pub struct s;

impl Add<s> for f32 {
    type Output = Polynomial;

    /// Adds a `f32` value to the continuous variable `s`, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = 5.0 + s;
    /// assert_eq!(result.coeff(), &[1.0, 5.0]);
    /// ```
    fn add(self, rhs: s) -> Self::Output {
        self + Polynomial::from(rhs)
    }
}

impl Add<f32> for s {
    type Output = Polynomial;

    /// Adds the continuous variable `s` to a `f32` value, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = s + 5.0;
    /// assert_eq!(result.coeff(), &[1.0, 5.0]);
    /// ```
    fn add(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) + rhs
    }
}

impl Sub<s> for f32 {
    type Output = Polynomial;

    /// Subtracts the continuous variable `s` from a `f32` value, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = 5.0 - s;
    /// assert_eq!(result.coeff(), &[-1.0, 5.0]);
    /// ```
    fn sub(self, rhs: s) -> Self::Output {
        self - Polynomial::from(rhs)
    }
}

impl Sub<f32> for s {
    type Output = Polynomial;

    /// Subtracts a `f32` value from the continuous variable `s`, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = s - 5.0;
    /// assert_eq!(result.coeff(), &[1.0, -5.0]);
    /// ```
    fn sub(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) - rhs
    }
}

impl Mul<s> for f32 {
    type Output = Polynomial;

    /// Multiplies a `f32` value by the continuous variable `s`, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = 2.0 * s;
    /// assert_eq!(result.coeff(), &[2.0, 0.0]);
    /// ```
    fn mul(self, rhs: s) -> Self::Output {
        self * Polynomial::from(rhs)
    }
}

impl Mul<f32> for s {
    type Output = Polynomial;

    /// Multiplies the continuous variable `s` by a `f32` value, returning a `Polynomial`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = s * 2.0;
    /// assert_eq!(result.coeff(), &[2.0, 0.0]);
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) * rhs
    }
}

impl Div<s> for f32 {
    type Output = Tf;

    /// Divides a `f32` value by the continuous variable `s`, returning a `Tf`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = 3.0 / s;
    /// assert_eq!(result, Tf::new(&[3.0], &[1.0, 0.0]));
    /// ```
    fn div(self, rhs: s) -> Self::Output {
        self / Polynomial::from(rhs)
    }
}

impl Div<f32> for s {
    type Output = Tf;

    /// Divides the continuous variable `s` by a `f32` value, returning a `Tf`.
    ///
    /// # Examples
    /// ```
    /// use aule::prelude::*;
    ///
    /// let result = s / 2.0;
    /// assert_eq!(result, Tf::new(&[1.0, 0.0], &[2.0]));
    /// ```
    fn div(self, rhs: f32) -> Self::Output {
        Polynomial::from(self) / rhs
    }
}
