use crate::{
    block::Block,
    math::{float_point::FloatPoint, sample::Sample},
    prelude::SimulationState,
};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct GoodHart<T>
where
    T: Sample,
{
    error: Vec<T>,
    control_signal: Vec<T>,
    alphas: (T::Alpha, T::Alpha, T::Alpha),
}

impl<T> GoodHart<T>
where
    T: Sample,
{
    pub fn new(alpha1: T::Alpha, alpha2: T::Alpha, alpha3: T::Alpha) -> Self {
        Self {
            error: Vec::new(),
            control_signal: Vec::new(),
            alphas: (alpha1, alpha2, alpha3),
        }
    }

    pub fn value(&self) -> T {
        if self.error.is_empty() || self.control_signal.is_empty() {
            return T::zero();
        }

        let n = self.error.len();
        let one_n = <T::Alpha as FloatPoint>::recip_of_count(n);

        let e1 = self.control_signal.iter().cloned().sum::<T>().scale(one_n);
        let e2 = self
            .control_signal
            .iter()
            .map(|u| u.clone() - e1.clone())
            .sum::<T>()
            .scale(one_n);
        let e3 = self
            .error
            .iter()
            .map(|e| e.absolute())
            .sum::<T>()
            .scale(one_n);

        e1.scale(self.alphas.0) + e2.scale(self.alphas.1) + e3.scale(self.alphas.2)
    }
}

