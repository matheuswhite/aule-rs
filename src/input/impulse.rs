use crate::{block::Block, math::sample::Sample, prelude::SimulationState};

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T> {
    value: Option<T>,
    idle: T,
}

impl<T> Impulse<T>
where
    T: Sample,
{
    pub fn new() -> Self {
        let idle = T::zero();
        Impulse {
            value: Some(T::max_real()),
            idle,
        }
    }
}

impl<T> Block for Impulse<T>
where
    T: Sample,
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
    use nalgebra::SMatrix;
    use num_complex::Complex;

    fn first_state() -> SimulationState {
        Simulation::new(1e-3, 1.0)
            .next()
            .expect("simulation should yield at least one state")
    }

    // ───────────────────────────── f32 ─────────────────────────────

    #[test]
    fn f32_default_uses_max_value() {
        let mut imp = Impulse::<f32>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), f32::MAX);
        assert_eq!(imp.block((), state), 0.0_f32);
    }

    // ───────────────────────────── f64 ─────────────────────────────

    #[test]
    fn f64_default_uses_max_value() {
        let mut imp = Impulse::<f64>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), f64::MAX);
        assert_eq!(imp.block((), state), 0.0_f64);
    }

    // ───────────────────────────── Complex<f32> (c32) ─────────────────────────────

    #[test]
    fn complex_f32_fires_once_then_returns_zero() {
        let mut imp = Impulse::<Complex<f32>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), Complex::<f32>::max_real());
        assert_eq!(imp.block((), state), Complex::<f32>::zero());
    }

    // ───────────────────────────── Complex<f64> (c64) ─────────────────────────────

    #[test]
    fn complex_f64_fires_once_then_returns_zero() {
        let mut imp = Impulse::<Complex<f64>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), Complex::<f64>::max_real());
        assert_eq!(imp.block((), state), Complex::<f64>::zero());
    }

    // ───────────────────────────── SMatrix<f32, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f32_column_vector_fires_once_then_returns_zero() {
        let mut imp = Impulse::<SMatrix<f32, 3, 1>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), SMatrix::<f32, 3, 1>::max_real());
        assert_eq!(imp.block((), state), SMatrix::<f32, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f32_square_matrix_fires_once_then_returns_zero() {
        let mut imp = Impulse::<SMatrix<f32, 2, 2>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), SMatrix::<f32, 2, 2>::max_real());
        assert_eq!(imp.block((), state), SMatrix::<f32, 2, 2>::zeros());
    }

    // ───────────────────────────── SMatrix<f64, R, C> ─────────────────────────────

    #[test]
    fn smatrix_f64_column_vector_fires_once_then_returns_zero() {
        let mut imp = Impulse::<SMatrix<f64, 3, 1>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), SMatrix::<f64, 3, 1>::max_real());
        assert_eq!(imp.block((), state), SMatrix::<f64, 3, 1>::zeros());
    }

    #[test]
    fn smatrix_f64_square_matrix_fires_once_then_returns_zero() {
        let mut imp = Impulse::<SMatrix<f64, 2, 2>>::new();
        let state = first_state();
        assert_eq!(imp.block((), state), SMatrix::<f64, 2, 2>::max_real());
        assert_eq!(imp.block((), state), SMatrix::<f64, 2, 2>::zeros());
    }
}
