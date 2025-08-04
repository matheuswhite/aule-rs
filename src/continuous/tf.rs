use std::fmt::Debug;

use crate::{continuous::ss::SS, poly::Polynomial, prelude::Integrator};

/// Represents a transfer function in the continuous domain.
/// It is defined by a numerator and denominator polynomial.
///
/// # Example
/// ```
/// use aule::prelude::*;
///
/// let tf = Tf::new(&[1.0, 2.0], &[1.0, 3.0, 4.0]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Tf {
    numerator: crate::continuous::poly::Polynomial,
    denominator: crate::continuous::poly::Polynomial,
}

impl Tf {
    /// Creates a new transfer function with the given numerator and denominator coefficients.
    ///
    /// # Arguments
    /// * `numerator` - A slice of coefficients for the numerator polynomial.
    /// * `denominator` - A slice of coefficients for the denominator polynomial.
    ///
    /// # Returns
    /// A new `Tf` instance representing the transfer function.
    /// # Panics
    /// Panics if the denominator polynomial is zero or has a degree less than the numerator polynomial.
    ///
    /// # Example
    /// ```
    /// use aule::continuous::tf::Tf;
    ///
    /// let tf = Tf::new(&[1.0, 2.0], &[1.0, 3.0, 4.0]);
    /// ```
    pub fn new(numerator: &[f32], denominator: &[f32]) -> Self {
        assert!(!denominator.is_empty(), "Denominator cannot be empty.");
        assert!(
            denominator.len() >= numerator.len(),
            "Denominator must have degree greater than or equal to numerator."
        );

        Tf {
            numerator: crate::continuous::poly::Polynomial::new(numerator),
            denominator: crate::continuous::poly::Polynomial::new(denominator),
        }
    }
}

impl<I> From<Tf> for SS<I>
where
    I: Integrator + Debug,
{
    /// Converts the transfer function into a state-space representation.
    ///
    /// # Returns
    /// An `SS` instance that represents the state-space equivalent of the transfer function.
    ///
    /// # Example
    /// ```
    /// use aule::prelude::*;
    ///
    /// let tf = Tf::new(&[1.0, 2.0], &[1.0, 3.0, 4.0]);
    /// let ss: SS<Euler> = SS::from(tf);
    /// ```
    fn from(tf: Tf) -> Self {
        // Controllable Canonical Form

        // safe because isn't empty
        let n = tf.denominator.degree() as usize;
        // safe because isn't empty
        let m = tf.numerator.degree() as usize;

        let a0 = tf.denominator.lead_coeff();
        let a = tf
            .denominator
            .coeff()
            .iter()
            .map(|x| x / a0)
            .collect::<Vec<_>>();

        let mut b = tf
            .numerator
            .coeff()
            .iter()
            .map(|x| x / a0)
            .collect::<Vec<_>>();
        for _ in 0..n - m {
            b.insert(0, 0.0);
        }
        let b0 = b[0];

        let a_mat = Polynomial::new(&a).companion_matrix();

        let mut b_mat = vec![0.0; n];
        b_mat[n - 1] = 1.0;

        let c_mat = b[1..].iter().rev().map(|c| *c).collect();

        SS::new(a_mat, b_mat, c_mat, b0)
    }
}
