use crate::continuous::{Tf, s};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A polynomial in the form of a transfer function.
/// It can be used to represent continuous-time systems.
///
/// # Examples
/// ```
/// use aule::continuous::Polynomial;
///
/// let empty: &[f32] = &[];
/// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
/// assert_eq!(p.degree(), 2);
/// assert_eq!(p.coeff(), &[1.0, 2.0, 3.0]);
/// assert_eq!(p.lead_coeff(), 1.0);
/// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
/// assert_eq!(p2.degree(), 1);
/// assert_eq!(p2.coeff(), &[1.0, 2.0]);
/// assert_eq!(p2.lead_coeff(), 1.0);
/// let p3 = Polynomial::empty();
/// assert_eq!(p3.degree(), -1);
/// assert_eq!(p3.coeff(), empty);
/// assert_eq!(p3.lead_coeff(), 0.0);
/// let p4 = Polynomial::new(&[]);
/// assert_eq!(p4.degree(), -1);
/// assert_eq!(p4.coeff(), empty);
/// assert_eq!(p4.lead_coeff(), 0.0);
/// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(p5.degree(), 3);
/// assert_eq!(p5.coeff(), &[1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(p5.lead_coeff(), 1.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial(crate::poly::Polynomial);

impl Polynomial {
    /// Creates a new polynomial from the given coefficients.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.degree(), 2);
    /// assert_eq!(p.coeff(), &[1.0, 2.0, 3.0]);
    /// assert_eq!(p.lead_coeff(), 1.0);
    /// ```
    pub fn new(coeff: &[f32]) -> Self {
        Polynomial(crate::poly::Polynomial::new(coeff))
    }

    /// Creates an empty polynomial.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let empty: &[f32] = &[];
    /// let p = Polynomial::empty();
    /// assert_eq!(p.degree(), -1);
    /// assert_eq!(p.coeff(), empty);
    /// assert_eq!(p.lead_coeff(), 0.0);
    /// ```
    pub fn empty() -> Self {
        Polynomial(crate::poly::Polynomial::empty())
    }

    /// Raises the polynomial to the given power.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = p.pow(2);
    /// assert_eq!(p2.coeff(), &[1.0, 4.0, 4.0]);
    /// assert_eq!(p2.degree(), 2);
    /// ```
    pub fn pow(self, exp: usize) -> Self {
        Polynomial(self.0.pow(exp))
    }

    /// Returns the degree of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.degree(), 2);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.degree(), 1);
    /// let p3 = Polynomial::empty();
    /// assert_eq!(p3.degree(), -1);
    /// let p4 = Polynomial::new(&[]);
    /// assert_eq!(p4.degree(), -1);
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(p5.degree(), 3);
    /// ```
    pub fn degree(&self) -> isize {
        self.0.degree()
    }

    /// Returns the coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let empty: &[f32] = &[];
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.coeff(), &[1.0, 2.0, 3.0]);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.coeff(), &[1.0, 2.0]);
    /// let p3 = Polynomial::empty();
    /// assert_eq!(p3.coeff(), empty);
    /// let p4 = Polynomial::new(&[]);
    /// assert_eq!(p4.coeff(), empty);
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(p5.coeff(), &[1.0, 2.0, 3.0, 4.0]);
    /// ```
    pub fn coeff(&self) -> &[f32] {
        self.0.coeff()
    }

    /// Returns the leading coefficient of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(p.lead_coeff(), 1.0);
    /// let p2 = Polynomial::new(&[0.0, 0.0, 1.0, 2.0]);
    /// assert_eq!(p2.lead_coeff(), 1.0);
    /// let p3 = Polynomial::empty();
    /// assert_eq!(p3.lead_coeff(), 0.0);
    /// let p4 = Polynomial::new(&[]);
    /// assert_eq!(p4.lead_coeff(), 0.0);
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(p5.lead_coeff(), 1.0);
    /// ```
    pub fn lead_coeff(&self) -> f32 {
        self.0.lead_coeff()
    }

    /// Returns the inner polynomial representation.
    /// This is useful for interoperability with other parts of the library.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// let inner = p.inner();
    /// assert_eq!(inner.degree(), 2);
    /// assert_eq!(inner.coeff(), &[1.0, 2.0, 3.0]);
    /// assert_eq!(inner.lead_coeff(), 1.0);
    /// ```
    pub fn inner(&self) -> &crate::poly::Polynomial {
        &self.0
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    /// Adds two polynomials together.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = Polynomial::new(&[3.0, 4.0]);
    /// let p3 = p1 + p2;
    /// assert_eq!(p3.coeff(), &[4.0, 6.0]);
    /// assert_eq!(p3.degree(), 1);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 + rhs.0)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    /// Subtracts one polynomial from another.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[5.0, 6.0]);
    /// let p2 = Polynomial::new(&[3.0, 4.0]);
    /// let p3 = p1 - p2;
    /// assert_eq!(p3.coeff(), &[2.0, 2.0]);
    /// assert_eq!(p3.degree(), 1);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 - rhs.0)
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    /// Multiplies two polynomials together.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = Polynomial::new(&[3.0, 4.0]);
    /// let p3 = p1 * p2;
    /// assert_eq!(p3.coeff(), &[3.0, 10.0, 8.0]);
    /// assert_eq!(p3.degree(), 2);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Polynomial(self.0 * rhs.0)
    }
}

