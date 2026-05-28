use crate::{
    block::Block,
    math::{float_point::FloatPoint, sample::Sample},
    prelude::SimulationState,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IAE<T> {
    acc: T,
    n: usize,
}

impl<T> IAE<T>
where
    T: Sample,
{
    pub fn value(&self) -> T {
        if self.n == 0 {
            T::zero()
        } else {
            self.acc
                .clone()
                .scale(<T::Alpha as FloatPoint>::recip_of_count(self.n))
        }
    }
}

impl<T> Block for IAE<T>
where
    T: Sample,
{
    type Input = T;
    type Output = T;

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.acc = self.acc.clone() + input.absolute();
        self.n += 1;
        input
    }

    fn reset(&mut self) {
        self.acc = T::zero();
        self.n = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use nalgebra::SMatrix;
    use num_complex::Complex;

    fn first_state() -> SimulationState {
        Simulation::new(1e-3, 1.0)
            .next()
            .expect("simulation should yield at least one state")
    }

    // ───────────────────────────── f32 ─────────────────────────────

    #[test]
    fn f32_empty_returns_zero() {
        let iae = IAE::<f32>::default();
        assert_eq!(iae.value(), 0.0_f32);
    }

    #[test]
    fn f32_averages_absolute_values() {
        let mut iae = IAE::<f32>::default();
        let state = first_state();
        iae.block(3.0, state);
        iae.block(-2.0, state);
        iae.block(4.0, state);
        assert_eq!(iae.value(), 3.0_f32);
    }

    #[test]
    fn f32_block_returns_input_unchanged() {
        let mut iae = IAE::<f32>::default();
        let state = first_state();
        assert_eq!(iae.block(-5.0, state), -5.0_f32);
        assert_eq!(iae.block(7.0, state), 7.0_f32);
    }

    #[test]
    fn f32_reset_clears_accumulator() {
        let mut iae = IAE::<f32>::default();
        let state = first_state();
        iae.block(10.0, state);
        iae.block(-6.0, state);
        iae.reset();
        assert_eq!(iae.value(), 0.0_f32);
        iae.block(2.0, state);
        assert_eq!(iae.value(), 2.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_empty_returns_zero() {
        let iae = IAE::<f64>::default();
        assert_eq!(iae.value(), 0.0_f64);
    }

    #[test]
    fn f64_averages_absolute_values() {
        let mut iae = IAE::<f64>::default();
        let state = first_state();
        iae.block(1.5, state);
        iae.block(-2.5, state);
        iae.block(2.0, state);
        assert_eq!(iae.value(), 2.0_f64);
    }

    #[test]
    fn f64_reset_clears_accumulator() {
        let mut iae = IAE::<f64>::default();
        let state = first_state();
        iae.block(8.0, state);
        iae.block(-4.0, state);
        iae.reset();
        assert_eq!(iae.value(), 0.0_f64);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_empty_returns_zero() {
        let iae = IAE::<Complex<f32>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(iae.value(), Complex::new(0.0_f32, 0.0));
    }

    #[test]
    fn complex_f32_divides_accumulated_by_count() {
        let iae = IAE::<Complex<f32>> {
            acc: Complex::new(6.0, 8.0),
            n: 2,
        };
        assert_eq!(iae.value(), Complex::new(3.0_f32, 4.0));
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_empty_returns_zero() {
        let iae = IAE::<Complex<f64>> {
            acc: Complex::new(0.0, 0.0),
            n: 0,
        };
        assert_eq!(iae.value(), Complex::new(0.0_f64, 0.0));
    }

    #[test]
    fn complex_f64_divides_accumulated_by_count() {
        let iae = IAE::<Complex<f64>> {
            acc: Complex::new(9.0, -12.0),
            n: 3,
        };
        assert_eq!(iae.value(), Complex::new(3.0_f64, -4.0));
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_empty_returns_zero() {
        let iae = IAE::<SMatrix<f32, 3, 1>> {
            acc: SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0),
            n: 0,
        };
        assert_eq!(iae.value(), SMatrix::<f32, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f32_column_vector_divides_by_count() {
        let iae = IAE::<SMatrix<f32, 3, 1>> {
            acc: SMatrix::<f32, 3, 1>::new(2.0, 4.0, 6.0),
            n: 2,
        };
        assert_eq!(iae.value(), SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn smatrix_f32_square_divides_by_count() {
        let iae = IAE::<SMatrix<f32, 2, 2>> {
            acc: SMatrix::<f32, 2, 2>::new(3.0, 6.0, 9.0, 12.0),
            n: 3,
        };
        assert_eq!(iae.value(), SMatrix::<f32, 2, 2>::new(1.0, 2.0, 3.0, 4.0));
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_empty_returns_zero() {
        let iae = IAE::<SMatrix<f64, 3, 1>> {
            acc: SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0),
            n: 0,
        };
        assert_eq!(iae.value(), SMatrix::<f64, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f64_column_vector_divides_by_count() {
        let iae = IAE::<SMatrix<f64, 3, 1>> {
            acc: SMatrix::<f64, 3, 1>::new(5.0, 10.0, 15.0),
            n: 5,
        };
        assert_eq!(iae.value(), SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn smatrix_f64_square_divides_by_count() {
        let iae = IAE::<SMatrix<f64, 2, 2>> {
            acc: SMatrix::<f64, 2, 2>::new(2.0, 4.0, 6.0, 8.0),
            n: 2,
        };
        assert_eq!(iae.value(), SMatrix::<f64, 2, 2>::new(1.0, 2.0, 3.0, 4.0));
    }

    // ───────────────────────────── SMatrix<Complex<f32>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f32_empty_returns_zero() {
        let iae = IAE::<SMatrix<Complex<f32>, 3, 1>> {
            acc: SMatrix::<Complex<f32>, 3, 1>::new(
                Complex::new(1.0, 2.0),
                Complex::new(3.0, 4.0),
                Complex::new(5.0, 6.0),
            ),
            n: 0,
        };
        assert_eq!(iae.value(), SMatrix::<Complex<f32>, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_complex_f32_divides_by_count() {
        let iae = IAE::<SMatrix<Complex<f32>, 3, 1>> {
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
        assert_eq!(iae.value(), expected);
    }

    // ───────────────────────────── SMatrix<Complex<f64>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f64_empty_returns_zero() {
        let iae = IAE::<SMatrix<Complex<f64>, 2, 2>> {
            acc: SMatrix::<Complex<f64>, 2, 2>::new(
                Complex::new(1.0, 0.0),
                Complex::new(2.0, 0.0),
                Complex::new(3.0, 0.0),
                Complex::new(4.0, 0.0),
            ),
            n: 0,
        };
        assert_eq!(iae.value(), SMatrix::<Complex<f64>, 2, 2>::zeros());
    }

    #[test]
    fn smatrix_complex_f64_divides_by_count() {
        let iae = IAE::<SMatrix<Complex<f64>, 2, 2>> {
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
        assert_eq!(iae.value(), expected);
    }
}
