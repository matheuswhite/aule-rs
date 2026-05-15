use crate::{
    block::Block,
    math::{from_f64::FromF64, scale::Scale},
    prelude::SimulationState,
};
use core::{f32::consts::PI, ops::Add, time::Duration};
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
pub struct Sawtooth<T> {
    amplitude: T,
    period: Duration,
    offset: T,
}

impl<T> Sawtooth<T> {
    pub fn new(amplitude: T, period: Duration, offset: T) -> Self {
        Sawtooth {
            amplitude,
            period,
            offset,
        }
    }
}

impl<T> Default for Sawtooth<T>
where
    T: Zero + One,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f32(2.0 * PI),
            offset: T::zero(),
        }
    }
}

impl<T> Block for Sawtooth<T>
where
    T: Clone + Scale + Add<Output = T>,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let t = sim_state.sim_time().as_secs_f64();
        let period_secs = self.period.as_secs_f64();
        let alpha_f64 = (t % period_secs) / period_secs;
        let alpha = <T::Alpha as FromF64>::from_f64(alpha_f64);

        self.amplitude.clone().scale(alpha) + self.offset.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
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

    // At t = period/4 → alpha = 0.25 → value = amplitude * 0.25 + offset

    #[test]
    fn f32_sawtooth_at_quarter_period() {
        let mut s = Sawtooth::new(4.0_f32, Duration::from_secs_f64(1.0), 0.5_f32);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v, 1.5), "got {v}");
    }

    #[test]
    fn f64_sawtooth_at_quarter_period() {
        let mut s = Sawtooth::new(4.0_f64, Duration::from_secs_f64(1.0), 0.5_f64);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v, 1.5), "got {v}");
    }

    #[test]
    fn complex_f32_sawtooth_at_quarter_period() {
        let mut s = Sawtooth::new(
            Complex::new(4.0_f32, 8.0),
            Duration::from_secs_f64(1.0),
            Complex::new(0.5_f32, 1.0),
        );
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v.re, 1.5), "re: {}", v.re);
        assert!(approx_eq_f32(v.im, 3.0), "im: {}", v.im);
    }

    #[test]
    fn complex_f64_sawtooth_at_quarter_period() {
        let mut s = Sawtooth::new(
            Complex::new(4.0_f64, 8.0),
            Duration::from_secs_f64(1.0),
            Complex::new(0.5_f64, 1.0),
        );
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v.re, 1.5), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 3.0), "im: {}", v.im);
    }

    #[test]
    fn dmatrix_f32_sawtooth_at_quarter_period() {
        let amp = DMatrix::<f32>::from_row_slice(2, 1, &[4.0, 8.0]);
        let off = DMatrix::<f32>::from_row_slice(2, 1, &[0.5, 1.0]);
        let mut s = Sawtooth::new(amp, Duration::from_secs_f64(1.0), off);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v[(0, 0)], 1.5));
        assert!(approx_eq_f32(v[(1, 0)], 3.0));
    }

    #[test]
    fn dmatrix_f64_sawtooth_at_quarter_period() {
        let amp = DMatrix::<f64>::from_row_slice(2, 1, &[4.0, 8.0]);
        let off = DMatrix::<f64>::from_row_slice(2, 1, &[0.5, 1.0]);
        let mut s = Sawtooth::new(amp, Duration::from_secs_f64(1.0), off);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v[(0, 0)], 1.5));
        assert!(approx_eq_f64(v[(1, 0)], 3.0));
    }

    #[test]
    fn smatrix_f32_sawtooth_at_quarter_period() {
        let amp = SMatrix::<f32, 2, 1>::new(4.0, 8.0);
        let off = SMatrix::<f32, 2, 1>::new(0.5, 1.0);
        let mut s = Sawtooth::new(amp, Duration::from_secs_f64(1.0), off);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v[(0, 0)], 1.5));
        assert!(approx_eq_f32(v[(1, 0)], 3.0));
    }

    #[test]
    fn smatrix_f64_sawtooth_at_quarter_period() {
        let amp = SMatrix::<f64, 2, 1>::new(4.0, 8.0);
        let off = SMatrix::<f64, 2, 1>::new(0.5, 1.0);
        let mut s = Sawtooth::new(amp, Duration::from_secs_f64(1.0), off);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v[(0, 0)], 1.5));
        assert!(approx_eq_f64(v[(1, 0)], 3.0));
    }
}