impl<T> Block for GoodHart<T>
where
    T: Sample,
{
    type Input = (T, T);
    type Output = (T, T);

    fn block(&mut self, input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let input_copy = input.clone();
        let error = input_copy.0;
        let control_signal = input_copy.1;
        self.error.push(error);
        self.control_signal.push(control_signal);

        input
    }

    fn reset(&mut self) {
        self.error.clear();
        self.control_signal.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use alloc::vec;
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
        let gh = GoodHart::<f32>::new(1.0, 1.0, 1.0);
        assert_eq!(gh.value(), 0.0_f32);
    }

    #[test]
    fn f32_weighted_sum_of_components() {
        // data: error=[1,-2,3], u=[1,2,3]; e1=2, e2=0, e3=2
        let mut gh = GoodHart::<f32>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((1.0, 1.0), state);
        gh.block((-2.0, 2.0), state);
        gh.block((3.0, 3.0), state);
        assert!((gh.value() - 4.0_f32).abs() < 1e-6);
    }

    #[test]
    fn f32_uses_alpha_weights() {
        // Same data; alphas (2, 1, 0.5) → 2*2 + 1*0 + 0.5*2 = 5
        let mut gh = GoodHart::<f32>::new(2.0, 1.0, 0.5);
        let state = first_state();
        gh.block((1.0, 1.0), state);
        gh.block((-2.0, 2.0), state);
        gh.block((3.0, 3.0), state);
        assert!((gh.value() - 5.0_f32).abs() < 1e-6);
    }

    #[test]
    fn f32_block_returns_input_unchanged() {
        let mut gh = GoodHart::<f32>::new(1.0, 1.0, 1.0);
        let state = first_state();
        assert_eq!(gh.block((-5.0, 2.0), state), (-5.0_f32, 2.0));
    }

    #[test]
    fn f32_reset_clears_buffers() {
        let mut gh = GoodHart::<f32>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((1.0, 1.0), state);
        gh.block((2.0, 2.0), state);
        gh.reset();
        assert_eq!(gh.value(), 0.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_empty_returns_zero() {
        let gh = GoodHart::<f64>::new(1.0, 1.0, 1.0);
        assert_eq!(gh.value(), 0.0_f64);
    }

    #[test]
    fn f64_weighted_sum_of_components() {
        let mut gh = GoodHart::<f64>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((1.0, 1.0), state);
        gh.block((-2.0, 2.0), state);
        gh.block((3.0, 3.0), state);
        assert!((gh.value() - 4.0_f64).abs() < 1e-12);
    }

    #[test]
    fn f64_reset_clears_buffers() {
        let mut gh = GoodHart::<f64>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((1.0, 1.0), state);
        gh.block((2.0, 2.0), state);
        gh.reset();
        assert_eq!(gh.value(), 0.0_f64);
    }

    // Non-scalar value() tests follow the same pattern as f32/f64:
    // error=[+1,-2,+3], u=[+1,+2,+3] → e1=2, e2=0, e3=2 → result = 4 (with alphas=(1,1,1))
    // Assumes elementwise abs for matrices and Complex { re: norm, im: 0 } for complex.

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_block_accumulates_samples() {
        let mut gh = GoodHart::<Complex<f32>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        gh.block((Complex::new(1.0, 0.5), Complex::new(2.0, 0.0)), state);
        gh.block((Complex::new(-3.0, 1.0), Complex::new(4.0, -1.0)), state);
        assert_eq!(gh.error.len(), 2);
        assert_eq!(gh.control_signal.len(), 2);
        assert_eq!(gh.error[0], Complex::new(1.0, 0.5));
        assert_eq!(gh.control_signal[1], Complex::new(4.0, -1.0));
        gh.reset();
        assert!(gh.error.is_empty());
        assert!(gh.control_signal.is_empty());
    }

    #[test]
    fn complex_f32_value_weighted_sum() {
        let mut gh = GoodHart::<Complex<f32>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)), state);
        gh.block((Complex::new(-2.0, 0.0), Complex::new(2.0, 0.0)), state);
        gh.block((Complex::new(3.0, 0.0), Complex::new(3.0, 0.0)), state);
        let v = gh.value();
        assert!((v.re - 4.0_f32).abs() < 1e-6);
        assert!(v.im.abs() < 1e-6);
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_block_accumulates_samples() {
        let mut gh = GoodHart::<Complex<f64>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        gh.block((Complex::new(0.0, 1.0), Complex::new(2.0, 3.0)), state);
        assert_eq!(gh.error.len(), 1);
        assert_eq!(gh.control_signal.len(), 1);
        gh.reset();
        assert!(gh.error.is_empty());
    }

    #[test]
    fn complex_f64_value_weighted_sum() {
        let mut gh = GoodHart::<Complex<f64>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block((Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)), state);
        gh.block((Complex::new(-2.0, 0.0), Complex::new(2.0, 0.0)), state);
        gh.block((Complex::new(3.0, 0.0), Complex::new(3.0, 0.0)), state);
        let v = gh.value();
        assert!((v.re - 4.0_f64).abs() < 1e-12);
        assert!(v.im.abs() < 1e-12);
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_block_accumulates_samples() {
        let mut gh = GoodHart::<SMatrix<f32, 2, 2>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        let err = SMatrix::<f32, 2, 2>::new(1.0, 2.0, 3.0, 4.0);
        let u = SMatrix::<f32, 2, 2>::new(5.0, 6.0, 7.0, 8.0);
        gh.block((err, u), state);
        gh.block((err, u), state);
        assert_eq!(gh.error.len(), 2);
        assert_eq!(gh.control_signal.len(), 2);
        assert_eq!(gh.error[0], err);
        gh.reset();
        assert!(gh.error.is_empty());
    }

    #[test]
    fn smatrix_f32_value_weighted_sum() {
        let mut gh = GoodHart::<SMatrix<f32, 3, 1>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block(
            (
                SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0),
                SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<f32, 3, 1>::new(-2.0, -4.0, -6.0),
                SMatrix::<f32, 3, 1>::new(2.0, 4.0, 6.0),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<f32, 3, 1>::new(3.0, 6.0, 9.0),
                SMatrix::<f32, 3, 1>::new(3.0, 6.0, 9.0),
            ),
            state,
        );
        let expected = SMatrix::<f32, 3, 1>::new(4.0, 8.0, 12.0);
        let diff = gh.value() - expected;
        assert!(diff.iter().all(|x| x.abs() < 1e-6));
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_block_accumulates_samples() {
        let mut gh = GoodHart::<SMatrix<f64, 3, 1>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        let err = SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0);
        let u = SMatrix::<f64, 3, 1>::new(4.0, 5.0, 6.0);
        gh.block((err, u), state);
        assert_eq!(gh.error.len(), 1);
        assert_eq!(gh.control_signal.len(), 1);
        gh.reset();
        assert!(gh.error.is_empty());
    }

    #[test]
    fn smatrix_f64_value_weighted_sum() {
        let mut gh = GoodHart::<SMatrix<f64, 3, 1>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block(
            (
                SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0),
                SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<f64, 3, 1>::new(-2.0, -4.0, -6.0),
                SMatrix::<f64, 3, 1>::new(2.0, 4.0, 6.0),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<f64, 3, 1>::new(3.0, 6.0, 9.0),
                SMatrix::<f64, 3, 1>::new(3.0, 6.0, 9.0),
            ),
            state,
        );
        let expected = SMatrix::<f64, 3, 1>::new(4.0, 8.0, 12.0);
        let diff = gh.value() - expected;
        assert!(diff.iter().all(|x| x.abs() < 1e-12));
    }

    // ───────────────────────────── SMatrix<Complex<f32>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f32_block_accumulates_samples() {
        let mut gh = GoodHart::<SMatrix<Complex<f32>, 2, 2>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        let err = SMatrix::<Complex<f32>, 2, 2>::new(
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.5),
            Complex::new(3.0, -1.0),
            Complex::new(4.0, 0.0),
        );
        let u = SMatrix::<Complex<f32>, 2, 2>::new(
            Complex::new(5.0, 0.0),
            Complex::new(6.0, 1.0),
            Complex::new(7.0, 0.0),
            Complex::new(8.0, -1.0),
        );
        gh.block((err, u), state);
        gh.block((err, u), state);
        assert_eq!(gh.error.len(), 2);
        assert_eq!(gh.control_signal.len(), 2);
        gh.reset();
        assert!(gh.error.is_empty());
    }

    #[test]
    fn smatrix_complex_f32_value_weighted_sum() {
        let mut gh = GoodHart::<SMatrix<Complex<f32>, 3, 1>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block(
            (
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(1.0, 0.0),
                    Complex::new(2.0, 0.0),
                    Complex::new(3.0, 0.0),
                ),
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(1.0, 0.0),
                    Complex::new(2.0, 0.0),
                    Complex::new(3.0, 0.0),
                ),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(-2.0, 0.0),
                    Complex::new(-4.0, 0.0),
                    Complex::new(-6.0, 0.0),
                ),
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(2.0, 0.0),
                    Complex::new(4.0, 0.0),
                    Complex::new(6.0, 0.0),
                ),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(3.0, 0.0),
                    Complex::new(6.0, 0.0),
                    Complex::new(9.0, 0.0),
                ),
                SMatrix::<Complex<f32>, 3, 1>::new(
                    Complex::new(3.0, 0.0),
                    Complex::new(6.0, 0.0),
                    Complex::new(9.0, 0.0),
                ),
            ),
            state,
        );
        let expected = SMatrix::<Complex<f32>, 3, 1>::new(
            Complex::new(4.0, 0.0),
            Complex::new(8.0, 0.0),
            Complex::new(12.0, 0.0),
        );
        let diff = gh.value() - expected;
        assert!(diff.iter().all(|x| x.re.abs() < 1e-6 && x.im.abs() < 1e-6));
    }

    // ───────────────────────────── SMatrix<Complex<f64>, R, C> ─────────────────────────────

    #[test]
    fn smatrix_complex_f64_block_accumulates_samples() {
        let mut gh = GoodHart::<SMatrix<Complex<f64>, 3, 1>> {
            error: vec![],
            control_signal: vec![],
            alphas: (1.0, 1.0, 1.0),
        };
        let state = first_state();
        let err = SMatrix::<Complex<f64>, 3, 1>::new(
            Complex::new(1.0, 0.5),
            Complex::new(2.0, -0.5),
            Complex::new(3.0, 1.0),
        );
        let u = SMatrix::<Complex<f64>, 3, 1>::new(
            Complex::new(4.0, 0.0),
            Complex::new(5.0, 1.0),
            Complex::new(6.0, -1.0),
        );
        gh.block((err, u), state);
        assert_eq!(gh.error.len(), 1);
        assert_eq!(gh.control_signal.len(), 1);
        gh.reset();
        assert!(gh.error.is_empty());
    }

    #[test]
    fn smatrix_complex_f64_value_weighted_sum() {
        let mut gh = GoodHart::<SMatrix<Complex<f64>, 3, 1>>::new(1.0, 1.0, 1.0);
        let state = first_state();
        gh.block(
            (
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(1.0, 0.0),
                    Complex::new(2.0, 0.0),
                    Complex::new(3.0, 0.0),
                ),
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(1.0, 0.0),
                    Complex::new(2.0, 0.0),
                    Complex::new(3.0, 0.0),
                ),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(-2.0, 0.0),
                    Complex::new(-4.0, 0.0),
                    Complex::new(-6.0, 0.0),
                ),
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(2.0, 0.0),
                    Complex::new(4.0, 0.0),
                    Complex::new(6.0, 0.0),
                ),
            ),
            state,
        );
        gh.block(
            (
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(3.0, 0.0),
                    Complex::new(6.0, 0.0),
                    Complex::new(9.0, 0.0),
                ),
                SMatrix::<Complex<f64>, 3, 1>::new(
                    Complex::new(3.0, 0.0),
                    Complex::new(6.0, 0.0),
                    Complex::new(9.0, 0.0),
                ),
            ),
            state,
        );
        let expected = SMatrix::<Complex<f64>, 3, 1>::new(
            Complex::new(4.0, 0.0),
            Complex::new(8.0, 0.0),
            Complex::new(12.0, 0.0),
        );
        let diff = gh.value() - expected;
        assert!(
            diff.iter()
                .all(|x| x.re.abs() < 1e-12 && x.im.abs() < 1e-12)
        );
    }
}
