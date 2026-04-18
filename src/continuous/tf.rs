use crate::{continuous::ss::SS, poly::Polynomial, prelude::Solver};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::AddAssign;
use faer::Mat;
use faer::traits::ComplexField;
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Tf<T>
where
    T: Float + Default + AddAssign<T> + ComplexField,
{
    numerator: crate::continuous::poly::Polynomial<T>,
    denominator: crate::continuous::poly::Polynomial<T>,
}

impl<T> Tf<T>
where
    T: Float + Default + AddAssign<T> + ComplexField,
{
    pub fn new(numerator: &[T], denominator: &[T]) -> Self {
        assert!(!numerator.is_empty(), "Numerator cannot be empty.");
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

    pub fn to_ss_controllable<I>(self, _integrator: I) -> SS<I, T>
    where
        I: Solver<T> + Debug,
    {
        // safe because isn't empty
        let n = self.denominator.degree() as usize;
        // safe because isn't empty
        let m = self.numerator.degree() as usize;

        assert!(
            m <= n,
            "The denominator degree must be greater than numerator degree"
        );

        let (quotient, remainder) = if n == m {
            self.numerator.inner().poly_div(self.denominator.inner())
        } else {
            (
                Polynomial::new(&[T::zero()]),
                self.numerator.inner().clone(),
            )
        };

        let numerator = remainder;
        let denominator = self.denominator;

        let d = quotient.coeff().first().copied().unwrap_or(T::zero());

        let a0 = denominator.lead_coeff();
        let a = denominator
            .coeff()
            .iter()
            .map(|x| *x / a0)
            .collect::<Vec<_>>();

        let mut b = numerator
            .coeff()
            .iter()
            .map(|x| *x / a0)
            .collect::<Vec<_>>();
        while b.len() < n {
            b.insert(0, T::zero());
        }

        let a_mat = Polynomial::new(&a).transposed_companion_matrix();

        let mut b_mat = vec![T::zero(); n];
        b_mat[n - 1] = T::one();
        let b_mat = Mat::from_fn(n, 1, |i, _| b_mat[i]);

        let c_mat = b.iter().rev().copied().collect::<Vec<_>>();
        let c_mat = Mat::from_fn(1, n, |_, j| c_mat[j]);

        SS::new(a_mat, b_mat, c_mat, d)
    }
}
