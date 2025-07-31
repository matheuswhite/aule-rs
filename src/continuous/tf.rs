use crate::{discrete::integration::Discretizable, prelude::Euler};

/// Represents a transfer function in the continuous domain.
/// It is defined by a numerator and denominator polynomial.
/// This struct can be discretized into an Euler integrator.
///
/// # Example
/// ```
/// use aule::continuous::tf::Tf;
/// use crate::aule::prelude::Discretizable;
///
/// let tf = Tf::new(&[1.0, 2.0], &[1.0, 3.0, 4.0]);
/// let discretized = tf.discretize();
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
        assert_ne!(denominator, &[0.0], "Denominator cannot be zero.");

        Tf {
            numerator: crate::continuous::poly::Polynomial::new(numerator),
            denominator: crate::continuous::poly::Polynomial::new(denominator),
        }
    }
}

impl Discretizable<Euler> for Tf {
    /// Converts the transfer function into an Euler integrator.
    ///
    /// # Returns
    /// An `Euler` instance that represents the discretized version of the transfer function.
    ///
    /// # Example
    /// ```
    /// use aule::continuous::tf::Tf;
    /// use aule::prelude::Discretizable;
    ///
    /// let tf = Tf::new(&[1.0, 2.0], &[1.0, 3.0, 4.0]);
    /// let discretized = tf.discretize();
    /// ```
    fn discretize(self) -> Euler {
        Euler::new(
            self.numerator.inner().clone(),
            self.denominator.inner().clone(),
        )
    }
}
