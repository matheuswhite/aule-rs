use crate::{
    block::Block,
    math::{recip_of_count::RecipOfCount, scale::Scale, zeroish::Zeroish},
    prelude::SimulationState,
};
use core::ops::{AddAssign, Mul};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ISE<T> {
    acc: T,
    n: usize,
}

impl<T> ISE<T>
where
    T: Zeroish + Clone + Scale,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zeroish(&self.acc)
        } else {
            self.acc
                .clone()
                .scale(<T::Alpha as RecipOfCount>::recip_of_count(self.n))
        }
    }
}

impl<T> Block for ISE<T>
where
    T: Mul<Output = T> + AddAssign + Clone + Zeroish,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.acc += input.clone() * input.clone();
        self.n += 1;
        input
    }

    fn reset(&mut self) {
        self.acc = T::zeroish(&self.acc);
        self.n = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use nalgebra::{DMatrix, SMatrix};
    use num_complex::Complex;

    fn first_state() -> SimulationState {
        Simulation::new(1e-3, 1.0)
            .next()
            .expect("simulation should yield at least one state")
    }

    // ───────────────────────────── f32 ─────────────────────────────

    #[test]
    fn f32_empty_returns_zero() {
        let ise = ISE::<f32>::default();
        assert_eq!(ise.value(), 0.0_f32);
    }

    #[test]
    fn f32_averages_squared_values() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        ise.block(2.0, state);
        ise.block(-3.0, state);
        ise.block(4.0, state);
        // (4 + 9 + 16) / 3 = 29 / 3
        let expected = 29.0_f32 / 3.0;
        assert!((ise.value() - expected).abs() < 1e-6);
    }

    #[test]
    fn f32_block_returns_input_unchanged() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        assert_eq!(ise.block(-5.0, state), -5.0_f32);
        assert_eq!(ise.block(7.0, state), 7.0_f32);
    }

    #[test]
    fn f32_reset_clears_accumulator() {
        let mut ise = ISE::<f32>::default();
        let state = first_state();
        ise.block(10.0, state);
        ise.block(-6.0, state);
        ise.reset();
        assert_eq!(ise.value(), 0.0_f32);
        ise.block(2.0, state);
        assert_eq!(ise.value(), 4.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_empty_returns_zero() {
        let ise = ISE::<f64>::default();
        assert_eq!(ise.value(), 0.0_f64);
    }

    #[test]
    fn f64_averages_squared_values() {
        let mut ise = ISE::<f64>::default();
        let state = first_state();
        ise.block(1.0, state);
        ise.block(-2.0, state);
        ise.block(3.0, state);
        // (1 + 4 + 9) / 3 = 14 / 3
        let expected = 14.0_f64 / 3.0;
        assert!((ise.value() - expected).abs() < 1e-12);
    }

    #[test]
    fn f64_reset_clears_accumulator() {
        let mut ise = ISE::<f64>::default();
        let state = first_state();
        ise.block(8.0, state);
        ise.block(-4.0, state);
        ise.reset();
        assert_eq!(ise.value(), 0.0_f64);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_empty_returns_zero() {
        let ise = ISE::<Complex<f32>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(ise.value(), Complex::new(0.0_f32, 0.0));
    }

    #[test]
    fn complex_f32_divides_accumulated_by_count() {
        let ise = ISE::<Complex<f32>> {
            acc: Complex::new(6.0, 8.0),
            n: 2,
        };
        assert_eq!(ise.value(), Complex::new(3.0_f32, 4.0));
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_empty_returns_zero() {
        let ise = ISE::<Complex<f64>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(ise.value(), Complex::new(0.0_f64, 0.0));
    }

    #[test]
    fn complex_f64_divides_accumulated_by_count() {
        let ise = ISE::<Complex<f64>> {
            acc: Complex::new(9.0, -12.0),
            n: 3,
        };
        assert_eq!(ise.value(), Complex::new(3.0_f64, -4.0));
    }

    // ───────────────────────────── DMatrix<f32> ─────────────────────────────

    #[test]
    fn dmatrix_f32_empty_returns_zero_shaped_like_prototype() {
        let ise = ISE::<DMatrix<f32>> {
            acc: DMatrix::<f32>::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            n: 0,
        };
        assert_eq!(ise.value(), DMatrix::<f32>::zeros(2, 3));
    }

    #[test]
    fn dmatrix_f32_divides_accumulated_by_count() {
        let ise = ISE::<DMatrix<f32>> {
            acc: DMatrix::<f32>::from_row_slice(2, 2, &[2.0, 4.0, 6.0, 8.0]),
            n: 2,
        };
        let expected = DMatrix::<f32>::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(ise.value(), expected);
    }

    // ───────────────────────────── DMatrix<f64> ─────────────────────────────

    #[test]
    fn dmatrix_f64_empty_returns_zero_shaped_like_prototype() {
        let ise = ISE::<DMatrix<f64>> {
            acc: DMatrix::<f64>::from_row_slice(3, 1, &[1.0, 2.0, 3.0]),
            n: 0,
        };
        assert_eq!(ise.value(), DMatrix::<f64>::zeros(3, 1));
    }

    #[test]
    fn dmatrix_f64_divides_accumulated_by_count() {
        let ise = ISE::<DMatrix<f64>> {
            acc: DMatrix::<f64>::from_row_slice(2, 2, &[4.0, 8.0, 12.0, 16.0]),
            n: 4,
        };
        let expected = DMatrix::<f64>::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(ise.value(), expected);
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_empty_returns_zero() {
        let ise = ISE::<SMatrix<f32, 3, 1>> {
            acc: SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0),
            n: 0,
        };
        assert_eq!(ise.value(), SMatrix::<f32, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f32_column_vector_divides_by_count() {
        let ise = ISE::<SMatrix<f32, 3, 1>> {
            acc: SMatrix::<f32, 3, 1>::new(2.0, 4.0, 6.0),
            n: 2,
        };
        assert_eq!(ise.value(), SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn smatrix_f32_square_divides_by_count() {
        let ise = ISE::<SMatrix<f32, 2, 2>> {
            acc: SMatrix::<f32, 2, 2>::new(3.0, 6.0, 9.0, 12.0),
            n: 3,
        };
        assert_eq!(ise.value(), SMatrix::<f32, 2, 2>::new(1.0, 2.0, 3.0, 4.0));
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_empty_returns_zero() {
        let ise = ISE::<SMatrix<f64, 3, 1>> {
            acc: SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0),
            n: 0,
        };
        assert_eq!(ise.value(), SMatrix::<f64, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f64_column_vector_divides_by_count() {
        let ise = ISE::<SMatrix<f64, 3, 1>> {
            acc: SMatrix::<f64, 3, 1>::new(5.0, 10.0, 15.0),
            n: 5,
        };
        assert_eq!(ise.value(), SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn smatrix_f64_square_divides_by_count() {
        let ise = ISE::<SMatrix<f64, 2, 2>> {
            acc: SMatrix::<f64, 2, 2>::new(2.0, 4.0, 6.0, 8.0),
            n: 2,
        };
        assert_eq!(ise.value(), SMatrix::<f64, 2, 2>::new(1.0, 2.0, 3.0, 4.0));
    }

    // ───────────────────────────── DMatrix<Complex<f32>> ─────────────────────────────

    #[test]
    fn dmatrix_complex_f32_empty_returns_zero_shaped_like_prototype() {
        let ise = ISE::<DMatrix<Complex<f32>>> {
            acc: DMatrix::<Complex<f32>>::from_row_slice(
                2,
                2,
                &[
                    Complex::new(1.0, 2.0),
                    Complex::new(3.0, 4.0),
                    Complex::new(5.0, 6.0),
                    Complex::new(7.0, 8.0),
                ],
            ),
            n: 0,
        };
        assert_eq!(ise.value(), DMatrix::<Complex<f32>>::zeros(2, 2));
    }

    #[test]
    fn dmatrix_complex_f32_divides_accumulated_by_count() {
        let ise = ISE::<DMatrix<Complex<f32>>> {
            acc: DMatrix::<Complex<f32>>::from_row_slice(
                2,
                2,
                &[
                    Complex::new(2.0, 4.0),
                    Complex::new(4.0, 8.0),
                    Complex::new(6.0, 12.0),
                    Complex::new(8.0, 16.0),
                ],
            ),
            n: 2,
        };
        let expected = DMatrix::<Complex<f32>>::from_row_slice(
            2,
            2,
            &[
                Complex::new(1.0, 2.0),
                Complex::new(2.0, 4.0),
                Complex::new(3.0, 6.0),
                Complex::new(4.0, 8.0),
            ],
        );
        assert_eq!(ise.value(), expected);
    }

    // ───────────────────────────── DMatrix<Complex<f64>> ─────────────────────────────

    #[test]
    fn dmatrix_complex_f64_empty_returns_zero_shaped_like_prototype() {
        let ise = ISE::<DMatrix<Complex<f64>>> {
            acc: DMatrix::<Complex<f64>>::from_row_slice(
                3,
                1,
                &[
                    Complex::new(1.0, 2.0),
                    Complex::new(3.0, 4.0),
                    Complex::new(5.0, 6.0),
                ],
            ),
            n: 0,
        };
        assert_eq!(ise.value(), DMatrix::<Complex<f64>>::zeros(3, 1));
    }

    #[test]
    fn dmatrix_complex_f64_divides_accumulated_by_count() {
        let ise = ISE::<DMatrix<Complex<f64>>> {
            acc: DMatrix::<Complex<f64>>::from_row_slice(
                2,
                2,
                &[
                    Complex::new(4.0, -8.0),
                    Complex::new(8.0, -16.0),
                    Complex::new(12.0, -24.0),
                    Complex::new(16.0, -32.0),
                ],
            ),
            n: 4,
        };
        let expected = DMatrix::<Complex<f64>>::from_row_slice(
            2,
            2,
            &[
                Complex::new(1.0, -2.0),
                Complex::new(2.0, -4.0),
                Complex::new(3.0, -6.0),
                Complex::new(4.0, -8.0),
            ],
        );
        assert_eq!(ise.value(), expected);
    }

    // ───────────────────────────── SMatrix<Complex<f32>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f32_empty_returns_zero() {
        let ise = ISE::<SMatrix<Complex<f32>, 3, 1>> {
            acc: SMatrix::<Complex<f32>, 3, 1>::new(
                Complex::new(1.0, 2.0),
                Complex::new(3.0, 4.0),
                Complex::new(5.0, 6.0),
            ),
            n: 0,
        };
        assert_eq!(ise.value(), SMatrix::<Complex<f32>, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_complex_f32_divides_by_count() {
        let ise = ISE::<SMatrix<Complex<f32>, 3, 1>> {
            acc: SMatrix::<Complex<f32>, 3, 1>::new(
                Complex::new(2.0, 4.0),
                Complex::new(4.0, 8.0),
                Complex::new(6.0, 12.0),
            ),
            n: 2,
        };
        let expected = SMatrix::<Complex<f32>, 3, 1>::new(
            Complex::new(1.0, 2.0),
            Complex::new(2.0, 4.0),
            Complex::new(3.0, 6.0),
        );
        assert_eq!(ise.value(), expected);
    }

    // ───────────────────────────── SMatrix<Complex<f64>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f64_empty_returns_zero() {
        let ise = ISE::<SMatrix<Complex<f64>, 2, 2>> {
            acc: SMatrix::<Complex<f64>, 2, 2>::new(
                Complex::new(1.0, 0.0),
                Complex::new(2.0, 0.0),
                Complex::new(3.0, 0.0),
                Complex::new(4.0, 0.0),
            ),
            n: 0,
        };
        assert_eq!(ise.value(), SMatrix::<Complex<f64>, 2, 2>::zeros());
    }

    #[test]
    fn smatrix_complex_f64_divides_by_count() {
        let ise = ISE::<SMatrix<Complex<f64>, 2, 2>> {
            acc: SMatrix::<Complex<f64>, 2, 2>::new(
                Complex::new(3.0, -3.0),
                Complex::new(6.0, -6.0),
                Complex::new(9.0, -9.0),
                Complex::new(12.0, -12.0),
            ),
            n: 3,
        };
        let expected = SMatrix::<Complex<f64>, 2, 2>::new(
            Complex::new(1.0, -1.0),
            Complex::new(2.0, -2.0),
            Complex::new(3.0, -3.0),
            Complex::new(4.0, -4.0),
        );
        assert_eq!(ise.value(), expected);
    }
}
