use crate::{block::Block, math::sample::Sample, prelude::SimulationState};
use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step<T> {
    value: T,
}

impl<T> Step<T> {
    pub fn new(value: T) -> Self {
        Step { value }
    }
}

impl<T> Default for Step<T>
where
    T: Sample,
{
    fn default() -> Self {
        Step { value: T::one() }
    }
}

impl<T> Display for Step<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Step({})", self.value)
    }
}

impl<T> Block for Step<T>
where
    T: Sample,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        self.value.clone()
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

    #[test]
    fn f32_returns_constant_value() {
        let mut step = Step::new(42.0_f32);
        let state = first_state();
        assert_eq!(step.block((), state), 42.0_f32);
        assert_eq!(step.block((), state), 42.0_f32);
    }

    #[test]
    fn f64_returns_constant_value() {
        let mut step = Step::new(3.14_f64);
        let state = first_state();
        assert_eq!(step.block((), state), 3.14_f64);
        assert_eq!(step.block((), state), 3.14_f64);
    }

    #[test]
    fn complex_f32_returns_constant_value() {
        let v = Complex::new(1.0_f32, 2.0);
        let mut step = Step::new(v);
        let state = first_state();
        assert_eq!(step.block((), state), v);
        assert_eq!(step.block((), state), v);
    }

    #[test]
    fn complex_f64_returns_constant_value() {
        let v = Complex::new(3.0_f64, -4.0);
        let mut step = Step::new(v);
        let state = first_state();
        assert_eq!(step.block((), state), v);
        assert_eq!(step.block((), state), v);
    }

    #[test]
    fn smatrix_f32_returns_constant_value() {
        let v = SMatrix::<f32, 2, 2>::new(1.0, 2.0, 3.0, 4.0);
        let mut step = Step::new(v);
        let state = first_state();
        assert_eq!(step.block((), state), v);
        assert_eq!(step.block((), state), v);
    }

    #[test]
    fn smatrix_f64_returns_constant_value() {
        let v = SMatrix::<f64, 3, 1>::new(1.0, 2.0, 3.0);
        let mut step = Step::new(v);
        let state = first_state();
        assert_eq!(step.block((), state), v);
        assert_eq!(step.block((), state), v);
    }
}
