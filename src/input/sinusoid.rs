use crate::{
    block::Block,
    math::{float_point::FloatPoint, sample::Sample},
    prelude::SimulationState,
};
use core::{f64::consts::PI, time::Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct Sinusoid<T> {
    amplitude: T,
    period: Duration,
    phase: T,
}

impl<T> Sinusoid<T> {
    pub fn new(amplitude: T, period: Duration, phase: T) -> Self {
        Sinusoid {
            amplitude,
            period,
            phase,
        }
    }
}

impl<T> Default for Sinusoid<T>
where
    T: Sample,
{
    fn default() -> Self {
        Self {
            amplitude: T::one(),
            period: Duration::from_secs_f64(2.0 * PI),
            phase: T::zero(),
        }
    }
}

impl<T> Block for Sinusoid<T>
where
    T: Sample,
{
    type Input = ();
    type Output = T;

    fn block(&mut self, _input: Self::Input, sim_state: SimulationState) -> Self::Output {
        let two_pi = <T::Alpha as FloatPoint>::two_pi();
        let t = <T::Alpha as FloatPoint>::from_duration(sim_state.sim_time());
        let omega_t = two_pi * t / <T::Alpha as FloatPoint>::from_duration(self.period);

        T::sinusoid(&self.amplitude, omega_t, &self.phase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Simulation;
    use nalgebra::SMatrix;
    use num_complex::Complex;

    fn state_at(sim_time_s: f64, dt_s: f64) -> SimulationState {
        let mut sim = Simulation::new(dt_s as f32, (sim_time_s + dt_s * 2.0) as f32);
        let initial = sim
            .next()
            .expect("simulation should yield at least one state");
        let delta = Duration::from_secs_f64(sim_time_s) - initial.sim_time();
        initial + delta
    }

    fn approx_eq_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }
    fn approx_eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    // At t = period/4 → omega*t = pi/2 → sin = 1 → output = amplitude

    #[test]
    fn f32_at_quarter_period() {
        let mut s = Sinusoid::new(2.0_f32, Duration::from_secs_f64(1.0), 0.0_f32);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v, 2.0), "got {v}");
    }

    #[test]
    fn f64_at_quarter_period() {
        let mut s = Sinusoid::new(2.0_f64, Duration::from_secs_f64(1.0), 0.0_f64);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v, 2.0), "got {v}");
    }

    #[test]
    fn complex_f32_at_quarter_period() {
        // Real phase: sin(pi/2) = 1, output = (2 + 3i) * 1 = (2 + 3i)
        let amp = Complex::new(2.0_f32, 3.0);
        let mut s = Sinusoid::new(
            amp,
            Duration::from_secs_f64(1.0),
            Complex::new(0.0_f32, 0.0),
        );
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v.re, 2.0), "re: {}", v.re);
        assert!(approx_eq_f32(v.im, 3.0), "im: {}", v.im);
    }

    #[test]
    fn complex_f64_at_quarter_period() {
        let amp = Complex::new(2.0_f64, 3.0);
        let mut s = Sinusoid::new(
            amp,
            Duration::from_secs_f64(1.0),
            Complex::new(0.0_f64, 0.0),
        );
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v.re, 2.0), "re: {}", v.re);
        assert!(approx_eq_f64(v.im, 3.0), "im: {}", v.im);
    }

    #[test]
    fn smatrix_f32_at_quarter_period() {
        let amp = SMatrix::<f32, 2, 1>::new(2.0, 4.0);
        let phase = SMatrix::<f32, 2, 1>::zeros();
        let mut s = Sinusoid::new(amp, Duration::from_secs_f64(1.0), phase);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f32(v[(0, 0)], 2.0));
        assert!(approx_eq_f32(v[(1, 0)], 4.0));
    }

    #[test]
    fn smatrix_f64_at_quarter_period() {
        let amp = SMatrix::<f64, 2, 1>::new(2.0, 4.0);
        let phase = SMatrix::<f64, 2, 1>::zeros();
        let mut s = Sinusoid::new(amp, Duration::from_secs_f64(1.0), phase);
        let v = s.block((), state_at(0.25, 0.01));
        assert!(approx_eq_f64(v[(0, 0)], 2.0));
        assert!(approx_eq_f64(v[(1, 0)], 4.0));
    }
}
