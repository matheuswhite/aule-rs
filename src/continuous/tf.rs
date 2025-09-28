use crate::{continuous::ss::SS, poly::Polynomial, prelude::Integrator};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Tf {
    numerator: crate::continuous::poly::Polynomial,
    denominator: crate::continuous::poly::Polynomial,
}

impl Tf {
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
