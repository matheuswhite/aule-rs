use crate::{block::Block, math::zeroish::Zeroish, prelude::SimulationState};
use num_traits::Bounded;

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T> {
    value: Option<T>,
    idle: T,
}

impl<T> Impulse<T>
where
    T: Zeroish,
{
    pub fn new(value: T) -> Self {
        let idle = T::zeroish(&value);
        Impulse {
            value: Some(value),
            idle,
        }
    }
}

impl<T> Default for Impulse<T>
where
    T: Bounded + Zeroish,
{
    fn default() -> Self {
        Self::new(T::max_value())
    }
}

impl<T> Block for Impulse<T>
where
    T: Clone,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        match self.value.take() {
            Some(value) => value,
            None => self.idle.clone(),
        }
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
    fn f32_fires_once_then_returns_zero() {
        let mut imp = Impulse::new(42.0_f32);
        let state = first_state();
        assert_eq!(imp.block((), state), 42.0_f32);
        assert_eq!(imp.block((), state), 0.0_f32);
        assert_eq!(imp.block((), state), 0.0_f32);
    }

    #[test]
    fn f32_default_uses_max_value() {
        let mut imp = Impulse::<f32>::default();
        let state = first_state();
        assert_eq!(imp.block((), state), f32::MAX);
        assert_eq!(imp.block((), state), 0.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_fires_once_then_returns_zero() {
        let mut imp = Impulse::new(123.456_f64);
        let state = first_state();
        assert_eq!(imp.block((), state), 123.456_f64);
        assert_eq!(imp.block((), state), 0.0_f64);
    }

    #[test]
    fn f64_default_uses_max_value() {
        let mut imp = Impulse::<f64>::default();
        let state = first_state();
        assert_eq!(imp.block((), state), f64::MAX);
        assert_eq!(imp.block((), state), 0.0_f64);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_fires_once_then_returns_zero() {
        let value = Complex::new(1.0_f32, 2.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), Complex::new(0.0_f32, 0.0));
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_fires_once_then_returns_zero() {
        let value = Complex::new(3.0_f64, -4.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), Complex::new(0.0_f64, 0.0));
    }

    // ───────────────────────────── DMatrix<f32> ─────────────────────────────

    #[test]
    fn dmatrix_f32_fires_once_then_returns_zero() {
        let value = DMatrix::<f32>::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let mut imp = Impulse::new(value.clone());
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), DMatrix::<f32>::zeros(2, 2));
    }

    // ───────────────────────────── DMatrix<f64> ─────────────────────────────

    #[test]
    fn dmatrix_f64_fires_once_then_returns_zero() {
        let value = DMatrix::<f64>::from_row_slice(3, 1, &[1.0, 2.0, 3.0]);
        let mut imp = Impulse::new(value.clone());
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), DMatrix::<f64>::zeros(3, 1));
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_column_vector_fires_once_then_returns_zero() {
        let value = SMatrix::<f32, 3, 1>::new(1.0, 2.0, 3.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), SMatrix::<f32, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f32_square_matrix_fires_once_then_returns_zero() {
        let value = SMatrix::<f32, 2, 2>::new(1.0, 2.0, 3.0, 4.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), SMatrix::<f32, 2, 2>::zeros());
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_column_vector_fires_once_then_returns_zero() {
        let value = SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), SMatrix::<f64, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f64_square_matrix_fires_once_then_returns_zero() {
        let value = SMatrix::<f64, 2, 2>::new(1.0, 2.0, 3.0, 4.0);
        let mut imp = Impulse::new(value);
        let state = first_state();
        assert_eq!(imp.block((), state), value);
        assert_eq!(imp.block((), state), SMatrix::<f64, 2, 2>::zeros());
    }
}