impl Div for Polynomial {
    type Output = Tf;

    /// Divides one polynomial by another, returning a transfer function.
    ///
    /// # Panics
    /// Panics if the denominator polynomial is zero or has a degree less than the numerator polynomial
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::{Polynomial, Tf};
    ///
    /// let p1 = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = Polynomial::new(&[3.0, 4.0]);
    /// let tf = p1 / p2;
    /// assert_eq!(tf, Tf::new(&[1.0, 2.0], &[3.0, 4.0]));
    /// ```
    ///
    /// ```
    /// use aule::continuous::{Polynomial, Tf};
    /// use std::panic::catch_unwind;
    /// let p1 = Polynomial::new(&[1.0, 2.0]);
    /// let p2 = Polynomial::new(&[0.0]);
    /// let result = catch_unwind(|| {
    ///     let _ = p1 / p2;
    /// });
    /// assert!(result.is_err());
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        Tf::new(self.coeff(), rhs.coeff())
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    /// Negates the polynomial, effectively multiplying all coefficients by -1.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, -2.0, 3.0]);
    /// let neg_p = -p;
    /// assert_eq!(neg_p.coeff(), &[-1.0, 2.0, -3.0]);
    /// assert_eq!(neg_p.degree(), 2);
    /// ```
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
    /// Creates a polynomial from a single f32 value.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::from(5.0);
    /// assert_eq!(p.coeff(), &[5.0]);
    /// assert_eq!(p.degree(), 0);
    /// ```
    fn from(value: f32) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[value]))
    }
}

impl_poly_ops!(f32);

impl From<s> for Polynomial {
    /// Creates a polynomial from a single `s` value, which is typically used to represent the variable in transfer functions.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    /// use aule::prelude::*;
    /// use aule::s;
    ///
    /// let p = Polynomial::from(s);
    /// assert_eq!(p.coeff(), &[1.0, 0.0]);
    /// assert_eq!(p.degree(), 1);
    /// ```
    fn from(_value: s) -> Self {
        Polynomial(crate::poly::Polynomial::new(&[1.0, 0.0]))
    }
}

impl_poly_ops!(s);

impl Display for Polynomial {
    /// Formats the polynomial as a string.
    /// This is useful for debugging and displaying the polynomial in a human-readable form.
    ///
    /// # Examples
    /// ```
    /// use aule::continuous::Polynomial;
    ///
    /// let p = Polynomial::new(&[1.0, 2.0, 3.0]);
    /// assert_eq!(format!("{}", p), "1*s^2 + 2*s + 3");
    /// let p2 = Polynomial::new(&[0.0, 1.0, 2.0]);
    /// assert_eq!(format!("{}", p2), "1*s + 2");
    /// let p3 = Polynomial::empty();
    /// assert_eq!(format!("{}", p3), "");
    /// let p4 = Polynomial::new(&[]);
    /// assert_eq!(format!("{}", p4), "");
    /// let p5 = Polynomial::new(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(format!("{}", p5), "1*s^3 + 2*s^2 + 3*s + 4");
    /// ```
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
