use crate::{
    block::Block,
    math::{from_f64::FromF64, scale::Scale},
    prelude::SimulationState,
};
use num_traits::One;

#[derive(Debug, Clone, PartialEq)]
pub struct Ramp<T> {
    value: T,
}

impl<T> Ramp<T> {
    pub fn new(value: T) -> Self {
        Ramp { value }
    }
}

impl<T> Default for Ramp<T>
where
    T: One,
{
    fn default() -> Self {
        Self { value: T::one() }
    }
}

impl<T> Block for Ramp<T>
where
    T: Clone + Scale,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = sim_state.sim_time().as_secs_f64();
        let alpha = <T::Alpha as FromF64>::from_f64(t);
        self.value.clone().scale(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use core::time::Duration;
    use nalgebra::{DMatrix, SMatrix};
    use num_complex::Complex;

    fn state_at(sim_time_s: f64, dt_s: f64) -> SimulationState {
        let mut sim = Simulation::new(dt_s as f32, (sim_time_s + dt_s * 2.0) as f32);
        let initial = sim.next().expect("simulation should yield at least one state");
        let delta = Duration::from_secs_f64(sim_time_s) - initial.sim_time();
        initial + delta
    }

    fn approx_eq_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }
    fn approx_eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn f32_ramp_grows_with_time() {
        let mut ramp = Ramp::new(2.0_f32);
        // At t = 0.5, value = 2 * 0.5 = 1.0
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f32(v, 1.0), "got {v}");
    }

    #[test]
    fn f64_ramp_grows_with_time() {
        let mut ramp = Ramp::new(2.0_f64);
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f64(v, 1.0), "got {v}");
    }

    #[test]
    fn complex_f32_ramp_grows_with_time() {
        let mut ramp = Ramp::new(Complex::new(2.0_f32, 4.0));
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f32(v.re, 1.0), "re: {}", v.re);
        assert!(approx_eq_f32(v.im, 2.0), "im: {}", v.im);
    }

    #[test]
    fn complex_f64_ramp_grows_with_time() {
        let mut ramp = Ramp::new(Complex::new(2.0_f64, 4.0));
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f64(v.re, 1.0), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 2.0), "im: {}", v.im);
    }

    #[test]
    fn dmatrix_f32_ramp_grows_with_time() {
        let value = DMatrix::<f32>::from_row_slice(2, 1, &[2.0, 4.0]);
        let mut ramp = Ramp::new(value);
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f32(v[(0, 0)], 1.0));
        assert!(approx_eq_f32(v[(1, 0)], 2.0));
    }

    #[test]
    fn dmatrix_f64_ramp_grows_with_time() {
        let value = DMatrix::<f64>::from_row_slice(2, 1, &[2.0, 4.0]);
        let mut ramp = Ramp::new(value);
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f64(v[(0, 0)], 1.0));
        assert!(approx_eq_f64(v[(1, 0)], 2.0));
    }

    #[test]
    fn smatrix_f32_ramp_grows_with_time() {
        let value = SMatrix::<f32, 2, 1>::new(2.0, 4.0);
        let mut ramp = Ramp::new(value);
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f32(v[(0, 0)], 1.0));
        assert!(approx_eq_f32(v[(1, 0)], 2.0));
    }

    #[test]
    fn smatrix_f64_ramp_grows_with_time() {
        let value = SMatrix::<f64, 2, 1>::new(2.0, 4.0);
        let mut ramp = Ramp::new(value);
        let v = ramp.block((), state_at(0.5, 0.01));
        assert!(approx_eq_f64(v[(0, 0)], 1.0));
        assert!(approx_eq_f64(v[(1, 0)], 2.0));
    }
}
