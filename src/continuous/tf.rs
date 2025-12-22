use crate::{continuous::ss::SS, poly::Polynomial, prelude::Solver};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::AddAssign;
use num_traits::Float;

#[derive(Debug, Clone, PartialEq)]
pub struct Tf<T>
where
    T: Float + Default + AddAssign<T>,
{
    numerator: crate::continuous::poly::Polynomial<T>,
    denominator: crate::continuous::poly::Polynomial<T>,
}

impl<T> Tf<T>
where
    T: Float + Default + AddAssign<T>,
{
    pub fn new(numerator: &[T], denominator: &[T]) -> Self {
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

        let a0 = self.denominator.lead_coeff();
        let a = self
            .denominator
            .coeff()
            .iter()
            .map(|x| *x / a0)
            .collect::<Vec<_>>();

        let mut b = self
            .numerator
            .coeff()
            .iter()
            .map(|x| *x / a0)
            .collect::<Vec<_>>();
        for _ in 0..n - m {
            b.insert(0, T::zero());
        }
        let b0 = b[0];

        let a_mat = Polynomial::new(&a).companion_matrix();

        let mut b_mat = vec![T::zero(); n];
        b_mat[n - 1] = T::one();

        let c_mat = b[1..].iter().rev().copied().collect();

        SS::new(a_mat, b_mat, c_mat, b0)
    }
}
