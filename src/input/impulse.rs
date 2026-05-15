use crate::{block::Block, prelude::SimulationState};
use num_traits::{Bounded, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Impulse<T> {
    value: Option<T>,
}

impl<T> Impulse<T> {
    pub fn new(value: T) -> Self {
        Impulse { value: Some(value) }
    }
}

impl<T> Default for Impulse<T>
where
    T: Bounded,
{
    fn default() -> Self {
        Self {
            value: Some(T::max_value()),
        }
    }
}

impl<T> Block for Impulse<T>
where
    T: Zero,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, _sim_state: SimulationState) -> Self::Output {
        let Some(value) = self.value.take() else {
            return T::zero();
        };

        self.value = None;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
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
}
